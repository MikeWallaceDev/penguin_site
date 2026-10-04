use leptos::prelude::*;
use leptos_fluent::tr;

use crate::routes::*;

#[component]
pub fn ContactForm() -> impl IntoView {
    let save_contact = ServerAction::<SaveContact>::new();

    let is_success = move || {
        save_contact
            .value()
            .get()
            .map(|v| v.is_ok())
            .unwrap_or(false)
    };
    let is_error = move || {
        save_contact
            .value()
            .get()
            .map(|v| v.is_err())
            .unwrap_or(false)
    };

    view! {
        <section class="py-20 bg-white" id="contact">
            <div class="container px-6 mx-auto max-w-4xl">
                <h2 class="mb-8 text-4xl font-bold text-center text-slate-800">
                    {move || tr!("get-in-touch")}
                </h2>
                <div class="p-8 rounded-2xl border shadow-sm bg-slate-50 border-slate-100">
                    <ActionForm action=save_contact>
                        <div class="grid grid-cols-1 gap-6 mb-6 md:grid-cols-2">
                            <div>
                                <label class="block mb-2 text-sm font-semibold text-slate-600">
                                    {move || tr!("name")}
                                </label>
                                <input
                                    type="text"
                                    name="name"
                                    required
                                    class="py-3 px-4 w-full rounded-lg border transition outline-none focus:border-transparent focus:ring-2 focus:ring-amber-500 border-slate-200"
                                    placeholder=move || tr!("placeholder-name")
                                />
                            </div>
                            <div>
                                <label class="block mb-2 text-sm font-semibold text-slate-600">
                                    {move || tr!("email")}
                                </label>
                                <input
                                    type="email"
                                    name="email"
                                    required
                                    class="py-3 px-4 w-full rounded-lg border transition outline-none focus:border-transparent focus:ring-2 focus:ring-amber-500 border-slate-200"
                                    placeholder=move || tr!("placeholder-email")
                                />
                            </div>
                        </div>
                        <div class="mb-6">
                            <label class="block mb-2 text-sm font-semibold text-slate-600">
                                {move || tr!("message")}
                            </label>
                            <textarea
                                name="message"
                                required
                                rows="4"
                                class="py-3 px-4 w-full rounded-lg border transition outline-none focus:border-transparent focus:ring-2 focus:ring-amber-500 border-slate-200"
                                placeholder=move || tr!("placeholder-message")
                            ></textarea>
                        </div>
                        <button
                            type="submit"
                            class="py-4 w-full font-bold text-white bg-amber-500 rounded-lg transition transform hover:bg-amber-600 hover:-translate-y-1"
                        >
                            {move || tr!("send-message")}
                        </button>
                    </ActionForm>

                    {move || {
                        is_success()
                            .then(|| {
                                view! {
                                    <p class="mt-4 font-semibold text-center text-green-600">
                                        {move || tr!("send-message-success")}
                                    </p>
                                }
                            })
                    }}
                    {move || {
                        is_error()
                            .then(|| {
                                view! {
                                    <p class="mt-4 font-semibold text-center text-red-600">
                                        {move || tr!("send-message-failure")}
                                    </p>
                                }
                            })
                    }}
                </div>
            </div>
        </section>
    }
}
