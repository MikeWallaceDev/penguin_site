use leptos::html;
use leptos::prelude::*;
use leptos_fluent::tr;

use crate::routes::*;
use crate::validation::{self, ContactResponse, Field, FieldError, ValidationCode};

fn field_classes(invalid: bool, valid: bool) -> String {
    let state = if invalid {
        "border-red-500 focus:ring-red-500"
    } else if valid {
        "border-green-500 focus:ring-green-500"
    } else {
        "border-slate-200 focus:ring-amber-500"
    };

    format!(
        "py-3 px-4 w-full rounded-lg border transition outline-none focus:border-transparent focus:ring-2 {state}"
    )
}

/// Localized message for a field error. Each key is referenced with a plain
/// `tr!` call so leptos-fluent's translation checker sees every message.
fn validation_text(field: Field, code: ValidationCode) -> String {
    match field {
        Field::Name => {
            if code == ValidationCode::Required {
                tr!("validation-name-required")
            } else if code == ValidationCode::TooShort {
                tr!("validation-name-too-short")
            } else {
                tr!("validation-name-too-long")
            }
        }
        Field::Email => {
            if code == ValidationCode::Required {
                tr!("validation-email-required")
            } else {
                tr!("validation-email-invalid")
            }
        }
        Field::Message => {
            if code == ValidationCode::Required {
                tr!("validation-message-required")
            } else if code == ValidationCode::TooShort {
                tr!("validation-message-too-short")
            } else {
                tr!("validation-message-too-long")
            }
        }
    }
}

#[component]
pub fn ContactForm() -> impl IntoView {
    let save_contact = ServerAction::<SaveContact>::new();
    let action_value = save_contact.value();
    let is_pending = save_contact.pending();

    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());

    let name_touched = RwSignal::new(false);
    let email_touched = RwSignal::new(false);
    let message_touched = RwSignal::new(false);

    let server_errors = RwSignal::new(Vec::<FieldError>::new());

    let name_ref = NodeRef::<html::Input>::new();
    let email_ref = NodeRef::<html::Input>::new();
    let message_ref = NodeRef::<html::Textarea>::new();

    let touched = move |field: Field| match field {
        Field::Name => name_touched.get(),
        Field::Email => email_touched.get(),
        Field::Message => message_touched.get(),
    };

    let value = move |field: Field| match field {
        Field::Name => name.get(),
        Field::Email => email.get(),
        Field::Message => message.get(),
    };

    let server_code = move |field: Field| {
        server_errors
            .get()
            .into_iter()
            .find(|error| error.field == field)
            .map(|error| error.code)
    };

    // Client-side code for a field, only once the user has left it (non-jarring),
    // falling back to any error returned by the server.
    let field_code = move |field: Field| {
        let client = if touched(field) {
            match field {
                Field::Name => validation::validate_name(&value(field)),
                Field::Email => validation::validate_email(&value(field)),
                Field::Message => validation::validate_message(&value(field)),
            }
        } else {
            None
        };

        client.or_else(|| server_code(field))
    };

    let is_invalid = move |field: Field| field_code(field).is_some();

    let is_valid =
        move |field: Field| touched(field) && !is_invalid(field) && !value(field).trim().is_empty();

    let focus_field = move |field: Field| match field {
        Field::Name => {
            if let Some(element) = name_ref.get() {
                let _ = element.focus();
            }
        }
        Field::Email => {
            if let Some(element) = email_ref.get() {
                let _ = element.focus();
            }
        }
        Field::Message => {
            if let Some(element) = message_ref.get() {
                let _ = element.focus();
            }
        }
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        name_touched.set(true);
        email_touched.set(true);
        message_touched.set(true);
        server_errors.set(Vec::new());

        let errors = validation::validate_contact(&name.get(), &email.get(), &message.get());
        if let Some(first) = errors.first() {
            focus_field(first.field);
            return;
        }

        save_contact.dispatch(SaveContact {
            name: name.get_untracked(),
            email: email.get_untracked(),
            message: message.get_untracked(),
        });
    };

    // React to the server response: surface field errors or reset on success.
    Effect::new(move || match action_value.get() {
        Some(Ok(ContactResponse::Invalid(errors))) => {
            if let Some(first) = errors.first() {
                focus_field(first.field);
            }
            server_errors.set(errors);
        }
        Some(Ok(ContactResponse::Ok)) => {
            server_errors.set(Vec::new());
            name.set(String::new());
            email.set(String::new());
            message.set(String::new());
            name_touched.set(false);
            email_touched.set(false);
            message_touched.set(false);
        }
        _ => {}
    });

    let is_success = move || matches!(action_value.get(), Some(Ok(ContactResponse::Ok)));
    let is_failure = move || matches!(action_value.get(), Some(Err(_)));

    view! {
        <section class="py-20 bg-white" id="contact">
            <div class="container px-6 mx-auto max-w-4xl">
                <h2 class="mb-8 text-4xl font-bold text-center text-slate-800">
                    {move || tr!("get-in-touch")}
                </h2>
                <div class="p-8 rounded-2xl border shadow-sm bg-slate-50 border-slate-100">
                    <form on:submit=on_submit novalidate>
                        <div class="grid grid-cols-1 gap-6 mb-6 md:grid-cols-2">
                            <div>
                                <label for="contact-name" class="block mb-2 text-sm font-semibold text-slate-600">
                                    <span class="text-red-700">* </span>
                                    {move || tr!("name")}
                                </label>
                                <input
                                    id="contact-name"
                                    type="text"
                                    name="name"
                                    node_ref=name_ref
                                    prop:value=move || name.get()
                                    on:input=move |ev| name.set(event_target_value(&ev))
                                    on:blur=move |_| name_touched.set(true)
                                    aria-invalid=move || if is_invalid(Field::Name) { "true" } else { "false" }
                                    aria-describedby="contact-name-error"
                                    class=move || field_classes(is_invalid(Field::Name), is_valid(Field::Name))
                                    placeholder=move || tr!("placeholder-name")
                                />
                                {move || {
                                    field_code(Field::Name)
                                        .map(|code| {
                                            view! {
                                                <p
                                                    id="contact-name-error"
                                                    role="alert"
                                                    class="mt-1 text-sm font-medium text-red-600"
                                                >
                                                    {validation_text(Field::Name, code)}
                                                </p>
                                            }
                                        })
                                }}
                            </div>
                            <div>
                                <label for="contact-email" class="block mb-2 text-sm font-semibold text-slate-600">
                                    <span class="text-red-700">* </span>
                                    {move || tr!("email")}
                                </label>
                                <input
                                    id="contact-email"
                                    type="email"
                                    name="email"
                                    node_ref=email_ref
                                    prop:value=move || email.get()
                                    on:input=move |ev| email.set(event_target_value(&ev))
                                    on:blur=move |_| email_touched.set(true)
                                    aria-invalid=move || if is_invalid(Field::Email) { "true" } else { "false" }
                                    aria-describedby="contact-email-error"
                                    class=move || field_classes(is_invalid(Field::Email), is_valid(Field::Email))
                                    placeholder=move || tr!("placeholder-email")
                                />
                                {move || {
                                    field_code(Field::Email)
                                        .map(|code| {
                                            view! {
                                                <p
                                                    id="contact-email-error"
                                                    role="alert"
                                                    class="mt-1 text-sm font-medium text-red-600"
                                                >
                                                    {validation_text(Field::Email, code)}
                                                </p>
                                            }
                                        })
                                }}
                            </div>
                        </div>
                        <div class="mb-6">
                            <label for="contact-message" class="block mb-2 text-sm font-semibold text-slate-600">
                                <span class="text-red-700">* </span>
                                {move || tr!("message")}
                            </label>
                            <textarea
                                id="contact-message"
                                name="message"
                                rows="4"
                                node_ref=message_ref
                                prop:value=move || message.get()
                                on:input=move |ev| message.set(event_target_value(&ev))
                                on:blur=move |_| message_touched.set(true)
                                aria-invalid=move || if is_invalid(Field::Message) { "true" } else { "false" }
                                aria-describedby="contact-message-error"
                                class=move || field_classes(is_invalid(Field::Message), is_valid(Field::Message))
                                placeholder=move || tr!("placeholder-message")
                            ></textarea>
                            {move || {
                                field_code(Field::Message)
                                    .map(|code| {
                                        view! {
                                            <p
                                                id="contact-message-error"
                                                role="alert"
                                                class="mt-1 text-sm font-medium text-red-600"
                                            >
                                                {validation_text(Field::Message, code)}
                                            </p>
                                        }
                                    })
                            }}
                        </div>
                        <button
                            type="submit"
                            disabled=move || is_pending.get()
                            class="py-4 w-full font-bold text-white bg-amber-500 rounded-lg transition transform hover:bg-amber-600 hover:-translate-y-1 disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:translate-y-0 disabled:hover:bg-amber-500"
                        >
                            {move || tr!("send-message")}
                        </button>
                    </form>

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
                        is_failure()
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
