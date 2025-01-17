use clap;
use csv;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct AlumniRecord {
    name: String,
    age: u8,
    income: f32,
    gender: String,
    company: String,
}

#[tokio::main]
async fn main() {
    let matches = clap::App::new("Alumni Search Tool")
        .version("0.1.0")
        .author("Yucheng Yang")
        .about("A command-line tool for querying alumni data")
        .arg(clap::Arg::with_name("file")
            .help("Sets the input CSV file to use")
            .required(true)
            .index(1))
        .arg(clap::Arg::with_name("gender")
            .help("Filter by gender")
            .short("g")
            .long("gender")
            .takes_value(true))
        .arg(clap::Arg::with_name("company")
            .help("Filter by company")
            .short("c")
            .long("company")
            .takes_value(true))
        .arg(clap::Arg::with_name("low")
            .help("Sets the lower bound for the income range")
            .short("l")
            .long("low")
            .takes_value(true))
        .arg(clap::Arg::with_name("high")
            .help("Sets the upper bound for the income range")
            .short("h")
            .long("high")
            .takes_value(true))
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let gender_filter = matches.value_of("gender");
    let company_filter = matches.value_of("company");
    let income_low = matches.value_of("low");
    let income_high = matches.value_of("high");

    // Process the CSV file
    let records = process_csv(file_path).unwrap();

    // Filter the records based on the provided criteria
    let filtered_records = filter_records(records, gender_filter, company_filter, income_low, income_high).unwrap();

    // Print the filtered records
    print_records(filtered_records);
}

fn process_csv(file_path: &str) -> Result<Vec<AlumniRecord>, Box<dyn Error>> {
    let mut records = Vec::new();
    let mut reader = csv::Reader::from_path(file_path)?;
    for result in reader.records() {
        let record = result?;
        let alumni_record = AlumniRecord {
            name: record[0].to_string(),
            age: record[1].parse::<u8>()?,
            income: record[2].parse::<f32>()?,
            gender: record[3].to_string(),
            company: record[4].to_string(),
        };
        records.push(alumni_record);
    }
    Ok(records)
}

fn filter_records(records: Vec<AlumniRecord>, gender_filter: Option<&str>, company_filter: Option<&str>, income_low: Option<&str>, income_high: Option<&str>) -> Result<Vec<AlumniRecord>, Box<dyn Error>> {
    let filtered_records = records.into_iter().filter(|record| {
        let pass_gender = match gender_filter {
            Some(gender) => record.gender == gender,
            None => true,
        };

        let pass_company = match company_filter {
            Some(company) => record.company == company,
            None => true,
        };

        let pass_income = match (income_low, income_high) {
            (Some(low), Some(high)) => {
                let income = record.income;
                income >= low.parse::<f32>().unwrap_or(f32::MIN) && income <= high.parse::<f32>().unwrap_or(f32::MAX)
            }
            (Some(low), None) => {
                let income = record.income;
                income >= low.parse::<f32>().unwrap_or(f32::MIN)
            }
            (None, Some(high)) => {
                let income = record.income;
                income <= high.parse::<f32>().unwrap_or(f32::MAX)
            }
            (None, None) => true,
        };

        pass_gender && pass_company && pass_income
    }).collect();
    Ok(filtered_records)
}

fn print_records(records: Vec<AlumniRecord>) {
    for record in records {
        println!("{:?}", record);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_query_whole_dataset() {
        let output = Command::new("./target/release/cmdtool")
            .arg("test1.csv")
            .output()
            .expect("failed to execute process");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("John Smith") && stdout.contains("Sophia Moore"));
    }

    #[test]
    fn test_query_by_gender() {
        let output = Command::new("./target/release/cmdtool")
            .args(&["test1.csv", "--gender", "Male"])
            .output()
            .expect("failed to execute process");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("John Smith") && !stdout.contains("Emily Johnson"));
    }

    #[test]
    fn test_query_by_company() {
        let output = Command::new("./target/release/cmdtool")
            .args(&["test1.csv", "--company", "Google"])
            .output()
            .expect("failed to execute process");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("John Smith") && stdout.contains("Olivia Davis"));
    }

    #[test]
    fn test_query_by_income_range() {
        let output = Command::new("./target/release/cmdtool")
            .args(&["test1.csv", "--low", "90000", "--high", "120000"])
            .output()
            .expect("failed to execute process");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("John Smith") && stdout.contains("Sophia Moore") && !stdout.contains("Isabella Taylor"));
    }

    #[test]
    fn test_query_combination() {
        let output = Command::new("./target/release/cmdtool")
            .args(&["test1.csv", "--gender", "Male", "--company", "Facebook", "--low", "100000"])
            .output()
            .expect("failed to execute process");
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Matthew Wilson") && !stdout.contains("John Smith"));
    }
}
