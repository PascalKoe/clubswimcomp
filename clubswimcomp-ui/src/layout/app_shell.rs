use leptos::*;

use super::navbar::Navbar;

#[component]
pub fn AppShell(children: Children) -> impl IntoView {
    view! {
        <div class="drawer lg:drawer-open">
            <input id="app-shell-drawer" type="checkbox" class="drawer-toggle"/>
            <div class="drawer-content flex flex-col">
                <Navbar/>

                <main class="p-6">
                    <p class="text-2xl text-black">Competitions</p>
                    { children() }
                </main>
            </div>
            <div class="drawer-side">
                <label
                    for="app-shell-drawer"
                    aria-label="close sidebar"
                    class="drawer-overlay"
                ></label>
                <ul class="menu p-0 pt-6 w-80 min-h-full bg-base-200">
                    <li class="menu-title pt-0 text-2xl text-black">
                        ClubSwimComp
                    </li>
                    <li>
                        <a href="/">Home</a>
                    </li>
                    <li>
                        <a href="/competitions">Competitions</a>
                    </li>
                </ul>
            </div>
        </div>
    }
}
