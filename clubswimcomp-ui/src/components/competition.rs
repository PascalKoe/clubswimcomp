use clubswimcomp_types::model;
use leptos::*;
use leptos_router::*;

use super::{BirthdayDisplay, DistanceDisplay, GenderDisplay, StrokeDisplay};

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
