use clubswimcomp_types::model;
use leptos::html::details;
use leptos::*;
use phosphor_leptos::{IconWeight, Plus};

mod add;
mod details;

use crate::api_client;
use crate::conversions::*;
use crate::Page;

pub use add::AddCompetition;
pub use details::CompetitionDetails;

async fn api_competitions() -> Vec<model::Competition> {
    let response = gloo_net::http::Request::get("http://127.0.0.1:3000/competitions")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    leptos::logging::log!("{}", response);
    vec![]
}

#[component]
pub fn CompetitionOverview() -> impl IntoView {
    let competitions = create_local_resource(
        || (),
        |_| async { api_client::competition_overview().await.unwrap_or_default() },
    );

    let table_rows = move || {
        competitions()
            .unwrap_or_default()
            .into_iter()
            .map(|c| view! {<CompetitionRow competition=c />})
            .collect_view()
    };

    view! {
        <Page title="Competitions".to_string()>
            <div class="overflow-x-auto">
                <table class="table">
                    <thead>
                        <tr>
                            <th>Gender</th>
                            <th>Distance</th>
                            <th>Stroke</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        {table_rows}
                    </tbody>
                </table>
            </div>
            <div class="mt-4">
                <a href="/competitions/add" class="btn btn-sm btn-primary rounded-full">
                    <Plus/>
                    Add a new competition
                </a>
            </div>
        </Page>
    }
}

#[component]
fn CompetitionRow(competition: model::Competition) -> impl IntoView {
    let details_link = format!("/competitions/{}", competition.id);
    view! {
        <tr>
            <td>{gender_long(&competition.gender)}</td>
            <td>{competition.distance} m</td>
            <td>{stroke(&competition.stroke)}</td>
            <td class="w-0">
                <a class="btn btn-xs" href={details_link}>Details</a>
            </td>
        </tr>
    }
}
