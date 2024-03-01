use chrono::NaiveDate;
use clubswimcomp_types::model;
use leptos::*;
use leptos_router::*;

use crate::{api_client, components::*};

#[component]
pub fn FormItem(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! {
        <label class="form-control w-full max-w-2xl">
            <div class="label">
                <span class="label-text">{label}</span>
            </div>
            {children()}
        </label>
    }
}

#[component]
pub fn AddParticipant() -> impl IntoView {
    let navigate = leptos_router::use_navigate();

    let last_name: NodeRef<html::Input> = create_node_ref();
    let first_name: NodeRef<html::Input> = create_node_ref();
    let gender: NodeRef<html::Select> = create_node_ref();
    let birthday: NodeRef<html::Input> = create_node_ref();

    let add_participant_action =
        create_action(|input: &(String, String, model::Gender, NaiveDate)| {
            let last_name = input.0.clone();
            let first_name = input.1.clone();
            let gender = input.2.clone();
            let birthday = input.3.clone();

            async move {
                api_client::add_participant(first_name, last_name, gender, birthday)
                    .await
                    .unwrap()
            }
        });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let last_name = last_name().unwrap().value();
        let first_name = first_name().unwrap().value();
        let gender = gender().unwrap().value();
        let gender = match gender.as_str() {
            "Female" => model::Gender::Female,
            "Male" => model::Gender::Male,
            _ => panic!("Could not parse gender"),
        };
        let birthday = birthday().unwrap().value();
        let birthday = chrono::NaiveDate::parse_from_str(&birthday, "%Y-%m-%d").unwrap();

        add_participant_action.dispatch((last_name, first_name, gender, birthday));
    };

    let saving = move || add_participant_action.pending().get();

    let navigat_after_submit = move || {
        if let Some(participant_id) = add_participant_action.value().get() {
            navigate(
                &format!("/participants/{participant_id}"),
                Default::default(),
            );
        }
    };

    view! {
        {navigat_after_submit}
        <PageLayout>
            <PageTitle
                title="Add Participant"
                subtitle="Add a participant to this event.".to_string().into()
            />


            <form on:submit=on_submit>
                <FormItem label="Last Name">
                    <input class="input input-bordered" type="text" node_ref=last_name required/>
                </FormItem>

                <FormItem label="First Name">
                    <input class="input input-bordered" type="text" node_ref=first_name required/>
                </FormItem>

                <FormItem label="Gender">
                    <select class="select select-bordered" node_ref=gender required>
                        <option value="Female">Female</option>
                        <option value="Male">Male</option>
                    </select>
                </FormItem>

                <FormItem label="Birthday">
                    <input class="input input-bordered" type="date" node_ref=birthday required/>
                </FormItem>

                <div class="form-control w-full max-w-2xl mt-4">
                    <input class="btn btn-primary" type="submit" value="Add Participant" disabled=saving />
                </div>
                <div class="form-control w-full max-w-2xl mt-4">
                    <A href="/participants" class="btn btn-neutral">
                        Cancel
                    </A>
                </div>
            </form>
        </PageLayout>
    }
}
