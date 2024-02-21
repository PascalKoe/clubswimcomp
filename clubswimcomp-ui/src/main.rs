use leptos::*;
use leptos_router::*;

mod competitions;
mod layout;

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
                    <Route path="/competitions" view=competitions::CompetitionOverview/>
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

#[component]
fn Competitions() -> impl IntoView {
    view! {
        "Competitions"
    }
}

struct ApiContext {
    pub base_url: String,
}
