use std::io::stdin;

pub mod db;
pub mod models;

use crate::{
    db::{add_event, get_db, get_events, get_events_by_day},
    models::*,
};

fn get_input() -> Event {
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

    Event::new(title, desc, date, time, prio)
}

#[tokio::main]
async fn main() {
    let db = get_db().await.unwrap();
    // let e = get_input();
    // add_event(&db, e.clone()).await;
    get_events(&db).await.unwrap();
}
