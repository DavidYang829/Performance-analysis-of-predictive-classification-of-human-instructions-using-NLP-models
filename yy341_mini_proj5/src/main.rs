use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use rand::prelude::*;
use aws_config::load_from_env;
use aws_sdk_dynamodb::{Client, model::AttributeValue};
use lambda_runtime::{LambdaEvent, Error as LambdaError, service_fn};

#[derive(Deserialize)]
struct Request {
    address: Option<String>,
    nationality: Option<String>,
}

#[derive(Serialize)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    SimpleLogger::new().with_utc_timestamps().init()?;
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    let request: Request = serde_json::from_value(event.payload)?;

    let config = load_from_env().await;
    let client = Client::new(&config);

    let name = get_random_name(&client, request.address, request.nationality).await?;

    Ok(json!({ "name": name }))
}

async fn get_random_name(client: &Client, address: Option<String>, nationality: Option<String>) -> Result<String, LambdaError> {
    let table_name = "FamilyInfo"; // Make sure this matches your DynamoDB table name

    let mut expr_attr_values = HashMap::new();
    let mut filter_expression = String::new();

    if let Some(addr_val) = address {
        expr_attr_values.insert(":addr_val".to_string(), AttributeValue::S(addr_val));
        if !filter_expression.is_empty() {
            filter_expression.push_str(" and ");
        }
        filter_expression.push_str("address = :addr_val");
    }

    if let Some(nation_val) = nationality {
        if !filter_expression.is_empty() {
            filter_expression.push_str(" and ");
        }
        expr_attr_values.insert(":nation_val".to_string(), AttributeValue::S(nation_val));
        filter_expression.push_str("nationality = :nation_val");
    }

    let result = client.scan()
        .table_name(table_name)
        .set_expression_attribute_values(Some(expr_attr_values))
        .set_filter_expression(Some(filter_expression))
        .send()
        .await?;

    let items = result.items.unwrap_or_default();

    let selected_item = items.iter().choose(&mut thread_rng());

    match selected_item {
        Some(item) => {
            let name_attr = item.get("name").and_then(|val| val.as_s().ok());
            match name_attr {
                Some(name) => Ok(name.to_string()),
                None => Err(LambdaError::from("No name found in the selected item")),
            }
        },
        None => Err(LambdaError::from("No matching names found")),
    }
}

