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
                    <Route path="/competitions/:competition_id" view=pages::CompetitionDetails/>
                    <Route path="/competitions/:competition_id/scoreboard" view=pages::CompetitionScoreboard/>

                    <Route path="/participants" view=pages::ParticipantOverview/>
                    <Route path="/participants/:participant_id" view=pages::ParticipantDetails/>/>

                    <Route path="/registrations/ingest" view=pages::ResultIngest/>

                    <Route path="/groups" view=pages::GroupOverview/>
                    <Route path="/groups/:group_id" view=pages::GroupDetails/>
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
