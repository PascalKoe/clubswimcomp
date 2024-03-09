use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use crate::{api_client, components::*};

#[component]
pub fn GroupDetails() -> impl IntoView {
    let params = use_params_map();
    let group_id = move || {
        params()
            .get("group_id")
            .map(|s| Uuid::parse_str(s).ok())
            .flatten()
            .unwrap()
    };

    let group_details = create_local_resource(
        move || group_id(),
        move |group_id| {
            let group_id = group_id;
            async move { api_client::group_details(group_id).await.unwrap() }
        },
    );

    view! {
        <PageLayout>
            <PageTitle
                title="Group Details"
                subtitle="The details of a group including the results of the participants.".to_string().into()
            />
            <ActionRow>
                <button class="btn btn-sm btn-error rounded-full">
                    <phosphor_leptos::Trash />
                    Delete Group
                </button>
            </ActionRow>

            <SectionTitle title="Scoreboard" />
            <Transition fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
                {
                    move || group_details.get().map(|g|
                        view! {
                            <tables::GroupScores group_scores=g.scores/>
                        }
                    )
                }
            </Transition>
        </PageLayout>
    }
}
