use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "flex bg-pink-400 items-center justify-between w-full h-screen",

            div { class: "left w-3/10 flex flex-col bg-gray-300 h-full items-end gap-3 p-5",
                button { class: "pointer-cursor bg-green-500 mt-10 mb-10 p-2 rounded-md",
                    "+create new"
                }
                div { class: "", "home" }
                div { class: "", "popular" }
                div { class: "", "categories" }
                div { class: "", "about" }
            }

            div { class: "p-10 right w-7/10 flex flex-col bg-gray-200 h-full gap-10",
                form { class: "pt-3 right flex flex-col gap-10",
                    input { class: "text-4xl font-semibold", placeholder: "title", "title" }
                    input { class: "text-lg font-normal", placeholder: "blog", "blog" }
                }
            }

        }
    }
}
