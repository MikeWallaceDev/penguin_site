use leptos::prelude::*;
use leptos_fluent::tr;
use leptos_meta::{provide_meta_context, Link, Meta, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use crate::components::assembly::AssemblyPage;
use crate::components::footer::*;
use crate::components::home::HomePage;
use crate::components::i18n_provider::*;
use crate::components::nav_bar::*;
use crate::components::training::TrainingPage;
use crate::components::tune_ups::TuneUpsPage;

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
