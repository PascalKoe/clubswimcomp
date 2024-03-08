use leptos::*;
use leptos_router::*;

use crate::api_client;
use crate::components::*;

#[component]
pub fn CompetitionOverview() -> impl IntoView {
    let competitions = create_local_resource(
        || (),
        move |_| async move { api_client::list_competitions().await.unwrap() },
    );

    let add_competition_dialog_open = create_rw_signal(false);
    let competition_added = Callback::new(move |_| competitions.refetch());
    let add_competition_click = move |_| {
        add_competition_dialog_open.set(true);
    };

    view! {
        <PageLayout>
            <PageTitle
                title="Competition Overview"
                subtitle="An overview about every single competition that exists for this event.".to_string().into()
            />
            <AddCompetitionDialog on_competition_added=competition_added show=add_competition_dialog_open/>
            <ActionRow>
                <button class="btn btn-sm btn-primary rounded-full" on:click=add_competition_click>
                    <phosphor_leptos::Plus />
                    Add a new competition
                </button>
            </ActionRow>
            
            <SectionTitle title="Competitions" subtitle="List of all competitions.".to_string().into() />
            <Transition fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
                {
                    move || competitions.get().map(|c|
                        view! {
                            <CompetitionOverviewTable competitions=c />
                        }
                    )
                }
            </Transition>
        </PageLayout>
    }
}
