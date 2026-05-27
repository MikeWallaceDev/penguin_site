use leptos::prelude::*;

#[component]
pub fn AssemblyPage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-6 py-24">
            <h1 class="text-5xl font-extrabold text-slate-900 mb-8">"Bicycle Assembly"</h1>
            
            <p class="text-xl text-slate-600 mb-12 max-w-3xl leading-relaxed">
                "We have years of experience in assembling high-end bicycles and performance-oriented machinery. Our precision-focused approach ensures that every component is perfectly aligned and calibrated for maximum efficiency and safety."
            </p>

            <div class="overflow-hidden border border-slate-200 rounded-2xl">
                <table class="min-w-full divide-y divide-slate-200">
                    <thead class="bg-slate-50">
                        <tr>
                            <th class="px-6 py-4 text-left text-xs font-bold text-slate-500 uppercase tracking-wider">"Service Type"</th>
                            <th class="px-6 py-4 text-left text-xs font-bold text-slate-500 uppercase tracking-wider">"Turnaround"</th>
                            <th class="px-6 py-4 text-left text-xs font-bold text-slate-500 uppercase tracking-wider">"Starting Price"</th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-slate-200">
                        <tr>
                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-slate-900">"Road Bike Pro Build"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"48 Hours"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"$299"</td>
                        </tr>
                        <tr>
                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-slate-900">"MTB Custom Assembly"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"72 Hours"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"$349"</td>
                        </tr>
                        <tr>
                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-slate-900">"E-Bike Technical Setup"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"24 Hours"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"$199"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
