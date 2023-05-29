use std::io::stdin;

pub mod db;
pub mod models;

use chrono::NaiveDate;

use crate::{
    db::{add_event, get_db, get_events, get_events_by_day},
    models::*,
};

#[tokio::main]
async fn main() {
    // let mut c = Calendar::new();
    let db = get_db().await.unwrap();

    println!("Enter a title:");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    title = title.trim().to_string();

    println!("Enter a description:");
    let mut desc = String::new();
    stdin().read_line(&mut desc).unwrap();
    desc = desc.trim().to_string();

    println!("Enter the date (m/d/y):");
    let mut date = String::new();
    stdin().read_line(&mut date).unwrap();
    let date = date
        .trim()
        .split("/")
        .map(|t| t.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("Enter the time (h:m):");
    let mut time = String::new();
    stdin().read_line(&mut time).unwrap();
    let time = time
        .trim()
        .split(":")
        .map(|t| t.parse().unwrap())
        .collect::<Vec<u32>>();

    println!("Enter a priority (0 - 10):");
    let mut prio = String::new();
    stdin().read_line(&mut prio).unwrap();
    let prio: u8 = prio.trim().parse().unwrap();

    let e = Event::new(title, desc, date, time, prio);
    add_event(&db, e.clone()).await;
    // c.add(e);
    // get_events(&db).await;
    let bday = NaiveDate::from_ymd_opt(2023, 6, 12).unwrap();
    get_events_by_day(&db, bday.to_string()).await;
}
