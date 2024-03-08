use leptos::*;
use leptos_router::*;

use crate::{api_client, components::*};

#[component]
pub fn GroupOverview() -> impl IntoView {
    let groups = create_local_resource(
        || (),
        move |_| async { api_client::list_groups().await.unwrap() },
    );

    let add_group_dialog_open = create_rw_signal(false);
    let group_added = Callback::new(move |_| groups.refetch());
    let add_group_clicked = move |_| {
        add_group_dialog_open.set(true);
    };

    view! {
        <PageLayout>
            <PageTitle
                title="Group Overview"
                subtitle="An overview about every single group that exists for this event.".to_string().into()
            />
            <AddGroupDialog on_group_added=group_added show=add_group_dialog_open />
            <ActionRow>
                <button class="btn btn-sm btn-primary rounded-full" on:click=add_group_clicked>
                    <phosphor_leptos::Plus />
                    Add Group
                </button>
            </ActionRow>

            <SectionTitle title="Groups" subtitle="List of all groups.".to_string().into() />
            <Transition fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
                {
                    move || groups.get().map(|g|
                        view! {
                            <tables::GroupOverview groups=g />
                        }
                    )
                }
            </Transition>
        </PageLayout>
    }
}
