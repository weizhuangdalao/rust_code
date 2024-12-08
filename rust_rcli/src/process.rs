use anyhow::{Ok, Result};
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

use crate::OutputFormat;

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

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret: Vec<Value> = Vec::with_capacity(128);
    //csv文件header
    let headers = reader.headers()?.clone();
    //csv文件数据
    for result in reader.records() {
        let record = result?;
        //headers.iter()使用headers的迭代器
        //record.iter()使用record的迭代器
        //zip将两个迭代器合并为一个元组迭代器 [(header,record)]
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();

        println!("{:?}", record);
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        // OutputFormat::Toml => toml::to_string(&ret)?,
    };
    fs::write(output, content)?;
    Ok(())
}
