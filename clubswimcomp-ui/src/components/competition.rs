use clubswimcomp_types::model;
use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use crate::{api_client, components::*};

#[component]
pub fn CompetitionInfoTable(
    #[prop(into)] competition: MaybeSignal<model::Competition>,
) -> impl IntoView {
    view! {
        <table class="table table-xs w-80">
            <tbody>
                <tr>
                    <td class="font-bold w-40">Gender</td>
                    <td><GenderDisplay gender={competition().gender}/></td>
                </tr>
                <tr>
                    <td class="font-bold w-40">Distance</td>
                    <td><DistanceDisplay distance={competition().distance}/></td>
                </tr>
                <tr>
                    <td class="font-bold w-40">Stroke</td>
                    <td><StrokeDisplay stroke={competition().stroke}/></td>
                </tr>
                <tr>
                    <td class="font-bold w-40">Target Time</td>
                    <td><TimeDisplay millis={competition().target_time}/></td>
                </tr>
            </tbody>
        </table>
    }
}

#[component]
pub fn CompetitionRegistrationsTable(
    #[prop(into)] registrations: MaybeSignal<Vec<model::CompetitionRegistration>>,
) -> impl IntoView {
    let rows = move || {
        registrations()
            .into_iter()
            .map(|r| {
                let details_link = format!("/registrations/{}", r.id);
                let participant_link = format!("/participants/{}", r.participant.id);

                view! {
                    <tr>
                        <td class="w-0">
                            <A class="btn btn-xs" href=participant_link>
                                <phosphor_leptos::MagnifyingGlass />
                            </A>
                        </td>
                        <td>{r.participant.short_code}</td>
                        <td>{r.participant.last_name}</td>
                        <td>{r.participant.first_name}</td>
                        <td><GenderDisplay gender=r.participant.gender /></td>
                        <td><BirthdayDisplay birthday=r.participant.birthday /></td>
                        <td>
                            {
                                if r.result.is_some() {
                                    view! {
                                        <phosphor_leptos::Check />
                                    }.into_view()
                                } else {
                                    view! {
                                        <phosphor_leptos::X />
                                    }.into_view()
                                }
                            }
                        </td>

                        <td class="w-0">
                            <A class="btn btn-xs" href=details_link>Details</A>
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
                        <th></th>
                        <th>Code</th>
                        <th>Last Name</th>
                        <th>First Name</th>
                        <th>Gender</th>
                        <th>Birthday</th>
                        <th>Has Result</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {move || rows()}
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn CompetitionOverviewTable(
    #[prop(into)] competitions: MaybeSignal<Vec<model::Competition>>,
) -> impl IntoView {
    let rows = move || {
        competitions()
            .into_iter()
            .map(|c| {
                let details_link = format!("/competitions/{}", c.id);
                view! {
                    <tr>
                        <td><GenderDisplay gender=c.gender /></td>
                        <td><DistanceDisplay distance=c.distance /></td>
                        <td><StrokeDisplay stroke=c.stroke /></td>
                        <td><TimeDisplay millis=c.target_time /></td>
                        <td class="w-0">
                            <A class="btn btn-xs" href=details_link>Details</A>
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
                        <th>Gender</th>
                        <th>Distance</th>
                        <th>Stroke</th>
                        <th>Target Time</th>
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

#[component]
pub fn AddCompetitionForm(
    on_competition_added: Callback<Uuid>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let (error_message, set_error_message) = create_signal(None);

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

    let on_competition_added_handler = move || match add_competition_action.value().get() {
        Some(Ok(competition_id)) => on_competition_added(competition_id),
        Some(Err(e)) => set_error_message(Some(e)),
        None => (),
    };

    let competition_saving = move || add_competition_action.pending().get();

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

    let on_cancel_button_clicked = move |_| {
        on_cancel(());
    };

    view! {
        {on_competition_added_handler}

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
                <input class="btn btn-primary" type="submit" value="Add Competition" disabled=competition_saving />
            </div>
            <div class="form-control w-full max-w-2xl mt-4">
                <button class="btn btn-neutral" on:click=on_cancel_button_clicked>
                    Cancel
                </button>
            </div>
        </form>
    }
}

#[component]
pub fn AddCompetitionDialog(
    #[prop(into)] show: RwSignal<bool>,
    on_competition_added: Callback<Uuid>,
) -> impl IntoView {
    let on_added_callback = Callback::new(move |competition_id| {
        show.set(false);
        on_competition_added(competition_id);
    });

    let on_cancel = Callback::new(move |()| show.set(false));

    view! {
        <dialog class="modal bg-black bg-opacity-30" autofocus open=show>
            <div class="modal-box">
                <h3 class="text-xl text-black">Add a Competition</h3>
                <AddCompetitionForm
                    on_competition_added=on_added_callback
                    on_cancel
                />
            </div>
        </dialog>
    }
}
