use anyhow::{Ok, Result};
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    #[serde(rename = "姓名")]
    name: String,
    #[serde(rename = "年龄")]
    age: u8,
    #[serde(rename = "性别")]
    gender: String,
    #[serde(rename = "班级")]
    class: String,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret: Vec<Person> = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Person = result?;

        println!("{:?}", record);
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
