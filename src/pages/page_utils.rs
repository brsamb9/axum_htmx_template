use maud::{html, Markup, DOCTYPE};

/// A basic header with a dynamic `page_title`.
fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        script src="https://unpkg.com/htmx.org@1.9.5" {};
        script src="https://unpkg.com/htmx.org/dist/ext/sse.js" {};
        link href="/assets/main.css" rel="stylesheet";
        link href="https://rsms.me/inter/inter.css" rel="stylesheet";
        h1 class="font-bold text-indigo-600" { (page_title) };
    }
}

/// A static footer.
fn footer() -> Markup {
    html! {
        footer {
            a href="/rss.atom" class="text-indigo-600" { "RSS Feed" }
        }
    }
}

/// The final Markup, including `header` and `footer`.
///
/// Additionally takes a `greeting_box` that's `Markup`, not `&str`.
pub fn page(title: &str, greeting_box: Markup) -> Markup {
    html! {
        // Add the header markup to the page
        (header(title))
        div id="content" { (greeting_box) }
        (footer())
    }
}
