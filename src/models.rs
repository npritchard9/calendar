use chrono::{DateTime, FixedOffset, TimeZone};
use std::{collections::HashMap, fmt::Display};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Calendar {
    pub days: HashMap<DateTime<FixedOffset>, Vec<Event>>,
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
            writeln!(f, "Date: {}", k.date_naive()).unwrap();
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
    pub date: DateTime<FixedOffset>,
    pub prio: u8,
    pub id: Uuid,
}

impl Event {
    pub fn new(title: String, desc: String, date: Vec<u32>, time: Vec<u32>, prio: u8) -> Self {
        let hour = 3600;
        let d = FixedOffset::west_opt(4 * hour)
            .unwrap()
            .with_ymd_and_hms(date[2] as i32, date[0], date[1], time[0], time[1], 0)
            .unwrap();
        Event {
            title,
            desc,
            date: d,
            prio,
            id: Uuid::new_v4(),
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.title).unwrap();
        writeln!(f, "{}", self.date.time())
    }
}
