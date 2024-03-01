use leptos::*;

#[component]
pub fn PageLayout(children: Children) -> impl IntoView {
    view! {
        <main>
            {children()}
        </main>
    }
}

#[component]
pub fn PageTitle(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(optional)] subtitle: Option<MaybeSignal<String>>,
) -> impl IntoView {
    view! {
        <h2 class="text-2xl text-black max-w-xl">{title}</h2>
        <p class="text-sm font-light text-gray-800 max-w-2xl mb-8">{subtitle}</p>
    }
}

#[component]
pub fn SectionTitle(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(optional)] subtitle: Option<MaybeSignal<String>>,
) -> impl IntoView {
    view! {
        <h3 class="text-xl text-black max-w-xl mt-6">{title}</h3>
        <p class="text-sm font-light text-gray-800 max-w-2xl mb-4">{subtitle}</p>
    }
}
