use crate::models::SuggesticRecipe;
use serde::{Serialize};
use wasm_bindgen::prelude::*;

// 1. ZMIANA: Dodajemy 'catch' i zmieniamy zwracany typ na Result
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize)]
struct SearchArgs {
    query: String,
}

pub async fn search_suggestic(query: Option<String>) -> Result<Vec<SuggesticRecipe>, String> {
    leptos::logging::log!("FRONTEND: Wołam Tauri invoke dla: {query:?}");

    let args = match query {
        Some(term) => Some(serde_wasm_bindgen::to_value(&SearchArgs { query: term }).unwrap()),
        None => None,
    };

    match args {
        Some(arg) => match invoke("fetch_recipes_backend_by_name", arg).await {
            Ok(result_js) => {
                let recipes: Vec<SuggesticRecipe> = serde_wasm_bindgen::from_value(result_js)
                    .map_err(|e| format!("Błąd deserializacji danych: {:?}", e))?;
                return Ok(recipes);
            },
            Err(error_js) => {
                // Konwertujemy błąd JS na String
                let error_msg = error_js.as_string().unwrap_or("Nieznany błąd Tauri".to_string());
                return Err(error_msg);
            }
        },
        None => match invoke("fetch_recipes_backend", JsValue::NULL).await {
            Ok(result_js) => {
                let recipes: Vec<SuggesticRecipe> = serde_wasm_bindgen::from_value(result_js)
                    .map_err(|e| format!("Błąd deserializacji danych: {:?}", e))?;
                return Ok(recipes);
            },
            Err(error_js) => {
                // Konwertujemy błąd JS na String
                let error_msg = error_js.as_string().unwrap_or("Nieznany błąd Tauri".to_string());
                return Err(error_msg);
            }
        }
    }
}