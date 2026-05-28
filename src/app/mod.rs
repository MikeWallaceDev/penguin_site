pub mod assembly;
pub mod contact;
pub mod home;
pub mod training;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use crate::app::assembly::AssemblyPage;
use crate::app::home::HomePage;
use crate::app::training::TrainingPage;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/techno_penguin.css" />
        <Link rel="shortcut icon" type_="image/svg+xml" href="/favicon.svg" />
        <Title text="Techno Penguin | Professional Technical Services" />
        <Meta name="description" content="Expert technical assembly and training services." />

        <Router>
            <Navbar />
            <main class="min-h-screen bg-white">
                <Routes fallback=|| "Page not found.">
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("training") view=TrainingPage />
                    <Route path=StaticSegment("assembly") view=AssemblyPage />
                </Routes>
            </main>
            <Footer />
        </Router>
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
                        "Techno Penguin"
                    </span>
                </a>
                <div class="hidden items-center space-x-8 font-semibold md:flex text-slate-600">
                    <a href="/#pricing" class="transition hover:text-amber-500">
                        "Pricing"
                    </a>
                    <a href="/training" class="transition hover:text-amber-500">
                        "Training"
                    </a>
                    <a href="/assembly" class="transition hover:text-amber-500">
                        "Assembly"
                    </a>
                    <a
                        href="/#contact"
                        class="py-2 px-6 text-white rounded-full transition bg-slate-900 hover:bg-slate-800"
                    >
                        "Contact"
                    </a>
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
                            "Techno Penguin"
                        </span>
                    </div>
                    <p class="max-w-xs">
                        "Delivering precision engineering and technical training for modern enterprises."
                    </p>
                </div>
                <div>
                    <h4 class="mb-4 font-bold text-white">"Services"</h4>
                    <ul class="space-y-2">
                        <li>
                            <a href="/assembly" class="transition hover:text-amber-500">
                                "Assembly"
                            </a>
                        </li>
                        <li>
                            <a href="/training" class="transition hover:text-amber-500">
                                "Training"
                            </a>
                        </li>
                        <li>
                            <a href="/#pricing" class="transition hover:text-amber-500">
                                "Pricing"
                            </a>
                        </li>
                    </ul>
                </div>
                <div>
                    <h4 class="mb-4 font-bold text-white">"Connect"</h4>
                    <ul class="space-y-2">
                        <li>
                            <a href="#" class="transition hover:text-amber-500">
                                "Twitter"
                            </a>
                        </li>
                        <li>
                            <a href="#" class="transition hover:text-amber-500">
                                "LinkedIn"
                            </a>
                        </li>
                        <li>
                            <a href="/#contact" class="transition hover:text-amber-500">
                                "Contact"
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
            <div class="container px-6 pt-12 mx-auto mt-12 text-sm text-center border-t border-slate-800">
                "© 2026 Techno Penguin. All rights reserved."
            </div>
        </footer>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_logic() {
        assert_eq!(1 + 1, 2);
    }
}
