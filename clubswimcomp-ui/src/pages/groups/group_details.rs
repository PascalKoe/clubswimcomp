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

    let groups = create_local_resource(
        || (),
        move |_| async { api_client::group_details(group_id).await.unwrap() },
    );

    let add_group_dialog_open = create_rw_signal(false);
    let group_added = Callback::new(move |_| groups.refetch());
    let add_group_clicked = move |_| {
        add_group_dialog_open.set(true);
    };

    view! {
        <PageLayout>
            <PageTitle
                title="Group Details"
                subtitle="The details of a group including the results of the participants.".to_string().into()
            />
            <ActionRow>
                <button class="btn btn-sm btn-primary rounded-full">
                    <phosphor_leptos::Plus />
                    Add Group
                </button>
            </ActionRow>

            <SectionTitle title="Groups" subtitle="Registrations with missing Result.".to_string().into() />
            <Transition fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
                {
                    move || groups.get().map(|g|
                        view! {
                            <GroupOverviewTable groups=g />
                        }
                    )
                }
            </Transition>
        </PageLayout>
    }
}
