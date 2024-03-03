use leptos::*;
use leptos_router::*;

mod api_client;
mod components;
mod layout;
mod pages;

use layout::*;
use uuid::Uuid;

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

                    <Route path="/event" view=pages::Event/>

                    <Route path="/competitions" view=pages::CompetitionOverview/>
                    <Route path="/competitions/add" view=pages::AddCompetition/>
                    <Route path="/competitions/:competition_id" view=pages::CompetitionDetails/>

                    <Route path="/participants" view=pages::ParticipantOverview/>
                    <Route path="/participants/add" view=pages::AddParticipant/>
                    <Route path="/participants/:participant_id" view=pages::ParticipantDetails/>/>

                    <Route path="/registrations/ingest" view=pages::ResultIngest/>
                </Routes>
            </AppShell>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <p>Home</p>
    }
}
