// https://mondeja.github.io/leptos-fluent/latest/intro.html

use leptos::prelude::*;
use leptos_fluent::leptos_fluent;

#[component]
pub fn I18nProvider(children: Children) -> impl IntoView {
    // See all options in the reference at
    // https://mondeja.github.io/leptos-fluent/latest/leptos_fluent.html
    leptos_fluent! {
        children: children(),

        // Path to the locales directory, relative to Cargo.toml.
        locales: "./locales",

        // Initial language when the user don't load any with
        // the provided configuration.
        default_language: "en",

        // Check translations correctness in the specified files.
        // #[cfg(debug_assertions)]
        // check_translations: "./src/**/*.rs",
        // #[cfg(all(debug_assertions, not(feature = "ssr")))]
        check_translations: true,

        // Client side options
        // -------------------
        //
        // Synchronize `<html lang="...">` attribute with current active language.
        sync_html_tag_lang: true,

        // Update language on URL parameter when changes.
        set_language_to_url_param: true,

        // Set initial language of user from URL in local storage.
        initial_language_from_url_param_to_local_storage: true,

        // Set initial language of user from URL in a cookie.
        initial_language_from_url_param_to_cookie: true,

        // Key used to get and set the current language of the user on local storage.
        // By default is `"lang"`.
        local_storage_key: "language",

        // Get initial language from local storage if not found in an URL param.
        initial_language_from_local_storage: true,

        // Set the initial language of the user from local storage to a cookie.
        initial_language_from_local_storage_to_cookie: true,

        // Update language on local storage when changes.
        set_language_to_local_storage: true,

        // Get initial language from `navigator.languages` if not found in local storage.
        initial_language_from_navigator: true,

        // Set initial language of user from navigator to local storage.
        initial_language_from_navigator_to_local_storage: true,

        // Set initial language of user from navigator to a cookie.
        initial_language_from_navigator_to_cookie: true,

        // Attributes to set for language cookie.
        // By default `""`.
        cookie_attrs: "Secure; Path=/; Max-Age=600",

        // Update language on cookie when the language changes.
        set_language_to_cookie: true,

        // Set initial language from a cookie to local storage.
        initial_language_from_cookie_to_local_storage: true,

        // Server side options
        // -------------------
        //
        // Set initial language from the `Accept-Language` header of the request.
        initial_language_from_accept_language_header: true,

        // Server and client side options
        // ------------------------------
        //
        // Name of the cookie to get and set the current active language.
        // By default `"lf-lang"`.
        cookie_name: "lang",

        // Set initial language from cookie.
        initial_language_from_cookie: true,

        // URL parameter to use setting the language in the URL.
        // By default `"lang"`.
        url_param: "lang",

        // Set initial language of the user from an URL parameter.
        initial_language_from_url_param: true,
    }
}
