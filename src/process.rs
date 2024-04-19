use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let header = reader.headers()?.clone();
    let mut result = vec![];
    for record in reader.records() {
        let player = record?;
        let json_value = header.iter().zip(player.iter()).collect::<Value>();
        result.push(json_value);
    }

    let output_json = serde_json::to_string_pretty(&result)?;
    std::fs::write(output, output_json)?;
    Ok(())
}
