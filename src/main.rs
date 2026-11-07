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
    let toggle_post = use_signal(|| false);
    let title = use_signal(String::new);
    let body = use_signal(String::new);
    let mut posts = use_signal(Vec::<Post>::new);

    use_future(move || async move {
        for post in get_posts().await.unwrap() {
            println!("posts getting written!");
            posts.write().push(post);
        }
    });

    rsx! {

        div {
            id: "hero",
            class: "flex bg-gray-200 items-center justify-between w-full h-screen",

            menu_component { toggle_post }

            if toggle_post() {
                posts_component { posts }
            } else {
                add_post_component { title, body }
            }
        }
    }
}

#[component]
fn menu_component(toggle_post: Signal<bool>) -> Element {
    let (btn_class, btn_text) = if toggle_post() {
        (
            "bg-green-500 pointer-cursor mt-10 mb-10 p-2 rounded-md",
            "+ create new",
        )
    } else {
        (
            "bg-red-400 pointer-cursor mt-10 mb-10 p-2 rounded-md",
            "cancel",
        )
    };

    rsx! {
        div { class: "left w-2/10 flex flex-col bg-gray-300 h-full items-end gap-3 p-5",
            button {
                class: btn_class,
                onclick: move |_| {
                    if toggle_post() { toggle_post.set(false) } else { toggle_post.set(true) }
                },
                "{btn_text}"
            }
            div { class: "", "home" }
            div { class: "", "popular" }
            div { class: "", "categories" }
            div { class: "", "about" }
        }
    }
}

#[component]
fn posts_component(posts: Signal<Vec<Post>>) -> Element {
    rsx! {
        div {
            for post in posts.iter() {
                post_card { post: post.clone() }
            }
        }
    }
}

#[component]
fn add_post_component(title: Signal<String>, body: Signal<String>) -> Element {
    rsx! {
        div { class: "p-10 right w-8/10 flex flex-col bg-gray-200 h-full gap-10 justify-between",
            form { class: "pt-3 right flex flex-col gap-10 h-full",
                input {
                    class: "text-4xl font-semibold",
                    placeholder: "title",
                    oninput: move |event| {
                        title.set(event.value());
                    },
                    value: "{title}",
                }
                textarea {
                    class: "h-full text-lg font-normal",
                    placeholder: "blog",
                    value: "{body}",
                    oninput: move |event| {
                        body.set(event.value());
                    },
                }
            }
            button {
                class: "pointer-cursor bg-green-500 mt-10 mb-10 p-2 rounded-md",
                onclick: move |_| async move {
                    match create_post(title(), body()).await {
                        Ok(()) => {
                            tracing::info!("post created successfully");
                            // optionally clear the form:
                            title.set(String::new());
                            body.set(String::new());
                        }
                        Err(e) => {
                            tracing::error!("post creation failed: {e:?}");
                            // TODO: surface to user (toast, error signal, etc.)
                        }
                    }
                },
                "publish"
            }
        }
    }
}

#[component]
fn post_card(post: Post) -> Element {
    rsx! {
        div { class: "w-50 h-50 bg-blue-500",
            div { class: "", "{post.title}" }
            div { class: "", "{post.body}" }
            div { class: "", "{post.time_created}" }
        }
    }
}

#[server]
async fn create_post(title: String, body: String) -> Result<(), ServerFnError> {
    println!("{} {}", title, body);
    println!("app started");
    let con = rusqlite::Connection::open("./data.db3").unwrap();

    // let sql = r#"
    //     INSERT INTO posts (
    //         user_id,
    //         title,
    //         body,
    //         time_created
    //     )
    //     VALUES (
    //         1,
    //         ?1,
    //         ?2,
    //         CURRENT_TIMESTAMP
    //     )
    // "#;

    //con.execute(sql, [&title, &body]).unwrap();

    let sql = r#"
        CREATE TABLE IF NOT EXISTS posts (
            id              INTEGER PRIMARY KEY,
            user_id         INTEGER NOT NULL,
            title           TEXT,
            body            TEXT,
            time_created    TIMESTAMP
        )
    "#;

    con.execute(sql, ()).unwrap();

    Ok(())
}

#[server]
async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    let con = rusqlite::Connection::open("./data.db3").unwrap();

    println!("getting posts");

    let mut stm = con
        .prepare("SELECT id, title, body, time_created FROM posts ORDER BY id DESC")
        .unwrap();

    let rows = stm
        .query_map([], |row| {
            Ok(Post {
                id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                body: row.get(2).unwrap(),
                time_created: row.get(3).unwrap(),
            })
        })
        .unwrap();

    let mut posts: Vec<Post> = Vec::new();

    println!("gotten them posts");

    for row in rows {
        let post = row.unwrap();
        posts.push(post);
    }

    println!("after for statement ");

    Ok(posts)
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Post {
    id: u32,
    title: String,
    body: String,
    time_created: String,
}
