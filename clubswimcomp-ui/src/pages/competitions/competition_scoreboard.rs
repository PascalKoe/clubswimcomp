use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use crate::api_client;
use crate::components::*;

#[component]
pub fn CompetitionScoreboard() -> impl IntoView {
    let params = use_params_map();
    let competition_id = move || {
        params()
            .get("competition_id")
            .map(|s| Uuid::parse_str(s).ok())
            .flatten()
            .unwrap()
    };

    let scoreboard = create_local_resource(competition_id, move |c| async move {
        api_client::competition_scoreboard(c).await.unwrap()
    });

    view! {
        <PageLayout>
            <PageTitle
                title="Competition Scoreboard"
                subtitle="The results of the competition including the ranking.".to_string().into()
            />

            <Transition fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
                {
                    move || scoreboard.get().map(|s|
                        view! {
                            <data::CompetitionInfo competition=s.competition />

                            <SectionTitle title="Results".to_string() />
                            <tables::CompetitionScores scores=s.scores />

                            <SectionTitle title="Disqualifications".to_string() />
                            <tables::CompetitionDisqualifications disqualifications=s.disqualifications />

                            <SectionTitle title="Missing".to_string() />
                            <tables::CompetitionMissingResults missing=s.missing_results />
                        }
                    )
                }
            </Transition>
        </PageLayout>
    }
}
