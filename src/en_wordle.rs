use reqwest::blocking::get;

use crate::ntp::get_formatted_date;

const NYT_API: &str = "https://www.nytimes.com/svc/wordle/v2/";

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct NytApiResponse {
    solution: String,
}

pub fn get_daily_word() -> Result<String, String> {
    let date = get_formatted_date().map_err(|e| e.to_string())?;

    let request_url = format!("{NYT_API}{date}.json");
    let response: NytApiResponse = get(request_url)
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;

    Ok(response.solution.to_uppercase())
}
