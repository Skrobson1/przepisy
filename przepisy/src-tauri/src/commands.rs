// PLIK: src-tauri/src/commands.rs

use dotenv_codegen::dotenv;
use serde_json::json;
use crate::models::{SuggesticRecipe, SuggesticResponse, SuggesticResponsePop}; // Importujemy modele z głównego crate'a

#[tauri::command]
pub async fn fetch_recipes_backend() -> Result<Vec<SuggesticRecipe>, String> {
    
    #[cfg(target_os = "android")]
    log::info!("BACKEND (commands): Szukam");

    let api_key = dotenv!("SUGGESTIC_API_KEY");

    let graphql_query = json!({
        "query": r#"
            query {
                popularRecipes(first: 3) {
                    edges {
                        node {
                            id
                            name
                            mainImage
                            totalTime
                            serving
                        }
                    }
                }
            }
        "#
        // Usuwamy "variables" bo popularRecipes ich nie potrzebuje w tej wersji
    });

    // Klient z rustls
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .build()
        .map_err(|e| format!("Błąd klienta: {}", e))?;

    let resp = client
        .post("https://production.suggestic.com/graphql")
        .header("Authorization", format!("Token {}", api_key))
        .header("Content-Type", "application/json")
        .json(&graphql_query)
        .send()
        .await
        .map_err(|e| format!("Błąd sieci: {}", e))?;

    if !resp.status().is_success() {
        let err_text = resp.text().await.unwrap_or_default();
        return Err(format!("API Error: {}", err_text));
    }

    let raw_body = resp.text().await.map_err(|e| format!("Body Error: {}", e))?;
    
    let response_data: SuggesticResponsePop = serde_json::from_str(&raw_body)
        .map_err(|e| format!("JSON Error: {}. Body: {}...", e, raw_body))?;

    let recipes = response_data.data.popular_recipes.edges
        .into_iter()
        .map(|edge| edge.node)
        .collect();

    Ok(recipes)
}

#[tauri::command]
pub async fn fetch_recipes_backend_by_name(query: String) -> Result<Vec<SuggesticRecipe>, String> {
    
    #[cfg(target_os = "android")]
    log::info!("BACKEND (commands): Szukam: {}", query);

    let api_key = dotenv!("SUGGESTIC_API_KEY");

    let graphql_query = json!({
        "query": r#"
            query Search($query: String!) {
                recipeSearch(query: $query, first: 10) {
                    edges {
                        node {
                            id
                            name
                            mainImage
                            totalTime
                            serving
                        }
                    }
                }
            }
        "#,
        "variables": {
            "query": query
        }
    });

    // Klient z rustls
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .build()
        .map_err(|e| format!("Błąd klienta: {}", e))?;

    let resp = client
        .post("https://production.suggestic.com/graphql")
        .header("Authorization", format!("Token {}", api_key))
        .header("Content-Type", "application/json")
        .json(&graphql_query)
        .send()
        .await
        .map_err(|e| format!("Błąd sieci: {}", e))?;

    if !resp.status().is_success() {
        let err_text = resp.text().await.unwrap_or_default();
        return Err(format!("API Error: {}", err_text));
    }

    let raw_body = resp.text().await.map_err(|e| format!("Body Error: {}", e))?;
    
    let response_data: SuggesticResponse = serde_json::from_str(&raw_body)
        .map_err(|e| format!("JSON Error: {}. Body: {:.50}...", e, raw_body))?;

    let recipes = response_data.data.popular_recipes.edges
        .into_iter()
        .map(|edge| edge.node)
        .collect();

    Ok(recipes)
}