use crate::models::{SuggesticRecipe, SuggesticResponse, SuggesticResponsePop};
use dotenv_codegen::dotenv;
use serde_json::json;

#[tauri::command]
pub async fn fetch_recipes_backend(
    cursor: Option<String>,
) -> Result<(Vec<SuggesticRecipe>, Option<String>), String> {
    #[cfg(target_os = "android")]
    log::info!("BACKEND: Szukam popularnych (kursor: {:?})", cursor);

    let api_key = dotenv!("SUGGESTIC_API_KEY");

    let graphql_query = json!({
        "query": r#"
            query Popular($after: String) {
                popularRecipes(first: 10, after: $after) {
                    edges {
                        node {
                            id
                            name
                            mainImage
                            totalTime
                            serving
                        }
                    }
                    pageInfo {
                        endCursor
                        hasNextPage
                    }
                }
            }
        "#,
        "variables": {
            "after": cursor
        }
    });

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
        return Err(format!("API Error: {}", resp.status()));
    }

    let raw_body = resp
        .text()
        .await
        .map_err(|e| format!("Body Error: {}", e))?;

    let response_data: SuggesticResponsePop = serde_json::from_str(&raw_body)
        .map_err(|e| format!("JSON Error (Popular): {}. Body: {:.50}...", e, raw_body))?;

    let connection = response_data.data.popular_recipes;

    let recipes = connection.edges.into_iter().map(|edge| edge.node).collect();

    let next_cursor = if connection.page_info.has_next_page {
        connection.page_info.end_cursor
    } else {
        None
    };

    Ok((recipes, next_cursor))
}

#[tauri::command]
pub async fn fetch_recipes_backend_by_name(
    query: String,
    cursor: Option<String>,
) -> Result<(Vec<SuggesticRecipe>, Option<String>), String> {
    #[cfg(target_os = "android")]
    log::info!(
        "BACKEND: Szukam po nazwie: {} (kursor: {:?})",
        query,
        cursor
    );

    let api_key = dotenv!("SUGGESTIC_API_KEY");

    let graphql_query = json!({
        "query": r#"
            query Search($query: String!, $after: String) {
                recipeSearch(query: $query, first: 10, after: $after) {
                    edges {
                        node {
                            id
                            name
                            mainImage
                            totalTime
                            serving
                        }
                    }
                    pageInfo {
                        endCursor
                        hasNextPage
                    }
                }
            }
        "#,
        "variables": {
            "query": query,
            "after": cursor
        }
    });

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
        return Err(format!("API Error: {}", resp.status()));
    }

    let raw_body = resp
        .text()
        .await
        .map_err(|e| format!("Body Error: {}", e))?;

    let response_data: SuggesticResponse = serde_json::from_str(&raw_body)
        .map_err(|e| format!("JSON Error (Search): {}. Body: {:.50}...", e, raw_body))?;

    let connection = response_data.data.recipe_search;

    let recipes = connection.edges.into_iter().map(|edge| edge.node).collect();

    let next_cursor = if connection.page_info.has_next_page {
        connection.page_info.end_cursor
    } else {
        None
    };

    Ok((recipes, next_cursor))
}

#[tauri::command]
pub async fn fetch_translation(lang: String, data: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl={}&dt=t&q={}",
        lang, data
    );
    let body = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    Ok(body)
}
use crate::models::{SuggesticRecipeDetails, SuggesticResponseDetailsWrapper};

#[tauri::command]
pub async fn fetch_recipe_details(id: String) -> Result<SuggesticRecipeDetails, String> {
    #[cfg(target_os = "android")]
    log::info!("BACKEND: Pobieram szczegóły ID: {}", id);

    let api_key = dotenv!("SUGGESTIC_API_KEY");

    let graphql_query = json!({
        "query": r#"
            query Recipe($id: ID!) {
                recipe(id: $id) {
                    name
                    mainImage
                    totalTime
                    serving
                    ingredients {
                        name
                    }
                    instructions
                }
            }
        "#,
        "variables": {
            "id": id,
        }
    });

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

    let status = resp.status();

    if !status.is_success() {
        let error_text = resp.text().await.unwrap_or_default();
        #[cfg(target_os = "android")]
        log::error!("API Error {}: {}", status, error_text);
        return Err(format!("API Error {}: {}", status, error_text));
    }

    let raw_body = resp
        .text()
        .await
        .map_err(|e| format!("Body Error: {}", e))?;

    let response_data: SuggesticResponseDetailsWrapper = serde_json::from_str(&raw_body)
        .map_err(|e| format!("JSON Error (Details): {}. Body: {:.50}...", e, raw_body))?;

    Ok(response_data.data.recipe)
}
