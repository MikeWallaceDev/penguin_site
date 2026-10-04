use leptos::prelude::*;
use leptos_fluent::tr;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="py-12 bg-slate-900 text-slate-400">
            <div class="container grid grid-cols-1 gap-12 px-6 mx-auto md:grid-cols-4">
                <div class="col-span-1 md:col-span-2">
                    <div class="flex items-center mb-6 space-x-2">
                        <div class="flex justify-center items-center w-8 h-8 bg-white p-0.5 border-1 border-amber-500 rounded-lg">
                            <img
                                src="/penguin-logo.svg"
                                alt="Techno Penguin"
                                class="w-full h-full object-contain"
                            />
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
