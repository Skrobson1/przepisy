use leptos::prelude::*;
use crate::enums::{theme::Theme, currentview::CurrentView};

#[component]
pub fn Settings() -> impl IntoView {
    let set_theme = use_context::<WriteSignal<Theme>>().expect("Brak kontekstu");
    let theme = use_context::<ReadSignal<Theme>>().expect("Brak kontekstu");
    let set_view_state = use_context::<WriteSignal<CurrentView>>().expect("View state context missing");
    let set_page_counter_vis = use_context::<WriteSignal<bool>>().expect("Navbar state missing");

    Effect::new(move |_| {
        let doc = window().document().unwrap().document_element().unwrap();
        
        match theme.get() {
            Theme::Dark => {
                // Dodaje klasę 'dark' do <html>
                let _ = doc.class_list().add_1("dark");
            }
            Theme::Light => {
                // Usuwa klasę 'dark' z <html>
                let _ = doc.class_list().remove_1("dark");
            }
        }
    });

    let toggle_theme = move |_| {
        set_theme.update(|t| *t = if *t == Theme::Light { Theme::Dark } else { Theme::Light });
    };

    view! {
<div class="flex items-center justify-between mb-6 sticky top-4 z-20">
    <button on:click=move |_| { set_view_state.set(CurrentView::Home); set_page_counter_vis.set(true); } class="p-2 rounded-full bg-surface/80 backdrop-blur-md shadow-sm hover:bg-surface text-text-main hover:text-primary transition-all duration-300">
    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path></svg>
    </button>
</div>
<h3 class="mb-5 text-lg font-medium text-text-main">Motyw</h3>
<ul class="grid w-full gap-6 md:grid-cols-2">
    <li>
        <input prop:checked=move || theme.get() == Theme::Light on:change=toggle_theme type="radio" id="theme-light" name="theme" value="theme-light" class="hidden peer" required />
        
        <label for="theme-light" class="inline-flex items-center justify-between w-full p-5 text-text-muted bg-surface border border-text-muted/20 rounded-xl cursor-pointer transition-all duration-200 hover:border-primary/50 hover:shadow-sm peer-checked:border-primary peer-checked:text-primary peer-checked:bg-primary/5">                           
            <div class="block">
                <div class="w-full font-semibold text-text-main peer-checked:text-primary">Jasny</div>
                <div class="w-full text-sm opacity-80">Idealny na dzień</div>
            </div>
            <Show when=move || theme.get() == Theme::Light>
                <svg class="fill-current" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"/></svg>
            </Show>
        </label>
    </li>
    <li>
        <input prop:checked=move || theme.get() == Theme::Dark on:change=toggle_theme type="radio" id="theme-dark" name="theme" value="theme-dark" class="hidden peer" />
        
        <label for="theme-dark" class="inline-flex items-center justify-between w-full p-5 text-text-muted bg-surface border border-text-muted/20 rounded-xl cursor-pointer transition-all duration-200 hover:border-primary/50 hover:shadow-sm peer-checked:border-primary peer-checked:text-primary peer-checked:bg-primary/5">
            <div class="block">
                <div class="w-full font-semibold text-text-main peer-checked:text-primary">Ciemny</div>
                <div class="w-full text-sm opacity-80">Wieczorne gotowanie</div>
            </div>
            <Show when=move || theme.get() == Theme::Dark>
                <svg class="fill-current" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z"/></svg>
            </Show>
        </label>
    </li>
</ul>

    }
}

// #[component]
// pub fn Theme_wrapper(children: Children) -> impl IntoView {
//     let theme = use_context::<ReadSignal<Theme>>().expect("Brak kontekstu");
//     view! {
//         <body class:dark=move || theme.get() == Theme::Dark>
//         <div 
//         class="h-screen w-screen">
//         {children()}
//         </div>
//         </body>
//     }
// }