use crate::models::{SuggesticRecipe, SuggesticRecipeDetails};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize)]
struct SearchArgs {
    query: String,
}

#[derive(Serialize)]
struct SearchArgsCursor {
    query: String,
    cursor: String,
}

#[derive(Serialize)]
struct PopularArgs {
    cursor: Option<String>,
}

pub async fn search_suggestic(
    query: Option<String>,
    cursor: Option<String>,
) -> Result<(Vec<SuggesticRecipe>, Option<String>), String> {
    match &query {
        Some(q) => {
            leptos::logging::log!("FRONTEND: Szukam po nazwie: '{}', kursor: {:?}", q, cursor)
        }
        None => leptos::logging::log!("FRONTEND: Pobieram popularne, kursor: {:?}", cursor),
    }

    match query {
        Some(term) => {
            let args = match cursor {
                Some(cur) => serde_wasm_bindgen::to_value(&SearchArgsCursor {
                    query: term,
                    cursor: cur,
                })
                .unwrap(),
                None => serde_wasm_bindgen::to_value(&SearchArgs { query: term }).unwrap(),
            };

            match invoke("fetch_recipes_backend_by_name", args).await {
                Ok(result_js) => {
                    let recipes: (Vec<SuggesticRecipe>, Option<String>) =
                        serde_wasm_bindgen::from_value(result_js)
                            .map_err(|e| format!("Błąd deserializacji danych (search): {:?}", e))?;
                    return Ok(recipes);
                }
                Err(error_js) => {
                    let error_msg = error_js
                        .as_string()
                        .unwrap_or("Nieznany błąd Tauri".to_string());
                    return Err(error_msg);
                }
            }
        }
        None => {
            let args = serde_wasm_bindgen::to_value(&PopularArgs { cursor: cursor }).unwrap();

            match invoke("fetch_recipes_backend", args).await {
                Ok(result_js) => {
                    let result: (Vec<SuggesticRecipe>, Option<String>) =
                        serde_wasm_bindgen::from_value(result_js)
                            .map_err(|e| format!("Błąd deserializacji Popular: {:?}", e))?;

                    return Ok(result);
                }
                Err(error_js) => {
                    let error_msg = error_js
                        .as_string()
                        .unwrap_or("Nieznany błąd Tauri".to_string());
                    return Err(error_msg);
                }
            }
        }
    }
}

pub async fn translate_to(lang: String, text: String) -> Result<String, String> {
    #[derive(Serialize, Deserialize)]
    struct Args {
        lang: String,
        data: String,
    }

    let args_s = Args {
        lang: lang,
        data: text,
    };

    let args = serde_wasm_bindgen::to_value(&args_s).unwrap();

    match invoke("fetch_translation", args).await {
        Ok(body) => {
            let re = regex::Regex::new(r#"\[\[\["(.*?)""#).map_err(|e| e.to_string())?;
            if let Some(c) = re.captures(&body.as_string().unwrap_or("".to_string())) {
                Ok(c[1].to_string().replace(r#"\""#, r#"""#))
            } else {
                Err("Nie znaleziono tlumaczenia".to_string())
            }
        }
        Err(e) => return Err(e.as_string().unwrap_or("Blad".to_string())),
    }
}

#[derive(Serialize)]
struct RecipeById {
    id: String,
}

pub async fn get_recipe_by_id(id: String) -> Result<SuggesticRecipeDetails, String> {
    let arg = serde_wasm_bindgen::to_value(&RecipeById { id }).unwrap();

    match invoke("fetch_recipe_details", arg).await {
        Ok(result_js) => {
            let recipe: SuggesticRecipeDetails = serde_wasm_bindgen::from_value(result_js)
                .map_err(|e| format!("Błąd deserializacji (details): {:?}", e))?;

            return Ok(recipe);
        }
        Err(error_js) => {
            let error_msg = error_js
                .as_string()
                .unwrap_or("Nieznany błąd Tauri".to_string());
            return Err(error_msg);
        }
    }
}
