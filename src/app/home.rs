use leptos::prelude::*;
use crate::app::contact::ContactForm;

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
        <section class="relative overflow-hidden bg-white pt-24 pb-32">
            <div class="absolute top-0 right-0 -translate-y-1/2 translate-x-1/3 w-[800px] h-[800px] bg-amber-50 rounded-full blur-3xl opacity-50 -z-10"></div>
            <div class="absolute bottom-0 left-0 translate-y-1/2 -translate-x-1/3 w-[600px] h-[600px] bg-slate-50 rounded-full blur-3xl opacity-50 -z-10"></div>
            
            <div class="container mx-auto px-6 text-center">
                <h1 class="text-6xl md:text-7xl font-extrabold text-slate-900 mb-6 tracking-tight">
                    "Precision Engineering for the "
                    <span class="text-amber-500">"Digital Era"</span>
                </h1>
                <p class="text-xl text-slate-600 mb-10 max-w-2xl mx-auto leading-relaxed">
                    "Techno Penguin delivers world-class technical services, from expert assembly to advanced training. We empower your business with reliable infrastructure."
                </p>
                <div class="flex justify-center space-x-4">
                    <a href="#contact" class="px-8 py-4 bg-amber-500 hover:bg-amber-600 text-white font-bold rounded-full transition transform hover:scale-105 shadow-lg shadow-amber-200">
                        "Get Started"
                    </a>
                    <a href="/training" class="px-8 py-4 bg-slate-100 hover:bg-slate-200 text-slate-700 font-bold rounded-full transition">
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
            <div class="container mx-auto px-6 text-center">
                <p class="text-sm font-bold text-slate-400 uppercase tracking-widest mb-10">"Trusted by Industry Leaders"</p>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-12 items-center opacity-60 grayscale">
                    <div class="flex justify-center"><span class="text-2xl font-bold">"TECHCORP"</span></div>
                    <div class="flex justify-center"><span class="text-2xl font-bold">"DATASYNC"</span></div>
                    <div class="flex justify-center"><span class="text-2xl font-bold">"VELOFLOW"</span></div>
                    <div class="flex justify-center"><span class="text-2xl font-bold">"INFRASET"</span></div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Pricing() -> impl IntoView {
    view! {
        <section class="py-24 bg-white" id="pricing">
            <div class="container mx-auto px-6">
                <h2 class="text-4xl font-bold text-slate-800 mb-16 text-center">Simple, Transparent Pricing</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    // Tier 1
                    <div class="p-8 bg-white border border-slate-100 rounded-3xl shadow-sm hover:shadow-md transition">
                        <h3 class="text-xl font-bold text-slate-800 mb-4">"Starter"</h3>
                        <p class="text-4xl font-black text-slate-900 mb-6">"$499" <span class="text-base font-normal text-slate-400">"/ project"</span></p>
                        <ul class="space-y-4 mb-8 text-slate-600">
                            <li>"✓ Basic Bike Assembly"</li>
                            <li>"✓ Standard Component Check"</li>
                            <li>"✓ 1 Year Support"</li>
                        </ul>
                        <button class="w-full py-3 bg-slate-100 hover:bg-slate-200 text-slate-700 font-bold rounded-xl transition">"Get Started"</button>
                    </div>
                    
                    // Tier 2 - Popular
                    <div class="p-8 bg-white border-2 border-amber-500 rounded-3xl shadow-xl relative transform scale-105">
                        <div class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-amber-500 text-white px-4 py-1 rounded-full text-xs font-bold uppercase tracking-wider">
                            "Most Popular"
                        </div>
                        <h3 class="text-xl font-bold text-slate-800 mb-4">"Professional"</h3>
                        <p class="text-4xl font-black text-slate-900 mb-6">"$999" <span class="text-base font-normal text-slate-400">"/ project"</span></p>
                        <ul class="space-y-4 mb-8 text-slate-600">
                            <li>"✓ Advanced Assembly"</li>
                            <li>"✓ Custom Part Integration"</li>
                            <li>"✓ 24/7 Priority Support"</li>
                            <li>"✓ Personalized Training Session"</li>
                        </ul>
                        <button class="w-full py-3 bg-amber-500 hover:bg-amber-600 text-white font-bold rounded-xl transition">"Get Started"</button>
                    </div>
                    
                    // Tier 3
                    <div class="p-8 bg-white border border-slate-100 rounded-3xl shadow-sm hover:shadow-md transition">
                        <h3 class="text-xl font-bold text-slate-800 mb-4">"Enterprise"</h3>
                        <p class="text-4xl font-black text-slate-900 mb-6">"Custom" <span class="text-base font-normal text-slate-400">"pricing"</span></p>
                        <ul class="space-y-4 mb-8 text-slate-600">
                            <li>"✓ Fleet Scale Assembly"</li>
                            <li>"✓ Dedicated Project Manager"</li>
                            <li>"✓ Global On-site Training"</li>
                        </ul>
                        <button class="w-full py-3 bg-slate-100 hover:bg-slate-200 text-slate-700 font-bold rounded-xl transition">"Contact Us"</button>
                    </div>
                </div>
            </div>
        </section>
    }
}
