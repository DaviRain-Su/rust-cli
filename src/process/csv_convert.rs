use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::csv_opt::OutputFormat;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let header = reader.headers()?.clone();
    let mut result = vec![];
    for record in reader.records() {
        let player = record?;
        let json_value = header.iter().zip(player.iter()).collect::<Value>();
        result.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&result)?,
        OutputFormat::Yaml => serde_yaml::to_string(&result)?,
        // OutputFormat::Toml => toml::to_string(&result)?,
    };

    std::fs::write(output, content)?;
    Ok(())
}
