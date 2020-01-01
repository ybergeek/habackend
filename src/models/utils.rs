extern crate chrono_tz;
extern crate diesel;

use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};

pub struct TimeRange {
    pub before: DateTime<Utc>,
    pub after: DateTime<Utc>,
}



pub fn halfourago(date_time: DateTime<Utc>) -> TimeRange {
    let dt = date_time - Duration::minutes(30);
    afterbefore(dt)
}

pub fn oneday(date_time: DateTime<Utc>) -> TimeRange {
    let dt = date_time - Duration::days(1);
    afterbefore(dt)
}
pub fn onedayavg(date_time: DateTime<Utc>) -> TimeRange {
    TimeRange {
        before: date_time,
        after: date_time - Duration::days(1),
    }
}
pub fn oneweek(date_time: DateTime<Utc>) -> TimeRange {
    TimeRange {
        before: date_time,
        after: date_time - Duration::days(7),
    }
}


pub fn onemonth(date_time: DateTime<Utc>) -> TimeRange {
    TimeRange {
        before: date_time,
        after: date_time - Duration::days(30),
    }
}

pub fn oneyear(date_time: DateTime<Utc>) -> TimeRange {
    let dt = Utc
        .ymd(date_time.year() - 1, date_time.month(), date_time.day())
        .and_hms(date_time.hour(), date_time.minute(), date_time.second());
    afterbefore(dt)
}

pub fn twoyear(date_time: DateTime<Utc>) -> TimeRange {
    let dt = Utc
        .ymd(date_time.year() - 2, date_time.month(), date_time.day())
        .and_hms(date_time.hour(), date_time.minute(), date_time.second());
    afterbefore(dt)
}

pub fn threeyear(date_time: DateTime<Utc>) -> TimeRange {
    let dt = Utc
        .ymd(date_time.year() - 3, date_time.month(), date_time.day())
        .and_hms(date_time.hour(), date_time.minute(), date_time.second());
    afterbefore(dt)
}
fn afterbefore(dt: DateTime<Utc>) -> TimeRange {
    TimeRange {
        before: dt - Duration::minutes(1),
        after: dt - Duration::minutes(5),
    }
}
