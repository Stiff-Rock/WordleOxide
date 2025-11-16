use chrono::NaiveDate;
use reqwest::blocking::get;
use rmp_serde::from_slice;
use serde::Deserialize;
use std::collections::HashSet;

use crate::ntp::get_date_native;

const XOR_KEY: &str = "marissa-peral-morchito";

const BIN_SOLUTIONS_URL: &str = "https://lapalabradeldia.com/solutions/normal.bin";
const BIN_DICTIONARY_URL: &str = "https://lapalabradeldia.com/words/5.bin";

#[derive(Debug, Deserialize)]
struct Palabra {
    solution: String,
}

pub fn get_daily_word() -> Result<(String, usize), String> {
    let bin_data = get(BIN_SOLUTIONS_URL)
        .map_err(|e| e.to_string())?
        .bytes()
        .map_err(|e| e.to_string())?
        .to_vec();

    let key_bytes = XOR_KEY.as_bytes();

    let decrypted_bytes: Vec<u8> = bin_data
        .iter()
        .zip(key_bytes.iter().cycle())
        .map(|(&data_byte, &key_byte)| data_byte ^ key_byte)
        .collect();

    let master_list: Vec<Palabra> = from_slice(&decrypted_bytes).map_err(|e| e.to_string())?;

    let start_date = NaiveDate::from_ymd_opt(2022, 1, 7).unwrap();
    let today = get_date_native().map_err(|e| e.to_string())?;

    let days_passed = today.signed_duration_since(start_date).num_days();

    let index = (days_passed as usize) % master_list.len();

    let palabra_del_dia = &master_list[index];

    let wordle = palabra_del_dia.solution.to_uppercase();

    Ok((wordle, index))
}

pub fn get_word_dictionary() -> Result<HashSet<String>, String> {
    let bin_data = get(BIN_DICTIONARY_URL)
        .map_err(|e| e.to_string())?
        .bytes()
        .map_err(|e| e.to_string())?
        .to_vec();

    let master_list: Vec<String> =
        from_slice(&bin_data).map_err(|e| format!("Error deserializando MessagePack: {}", e))?;

    Ok(master_list.into_iter().collect())
}
