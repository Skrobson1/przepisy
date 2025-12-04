use leptos::prelude::*;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="h-screen w-screen flex flex-col justify-center items-center bg-background transition-colors duration-300">
    
    <div class="w-full max-w-sm px-4">
        
        <div class="mb-4">
            <label for="email" class="block text-sm font-medium text-text-main mb-1">Adres Email</label>
            <input
                type="email"
                id="email"
                class="w-full px-4 py-2 bg-surface border border-text-muted/20 text-text-main rounded-md focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent placeholder:text-text-muted/50 transition duration-200"
                placeholder="email@domain.com"
                required
            />
        </div>

        <div class="mb-6">
            <label for="password" class="block text-sm font-medium text-text-main mb-1">Hasło</label>
            <input 
                type="password" 
                id="password" 
                class="w-full px-4 py-2 bg-surface border border-text-muted/20 text-text-main rounded-md focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent placeholder:text-text-muted/50 transition duration-200" 
                placeholder="••••••••"
                required
            />
            
            <div class="flex justify-end mt-2">
                <a href="#" class="text-sm text-primary hover:text-primary/80 hover:underline transition-colors">
                    Zapomniałeś hasła?
                </a>
            </div>
        </div>

        <button 
            type="submit" 
            class="w-full bg-primary text-white font-semibold py-2 px-4 rounded-md hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary/50 focus:ring-offset-2 focus:ring-offset-background transition duration-200"
        >
            Zaloguj się
        </button>
        
    </div>
</div>
    }
}