use chrono::{NaiveDate, NaiveTime};
use libsql_client::Value;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Event {
    pub title: String,
    pub desc: String,
    pub prio: u8,
    pub date: NaiveDate,
    pub time: NaiveTime,
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
            prio,
            date: d,
            time: t,
            id: Uuid::new_v4(),
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} • {} • {}", self.title, self.date, self.time)
    }
}

impl Into<Event> for Vec<Value> {
    fn into(self) -> Event {
        let mut title = String::new();
        if let Value::Text { value } = self[0].clone() {
            title = value;
        }
        let mut desc = String::new();
        if let Value::Text { value } = self[1].clone() {
            desc = value;
        }
        let mut prio: u8 = 0;
        if let Value::Integer { value } = self[2].clone() {
            prio = value as u8;
        }
        let mut date = String::new();
        if let Value::Text { value } = self[3].clone() {
            date = value;
        }
        let mut time = String::new();
        if let Value::Text { value } = self[4].clone() {
            time = value;
        }
        let mut id = String::new();
        if let Value::Text { value } = self[5].clone() {
            id = value;
        }

        Event {
            title,
            desc,
            prio,
            date: NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d").unwrap(),
            time: NaiveTime::parse_from_str(time.as_str(), "%H:%M:%S").unwrap(),
            id: Uuid::parse_str(id.as_str()).unwrap(),
        }
    }
}
