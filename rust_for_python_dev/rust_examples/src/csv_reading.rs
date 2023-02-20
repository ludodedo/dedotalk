#![allow(dead_code)]
use std::ops::Range;

use chrono::prelude::*;
use serde::{de::Unexpected, Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct GameStruct {
    appid: u64,
    name: String,
    release_date: NaiveDate,
    #[serde(deserialize_with = "bool_from_string")]
    english: bool,
    developer: String,
    publisher: String,
    #[serde(deserialize_with = "string_with_semicolon")]
    platforms: Vec<String>,
    required_age: u8,
    #[serde(deserialize_with = "string_with_semicolon")]
    categories: Vec<String>,
    #[serde(deserialize_with = "string_with_semicolon")]
    genres: Vec<String>,
    #[serde(deserialize_with = "string_with_semicolon")]
    steamspy_tags: Vec<String>,
    achievements: u32,
    positive_ratings: i64,
    negative_ratings: i64,
    average_playtime: i64,
    median_playtime: i64,
    #[serde(deserialize_with = "player_range")]
    owners: Range<u64>,
    price: f64,
}

#[derive(Debug, Deserialize)]
struct StringWithSemicolon(String);

impl From<StringWithSemicolon> for Vec<String> {
    fn from(value: StringWithSemicolon) -> Self {
        value.0.as_str().split(";").map(|s| s.into()).collect()
    }
}

/// Deserialize bool from String with custom value mapping
fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(serde::de::Error::invalid_value(
            Unexpected::Str(other),
            &"1 or 0",
        )),
    }
}

fn string_with_semicolon<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(";").map(|s| s.into()).collect())
}

fn player_range<'de, D>(deserializer: D) -> Result<Range<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str().split_once("-") {
        Some((a, b)) => Ok(Range {
            start: a.parse().unwrap(),
            end: b.parse().unwrap(),
        }),
        None => Err(serde::de::Error::invalid_value(
            Unexpected::Str(&s),
            &"'a-b' range",
        )),
    }
}

#[cfg(test)]
mod dedo_talk_test {
    use super::GameStruct;

    #[test]
    fn read_and_parse_csv() {
        // https://raw.githubusercontent.com/JadenHow/Steam-Games-Recommendations/main/datasets/steam.csv
        let mut reader = csv::Reader::from_path("/path/to/steam.csv").unwrap();

        let data: Vec<GameStruct> = reader.deserialize().map(|record| record.unwrap()).collect();

        dbg!(&data[0]);
        dbg!(&data.len());
    }
}
