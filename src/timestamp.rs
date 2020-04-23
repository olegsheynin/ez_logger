extern crate strfmt;
use strfmt::strfmt;

use chrono::{Timelike, Local};
use std::time::{SystemTime, UNIX_EPOCH};


pub fn tstamp() -> String {
    return timestamp_fmt(None);
}

pub fn timestamp_fmt(frmt: Option<&String>) -> String {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let dt = Local::now();
    if frmt.is_none() {
        // Default
        return format!("{hours:02}:{minutes:02}:{seconds:02}.{micros:06}"
        , hours=dt.hour()
        , minutes=dt.minute()
        , seconds=dt.second()
        , micros=since_epoch.as_micros() % 1_000_000);
    }

    use std::collections::HashMap;
    let mut vars = HashMap::new();
    vars.insert("hours".to_string(), format!("{:02}", dt.hour()));
    vars.insert("minutes".to_string(), format!("{:02}", dt.minute()));
    vars.insert("seconds".to_string(), format!("{:02}", dt.second()));
    vars.insert("micros".to_string(), format!("{:06}", since_epoch.as_micros() % 1_000_000));

    let fmt = frmt.unwrap().to_string();
    return strfmt(&fmt, &vars).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn timestamp_test() {
        let ts = crate::timestamp::tstamp();
        assert_eq!(ts.len(), 15);
    }

}
