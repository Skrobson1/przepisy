use leptos::prelude::*;
use crate::models::{SuggesticRecipe, SavedRecipes};
use crate::api::translate_to;
use leptos::task::spawn_local;
use crate::enums::currentview::CurrentView;

#[component]
pub fn SuggesticCard(recipe: SuggesticRecipe) -> impl IntoView {
    let name_for_translation = recipe.name.clone(); 
    let image_url = recipe.main_image.clone().unwrap_or("https://via.placeholder.com/300".to_string());
    
    let (name, set_name) = signal(recipe.name.clone());

    let set_view_state = use_context::<WriteSignal<CurrentView>>().expect("View state context missing");
    let set_page_counter_vis = use_context::<WriteSignal<bool>>().expect("Missing navbar state");
    let set_saved_recipes = use_context::<WriteSignal<SavedRecipes>>().expect("Missing saved recipes write");

    let saved_recipes_read = use_context::<ReadSignal<SavedRecipes>>().expect("Missing saved recipes read");

    let is_initially_liked = saved_recipes_read
        .get_untracked()
        .recipes
        .iter()
        .any(|r| r.id == recipe.id);
        
    let (is_liked, set_is_liked) = signal(is_initially_liked); 

    spawn_local(async move {
        let res = translate_to("pl".to_string(), name_for_translation).await;
        match res {
            Ok(translated_name) => set_name.set(translated_name),
            Err(e) => leptos::logging::error!("Błąd tłumaczenia: {:?}", e),
        }
    });

    let recipe_for_toggle = recipe.clone();

    let toggle_like = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        
        set_saved_recipes.update(|saved| {
            let current_recipe = recipe_for_toggle.clone();
            
            if let Some(index) = saved.recipes.iter().position(|r| r.id == current_recipe.id) {
                saved.recipes.remove(index);
                set_is_liked.set(false);
            } else {
                saved.recipes.push(current_recipe);
                set_is_liked.set(true);
            }
        });
    };

    let recipe_id_for_nav = recipe.id.clone();

    view! {
        <div on:click=move |_| {
                set_page_counter_vis.set(false);
                set_view_state.set(CurrentView::RecipeDetail(recipe_id_for_nav.clone()));
            }
            class="bg-surface cursor-pointer block w-full border border-text-muted/10 rounded-xl shadow-sm hover:shadow-md hover:-translate-y-1 transition-all duration-300 overflow-hidden group relative"
        >
            <div class="relative h-52 overflow-hidden">
                <img src=image_url alt={move || name.get()} class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110" />
                
                <div class="absolute bottom-2 right-2 bg-background/90 backdrop-blur-sm px-2 py-1 rounded-lg text-xs font-bold text-text-main shadow-sm border border-text-muted/10">
                    {recipe.total_time.clone().unwrap_or("30 min".to_string())}
                </div>
            </div>

            <div class="p-5">
                <h5 class="text-lg font-bold text-text-main leading-snug mb-2 line-clamp-2 min-h-[3.5rem]">
                    {move || name.get()}
                </h5>
                
                <div class="flex justify-between items-center mt-4">
                    <span class="text-xs text-text-muted">
                        "Porcji: " {recipe.serving.unwrap_or(1.0)}
                    </span>
                    
                    <div class="flex items-center gap-6">
                        <button 
                            on:click=toggle_like
                            class="p-2 -m-2 rounded-full hover:bg-gray-100 text-gray-400 hover:text-red-500 transition-colors group/heart"
                            title={move || if is_liked.get() { "Usuń z ulubionych" } else { "Dodaj do ulubionych" }}
                        >
                            {move || if is_liked.get() {
                                view! {
                                    <svg class="w-6 h-6 text-red-500 transition-transform active:scale-90" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z" clip-rule="evenodd" />
                                    </svg>
                                }.into_any()
                            } else {
                                view! {
                                    <svg class="w-6 h-6 group-hover/heart:text-red-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                                    </svg>
                                }.into_any()
                            }}
                        </button>
                        <div class="h-4 w-px bg-gray-300"></div>

                        <span class="text-primary group-hover:text-primary/80 text-sm font-semibold flex items-center transition-colors">
                            "Gotuj"
                            <svg class="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"></path></svg>
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}