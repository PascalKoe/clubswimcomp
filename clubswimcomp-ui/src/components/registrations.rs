use leptos::*;
use uuid::Uuid;

use crate::{api_client, components::*};

#[component]
pub fn AddResultForm(
    registration_id: Uuid,
    #[prop(optional, into)] on_added: Option<Callback<()>>,
    #[prop(optional, into)] on_cancel: Option<Callback<()>>,
) -> impl IntoView {
    // Form/Action error dispaly
    let (error, set_error) = create_signal(None);

    // Input signals
    let (time, set_time) = create_signal(None);
    let (disqualified, set_disqualified) = create_signal(false);

    // Action to submit the result to the server
    #[derive(Clone)]
    struct SaveToServer {
        registration_id: Uuid,
        disqualified: bool,
        time_millis: u32,
    }
    let save_to_server = create_action(|input: &SaveToServer| {
        let input = input.clone();
        async move {
            api_client::add_result(input.registration_id, input.disqualified, input.time_millis)
                .await
        }
    });

    // Execute callback after sucessfull action
    let save_to_server_done = move || {
        let response = save_to_server.value().get();
        match response {
            Some(Ok(())) => {
                if let Some(on_added) = on_added {
                    on_added(());
                }
                save_to_server.value().set(None);
            }
            Some(Err(e)) => set_error(Some(e)),
            None => (),
        };
    };

    // Saving to server is in progress
    let saving_to_server = move || save_to_server.pending()();

    // Form submission -> do add action
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let Some(time_millis) = time() else {
            set_error(Some("Invalid time input. Format = MMSSHH".to_string()));
            return;
        };

        let input = SaveToServer {
            registration_id,
            disqualified: disqualified(),
            time_millis,
        };
        save_to_server.dispatch(input);
    };

    // Cancel button has been pressed -> callback
    let cancel_clicked = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        if let Some(on_cancel) = on_cancel {
            on_cancel(());
        }
    };

    view! {
        {save_to_server_done}

        <form on:submit=on_submit>
            {/* error message display*/}
            <Show when=move || error().is_some()>
                <p class="font-bold text-error">
                    {error()}
                </p>
            </Show>

            {/* disqualified input field */}
            <div class="form-control">
                <label class="label cursor-pointer">
                    <InputDisqualified set_disqualified/>
                    <span class="label-text pl-4">Disqualified</span>
                    <span class="flex-1"></span>
                </label>
            </div>

            {/* time input field */}
            <FormItem label="Time">
                <InputTime set_time/>
            </FormItem>

            {/* Submit the form */}
            <div class="form-control w-full max-w-2xl mt-4">
                <input class="btn btn-primary" type="submit" value="Add Result" disabled=saving_to_server />
            </div>

            {/* Cancel the form */}
            <div class="form-control w-full max-w-2xl mt-4">
                <button class="btn btn-neutral" on:click=cancel_clicked>
                    Cancel
                </button>
            </div>
        </form>
    }
}

#[component]
pub fn AddResultDialog(
    #[prop(into)] registration_id: RwSignal<Option<Uuid>>,
    #[prop(default = None, into)] on_result_added: Option<Callback<()>>,
) -> impl IntoView {
    let on_added_callback = Callback::new(move |()| {
        registration_id.set(None);
        if let Some(on_result_added) = on_result_added {
            on_result_added(());
        }
    });

    view! {
        <dialog class="modal bg-black bg-opacity-30" autofocus open=move || registration_id.get().is_some()>
            <div class="modal-box">
                <h3 class="text-xl text-black">Add Result to Registration</h3>
                {
                    move || registration_id.get().map(|r| view! {
                        <AddResultForm
                            registration_id=r
                            on_added=on_added_callback
                            on_cancel=move |_| registration_id.set(None)
                        />
                    })
                }

            </div>
        </dialog>
    }
}

#[component]
pub fn ResultInfoTable(registration_result: model::RegistrationResult) -> impl IntoView {
    view! {
        <table class="table table-xs w-80">
            <tbody>
                <tr>
                    <td class="font-bold w-40">Disqualified</td>
                    <CellDisqualified disqualified=registration_result.disqualified />
                </tr>
                <tr>
                    <td class="font-bold w-40">Time</td>
                    <CellTime millis=registration_result.time_millis />
                </tr>
            </tbody>
        </table>
    }
}
