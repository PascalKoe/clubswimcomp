use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use crate::api_client;
use crate::components::*;

#[component]
pub fn CompetitionDetails() -> impl IntoView {
    let navigate = use_navigate();
    let params = use_params_map();
    let competition_id = move || {
        params()
            .get("competition_id")
            .map(|s| Uuid::parse_str(s).ok())
            .flatten()
            .unwrap()
    };

    let competition_details = create_local_resource(competition_id, move |c| async move {
        api_client::competition_details(c).await.unwrap()
    });

    let delete_competition_action = create_action(|competition_id: &Uuid| {
        let competition_id = *competition_id;
        async move { api_client::delete_competition(competition_id, false).await }
    });
    let (error_msg, set_error_msg) = create_signal(None);
    let redirect_after_delete = move || match delete_competition_action.value().get() {
        Some(Ok(_)) => navigate("/competitions", Default::default()),
        Some(Err(e)) => set_error_msg(Some(e)),
        None => (),
    };
    let can_be_deleted = move || {
        competition_details()
            .map(|cd| cd.registrations.is_empty())
            .unwrap_or_default()
    };

    view! {
        {redirect_after_delete}
        <PageLayout>
            <PageTitle
                title="Competition Details"
                subtitle="Details about a specific competition with all of it's registrations.".to_string().into()
            />
            { move || error_msg().map(|e| view! {<p class="text-error">{e}</p>}) }
            <Transition fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
                {
                    move || competition_details.get().map(|cd|
                        view! {
                            <div class="mb-8">
                                <button
                                    class="btn btn-sm btn-error rounded-full"
                                    on:click=move |_| delete_competition_action.dispatch(competition_id())
                                    disabled=move || !can_be_deleted()
                                >
                                    <phosphor_leptos::Trash />
                                    Delete Competition
                                </button>
                        </div>
                            <CompetitionInfoTable competition=cd.competition />

                            <SectionTitle
                                title="Registrations".to_string()
                                subtitle="This is an overview over all of the registrations that exist for this competition.".to_string().into()
                            />
                            <CompetitionRegistrationsTable registrations=cd.registrations />
                        }
                    )
                }
            </Transition>
        </PageLayout>
    }
}
