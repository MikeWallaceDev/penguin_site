use leptos::prelude::*;
use leptos_fluent::tr;

#[component]
pub fn TrainingPage() -> impl IntoView {
    view! {
        <div class="container py-24 px-6 mx-auto">
            <h1 class="mb-8 text-5xl font-extrabold text-slate-900">
                {move || tr!("training-title")}
            </h1>

            <p class="mb-12 max-w-3xl text-xl leading-relaxed text-slate-600">
                {move || tr!("training-description")}
            </p>

            <div class="overflow-hidden rounded-2xl border border-slate-200">
                <table class="min-w-full divide-y divide-slate-200">
                    <thead class="bg-slate-50">
                        <tr>
                            <th class="py-4 px-6 text-xs font-bold tracking-wider text-left uppercase text-slate-500">
                                {move || tr!("course-name")}
                            </th>
                            <th class="py-4 px-6 text-xs font-bold tracking-wider text-left uppercase text-slate-500">
                                {move || tr!("duration")}
                            </th>
                            <th class="py-4 px-6 text-xs font-bold tracking-wider text-left uppercase text-slate-500">
                                {move || tr!("price")}
                            </th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-slate-200">
                        <tr>
                            <td class="py-4 px-6 text-sm font-medium whitespace-nowrap text-slate-900">
                                "Basic Assembly 101"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "2 Days"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "$599"
                            </td>
                        </tr>
                        <tr>
                            <td class="py-4 px-6 text-sm font-medium whitespace-nowrap text-slate-900">
                                "Advanced Maintenance"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "5 Days"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "$1,299"
                            </td>
                        </tr>
                        <tr>
                            <td class="py-4 px-6 text-sm font-medium whitespace-nowrap text-slate-900">
                                "System Optimization"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "3 Days"
                            </td>
                            <td class="py-4 px-6 text-sm whitespace-nowrap text-slate-500">
                                "$899"
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
