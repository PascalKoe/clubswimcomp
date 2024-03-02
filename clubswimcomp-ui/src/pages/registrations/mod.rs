use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use crate::{api_client, components::*};

#[component]
pub fn AddResult() -> impl IntoView {
    let navigate = use_navigate();
    let (error_msg, set_error_msg) = create_signal(None);
    let params = use_params_map();
    let registration_id = move || {
        params()
            .get("registration_id")
            .map(|r| Uuid::parse_str(&r).unwrap())
            .unwrap()
    };

    let registration_details = create_resource(registration_id, |registration_id| {
        let registration_id = registration_id.clone();
        async move {
            api_client::registration_details(registration_id)
                .await
                .unwrap()
        }
    });

    let add_result_action = create_action(|input: &(Uuid, bool, i64)| {
        let registration_id = input.0;
        let disqualified = input.1;
        let time_millis = input.2;
        //
        async move { api_client::add_result(registration_id, disqualified, time_millis).await }
    });

    let add_result_saving = move || add_result_action.pending().get();

    let navigate_after_submit = move || {
        let action_result = add_result_action.value().get();
        match action_result {
            Some(Ok(_)) => navigate("/registrations", Default::default()),
            Some(Err(e)) => set_error_msg(Some(e)),
            None => (),
        }
    };

    view! {
        {navigate_after_submit}
        <PageLayout>
            <PageTitle title="Add Result for Registration" subtitle="Add a result to a registration.".to_string().into() />
            <Transition fallback=|| view!{<span class="loading loading-spinner loading-lg"></span>}>
            {
                move || registration_details.get().map(|rd| {
                    let disqualified: NodeRef<html::Input> = NodeRef::new();
                    let time: NodeRef<html::Input> = NodeRef::new();

                    let on_submit = move |ev: leptos::ev::SubmitEvent| {
                        ev.prevent_default();

                        let disqualified = disqualified().unwrap().checked();
                        let time_raw = time().unwrap().value();
                        let time_mins = &time_raw[0..=1].parse::<i64>().unwrap();
                        let time_secs = &time_raw[2..=3].parse::<i64>().unwrap();
                        let time_hundreths = &time_raw[4..=5].parse::<i64>().unwrap();
                        let millis = (time_mins * 60 * 1000) + (time_secs *1000) + (time_hundreths * 10);

                        add_result_action.dispatch((rd.id, disqualified, millis));
                    };

                    view! {
                        <div class="flex flex-row w-full">
                            <div class="">
                                <ParticipantInfoTable participant=rd.participant />
                            </div>
                            <div class="flex-1">
                                <CompetitionInfoTable competition=rd.competition />
                            </div>
                        </div>

                        <SectionTitle title="Result for Registration"/>
                        {error_msg().map(|err| view!{<p class="text-error">{err}</p>})}
                        <form on:submit=on_submit>
                            <div class="form-control">
                                <label class="label cursor-pointer">
                                    <input type="checkbox" class="checkbox" node_ref=disqualified/>
                                    <span class="label-text pl-4">Disqualified</span>
                                    <span class="flex-1"></span>
                                </label>
                            </div>
                            <FormItem label="Time">
                                <input class="input input-bordered" type="text"  minlength=6 maxlength=6 required node_ref=time/>
                            </FormItem>

                            <div class="form-control w-full max-w-2xl mt-4">
                                <input class="btn btn-primary" type="submit" value="Add Result" disabled=add_result_saving />
                            </div>
                            <div class="form-control w-full max-w-2xl mt-4">
                                <A href="/participants" class="btn btn-neutral">
                                    Cancel
                                </A>
                            </div>
                        </form>
                    }
                })
            }
        </Transition>

        </PageLayout>
    }
}
