use std::{io::stdin, process};

pub mod db;
pub mod models;

use chrono::NaiveDate;
use libsql_client::client::GenericClient;

use crate::{
    db::{add_event, get_db, get_events, get_events_by_day},
    models::*,
};

async fn create_event(db: &GenericClient) {
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
    add_event(&db, e).await.unwrap();
}

async fn get_day(db: &GenericClient) {
    println!("Enter a date (m/d/y):");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let date = input
        .trim()
        .split("/")
        .map(|t| t.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let d = NaiveDate::from_ymd_opt(date[2] as i32, date[0], date[1])
        .expect("The date needs to be correct");
    let events = get_events_by_day(&db, d.to_string()).await.unwrap();
    if events.is_empty() {
        println!("\nNo events to display.\n")
    } else {
        println!();
        for e in events {
            println!("{e}")
        }
    }
}

async fn event_loop(db: &GenericClient) {
    println!("Welcome, what do you need to do today?");
    println!("1. Add an event");
    println!("2. See events for a given day");
    println!("3. See all events");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>().unwrap();
    match input {
        1 => create_event(&db).await,
        2 => get_day(&db).await,
        3 => {
            let events = get_events(&db).await.unwrap();
            if events.is_empty() {
                println!("\nNo events to display.\n")
            } else {
                println!();
                for e in events {
                    println!("{e}")
                }
            }
        }
        _ => {
            println!("You did not enter a valid option. Quitting the program.");
            process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() {
    let db = get_db().await.unwrap();
    loop {
        event_loop(&db).await;
    }
}
