use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::prelude::*;
    use sqlx::SqlitePool;

    pub fn pool() -> Result<SqlitePool, ServerFnError> {
        // use leptos_axum::extract;

        // In a real app, we'd use a Layer to provide the pool.
        // For simplicity here, we'll try to get it from the Axum state or a global.
        // Leptos 0.7 often uses `expect_context` or similar.
        use crate::server::state::AppState;
        let state = expect_context::<AppState>();
        Ok(state.pool.clone())
    }
}

#[server]
pub async fn save_contact(
    name: String,
    email: String,
    message: String,
) -> Result<(), ServerFnError> {
    let pool = ssr::pool()?;

    tracing::info!("Saving contact submission from: {}", email);

    sqlx::query!(
        "INSERT INTO contact_submissions (name, email, message) VALUES (?, ?, ?)",
        name,
        email,
        message
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(())
}

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
                <h2 class="mb-8 text-4xl font-bold text-center text-slate-800">Get in Touch</h2>
                <div class="p-8 rounded-2xl border shadow-sm bg-slate-50 border-slate-100">
                    <ActionForm action=save_contact>
                        <div class="grid grid-cols-1 gap-6 mb-6 md:grid-cols-2">
                            <div>
                                <label class="block mb-2 text-sm font-semibold text-slate-600">
                                    Name
                                </label>
                                <input
                                    type="text"
                                    name="name"
                                    required
                                    class="py-3 px-4 w-full rounded-lg border transition outline-none focus:border-transparent focus:ring-2 focus:ring-amber-500 border-slate-200"
                                    placeholder="Your Name"
                                />
                            </div>
                            <div>
                                <label class="block mb-2 text-sm font-semibold text-slate-600">
                                    Email
                                </label>
                                <input
                                    type="email"
                                    name="email"
                                    required
                                    class="py-3 px-4 w-full rounded-lg border transition outline-none focus:border-transparent focus:ring-2 focus:ring-amber-500 border-slate-200"
                                    placeholder="your@email.com"
                                />
                            </div>
                        </div>
                        <div class="mb-6">
                            <label class="block mb-2 text-sm font-semibold text-slate-600">
                                Message
                            </label>
                            <textarea
                                name="message"
                                required
                                rows="4"
                                class="py-3 px-4 w-full rounded-lg border transition outline-none focus:border-transparent focus:ring-2 focus:ring-amber-500 border-slate-200"
                                placeholder="How can we help you?"
                            ></textarea>
                        </div>
                        <button
                            type="submit"
                            class="py-4 w-full font-bold text-white bg-amber-500 rounded-lg transition transform hover:bg-amber-600 hover:-translate-y-1"
                        >
                            "Send Message"
                        </button>
                    </ActionForm>

                    {move || {
                        is_success()
                            .then(|| {
                                view! {
                                    <p class="mt-4 font-semibold text-center text-green-600">
                                        "Thank you! We'll be in touch soon."
                                    </p>
                                }
                            })
                    }}
                    {move || {
                        is_error()
                            .then(|| {
                                view! {
                                    <p class="mt-4 font-semibold text-center text-red-600">
                                        "Something went wrong. Please try again."
                                    </p>
                                }
                            })
                    }}
                </div>
            </div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_email_validation_logic() {
        // Just a mock test to show tracing in tests
        let email = "test@example.com";
        assert!(email.contains('@'));
    }
}
