use leptos::prelude::*;
use crate::models::SuggesticRecipe;
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

    spawn_local(async move {
        let res = translate_to("pl".to_string(), name_for_translation).await;

        match res {
            Ok(translated_name) => {
                set_name.set(translated_name);
            },
            Err(e) => leptos::logging::error!("Błąd tłumaczenia: {:?}", e),
        }
    });

    view! {
        <div on:click=move |_| {
                set_page_counter_vis.set(false);
                set_view_state.set(CurrentView::RecipeDetail(recipe.id.clone()));
            }
            class="bg-surface cursor-pointer block w-full border border-text-muted/10 rounded-xl shadow-sm hover:shadow-md hover:-translate-y-1 transition-all duration-300 overflow-hidden group relative">
            
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
                    
                    <span class="text-primary group-hover:text-primary/80 text-sm font-semibold flex items-center transition-colors">
                        "Gotuj"
                        <svg class="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"></path></svg>
                    </span>
                </div>
            </div>
        </div>
    }
}