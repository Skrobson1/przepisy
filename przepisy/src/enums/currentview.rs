#[derive(Clone)]
pub enum CurrentView {
    Login,
    Home,
    Settings,
    RecipeDetail(String),
}