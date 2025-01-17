use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::f64::consts::PI; // Import the constant for π


#[derive(Deserialize)]
struct Request {
    body: String,
}


#[derive(Serialize)]
struct Response {
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
    is_base64_encoded: bool,
}

async fn function_handler(event: LambdaEvent<Value>) -> Result<Response, Error> {
    let request: Request = serde_json::from_value(event.payload).map_err(|e| {
        eprintln!("Error parsing event payload: {}", e);
        Error::from(e.to_string())
    })?;

    // Expecting a floating-point number from the parsed body
    let parsed_body: HashMap<String, f64> = serde_json::from_str(&request.body).map_err(|e| {
        eprintln!("Error parsing number from body: {}", e);
        Error::from(e.to_string())
    })?;

    let number = match parsed_body.get("number") {
        Some(num) => *num,
        None => return Err(Error::from("No number field in body")),
    };

    let result = number * PI; // Multiply the input number by π

    let response = Response {
        status_code: 200,
        headers: [("Content-Type".to_string(), "application/json".to_string())]
            .iter().cloned().collect(),
        body: json!({ "result": result }).to_string(), // Changed key to 'result' for clarity
        is_base64_encoded: false,
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(function_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
