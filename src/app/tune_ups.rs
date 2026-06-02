use leptos::prelude::*;

#[component]
pub fn TuneUpsPage() -> impl IntoView {
    view! {
        <div class="container py-24 px-6 mx-auto">
            <h1 class="mb-8 text-5xl font-extrabold text-slate-900">"Tune-Ups"</h1>

            <p class="mb-12 max-w-3xl text-xl leading-relaxed text-slate-600">
                "Our tune-ups are amazing."
            </p>

            <Pricing />
        </div>
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
                        <h3 class="mb-4 text-xl font-bold text-slate-800">"Silver"</h3>
                        <p class="mb-6 text-4xl font-black text-slate-900">"$74.99"</p>
                        <ul class="mb-8 space-y-4 text-slate-600">
                            <li>"✓ Visual Inspection"</li>
                            <li>"✓ Verify and tighten all bolts"</li>
                            <li>"✓ Adjust both brakes"</li>
                            <li>"✓ Adjust both dérailleurs"</li>
                            <li>"✓ Adjust both hubs"</li>
                            <li>"✓ Adjust both headset"</li>
                            <li>"✓ Clean and lube chain"</li>
                            <li>"✓ Verify tire pressure and wear"</li>
                            <li>"✓ Fill tires to recommended pressure"</li>
                            <li>"✓ Recommendations"</li>
                        </ul>
                    </div>

                    // Tier 2 - Popular
                    <div class="relative p-8 bg-white rounded-3xl border-2 border-amber-500 shadow-xl transform scale-105">
                        <div class="absolute top-0 left-1/2 py-1 px-4 text-xs font-bold tracking-wider text-white uppercase bg-amber-500 rounded-full -translate-x-1/2 -translate-y-1/2">
                            "Most Popular"
                        </div>
                        <h3 class="mb-4 text-xl font-bold text-slate-800">"Gold"</h3>
                        <p class="mb-6 text-4xl font-black text-slate-900">"$159.99"</p>
                        <ul class="mb-8 space-y-4 text-slate-600">
                            <li>"✓ Visual Inspection"</li>
                            <li>"✓ Verify and tighten all bolts"</li>
                            <li>"✓ Adjust both brakes"</li>
                            <li>"✓ Adjust both dérailleurs"</li>
                            <li>"✓ Adjust both hubs"</li>
                            <li>"✓ Adjust both headset"</li>
                            <li>"✓ Clean and lube chain"</li>
                            <li>"✓ Verify tire pressure and wear"</li>
                            <li>"✓ Fill tires to recommended pressure"</li>
                            <li>"✓ Recommendations"</li>
                            <li>"✓ Clean frame"</li>
                            <li>"✓ Clean wheels"</li>
                            <li>"✓ Clean and lube both dérailleurs"</li>
                            <li>"✓ Clean and grease the casette"</li>
                        </ul>
                    </div>

                    // Tier 3
                    <div class="p-8 bg-white rounded-3xl border shadow-sm transition hover:shadow-md border-slate-100">
                        <h3 class="mb-4 text-xl font-bold text-slate-800">"Platinum"</h3>
                        <p class="mb-6 text-4xl font-black text-slate-900">"$319.99"</p>
                        <ul class="mb-8 space-y-4 text-slate-600">
                            <li>"✓ Visual Inspection"</li>
                            <li>"✓ Verify and tighten all bolts"</li>
                            <li>"✓ Adjust both brakes"</li>
                            <li>"✓ Adjust both dérailleurs"</li>
                            <li>"✓ Adjust both hubs"</li>
                            <li>"✓ Adjust both headset"</li>
                            <li>"✓ Clean and lube chain"</li>
                            <li>"✓ Verify tire pressure and wear"</li>
                            <li>"✓ Fill tires to recommended pressure"</li>
                            <li>"✓ Recommendations"</li>
                            <li>"✓ Clean frame"</li>
                            <li>"✓ Clean wheels"</li>
                            <li>"✓ Clean and lube both dérailleurs"</li>
                            <li>"✓ Clean and grease the casette"</li>
                            <li>"✓ Clean and grease headset"</li>
                            <li>"✓ Clean and grease both hubs"</li>
                            <li>"✓ Align both wheels"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}
#[component]
fn Table() -> impl IntoView {
    view! {
        <div class="overflow-hidden rounded-2xl border border-slate-200">
            <table class="min-w-full divide-y divide-slate-200">
                <thead class="bg-slate-50">
                    <tr>
                        <th class="py-4 px-6 text-xs font-bold tracking-wider text-left uppercase text-slate-500">
                            "Course Name"
                        </th>
                        <th class="py-4 px-6 text-xs font-bold tracking-wider text-left uppercase text-slate-500">
                            "Duration"
                        </th>
                        <th class="py-4 px-6 text-xs font-bold tracking-wider text-left uppercase text-slate-500">
                            "Price"
                        </th>
                    </tr>
                </thead>
                <tbody class="bg-white divide-y divide-slate-200">
                    <tr>
                        <td class="py-4 px-6 text-sm font-medium whitespace-nowrap text-slate-900">
                            "Basic Assembly 101"
                        </td>
                        <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">"2 Days"</td>
                        <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">"$599"</td>
                    </tr>
                    <tr>
                        <td class="py-4 px-6 text-sm font-medium whitespace-nowrap text-slate-900">
                            "Advanced Maintenance"
                        </td>
                        <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">"5 Days"</td>
                        <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">"$1,299"</td>
                    </tr>
                    <tr>
                        <td class="py-4 px-6 text-sm font-medium whitespace-nowrap text-slate-900">
                            "System Optimization"
                        </td>
                        <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">"3 Days"</td>
                        <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">"$899"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
