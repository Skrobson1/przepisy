use leptos::prelude::*;
use crate::models::SuggesticRecipe;

#[component]
pub fn SuggesticCard(recipe: SuggesticRecipe) -> impl IntoView {
    // 1. Wyciągamy URL obrazka (tutaj clone jest bezpieczny)
    let image_url = recipe.main_image.clone().unwrap_or("https://via.placeholder.com/300".to_string());
    
    view! {
        <div class="bg-surface block w-full border border-text-muted/10 rounded-xl shadow-sm hover:shadow-md hover:-translate-y-1 transition-all duration-300 overflow-hidden group">
            
            <div class="relative h-52 overflow-hidden">
                // ZMIANA TUTAJ: Usuń '&' i dodaj .clone()
                // alt musi posiadać własny string, bo 'recipe' zaraz zniknie
                <img src=image_url alt=recipe.name.clone() class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110" />
                
                <div class="absolute bottom-2 right-2 bg-background/90 backdrop-blur-sm px-2 py-1 rounded-lg text-xs font-bold text-text-main shadow-sm border border-text-muted/10">
                    // Tutaj clone() jest potrzebny, bo unwrap_or konsumuje opcję
                    {recipe.total_time.clone().unwrap_or("30 min".to_string())}
                </div>
            </div>

            <div class="p-5">
                <h5 class="text-lg font-bold text-text-main leading-snug mb-2 line-clamp-2 min-h-[3.5rem]">
                    // ZMIANA: Tutaj możemy zużyć (move) recipe.name, jeśli to ostatnie użycie,
                    // ale bezpieczniej dać .clone(), jeśli używasz recipe.name wyżej w 'alt'
                    {recipe.name.clone()}
                </h5>
                
                <div class="flex justify-between items-center mt-4">
                    <span class="text-xs text-text-muted">
                        "Porcji: " {recipe.serving.unwrap_or(1.0)}
                    </span>
                    
                    <a href=format!("/przepis/{}", recipe.id) class="text-primary hover:text-primary/80 text-sm font-semibold flex items-center transition-colors">
                        "Gotuj"
                        <svg class="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"></path></svg>
                    </a>
                </div>
            </div>
        </div>
    }
}