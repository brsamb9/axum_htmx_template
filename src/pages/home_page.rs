use maud::{html, Markup};

use super::page_utils::page;

pub async fn home_page() -> Markup {
    page(
        "Home Page!",
        html! {
            button type="button"
                class="rounded-md bg-indigo-600 px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                hx-get="/api/click"
                // hx-swap="outerHTML"
                {
                "Test button"
            };

            div class="flex flex-col items-center justify-center h-screen bg-gray-100" {
                h1 class="text-4xl font-bold mb-4" {"Web Scraping Statistics"};
                div class="mb-4";
                // Statistics
                div class="grid grid-cols-3 gap-4" hx-ext="sse" sse-connect="/api/sse" {
                    div class="flex flex-col items-center justify-center bg-white p-4 shadow-md" {
                        h2 class="text-2xl font-semibold mb-2" { "Total Websites Scraped" }
                        p class="text-xl" sse-swap="WebsiteScraped" { "0" }
                    };
                    div class="flex flex-col items-center justify-center bg-white p-4 shadow-md" {
                        h2 class="text-2xl font-semibold mb-2" { "Total Data Points Collected" }
                        p class="text-xl" { "50000" }
                    }
                    div class="flex flex-col items-center justify-center bg-white p-4 shadow-md" {
                        h2 class="text-2xl font-semibold mb-2" { "Total Scraping Errors" }
                        p class="text-xl" { "50" }
                    }
                    div hx-get="/api/echo_hello_callback" hx-trigger="sse:WebsiteScraped" { "empty" };
                };

                // Manual input
                form class="flex justify-center" {
                    input type="text" class="border-2 border-gray-300 bg-white h-10 px-5 pr-16 rounded-lg text-sm focus:outline-none" placeholder="Enter website to track";
                    button type="submit" class="ml-2 px-4 py-2 rounded-lg bg-blue-500 text-white" { "Track" };
                }

                // Websites list
                div class="mt-4" {
                    h2 class="text-2xl font-semibold mb-2" { "Websites" };
                    ul {
                        li class="text-xl font-bold text-green-500" { "Website 1 (crawling)" }
                    }
                }
            }
        },
    )
}
