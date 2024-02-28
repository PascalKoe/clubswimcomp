use leptos::*;

use super::Drawer;
use super::Navbar;

#[component]
pub fn AppShell(children: Children) -> impl IntoView {
    view! {
        <div class="drawer lg:drawer-open">
            <input id="app-shell-drawer" type="checkbox" class="drawer-toggle"/>
            <div class="drawer-content flex flex-col">
                <Navbar/>
                <main class="p-6">{children()}</main>
            </div>
            <Drawer/>
        </div>
    }
}
