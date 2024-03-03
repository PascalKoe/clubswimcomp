use clubswimcomp_types::model;
use leptos::*;
use leptos_router::*;

use crate::{api_client, components::*};

#[component]
pub fn AddCompetition() -> impl IntoView {
    let navigate = leptos_router::use_navigate();

    let (gender, set_gender) = create_signal(model::Gender::Female);
    let (stroke, set_stroke) = create_signal(model::Stroke::Butterfly);
    let (distance, set_distance) = create_signal(25);
    let (target_time, set_target_time) = create_signal(None);

    #[derive(Clone)]
    struct AddCompetitionAction {
        distance: u32,
        stroke: model::Stroke,
        gender: model::Gender,
        target_time: u32,
    }
    let add_competition_action = create_action(|input: &AddCompetitionAction| {
        let input = input.clone();

        async move {
            api_client::add_competition(
                input.distance,
                input.gender,
                input.stroke,
                input.target_time as _,
            )
            .await
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let Some(target_time) = target_time() else {
            return;
        };

        let input = AddCompetitionAction {
            distance: distance(),
            stroke: stroke(),
            gender: gender(),
            target_time: target_time,
        };
        add_competition_action.dispatch(input);
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
                    <InputDistance set_distance />
                </FormItem>

                <FormItem label="Stroke">
                    <InputStroke set_stroke />
                </FormItem>

                <FormItem label="Gender">
                    <InputGender set_gender />
                </FormItem>

                <FormItem label="Target Time">
                    <InputTime set_time=set_target_time />
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
