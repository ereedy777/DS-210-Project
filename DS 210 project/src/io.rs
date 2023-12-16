use std::error::Error;
use std::fs::File;
use csv::Reader;

pub fn read_csv(file_path: &str) -> Result<Vec<(String, String, f64)>, Box<dyn Error>> {
    let mut records = Vec::new();
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        let source = record.get(0).ok_or("Missing source")?.to_string();
        let target = record.get(1).ok_or("Missing target")?.to_string();
        let weight: f64 = record.get(2).ok_or("Missing weight")?.parse()?;
        records.push((source, target, weight));
    }

    Ok(records)
}