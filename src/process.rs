use csv::Reader;
use serde::{Deserialize, Serialize};

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
    let mut result = vec![];
    for record in reader.deserialize() {
        let player: Player = record?;
        result.push(player);
    }
    let output_json = serde_json::to_string_pretty(&result)?;
    std::fs::write(output, output_json)?;
    Ok(())
}
