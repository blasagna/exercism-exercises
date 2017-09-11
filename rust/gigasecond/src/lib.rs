extern crate chrono;
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigasecond: Duration = Duration::seconds(1E9 as i64);
    start + gigasecond
}
