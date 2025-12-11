use leptos::prelude::*;
use crate::enums::currentview::CurrentView;
use crate::api::{get_recipe_by_id, translate_to};
use leptos::task::spawn_local;

#[component]
pub fn RecipeDetail(recipe_id: String) -> impl IntoView {
    let set_view_state = use_context::<WriteSignal<CurrentView>>().expect("View state context missing");
    let set_page_counter_vis = use_context::<WriteSignal<bool>>().expect("Navbar state missing");

    let recipe_resource = LocalResource::new(move || {
        let id = recipe_id.clone();
        async move {
            get_recipe_by_id(id).await
        }
    });

    let (translated_name, set_translated_name) = signal(None::<String>);
    let (translated_instructions, set_translated_instructions) = signal(Vec::<String>::new());
    let (translated_ingredients, set_translated_ingredients) = signal(Vec::<String>::new());

    Effect::new(move |_| {
        if let Some(Ok(recipe)) = recipe_resource.get().as_deref() {
            let name_to_translate = recipe.name.clone();
            let instructions_to_translate = recipe.instructions.clone();
            let ingredients_to_translate = recipe.ingredients.clone();

            spawn_local(async move {
                if let Ok(pl_name) = translate_to("pl".to_string(), name_to_translate).await {
                    set_translated_name.set(Some(pl_name));
                }

                let mut pl_ingredients = Vec::new();
                for ing in ingredients_to_translate {
                    let text = ing.name.clone();
                    if let Ok(pl_text) = translate_to("pl".to_string(), text.clone()).await {
                        pl_ingredients.push(pl_text);
                    } else {
                        pl_ingredients.push(text);
                    }
                }
                set_translated_ingredients.set(pl_ingredients);

                let mut pl_steps = Vec::new();
                for step in instructions_to_translate {
                     if let Ok(pl_step) = translate_to("pl".to_string(), step.clone()).await {
                         pl_steps.push(pl_step);
                     } else {
                         pl_steps.push(step);
                     }
                }
                set_translated_instructions.set(pl_steps);
            });
        }
    });

    view! {
        <div class="pt-6 pb-24 min-h-screen bg-background animate-in fade-in duration-300">
            <Suspense fallback=move || view! {
                <div class="flex flex-col items-center justify-center mt-20 text-primary">
                    <svg class="animate-spin h-10 w-10 mb-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    <span class="text-text-muted text-sm">"Ładowanie przepisu..."</span>
                </div>
            }>
                {move || {
                    match recipe_resource.get().as_deref() {
                        Some(Ok(r)) => {
                            let recipe = r.clone();

                            let display_name = move || translated_name.get().unwrap_or(recipe.name.clone());

                            let display_ingredients = move || {
                                let trans = translated_ingredients.get();
                                if trans.is_empty() {
                                    recipe.ingredients.iter().map(|i| i.name.clone()).collect()
                                } else {
                                    trans
                                }
                            };

                            let display_instructions = move || {
                                let trans = translated_instructions.get();
                                if trans.is_empty() { recipe.instructions.clone() } else { trans }
                            };

                            view! {
                                <div class="max-w-2xl mx-auto px-4">
                                    <div class="flex items-center justify-between mb-6 sticky top-4 z-20">
                                         <button on:click=move |_| { set_view_state.set(CurrentView::Home); set_page_counter_vis.set(true); } class="p-2 rounded-full bg-surface/80 backdrop-blur-md shadow-sm hover:bg-surface text-text-main hover:text-primary transition-all duration-300">
                                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path></svg>
                                         </button>
                                    </div>

                                    <div class="relative w-full h-72 md:h-96 rounded-3xl overflow-hidden shadow-lg mb-8 group">
                                         <img src=recipe.main_image.clone().unwrap_or("https://via.placeholder.com/600".to_string()) class="w-full h-full object-cover" />
                                         <div class="absolute bottom-0 p-6"><h1 class="text-2xl font-bold text-white">{display_name}</h1></div>
                                    </div>

                                    <div class="mb-10 animate-in slide-in-from-bottom-4 duration-500 delay-100">
                                         <h2 class="text-xl font-bold text-text-main mb-4 flex items-center">
                                            <span class="bg-primary/10 text-primary p-2 rounded-xl mr-3">
                                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"></path></svg>
                                            </span>
                                            "Składniki"
                                        </h2>
                                         <div class="bg-surface rounded-2xl p-1 shadow-sm border border-text-muted/10">
                                            <ul class="divide-y divide-text-muted/10">
                                                {move || display_ingredients().into_iter().map(|ing_text| view! {
                                                    <li class="p-3 hover:bg-background/50 transition-colors flex items-center group">
                                                        <div class="w-2 h-2 rounded-full bg-primary/40 mr-3 group-hover:bg-primary transition-colors"></div>
                                                        <span class="text-text-main font-medium">{ing_text}</span>
                                                    </li>
                                                }).collect_view()}
                                            </ul>
                                        </div>
                                    </div>

                                    <div class="animate-in slide-in-from-bottom-4 duration-500 delay-200">
                                        <h2 class="text-xl font-bold text-text-main mb-6">"Przygotowanie"</h2>
                                        <div class="space-y-6">
                                            {move || display_instructions().into_iter().enumerate().map(|(i, step)| view! {
                                                <div class="flex gap-4">
                                                    <div class="flex-shrink-0 flex flex-col items-center">
                                                        <span class="flex items-center justify-center w-8 h-8 rounded-full bg-primary text-white font-bold text-sm shadow-md ring-4 ring-background z-10">
                                                            {i + 1}
                                                        </span>
                                                        <div class="w-0.5 bg-text-muted/20 h-full -my-2"></div>
                                                    </div>
                                                    <div class="bg-surface p-5 rounded-2xl shadow-sm border border-text-muted/10 flex-grow hover:shadow-md transition-shadow mb-2">
                                                        <p class="text-text-main leading-relaxed text-sm md:text-base">{step}</p>
                                                    </div>
                                                </div>
                                            }).collect_view()}
                                        </div>
                                    </div>

                                </div>
                            }.into_any()
                        },
                        Some(Err(e)) => view! {
                            <div class="text-center text-red-500 mt-20 px-4">
                                <p class="font-bold">"Błąd ładowania"</p>
                                <p class="text-sm">{e.clone()}</p>
                                <button on:click=move |_| set_view_state.set(CurrentView::Home) class="mt-4 text-primary underline">"Wróć"</button>
                            </div>
                        }.into_any(),
                        None => view! { <div></div> }.into_any()
                    }
                }}
            </Suspense>
        </div>
    }
}
