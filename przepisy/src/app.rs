use crate::api::{search_suggestic, translate_to};
use crate::components::{login::*, recipe_card::*, recipe_detail::RecipeDetail, settings::*};
use crate::enums::{currentview::CurrentView, theme::Theme};
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn App() -> impl IntoView {
    let (view_state, set_view_state) = signal(CurrentView::Home);
    let (page_counter_vis, set_page_counter_vis) = signal(true);
    let (theme, set_theme) = signal(Theme::Light);

    let is_home = move || matches!(view_state.get(), CurrentView::Home);

    provide_context(set_theme);
    provide_context(theme);

    provide_context(view_state);
    provide_context(set_view_state);
    provide_context(page_counter_vis);
    provide_context(set_page_counter_vis);

    let (active_cursor, set_active_cursor) = signal(None::<String>);
    let (next_cursor, set_next_cursor) = signal(None::<String>);
    let (page_history, set_page_history) = signal(Vec::<Option<String>>::new());

    let (search_term, set_search_term) = signal(None::<String>);
    let (term_to_search, set_term_to_search) = signal(None::<String>);

    let recipe_resource = LocalResource::new(move || async move {
        let term = term_to_search.get();
        let cursor = active_cursor.get();

        leptos::logging::log!("RESOURCE RUN: term={:?}, cursor={:?}", term, cursor);

        let response = search_suggestic(term, cursor).await;

        match response {
            Ok((recipes, next_cursor_api)) => {
                set_next_cursor.set(next_cursor_api);
                recipes
            }
            Err(e) => {
                leptos::logging::log!("Błąd API: {}", e);
                vec![]
            }
        }
    });

    let handle_next_page = move |_| {
        if let Some(next) = next_cursor.get_untracked() {
            set_page_history.update(|h| h.push(active_cursor.get_untracked()));
            set_active_cursor.set(Some(next));
        }
    };

    let handle_prev_page = move |_| {
        set_page_history.update(|h| {
            if let Some(prev) = h.pop() {
                set_active_cursor.set(prev);
            }
        });
    };

    let search_handler = move || {
        let current_term_opt = search_term.get_untracked();
        leptos::logging::log!(
            "SEARCH HANDLER: Kliknięto szukaj. Wartość: {:?}",
            current_term_opt
        );

        if let Some(t) = current_term_opt {
            set_active_cursor.set(None);
            set_next_cursor.set(None);
            set_page_history.set(Vec::new());

            spawn_local(async move {
                let result = translate_to("en".to_string(), t).await;
                match result {
                    Ok(translated) => {
                        leptos::logging::log!("Przetłumaczono na: {}", translated);
                        set_term_to_search.set(Some(translated));
                    }
                    Err(e) => leptos::logging::error!("Błąd tłumaczenia: {:?}", e),
                }
            });
        }
        set_search_term.set(None);
    };

    let go_home = move || {
        leptos::logging::log!("GO HOME: Resetowanie stanu...");
        set_view_state.set(CurrentView::Home);
        set_search_term.set(None::<String>);
        set_term_to_search.set(None::<String>);
        set_active_cursor.set(None);
        set_next_cursor.set(None);
        set_page_history.set(Vec::new());
        set_page_counter_vis.set(true);
    };

    view! {
        <div class="container mx-auto p-4 bg-background min-h-screen transition-colors duration-300">

        <Show when=move || page_counter_vis.get()>
        <div class="max-w-md mx-auto fixed top-6 left-0 right-0 px-4 my-2 z-10">
            <label for="search" class="block mb-2.5 text-sm font-medium text-text-main sr-only">Search</label>
            <div class="relative">
                <div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
                    <svg class="w-4 h-4 text-text-muted" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z"/></svg>
                </div>
                <input type="search"
                on:input=move |ev| {
                    set_search_term.set(Some(event_target_value(&ev)));
                }
                on:keydown=move |ev| {
                    if ev.key() == "Enter" {
                        ev.prevent_default();
                        search_handler();
                    }
                }
                prop:value=move || search_term.get().unwrap_or_default()
                id="search" class="block w-full p-3 ps-9 bg-surface border border-text-muted/20 text-text-main text-sm rounded-base focus:ring-primary focus:border-primary shadow-sm placeholder:text-text-muted/70" placeholder="Szukaj przepisu..." required />
                <button on:click=move |_| search_handler()
                type="button" class="absolute end-1.5 bottom-1.5 bg-primary hover:bg-primary/90 text-white box-border border border-transparent focus:ring-4 focus:ring-primary/30 shadow-sm font-medium leading-5 rounded text-xs px-3 py-1.5 focus:outline-none transition-colors">Szukaj</button>
            </div>
        </div>
        </Show>

        {move || match view_state.get() {
                CurrentView::Login => view! {
                    <Login/>
                }.into_any(),
                CurrentView::Home => view! {
                <div class="pt-28 pb-24">
                    <Suspense fallback=move || view! { <div class="text-center mt-10">"Ładowanie..."</div> }>
                        {move || {
                            let recipes = recipe_resource.get();

                            match recipes {
                                Some(rec) => {
                                    let recipes_owned = (*rec).clone();

                                    if recipes_owned.is_empty() {
                                         view! {
                                            <div class="text-center text-gray-500 mt-10">
                                                "Nie znaleziono przepisów."
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="grid grid-cols-1 gap-6 w-full max-w-md mx-auto">
                                                {recipes_owned.into_iter()
                                                    .map(|recipe| {
                                                        view! { <SuggesticCard recipe=recipe /> }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        }.into_any()
                                    }
                                },
                                None => {
                                    view! {
                                        <div class="text-center text-gray-500 mt-10">
                                            "Ładowanie..."
                                        </div>
                                    }.into_any()
                                }
                            }
                        }}
                    </Suspense>
                </div>
                }.into_any(),
                CurrentView::Settings => view! {
                    <Settings/>
                }.into_any(),
                CurrentView::RecipeDetail(id) => view! {
                    <RecipeDetail recipe_id=id />
                }.into_any(),
            }}

        <Show when=is_home >
        <div class="fixed bottom-0 z-50 w-full h-16 -translate-x-1/2 bg-surface border-t border-text-muted/10 left-1/2 shadow-[0_-4px_6px_-1px_rgba(0,0,0,0.05)]">
        <div class="grid h-full max-w-lg grid-cols-6 mx-auto">

            <button
                on:click=move |_| go_home()
                data-tooltip-target="tooltip-document" type="button" class="inline-flex flex-col items-center justify-center px-5 hover:bg-background/50 group transition-colors"
            >
                <svg class="w-6 h-6 mb-1 text-text-muted group-hover:text-primary transition-colors" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m4 12 8-8 8 8M6 10.5V19a1 1 0 0 0 1 1h3v-3a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v3h3a1 1 0 0 0 1-1v-8.5"/></svg>
                <span class="sr-only">Home</span>
            </button>
            <div id="tooltip-document" role="tooltip" class="absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-background transition-opacity duration-300 bg-text-main rounded-base shadow-sm opacity-0 tooltip">
                Home
                <div class="tooltip-arrow" data-popper-arrow></div>
            </div>

            <button on:click=move |_| go_home() data-tooltip-target="tooltip-bookmark" type="button" class="inline-flex flex-col items-center justify-center px-5 hover:bg-background/50 group transition-colors">
                <svg class="w-6 h-6 mb-1 text-text-muted group-hover:text-primary transition-colors" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m17 21-5-4-5 4V3.889a.92.92 0 0 1 .244-.629.808.808 0 0 1 .59-.26h8.333a.81.81 0 0 1 .589.26.92.92 0 0 1 .244.63V21Z"/></svg>
                <span class="sr-only">Bookmark</span>
            </button>
            <div id="tooltip-bookmark" role="tooltip" class="absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-background transition-opacity duration-300 bg-text-main rounded-base shadow-sm opacity-0 tooltip">
                Bookmark
                <div class="tooltip-arrow" data-popper-arrow></div>
            </div>

            <div class="flex items-center justify-center col-span-2">
                <div class="flex items-center justify-between w-full text-text-muted bg-background rounded-base border border-text-muted/20 max-w-[128px] mx-2 shadow-inner">
                    <button
                    on:click=handle_prev_page
                    disabled=move || page_history.get().is_empty()
                     type="button" class="inline-flex items-center justify-center h-8 px-1 w-6 rounded-s-base hover:bg-primary/10 hover:text-primary focus:outline-none focus:ring-2 focus:ring-primary/50 transition-colors">
                        <svg class="w-3.5 h-3.5 rtl:rotate-180" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m15 19-7-7 7-7"/></svg>
                        <span class="sr-only">Previous page</span>
                    </button>
                    <span class="shrink-0 mx-1 text-sm font-medium text-text-main space-x-0.5 rtl:space-x-reverse">
                    {move || format!("Page {}", page_history.get().len() + 1)}
                    </span>
                    <button
                    on:click=handle_next_page
                    disabled=move || next_cursor.get().is_none()
                    type="button" class="inline-flex items-center justify-center h-8 px-1 w-6 rounded-e-base hover:bg-primary/10 hover:text-primary focus:outline-none focus:ring-2 focus:ring-primary/50 transition-colors">
                        <svg class="w-3.5 h-3.5 rtl:rotate-180" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m9 5 7 7-7 7"/></svg>
                        <span class="sr-only">Next page</span>
                    </button>
                </div>
            </div>

            <button on:click=move |_| {set_view_state.set(CurrentView::Settings); set_page_counter_vis.set(false);} data-tooltip-target="tooltip-settings" type="button" class="inline-flex flex-col items-center justify-center px-5 hover:bg-background/50 group transition-colors">
                <svg class="w-6 h-6 mb-1 text-text-muted group-hover:text-primary transition-colors" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="M6 4v10m0 0a2 2 0 1 0 0 4m0-4a2 2 0 1 1 0 4m0 0v2m6-16v2m0 0a2 2 0 1 0 0 4m0-4a2 2 0 1 1 0 4m0 0v10m6-16v10m0 0a2 2 0 1 0 0 4m0-4a2 2 0 1 1 0 4m0 0v2"/></svg>
                <span class="sr-only">Settings</span>
            </button>
            <div id="tooltip-settings" role="tooltip" class="absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-background transition-opacity duration-300 bg-text-main rounded-base shadow-sm opacity-0 tooltip">
                Settings
                <div class="tooltip-arrow" data-popper-arrow></div>
            </div>

            <button on:click=move |_| {set_view_state.set(CurrentView::Login); set_page_counter_vis.set(false);}  data-tooltip-target="tooltip-profile" type="button" class="inline-flex flex-col items-center justify-center px-5 hover:bg-background/50 group transition-colors">
                <svg class="w-6 h-6 mb-1 text-text-muted group-hover:text-primary transition-colors" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18Zm0 0a8.949 8.949 0 0 0 4.951-1.488A3.987 3.987 0 0 0 13 16h-2a3.987 3.987 0 0 0-3.951 3.512A8.948 8.948 0 0 0 12 21Zm3-11a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"/></svg>
                <span class="sr-only">Profile</span>
            </button>
            <div id="tooltip-profile" role="tooltip" class="absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-background transition-opacity duration-300 bg-text-main rounded-base shadow-sm opacity-0 tooltip">
                Profile
                <div class="tooltip-arrow" data-popper-arrow></div>
            </div>
        </div>
    </div>
    </Show>
    </div>
    }
}
