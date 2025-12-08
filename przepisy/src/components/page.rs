use leptos::prelude::*;
use crate::enums::currentview::CurrentView;

#[component]
pub fn Page() -> impl IntoView {
    let _current_view = use_context::<ReadSignal<CurrentView>>().expect("Brak kontekstu");
    view! {
        <div class="bg-surface block max-w-sm p-6 border border-text-muted/10 rounded-xl shadow-sm hover:shadow-md transition-shadow duration-300 my-1">
    
    <h5 class="mb-3 text-2xl font-semibold tracking-tight text-text-main leading-8">
        Najlepsze przepisy 2025
    </h5>
    
    <p class="text-text-muted mb-6">
        Odkryj listę najpopularniejszych dań tego roku, od wegańskich deserów po tradycyjne wypieki.
    </p>
    
    <a href="#" class="inline-flex items-center text-white bg-primary box-border border border-transparent hover:bg-primary/90 focus:ring-4 focus:ring-primary/30 shadow-sm font-medium leading-5 rounded-lg text-sm px-4 py-2.5 focus:outline-none transition-all">
        Więcej
        <svg class="w-4 h-4 ms-1.5 rtl:rotate-180 -me-0.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 12H5m14 0-4 4m4-4-4-4"/></svg>
    </a>
    
</div>
<div class="bg-surface block max-w-sm p-6 border border-text-muted/10 rounded-xl shadow-sm hover:shadow-md transition-shadow duration-300 my-1">
    
    <h5 class="mb-3 text-2xl font-semibold tracking-tight text-text-main leading-8">
        Najlepsze przepisy 2025
    </h5>
    
    <p class="text-text-muted mb-6">
        Odkryj listę najpopularniejszych dań tego roku, od wegańskich deserów po tradycyjne wypieki.
    </p>
    
    <a href="#" class="inline-flex items-center text-white bg-primary box-border border border-transparent hover:bg-primary/90 focus:ring-4 focus:ring-primary/30 shadow-sm font-medium leading-5 rounded-lg text-sm px-4 py-2.5 focus:outline-none transition-all">
        Więcej
        <svg class="w-4 h-4 ms-1.5 rtl:rotate-180 -me-0.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 12H5m14 0-4 4m4-4-4-4"/></svg>
    </a>
    
</div>
<div class="bg-surface block max-w-sm p-6 border border-text-muted/10 rounded-xl shadow-sm hover:shadow-md transition-shadow duration-300 my-1">
    
    <h5 class="mb-3 text-2xl font-semibold tracking-tight text-text-main leading-8">
        Najlepsze przepisy 2025
    </h5>
    
    <p class="text-text-muted mb-6">
        Odkryj listę najpopularniejszych dań tego roku, od wegańskich deserów po tradycyjne wypieki.
    </p>
    
    <a href="#" class="inline-flex items-center text-white bg-primary box-border border border-transparent hover:bg-primary/90 focus:ring-4 focus:ring-primary/30 shadow-sm font-medium leading-5 rounded-lg text-sm px-4 py-2.5 focus:outline-none transition-all">
        Więcej
        <svg class="w-4 h-4 ms-1.5 rtl:rotate-180 -me-0.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 12H5m14 0-4 4m4-4-4-4"/></svg>
    </a>
    
</div>
    }
}