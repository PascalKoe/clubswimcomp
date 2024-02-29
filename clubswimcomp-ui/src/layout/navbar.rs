use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="w-full navbar bg-base-300 lg:hidden">
            <div class="flex-none lg:hidden">
                <label for="app-shell-drawer" class="btn btn-square btn-ghost">
                    <IconHamburger/>
                </label>
            </div>
            <div class="flex-1 px-2 mx-2">
                <b>ClubSwimComp</b>
            </div>
            <div class="flex-none hidden lg:block">
                <NavigationDrawer/>
            </div>
        </div>
    }
}

#[component]
fn NavigationDrawer() -> impl IntoView {
    view! {
        // <a>Navbar Item 1</a>
        <ul class="menu menu-horizontal">// </li>
        // <li>
        // <a>Navbar Item 2</a>
        // </li>
        </ul>
    }
}

#[component]
fn IconHamburger() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            class="inline-block w-6 h-6 stroke-current"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 6h16M4 12h16M4 18h16"
            ></path>
        </svg>
    }
}
