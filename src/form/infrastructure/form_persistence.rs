use crate::database::connection::database_connection;
use crate::form::handlers::domain_for_user_input::raw_form::RawForm;
use crate::form::handlers::domain_for_user_input::raw_form_id::RawFormId;
use diesel::sql_types::{Integer, VarChar};
use diesel::{sql_query, QueryResult, RunQueryDsl};

/// formを生成する
pub fn create_form(_form: RawForm) -> QueryResult<usize> {
    let mut connection = database_connection();
    sql_query("INSERT INTO seichi_portal.forms (name) VALUES (?)")
        .bind::<VarChar, _>(_form.form_name().name())
        .execute(&mut connection)
}

/// formを削除する
pub fn delete_form(_form_id: RawFormId) -> QueryResult<usize> {
    let mut connection = database_connection();
    sql_query("DELETE FROM seichi_portal.forms WHERE id = ?")
        .bind::<Integer, _>(_form_id.form_id())
        .execute(&mut connection)
}
