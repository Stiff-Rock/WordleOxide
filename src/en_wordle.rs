use std::collections::HashSet;

use reqwest::blocking::get;

use crate::ntp::get_formatted_date;

const NYT_API: &str = "https://www.nytimes.com/svc/wordle/v2/";
const NYT_DICT_URL: &str = "https://www.nytimes.com/games-assets/v2/7196.2448555c603aae8fb1d1.js";

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

pub fn get_word_dictionary() -> Result<HashSet<String>, String> {
    let js_script = get(NYT_DICT_URL).unwrap().text().unwrap();

    let list_declaration = "const s=[";
    let list_close = "]}";

    let list_start = js_script
        .find(list_declaration)
        .expect("Unable to find start of dictionary list on NYT Wordle")
        + list_declaration.len();

    let list_start_sub = &js_script[list_start..];

    let list_end = list_start_sub
        .find(list_close)
        .expect("Unable to find end of dictionary list on NYT Wordle");

    let dictionary_list = &list_start_sub[0..list_end];

    // TODO: THE DICTIONARY IS NOT WORKING
    let word_set = dictionary_list
        .trim()
        .split("\",\"")
        .map(|s| s.trim_matches('"'))
        .map(|s| s.to_string())
        .collect();

    Ok(word_set)
}
