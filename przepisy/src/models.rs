use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SuggesticResponse {
    pub data: DataData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SuggesticResponsePop {
    pub data: DataDataPop,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataData {
    #[serde(rename = "recipeSearch")] 
    pub popular_recipes: SearchRecipesConnection,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataDataPop {
    #[serde(rename = "popularRecipes")] 
    pub popular_recipes: SearchRecipesConnection,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchRecipesConnection {
    pub edges: Vec<RecipeEdge>,
}

// 4. Krawędź (Edge)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecipeEdge {
    pub node: SuggesticRecipe,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuggesticRecipe {
    pub id: String,
    pub name: String,
    pub main_image: Option<String>,
    pub total_time: Option<String>,
    pub serving: Option<f32>,    
}