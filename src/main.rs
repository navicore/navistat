use csv::ReaderBuilder;
use reqwest::blocking::get;
use serde::Deserialize;
use std::error::Error;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct SalaryRecord {
    work_year: i32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: f64,
    salary_currency: String,
    salary_in_usd: f64,
    employee_residence: String,
    remote_ratio: f64,
    company_location: String,
    company_size: String,
}

fn filter_and_convert(dataset: &[SalaryRecord]) -> Vec<(i32, String, f64)> {
    dataset
        .iter()
        .filter(|record| record.experience_level == "SE")
        .map(|record| {
            let salary_in_usd_rounded = record.salary_in_usd.round();
            (
                record.work_year,
                record.job_title.clone(),
                salary_in_usd_rounded,
            )
        })
        .collect()
}

fn fetch_dataset(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = get(url)?;
    let mut content = String::new();
    response.read_to_string(&mut content)?;
    Ok(content)
}

fn load_dataset(csv_data: &str) -> Result<Vec<SalaryRecord>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: SalaryRecord = result?;
        records.push(record);
    }
    Ok(records)
}

fn main() {
    let url =
        "https://raw.githubusercontent.com/kittenpub/database-repository/main/ds_salaries.csv";
    match fetch_dataset(url) {
        Ok(csv_data) => {
            match load_dataset(&csv_data) {
                Ok(dataset) => {
                    // The dataset is ready for processing
                    println!("Loaded {} records", dataset.len());
                    let parsed_data = filter_and_convert(&dataset);
                    println!("Filtered and converted data: {:?}", parsed_data);
                }
                Err(error) => {
                    eprintln!("Error loading dataset: {}", error);
                }
            }
        }
        Err(error) => {
            eprintln!("Error fetching dataset: {}", error);
        }
    }
}
