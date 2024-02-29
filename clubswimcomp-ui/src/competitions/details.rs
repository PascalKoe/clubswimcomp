use clubswimcomp_types::model;
use leptos::html::details;
use leptos::*;
use leptos_router::use_params_map;
use phosphor_leptos::{IconWeight, Plus};
use uuid::Uuid;

use crate::api_client;
use crate::conversions::*;
use crate::Page;

#[component]
pub fn CompetitionDetails() -> impl IntoView {
    let params = use_params_map();
    let competition_id = move || {
        params()
            .get("competition_id")
            .map(|s| Uuid::parse_str(s).ok())
            .flatten()
            .unwrap()
    };

    let competition_details = create_local_resource(competition_id, move |c| {
        let competition_id = c.clone();
        async move {
            api_client::competition_details(competition_id)
                .await
                .unwrap()
        }
    });

    view! {
        <Page title="Competition Details".to_string()>
            <Suspense>
                {move || match competition_details.get() {
                    Some(c) => view!{
                        <h3 class="text-xs font-light mb-4">{c.competition.id.to_string()}</h3>
                        <div class="overflow-x-auto">
                            <table class="table table-xs w-80">
                                <tbody>
                                    <tr>
                                        <td class="font-bold w-40">Gender</td>
                                        <td>{gender_long(&c.competition.gender)}</td>
                                    </tr>
                                    <tr>
                                        <td class="font-bold w-0">Distance</td>
                                        <td>{c.competition.distance} m</td>
                                    </tr>
                                    <tr>
                                        <td class="font-bold w-0">Stroke</td>
                                        <td>{stroke(&c.competition.stroke)}</td>
                                    </tr>
                                    <tr>
                                        <td class="font-bold w-0">Results Pending</td>
                                        <td>{format!("{}", c.results_pending)}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>

                        <h3 class="mt-8 text-xl">Registrations</h3>
                        <p class="font-light text-sm mb-2">This is an overview over all of the registrations that exist for this competition.</p>
                        <div class="overflow-x-auto">
                        <table class="table table-xs">
                            <thead>
                                <tr>
                                    <th>Last Name</th>
                                    <th>First Name</th>
                                    <th>Gender</th>
                                    <th>Birthday</th>
                                    <th>Disqualified</th>
                                    <th>Time</th>
                                    <th>Code</th>
                                    <th></th>
                                </tr>
                            </thead>
                            <tbody>
                                {c.registrations.into_iter().map(|r| view! {<CompetitionRegistrationRow r=r />}).collect_view()}
                            </tbody>
                        </table>
                    </div>
                    }.into_view(),
                    None => view!{<p>Unknown Competition</p>}.into_view()
                }}
            </Suspense>
        </Page>
    }
}

#[component]
fn CompetitionRegistrationRow(r: model::CompetitionRegistration) -> impl IntoView {
    let details_link = format!("/registrations/{}", r.id);
    view! {
        <tr>
            <td>{r.participant.last_name}</td>
            <td>{r.participant.first_name}</td>
            <td>{gender_long(&r.participant.gender)}</td>
            <td>{r.participant.birthday.format("%Y-%m-%d").to_string()}</td>
            <td>{r.result.as_ref().map(|r| format!("{}", r.disqualified))}</td>
            <td>{r.result.as_ref().map(|r| format!("{}", r.time_millis))}</td>
            <td>{r.participant.short_code}</td>
            <td class="w-0">
                <a class="btn btn-xs" href={details_link}>Details</a>
            </td>
        </tr>
    }
}
