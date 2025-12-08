use serde::{Deserialize, Serialize};

// 1. Główny wrapper (Root)
// To jest ta struktura, której używasz w: let response_data: SuggesticResponseSearch = ...
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SuggesticResponse {
    pub data: DataData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SuggesticResponsePop {
    pub data: DataDataPop,
}

// 2. Obiekt Data
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataData {
    // KLUCZOWE: API zwraca "recipeSearch", a my chcemy to przypisać do pola "popular_recipes".
    // Musimy wymusić na Serde szukanie klucza "recipeSearch".
    #[serde(rename = "recipeSearch")] 
    pub popular_recipes: SearchRecipesConnection,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataDataPop {
    // KLUCZOWE: API zwraca "recipeSearch", a my chcemy to przypisać do pola "popular_recipes".
    // Musimy wymusić na Serde szukanie klucza "recipeSearch".
    #[serde(rename = "popularRecipes")] 
    pub popular_recipes: SearchRecipesConnection,
}

// 3. Połączenie (Connection)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchRecipesConnection {
    pub edges: Vec<RecipeEdge>,
}

// 4. Krawędź (Edge)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecipeEdge {
    pub node: SuggesticRecipe, // Zakładam, że SuggesticRecipe to Twój typ przepisu
}

// 5. Przepis (Node)
// Upewnij się, że pola tutaj pasują do tego, o co pytasz w GraphQL (id, name, mainImage itd.)
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")] // Ważne dla pól takich jak mainImage czy totalTime
pub struct SuggesticRecipe {
    pub id: String,
    pub name: String,
    pub main_image: Option<String>, // GraphQL: mainImage
    pub total_time: Option<String>, // GraphQL: totalTime
    // serving w API może być stringiem lub liczbą, zależnie od implementacji Suggestic,
    // bezpieczniej użyć String lub Option<String> jeśli nie jesteś pewien typu.
    pub serving: Option<f32>,    
}