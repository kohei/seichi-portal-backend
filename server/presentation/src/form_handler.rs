use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum::extract::Query;
use domain::{form::models::FormTitle, repository::Repositories};
use resource::repository::RealInfrastructureRepository;
use serde_json::json;
use usecase::form::FormUseCase;

pub async fn create_form_handler(
    State(repository): State<RealInfrastructureRepository>,
    Json(form_title): Json<FormTitle>,
) -> impl IntoResponse {
    let form_use_case = FormUseCase {
        ctx: repository.form_repository(),
    };
    match form_use_case.create_form(form_title).await {
        Ok(id) => (StatusCode::CREATED, json!({ "id": id }).to_string()),
        Err(err) => {
            tracing::error!("{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "".to_owned())
        }
    }
}

pub async fn form_list_handler(
    State(repository): State<RealInfrastructureRepository>,
    Query(offset): Query<i32>,
    Query(limit): Query<i32>,
) -> impl IntoResponse {
    let form_use_case = FormUseCase {
        ctx: repository.form_repository(),
    };

    match form_use_case.form_list(offset, limit).await {
        Ok(forms) => (StatusCode::OK, json!(forms).to_string()),
        Err(err) => {
            tracing::error!("{}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "".to_owned())
        }
    }
}
