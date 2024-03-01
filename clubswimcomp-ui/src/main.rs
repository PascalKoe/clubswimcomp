use leptos::*;
use leptos_router::*;

mod api_client;
mod components;
mod layout;
mod pages;

use layout::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <ClubSwimCompUi />
        }
    })
}

#[component]
fn ClubSwimCompUi() -> impl IntoView {
    view! {
        <Router>
            <AppShell>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/competitions" view=pages::CompetitionOverview/>
                    <Route path="/competitions/:competition_id" view=pages::CompetitionDetails/>
                </Routes>
            </AppShell>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        "Home"
    }
}
