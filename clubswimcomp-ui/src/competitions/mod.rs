use leptos::*;
use phosphor_leptos::{IconWeight, Plus};

mod add;

use crate::Page;

pub use add::AddCompetition;

#[component]
pub fn CompetitionOverview() -> impl IntoView {
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
                        <tr>
                            <td>Male</td>
                            <td>25 m</td>
                            <td>Butterfly</td>
                            <td>
                                <button class="btn btn-xs">Details</button>
                            </td>
                        </tr>
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
