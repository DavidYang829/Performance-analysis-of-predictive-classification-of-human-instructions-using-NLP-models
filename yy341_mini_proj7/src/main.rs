use anyhow::{Result, anyhow};
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{
    Condition, CreateCollection, Filter, SearchPoints, VectorParams, VectorsConfig,
};
use qdrant_client::qdrant::vectors_config::Config;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = QdrantClient::new(Some(config))?;

    let collection_name = "test";
    delete_and_create_collection(&client, collection_name).await?;

    ingest_points(&client, collection_name).await?;

    visualize_output(&client, collection_name).await?;

    Ok(())
}

async fn delete_and_create_collection(client: &QdrantClient, collection_name: &str) -> Result<()> {
    let _ = client.delete_collection(collection_name).await;

    client.create_collection(&CreateCollection {
        collection_name: collection_name.into(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: 10,
                distance: Distance::Cosine.into(),
                ..Default::default()
            })),
        }),
        ..Default::default()
    }).await?;

    Ok(())
}

async fn ingest_points(client: &QdrantClient, collection_name: &str) -> Result<()> {
    for i in 0..5 {
        let payload: Payload = json!({
            "foo": format!("Bar{}", i),
            "bar": 12 + i,
            "baz": {"qux": format!("quux{}", i)}
        }).try_into().map_err(|e| anyhow!("Payload conversion error: {:?}", e))?;
        
        let points = vec![PointStruct::new(i, vec![12.0 + i as f32; 10], payload)];
        client.upsert_points_blocking(collection_name, None, points, None).await?;
    }
    
    Ok(())
}

async fn visualize_output(client: &QdrantClient, collection_name: &str) -> Result<()> {
    let search_result = client.search_points(&SearchPoints {
        collection_name: collection_name.into(),
        vector: vec![11.; 10],
        filter: None,
        limit: 5,
        with_payload: Some(true.into()),
        ..Default::default()
    }).await?;

    for (index, point) in search_result.result.iter().enumerate() {
        println!("Point {} Payload: {:?}", index + 1, point.payload);
    }

    Ok(())
}
