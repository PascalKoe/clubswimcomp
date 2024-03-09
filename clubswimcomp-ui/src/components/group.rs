use leptos::*;

use crate::components::*;

#[component]
pub fn AddGroupForm(on_group_added: Callback<Uuid>, on_cancel: Callback<()>) -> impl IntoView {
    let (error_message, set_error_message) = create_signal(None);

    let (group_name, set_group_name) = create_signal(String::new());

    #[derive(Clone)]
    struct AddGroupAction {
        group_name: String,
    }
    let add_group_action = create_action(|input: &AddGroupAction| {
        let input = input.clone();
        async move { api_client::add_group(input.group_name).await }
    });

    let on_group_added_handler = move || match add_group_action.value().get() {
        Some(Ok(group_id)) => on_group_added(group_id),
        Some(Err(e)) => set_error_message(Some(e)),
        None => (),
    };

    let group_saving = move || add_group_action.pending().get();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let input = AddGroupAction {
            group_name: group_name(),
        };
        add_group_action.dispatch(input);
    };

    let on_cancel_button_clicked = move |ev: ev::MouseEvent| {
        on_cancel(());
    };

    view! {
        {on_group_added_handler}

        <form on:submit=on_submit>
            <FormItem label="Group Name">
                <InputName set_name=set_group_name />
            </FormItem>
            {
                move|| error_message().map(|e| view!{<p class="text text-error">{e}</p>})
            }

            <div class="form-control w-full max-w-2xl mt-4">
                <input class="btn btn-primary" type="submit" value="Add Group" disabled=group_saving />
            </div>
        </form>
        <div class="form-control w-full max-w-2xl mt-4">
            <button class="btn btn-neutral" on:click=on_cancel_button_clicked>
                Cancel
            </button>
        </div>
    }
}

#[component]
pub fn AddGroupDialog(
    #[prop(into)] show: RwSignal<bool>,
    on_group_added: Callback<Uuid>,
) -> impl IntoView {
    let on_added_callback = Callback::new(move |group_id| {
        show.set(false);
        on_group_added(group_id);
    });

    let on_cancel = Callback::new(move |()| show.set(false));

    view! {
        <dialog class="modal bg-black bg-opacity-30" autofocus open=show>
            <div class="modal-box">
                <h3 class="text-xl text-black">Add a Group</h3>
                <AddGroupForm
                    on_group_added=on_added_callback
                    on_cancel
                />
            </div>
        </dialog>
    }
}

#[component]
pub fn GroupOverviewTable(#[prop(into)] groups: MaybeSignal<Vec<model::Group>>) -> impl IntoView {
    let rows = move || {
        groups()
            .into_iter()
            .map(|g| {
                view! {
                    <tr>
                        <td>{g.name}</td>
                        <td class="w-0">
                            <A class="btn btn-xs" href="">Details</A>
                        </td>
                    </tr>
                }
            })
            .collect_view()
    };

    view! {
        <div class="overflow-x-auto">
            <table class="table table-xs">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        </div>
    }
}
