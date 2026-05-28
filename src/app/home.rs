use crate::app::contact::ContactForm;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Hero />
        <SocialProof />
        <Pricing />
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
                <div class="flex justify-center space-x-4">
                    <a
                        href="#contact"
                        class="py-4 px-8 font-bold text-white bg-amber-500 rounded-full shadow-lg transition transform hover:bg-amber-600 hover:scale-105 shadow-amber-200"
                    >
                        "Get Started"
                    </a>
                    <a
                        href="/training"
                        class="py-4 px-8 font-bold rounded-full transition bg-slate-100 text-slate-700 hover:bg-slate-200"
                    >
                        "View Training"
                    </a>
                </div>
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
                <div class="grid grid-cols-2 gap-12 items-center opacity-60 md:grid-cols-4 grayscale">
                    <div class="flex justify-center">
                        <span class="text-2xl font-bold">"TECHCORP"</span>
                    </div>
                    <div class="flex justify-center">
                        <span class="text-2xl font-bold">"DATASYNC"</span>
                    </div>
                    <div class="flex justify-center">
                        <span class="text-2xl font-bold">"VELOFLOW"</span>
                    </div>
                    <div class="flex justify-center">
                        <span class="text-2xl font-bold">"INFRASET"</span>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Pricing() -> impl IntoView {
    view! {
        <section class="py-24 bg-white" id="pricing">
            <div class="container px-6 mx-auto">
                <h2 class="mb-16 text-4xl font-bold text-center text-slate-800">
                    Simple, Transparent Pricing
                </h2>
                <div class="grid grid-cols-1 gap-8 md:grid-cols-3">
                    // Tier 1
                    <div class="p-8 bg-white rounded-3xl border shadow-sm transition hover:shadow-md border-slate-100">
                        <h3 class="mb-4 text-xl font-bold text-slate-800">"Starter"</h3>
                        <p class="mb-6 text-4xl font-black text-slate-900">
                            "$499"
                            <span class="text-base font-normal text-slate-400">"/ project"</span>
                        </p>
                        <ul class="mb-8 space-y-4 text-slate-600">
                            <li>"✓ Basic Bike Assembly"</li>
                            <li>"✓ Standard Component Check"</li>
                            <li>"✓ 1 Year Support"</li>
                        </ul>
                        <button class="py-3 w-full font-bold rounded-xl transition bg-slate-100 text-slate-700 hover:bg-slate-200">
                            "Get Started"
                        </button>
                    </div>

                    // Tier 2 - Popular
                    <div class="relative p-8 bg-white rounded-3xl border-2 border-amber-500 shadow-xl transform scale-105">
                        <div class="absolute top-0 left-1/2 py-1 px-4 text-xs font-bold tracking-wider text-white uppercase bg-amber-500 rounded-full -translate-x-1/2 -translate-y-1/2">
                            "Most Popular"
                        </div>
                        <h3 class="mb-4 text-xl font-bold text-slate-800">"Professional"</h3>
                        <p class="mb-6 text-4xl font-black text-slate-900">
                            "$999"
                            <span class="text-base font-normal text-slate-400">"/ project"</span>
                        </p>
                        <ul class="mb-8 space-y-4 text-slate-600">
                            <li>"✓ Advanced Assembly"</li>
                            <li>"✓ Custom Part Integration"</li>
                            <li>"✓ 24/7 Priority Support"</li>
                            <li>"✓ Personalized Training Session"</li>
                        </ul>
                        <button class="py-3 w-full font-bold text-white bg-amber-500 rounded-xl transition hover:bg-amber-600">
                            "Get Started"
                        </button>
                    </div>

                    // Tier 3
                    <div class="p-8 bg-white rounded-3xl border shadow-sm transition hover:shadow-md border-slate-100">
                        <h3 class="mb-4 text-xl font-bold text-slate-800">"Enterprise"</h3>
                        <p class="mb-6 text-4xl font-black text-slate-900">
                            "Custom"
                            <span class="text-base font-normal text-slate-400">"pricing"</span>
                        </p>
                        <ul class="mb-8 space-y-4 text-slate-600">
                            <li>"✓ Fleet Scale Assembly"</li>
                            <li>"✓ Dedicated Project Manager"</li>
                            <li>"✓ Global On-site Training"</li>
                        </ul>
                        <button class="py-3 w-full font-bold rounded-xl transition bg-slate-100 text-slate-700 hover:bg-slate-200">
                            "Contact Us"
                        </button>
                    </div>
                </div>
            </div>
        </section>
    }
}
