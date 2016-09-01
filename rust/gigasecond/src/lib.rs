extern crate chrono;
use chrono::{DateTime, Duration, UTC};

pub fn after(date: DateTime<UTC>) -> DateTime<UTC> {
    let gigaseconds = Duration::seconds(10i64.pow(9));
    date + gigaseconds
}