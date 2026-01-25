#[derive(Clone)]
pub enum CurrentView {
    //Login,
    Home,
    Settings,
    Favorites,
    RecipeDetail(String),
}