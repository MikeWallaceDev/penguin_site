use leptos::prelude::*;

#[component]
pub fn TrainingPage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-6 py-24">
            <h1 class="text-5xl font-extrabold text-slate-900 mb-8">"Expert Training"</h1>
            
            <p class="text-xl text-slate-600 mb-12 max-w-3xl leading-relaxed">
                "Our training programs are designed to equip your team with the latest skills in technical assembly, maintenance, and system optimization. We provide hands-on experience and comprehensive theory to ensure your staff can handle any challenge with confidence."
            </p>

            <div class="overflow-hidden border border-slate-200 rounded-2xl">
                <table class="min-w-full divide-y divide-slate-200">
                    <thead class="bg-slate-50">
                        <tr>
                            <th class="px-6 py-4 text-left text-xs font-bold text-slate-500 uppercase tracking-wider">"Course Name"</th>
                            <th class="px-6 py-4 text-left text-xs font-bold text-slate-500 uppercase tracking-wider">"Duration"</th>
                            <th class="px-6 py-4 text-left text-xs font-bold text-slate-500 uppercase tracking-wider">"Price"</th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-slate-200">
                        <tr>
                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-slate-900">"Basic Assembly 101"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"2 Days"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"$599"</td>
                        </tr>
                        <tr>
                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-slate-900">"Advanced Maintenance"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"5 Days"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"$1,299"</td>
                        </tr>
                        <tr>
                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-slate-900">"System Optimization"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"3 Days"</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-slate-500">"$899"</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
