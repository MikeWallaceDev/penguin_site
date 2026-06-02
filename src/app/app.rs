use leptos::prelude::*;
use leptos_fluent::{tr, I18n};
use leptos_meta::{provide_meta_context, Link, Meta, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use crate::app::assembly::AssemblyPage;
use crate::app::home::HomePage;
use crate::app::i18n_provider::*;
use crate::app::training::TrainingPage;
use crate::app::tune_ups::TuneUpsPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/techno_penguin.css" />
        <Link rel="shortcut icon" type_="image/svg+xml" href="/favicon.svg" />
        <Title text=move || tr!("title") />
        <Meta name="description" content=move || tr!("description") />

        <I18nProvider>
            <Router>
                <Navbar />
                <main class="min-h-screen bg-white">
                    <Routes fallback=|| "Page not found.">
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=StaticSegment("training") view=TrainingPage />
                        <Route path=StaticSegment("assembly") view=AssemblyPage />
                        <Route path=StaticSegment("tune_ups") view=TuneUpsPage />
                    </Routes>
                </main>
                <Footer />
            </Router>
        </I18nProvider>
    }
}

#[component]
fn LanguageSwitcher() -> impl IntoView {
    let i18n = expect_context::<I18n>();
    view! {
        <div class="flex items-center p-1 space-x-1 rounded-full border bg-slate-100 border-slate-200">
            <button
                class=move || {
                    let active = i18n.language.get().id.to_string() == "en";
                    format!(
                        "px-3 py-1 rounded-full text-xs font-bold transition-all {}",
                        if active {
                            "bg-white text-slate-900 shadow-sm"
                        } else {
                            "text-slate-500 hover:text-slate-700"
                        },
                    )
                }
                on:click=move |_| {
                    if let Some(target_lang) = i18n
                        .languages
                        .iter()
                        .find(|l| l.id.to_string() == "en")
                    {
                        i18n.language.set(target_lang);
                    }
                }
            >
                "EN"
            </button>
            <button
                class=move || {
                    let active = i18n.language.get().id.to_string() == "fr";
                    format!(
                        "px-3 py-1 rounded-full text-xs font-bold transition-all {}",
                        if active {
                            "bg-white text-slate-900 shadow-sm"
                        } else {
                            "text-slate-500 hover:text-slate-700"
                        },
                    )
                }
                on:click=move |_| {
                    if let Some(target_lang) = i18n
                        .languages
                        .iter()
                        .find(|l| l.id.to_string() == "fr")
                    {
                        i18n.language.set(target_lang);
                    }
                }
            >
                "FR"
            </button>
        </div>
    }
}

#[component]
fn Navbar() -> impl IntoView {
    view! {
        <nav class="sticky top-0 z-50 border-b bg-white/80 backdrop-blur-md border-slate-100">
            <div class="container flex justify-between items-center px-6 mx-auto h-20">
                <a href="/" class="flex items-center space-x-2">
                    <div class="flex justify-center items-center w-10 h-10 bg-amber-500 rounded-lg">
                        <span class="text-xl font-black text-white">"TP"</span>
                    </div>
                    <span class="text-2xl font-bold tracking-tight text-slate-900">
                        {move || tr!("brand-name")}
                    </span>
                </a>
                <div class="hidden items-center space-x-8 font-semibold md:flex text-slate-600">
                    <a href="/training" class="transition hover:text-amber-500">
                        {move || tr!("training")}
                    </a>
                    <a href="/assembly" class="transition hover:text-amber-500">
                        {move || tr!("assembly")}
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
                    <LanguageSwitcher />
                </div>
            </div>
        </nav>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="py-12 bg-slate-900 text-slate-400">
            <div class="container grid grid-cols-1 gap-12 px-6 mx-auto md:grid-cols-4">
                <div class="col-span-1 md:col-span-2">
                    <div class="flex items-center mb-6 space-x-2">
                        <div class="flex justify-center items-center w-8 h-8 bg-amber-500 rounded-md">
                            <span class="text-sm font-bold text-white">"TP"</span>
                        </div>
                        <span class="text-xl font-bold tracking-tight text-white">
                            {move || tr!("brand-name")}
                        </span>
                    </div>
                    <p class="max-w-xs">{move || tr!("footer-tagline")}</p>
                </div>
                <div>
                    <h4 class="mb-4 font-bold text-white">{move || tr!("services")}</h4>
                    <ul class="space-y-2">
                        <li>
                            <a href="/assembly" class="transition hover:text-amber-500">
                                {move || tr!("assembly")}
                            </a>
                        </li>
                        <li>
                            <a href="/training" class="transition hover:text-amber-500">
                                {move || tr!("training")}
                            </a>
                        </li>
                        <li>
                            <a href="/tune_ups" class="transition hover:text-amber-500">
                                {move || tr!("tune-ups")}
                            </a>
                        </li>
                    </ul>
                </div>
                <div>
                    <h4 class="mb-4 font-bold text-white">{move || tr!("connect")}</h4>
                    <ul class="space-y-2">
                        <li>
                            <a href="/#contact" class="transition hover:text-amber-500">
                                {move || tr!("contact")}
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
            <div class="container px-6 pt-12 mx-auto mt-12 text-sm text-center border-t border-slate-800">
                {move || tr!("all-rights-reserved")}
            </div>
        </footer>
    }
}
