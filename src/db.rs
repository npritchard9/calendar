use libsql_client::{
    args, client::GenericClient, new_client_from_config, Config, DatabaseClient, Statement,
};
use std::env;

use crate::models::Event;
pub async fn get_db() -> Result<GenericClient, anyhow::Error> {
    dotenvy::dotenv()?;
    let db_url = env::var("DB_URL").expect("DB_URL needs to exist");
    let db_token = env::var("DB_TOKEN").expect("DB_TOKEN needs to exist");
    let client = new_client_from_config(Config {
        url: db_url.as_str().try_into()?,
        auth_token: Some(String::from(db_token)),
    })
    .await?;
    Ok(client)
}

pub async fn write_to_db(db: &GenericClient, e: Event) {
    db.execute(Statement::with_args(
        "insert into calendar values (?, ?, ?, ?, ?)",
        args!(
            e.title,
            e.desc,
            e.date.to_string(),
            e.prio,
            e.id.to_string()
        ),
    ))
    .await
    .expect("The insert to work");
}
