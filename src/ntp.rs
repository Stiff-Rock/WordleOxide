use chrono::{DateTime, Local, NaiveDate};
use rsntp::SntpClient;
use std::error::Error;

pub fn get_date_native() -> Result<NaiveDate, Box<dyn Error>> {
    let client = SntpClient::new();
    let result = client.synchronize("pool.ntp.org").unwrap();

    let local_time: DateTime<Local> =
        DateTime::from(result.datetime().into_chrono_datetime().unwrap());

    Ok(local_time.date_naive())
}

pub fn get_formatted_date() -> Result<String, Box<dyn Error>> {
    let client = SntpClient::new();
    let result = client.synchronize("pool.ntp.org").unwrap();

    let local_time: DateTime<Local> =
        DateTime::from(result.datetime().into_chrono_datetime().unwrap());

    let formatted_date = local_time.date_naive().format("%Y-%m-%d").to_string();

    Ok(formatted_date.to_string())
}
