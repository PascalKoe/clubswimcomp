use leptos::*;

#[component]
pub fn Page(title: String, children: Children) -> impl IntoView {
    view! {
        <main>
            <h2 class="text-2xl text-black">{title}</h2>
            {children()}
        </main>
    }
}
