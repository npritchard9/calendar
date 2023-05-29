use chrono::{NaiveDate, NaiveTime};
use std::{collections::HashMap, fmt::Display};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Calendar {
    pub days: HashMap<NaiveDate, Vec<Event>>,
}

impl Calendar {
    pub fn new() -> Self {
        Calendar {
            days: HashMap::new(),
        }
    }

    pub fn add(&mut self, e: Event) {
        self.days
            .entry(e.date)
            .and_modify(|day| day.push(e.clone()))
            .or_insert(vec![e.clone()]);
    }
}

impl Display for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in self.days.iter() {
            writeln!(f, "Date: {}", k).unwrap();
            for e in v {
                write!(f, "{e}").unwrap();
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub title: String,
    pub desc: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub prio: u8,
    pub id: Uuid,
}

impl Event {
    pub fn new(title: String, desc: String, date: Vec<u32>, time: Vec<u32>, prio: u8) -> Self {
        let d = NaiveDate::from_ymd_opt(date[2] as i32, date[0], date[1])
            .expect("The date needs to be correct");
        let t = NaiveTime::from_hms_opt(time[0], time[1], 0).expect("The time needs to be correct");
        Event {
            title,
            desc,
            date: d,
            time: t,
            prio,
            id: Uuid::new_v4(),
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.title).unwrap();
        writeln!(f, "{}", self.date)
    }
}
