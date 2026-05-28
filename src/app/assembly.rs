use leptos::prelude::*;

#[component]
pub fn AssemblyPage() -> impl IntoView {
    view! {
        <div class="container py-24 px-6 mx-auto">
            <h1 class="mb-8 text-5xl font-extrabold text-slate-900">"Product Assembly"</h1>

            <p class="mb-12 max-w-3xl text-xl leading-relaxed text-slate-600">
                "Our speciality is assembling high-end bicycles. Our precision-focused approach ensures that every component is perfectly aligned and calibrated for maximum efficiency and safety."
            </p>

            <div class="overflow-hidden rounded-2xl border border-slate-200">
                <table class="min-w-full divide-y divide-slate-200">
                    <thead class="bg-slate-50">
                        <tr>
                            <th class="py-4 px-6 text-xs font-bold tracking-wider text-left uppercase text-slate-500">
                                "Service Type"
                            </th>
                            <th class="py-4 px-6 text-xs font-bold tracking-wider text-left uppercase text-slate-500">
                                "Turnaround"
                            </th>
                            <th class="py-4 px-6 text-xs font-bold tracking-wider text-left uppercase text-slate-500">
                                "Starting Price"
                            </th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-slate-200">
                        <tr>
                            <td class="py-4 px-6 text-sm font-medium whitespace-nowrap text-slate-900">
                                "Road Bike Pro Build"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "48 Hours"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "$299"
                            </td>
                        </tr>
                        <tr>
                            <td class="py-4 px-6 text-sm font-medium whitespace-nowrap text-slate-900">
                                "MTB Custom Assembly"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "72 Hours"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "$349"
                            </td>
                        </tr>
                        <tr>
                            <td class="py-4 px-6 text-sm font-medium whitespace-nowrap text-slate-900">
                                "E-Bike Technical Setup"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "24 Hours"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "$199"
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
