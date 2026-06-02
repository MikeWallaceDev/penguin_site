use crate::app::contact::ContactForm;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Hero />
        <SocialProof />
        <ContactForm />
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="overflow-hidden relative pt-24 pb-32 bg-white">
            <div class="absolute top-0 right-0 bg-amber-50 rounded-full opacity-50 translate-x-1/3 -translate-y-1/2 w-[800px] h-[800px] blur-3xl -z-10"></div>
            <div class="absolute bottom-0 left-0 rounded-full opacity-50 -translate-x-1/3 translate-y-1/2 w-[600px] h-[600px] bg-slate-50 blur-3xl -z-10"></div>

            <div class="container px-6 mx-auto text-center">
                <h1 class="mb-6 text-6xl font-extrabold tracking-tight md:text-7xl text-slate-900">
                    "Expertise for the " <span class="text-amber-500">"Masses"</span>
                </h1>
                <p class="mx-auto mb-10 max-w-2xl text-xl leading-relaxed text-slate-600">
                    "Techno Penguin delivers world-class technical services, from expert assembly to advanced training. We empower your business with reliable services."
                </p>
            </div>
        </section>
    }
}

#[component]
fn SocialProof() -> impl IntoView {
    view! {
        <section class="py-16 bg-slate-50">
            <div class="container px-6 mx-auto text-center">
                <p class="mb-10 text-sm font-bold tracking-widest uppercase text-slate-400">
                    "Trusted by Industry Leaders"
                </p>
                <div class="grid grid-cols-2 gap-12 items-center opacity-60 md:grid-cols-2 grayscale">
                    <div class="flex justify-center">
                        <span class="text-2xl font-bold">"Parc de la Rivière-des-Mille-Iles"</span>
                    </div>
                    <div class="flex justify-center">
                        <span class="text-2xl font-bold">"Psi Vélo"</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
