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

pub async fn add_event(db: &GenericClient, e: Event) -> Result<(), anyhow::Error> {
    db.execute(Statement::with_args(
        "insert into calendar values (?, ?, ?, ?, ?, ?)",
        args!(
            e.title,
            e.desc,
            e.prio,
            e.date.to_string(),
            e.time.to_string(),
            e.id.to_string()
        ),
    ))
    .await
    .expect("The insert to work");
    Ok(())
}

pub async fn get_events(db: &GenericClient) -> Result<(), anyhow::Error> {
    let rs = db
        .execute("select * from calendar")
        .await
        .expect("The insert to work");
    let events = rs
        .rows
        .iter()
        .map(|r| r.values.clone().into())
        .collect::<Vec<Event>>();
    println!("events: {events:#?}");
    Ok(())
}

pub async fn get_events_by_day(db: &GenericClient, day: String) -> Result<(), anyhow::Error> {
    let rs = db
        .execute(Statement::with_args(
            "select * from calendar where date = ? order by time",
            args!(day),
        ))
        .await
        .expect("The insert to work");
    println!("{:#?}", rs.rows);
    Ok(())
}
