use anyhow::anyhow;
use async_trait::async_trait;
use domain::form::models::{
    Form, FormDescription, FormId, FormMeta, FormSettings, FormTitle, Question,
};
use entities::{
    form_choices, form_meta_data, form_questions,
    prelude::{FormChoices, FormMetaData, FormQuestions},
    response_period,
};
use errors::presentation::PresentationError::FormNotFound;
use futures::{stream, stream::StreamExt};
use itertools::Itertools;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ActiveValue, ActiveValue::Set, EntityTrait, ModelTrait,
    QueryFilter, QueryOrder, QuerySelect,
};

use crate::database::{components::FormDatabase, connection::ConnectionPool};

#[async_trait]
impl FormDatabase for ConnectionPool {
    async fn create(&self, title: FormTitle) -> anyhow::Result<FormId> {
        let form_id = form_meta_data::ActiveModel {
            id: ActiveValue::NotSet,
            title: Set(title.title().to_owned()),
            description: Set(None),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
        .insert(&self.pool)
        .await?
        .id;

        Ok(form_id.into())
    }

    async fn list(&self, offset: i32, limit: i32) -> anyhow::Result<Vec<Form>> {
        let forms = FormMetaData::find()
            .order_by_asc(form_meta_data::Column::Id)
            .offset(offset as u64)
            .limit(limit as u64)
            .all(&self.pool)
            .await?;

        let form_ids = forms.iter().map(|form| form.id).collect_vec();

        let all_questions = FormQuestions::find()
            .filter(Expr::col(form_questions::Column::FormId).is_in(form_ids.to_owned()))
            .all(&self.pool)
            .await?;

        let question_ids = all_questions
            .iter()
            .map(|question| question.question_id)
            .collect_vec();

        let all_choices = FormChoices::find()
            .filter(Expr::col(form_choices::Column::QuestionId).is_in(question_ids))
            .all(&self.pool)
            .await?;

        let all_periods = entities::response_period::Entity::find()
            .filter(Expr::col(entities::response_period::Column::FormId).is_in(form_ids.to_owned()))
            .all(&self.pool)
            .await?;

        forms
            .into_iter()
            .map(|form| {
                let questions = all_questions
                    .iter()
                    .filter(|question| question.form_id == form.id)
                    .map(|question| {
                        let choices = all_choices
                            .iter()
                            .filter(|choice| choice.question_id == question.question_id)
                            .cloned()
                            .map(|choice| choice.choice)
                            .collect_vec();

                        anyhow::Ok(
                            Question::builder()
                                .title(question.title.to_owned())
                                .description(question.description.to_owned())
                                .question_type(question.question_type.to_string().try_into()?)
                                .choices(choices)
                                .build(),
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let response_period = all_periods
                    .iter()
                    .filter(|period| period.form_id == form.id)
                    .map(|period| {
                        domain::form::models::ResponsePeriod::builder()
                            .start_at(period.start_at.and_utc())
                            .end_at(period.end_at.and_utc())
                            .build()
                    })
                    .next();

                anyhow::Ok(
                    Form::builder()
                        .id(FormId(form.id))
                        .title(FormTitle::builder().title(form.title).build())
                        .description(
                            FormDescription::builder()
                                .description(form.description)
                                .build(),
                        )
                        .questions(questions)
                        .metadata(
                            FormMeta::builder()
                                .created_at(form.created_at)
                                .update_at(form.updated_at)
                                .build(),
                        )
                        .settings(
                            FormSettings::builder()
                                .response_period(response_period)
                                .build(),
                        )
                        .build(),
                )
            })
            .collect()
    }

    async fn get(&self, form_id: FormId) -> anyhow::Result<Form> {
        let target_form = FormMetaData::find()
            .filter(Expr::col(form_meta_data::Column::Id).eq(form_id.0))
            .all(&self.pool)
            .await?
            .first()
            .ok_or(anyhow!("Form not found"))?
            .to_owned();

        let form_questions = stream::iter(
            FormQuestions::find()
                .filter(Expr::col(form_questions::Column::FormId).eq(form_id.0))
                .all(&self.pool)
                .await?,
        )
        .then(|question| async {
            let choices = FormChoices::find()
                .filter(
                    Expr::col(form_choices::Column::QuestionId).eq(question.to_owned().question_id),
                )
                .all(&self.pool)
                .await?
                .into_iter()
                .map(|choice| choice.choice)
                .collect_vec();

            Ok(Question::builder()
                .title(question.title)
                .description(question.description)
                .question_type(question.question_type.to_string().try_into()?)
                .choices(choices)
                .build())
        })
        .collect::<Vec<anyhow::Result<Question>>>()
        .await
        .into_iter()
        .collect::<anyhow::Result<Vec<Question>>>()?;

        let response_period = entities::response_period::Entity::find()
            .filter(Expr::col(entities::response_period::Column::FormId).eq(target_form.id))
            .all(&self.pool)
            .await?
            .first()
            .map(|period| {
                domain::form::models::ResponsePeriod::builder()
                    .start_at(period.start_at.to_owned().and_utc())
                    .end_at(period.end_at.to_owned().and_utc())
                    .build()
            });

        let form_settings = FormSettings::builder()
            .response_period(response_period)
            .build();

        Ok(Form::builder()
            .id(FormId(target_form.id.to_owned()))
            .title(
                FormTitle::builder()
                    .title(target_form.title.to_owned())
                    .build(),
            )
            .description(
                FormDescription::builder()
                    .description(target_form.description.to_owned())
                    .build(),
            )
            .questions(form_questions)
            .metadata(
                FormMeta::builder()
                    .created_at(target_form.created_at)
                    .update_at(target_form.updated_at)
                    .build(),
            )
            .settings(form_settings)
            .build())
    }

    async fn delete(&self, form_id: FormId) -> anyhow::Result<FormId> {
        let target_form = FormMetaData::find_by_id(form_id.0)
            .all(&self.pool)
            .await?
            .first()
            .ok_or(FormNotFound)?
            .to_owned();

        let question_ids = FormQuestions::find()
            .filter(Expr::col(form_questions::Column::FormId).eq(form_id.0))
            .all(&self.pool)
            .await?
            .iter()
            .map(|question| question.question_id)
            .collect_vec();

        FormChoices::delete_many()
            .filter(Expr::col(form_choices::Column::QuestionId).is_in(question_ids))
            .exec(&self.pool)
            .await?;

        response_period::Entity::delete_many()
            .filter(Expr::col(response_period::Column::FormId).eq(form_id.0))
            .exec(&self.pool)
            .await?;

        FormQuestions::delete_many()
            .filter(Expr::col(form_questions::Column::FormId).eq(form_id.0))
            .exec(&self.pool)
            .await?;

        target_form.delete(&self.pool).await?;

        Ok(form_id)
    }
}
