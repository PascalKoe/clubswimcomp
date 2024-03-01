use clubswimcomp_types::model;
use leptos::*;
use leptos_router::*;

use super::GenderDisplay;

#[component]
pub fn ParticipantOverviewTable(
    #[prop(into)] participants: MaybeSignal<Vec<model::Participant>>,
) -> impl IntoView {
    let rows = move || {
        participants()
            .into_iter()
            .map(|p| {
                let details_link = format!("/participants/{}", p.id);
                view! {
                    <tr>
                        <td>{p.short_code}</td>
                        <td>{p.last_name}</td>
                        <td>{p.first_name}</td>
                        <td><GenderDisplay gender=p.gender /></td>
                        <td>{p.birthday.format("%Y-%m-%d").to_string()}</td>
                        <td>{p.age}</td>
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
                        <th>Code</th>
                        <th>Last Name</th>
                        <th>First Name</th>
                        <th>Gender</th>
                        <th>Birthday</th>
                        <th>Age</th>
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
