use async_trait::async_trait;
use domain::form::models::{
    Form, FormId, FormTitle,
};
use entities::{
    form_meta_data,
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ActiveValue::Set, EntityTrait,
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

    async fn list(&self, _offset: i32, _limit: i32) -> anyhow::Result<Vec<Form>> {
        todo!()
        // let forms = FormMetaData::find()
        //     .order_by_asc(form_meta_data::Column::Id)
        //     .offset(offset as u64)
        //     .limit(limit as u64)
        //     .all(&self.pool)
        //     .await?;
        //
        // let form_ids = forms.iter().map(|form| form.id).collect_vec();
        //
        // let all_questions = FormQuestions::find()
        //     .filter(Expr::col(form_questions::Column::FormId).is_in(form_ids))
        //     .all(&self.pool)
        //     .await?;
        //
        // let question_ids = all_questions
        //     .iter()
        //     .map(|question| question.question_id)
        //     .collect_vec();
        //
        // let all_choices = FormChoices::find()
        //     .filter(Expr::col(form_choices::Column::QuestionId).is_in(question_ids))
        //     .all(&self.pool)
        //     .await?;
        //
        // forms
        //     .into_iter()
        //     .map(|form| {
        //         let questions = all_questions
        //             .iter()
        //             .filter(|question| question.form_id == form.id)
        //             .map(|question| {
        //                 let choices = all_choices
        //                     .iter()
        //                     .filter(|choice| choice.question_id == question.question_id)
        //                     .cloned()
        //                     .map(|choice| choice.choice)
        //                     .collect_vec();
        //
        //                 anyhow::Ok(
        //                     Question::builder()
        //                         .title(question.title.to_owned())
        //                         .description(question.description.to_owned())
        //                         .question_type(question.question_type.to_string().try_into()?)
        //                         .choices(choices)
        //                         .build(),
        //                 )
        //             })
        //             .collect::<Result<Vec<_>, _>>()?;
        //
        //         anyhow::Ok(
        //             Form::builder()
        //                 .id(FormId(form.id))
        //                 .title(FormTitle::builder().title(form.title).build())
        //                 .questions(questions)
        //                 .metadata(
        //                     FormMeta::builder()
        //                         .created_at(form.created_at)
        //                         .update_at(form.updated_at)
        //                         .build(),
        //                 )
        //                 .settings(
        //                     FormSettings::builder()
        //                         .response_period(
        //                             ResponsePeriod::builder().start_at().end_at().build(),
        //                         )
        //                         .build(),
        //                 )
        //                 .build(),
        //         )
        //     })
        //     .collect()
    }
}
