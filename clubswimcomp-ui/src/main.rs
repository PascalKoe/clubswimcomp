use leptos::*;
use leptos_router::*;

mod api_client;
mod competitions;
mod conversions;
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
                    <Route path="/competitions/add" view=competitions::AddCompetition/>
                    <Route path="/competitions/:competition_id" view=competitions::CompetitionDetails/>
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
