#![allow(warnings)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SuggesticRecipe {
    pub id: String,
    pub name: String,
    pub main_image: Option<String>,
    pub total_time: Option<String>,
    pub serving: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IngredientItem {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SuggesticRecipeDetails {
    pub name: String,
    pub main_image: Option<String>,
    pub ingredients: Vec<IngredientItem>,
    pub instructions: Vec<String>,
    pub total_time: Option<String>,
    pub serving: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SuggesticResponseDetailsWrapper {
    pub data: DataRecipe,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataRecipe {
    pub recipe: SuggesticRecipeDetails,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecipeEdge {
    pub node: SuggesticRecipe,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuggesticConnection {
    pub edges: Vec<RecipeEdge>,
    pub page_info: PageInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SuggesticResponse {
    pub data: DataSearch,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataSearch {
    #[serde(rename = "recipeSearch")]
    pub recipe_search: SuggesticConnection,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SuggesticResponsePop {
    pub data: DataPop,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataPop {
    #[serde(rename = "popularRecipes")]
    pub popular_recipes: SuggesticConnection,
}
