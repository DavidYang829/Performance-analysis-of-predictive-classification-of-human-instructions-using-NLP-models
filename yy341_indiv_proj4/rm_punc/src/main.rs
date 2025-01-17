use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{Value, json};
use regex::Regex;
use log::Level;

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(Level::Info).unwrap();
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: Value, _: Context) -> Result<Value, Error> {
    let re = Regex::new(r"[^\w\s]").unwrap();
    let text = event["text"].as_str().unwrap_or("");
    let processed_text = re.replace_all(text, "").to_string();

    Ok(json!({ "text": processed_text }))
}
