use leptos::*;
use leptos_router::*;

#[component]
pub fn drawer() -> impl IntoView {
    view! {
        <div class="drawer-side">
            <label for="app-shell-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
            <ul class="menu p-0 pt-6 w-80 min-h-full bg-base-200">
                <li class="menu-title pt-0 text-2xl text-black">ClubSwimComp</li>
                <li>
                    <A active_class="active" href="/">Home</A>
                </li>
                <li>
                    <A active_class="active" href="/competitions">Competitions</A>
                </li>
            </ul>
        </div>
    }
}
