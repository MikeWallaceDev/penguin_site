use crate::components::i18n_provider::Wgt_LanguageSwitcher;
use leptos::prelude::*;
use leptos_fluent::tr;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="sticky top-0 z-50 border-b bg-white/80 backdrop-blur-md border-slate-100">
            <div class="container flex justify-between items-center px-6 mx-auto h-20">
                <a href="/" class="flex items-center space-x-2">
                    <div class="flex justify-center items-center p-1 w-10 h-10 bg-white border-1 border-amber-500 rounded-lg">
                        <img
                            src="/penguin-logo.svg"
                            alt="Techno Penguin"
                            class="w-full h-full object-contain"
                        />
                    </div>
                    <span class="text-2xl font-bold tracking-tight text-slate-900">
                        {move || tr!("brand-name")}
                    </span>
                </a>
                <div class="hidden items-center space-x-8 font-semibold md:flex text-slate-600">
                    <a href="/assembly" class="transition hover:text-amber-500">
                        {move || tr!("assembly")}
                    </a>
                    <a href="/training" class="transition hover:text-amber-500">
                        {move || tr!("training")}
                    </a>
                    <a href="/tune_ups" class="transition hover:text-amber-500">
                        {move || tr!("tune-ups")}
                    </a>
                    <a
                        href="/#contact"
                        class="py-2 px-6 text-white rounded-full transition bg-slate-900 hover:bg-slate-800"
                    >
                        {move || tr!("contact")}
                    </a>
                    <Wgt_LanguageSwitcher />
                </div>
            </div>
        </nav>
    }
}
