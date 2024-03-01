use chrono::NaiveDate;
use clubswimcomp_types::model;
use leptos::*;
use leptos_router::*;

use crate::{api_client, components::*};

#[component]
pub fn AddCompetition() -> impl IntoView {
    let navigate = leptos_router::use_navigate();

    let gender: NodeRef<html::Select> = create_node_ref();
    let distance: NodeRef<html::Select> = create_node_ref();
    let stroke: NodeRef<html::Select> = create_node_ref();

    let add_competition_action = create_action(|input: &(model::Gender, u32, model::Stroke)| {
        let gender = input.0.clone();
        let distance = input.1.clone();
        let stroke = input.2.clone();

        async move { api_client::add_competition(distance, gender, stroke).await }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let gender = match gender().unwrap().value().as_str() {
            "Female" => model::Gender::Female,
            "Male" => model::Gender::Male,
            _ => panic!("Could not parse gender"),
        };

        let distance = match distance().unwrap().value().as_str() {
            "25" => 25,
            "50" => 50,
            _ => panic!("Could not parse distance"),
        };

        let stroke = match stroke().unwrap().value().as_str() {
            "Butterfly" => model::Stroke::Butterfly,
            "Back" => model::Stroke::Back,
            "Breast" => model::Stroke::Breast,
            "Freestyle" => model::Stroke::Freestyle,
            _ => panic!("Could not parse stroke"),
        };

        add_competition_action.dispatch((gender, distance, stroke));
    };

    let saving = move || add_competition_action.pending().get();

    let (error_message, set_error_message) = create_signal(None);
    let navigat_after_submit = move || match add_competition_action.value().get() {
        Some(Ok(competition_id)) => navigate(
            &format!("/competitions/{competition_id}"),
            Default::default(),
        ),
        Some(Err(e)) => {
            set_error_message(Some(e));
        }
        _ => (),
    };

    view! {
        {navigat_after_submit}
        <PageLayout>
            <PageTitle
                title="Add Competition"
                subtitle="Add a competition to the event. There must not be any competition with exactly the same parameters.".to_string().into()
            />


            <form on:submit=on_submit>
                <FormItem label="Distance">
                    <select class="select select-bordered" node_ref=distance required>
                        <option value="25">25 Meters</option>
                        <option value="50">50 Meters</option>
                    </select>
                </FormItem>

                <FormItem label="Stroke">
                    <select class="select select-bordered" node_ref=stroke required>
                        <option value="Butterfly">Butterfly</option>
                        <option value="Back">Back</option>
                        <option value="Breast">Breast</option>
                        <option value="Freestyle">Freestyle</option>
                    </select>
                </FormItem>

                <FormItem label="Gender">
                    <select class="select select-bordered" node_ref=gender required>
                        <option value="Female">Female</option>
                        <option value="Male">Male</option>
                    </select>
                </FormItem>

                {
                    move|| error_message().map(|e| view!{<p class="text text-error">{e}</p>})
                }

                <div class="form-control w-full max-w-2xl mt-4">
                    <input class="btn btn-primary" type="submit" value="Add Competition" disabled=saving />
                </div>
                <div class="form-control w-full max-w-2xl mt-4">
                    <A href="/competitions" class="btn btn-neutral">
                        Cancel
                    </A>
                </div>
            </form>
        </PageLayout>
    }
}
