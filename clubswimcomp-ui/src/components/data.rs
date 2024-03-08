use clubswimcomp_types::model;
use leptos::*;

use crate::components::values;

#[component]
pub fn Table(children: Children) -> impl IntoView {
    view! {
        <table class="table table-xs w-80">
            <tbody>
                {children()}
            </tbody>
        </table>
    }
}

#[component]
pub fn Item(#[prop(into)] key: String, children: Children) -> impl IntoView {
    view! {
        <tr>
            <td class="font-bold w-40">
                {key}
            </td>
            <td>
                {children()}
            </td>
        </tr>
    }
}

#[component]
pub fn CompetitionInfo(
    #[prop(into)] competition: MaybeSignal<model::Competition>,
) -> impl IntoView {
    move || {
        let competition = competition();
        view! {
            <Table>
                <Item key="Gender">
                    <values::Gender gender=competition.gender />
                </Item>
                <Item key="Distance">
                    <values::Distance distance=competition.distance />
                </Item>
                <Item key="Stroke">
                    <values::Stroke stroke=competition.stroke />
                </Item>
                <Item key="Target Time">
                    <values::Time millis=competition.target_time />
                </Item>
            </Table>
        }
    }
}

#[component]
pub fn ParticipantInfo(
    #[prop(into)] participant: MaybeSignal<model::Participant>,
) -> impl IntoView {
    move || {
        let participant = participant();
        view! {
            <Table>
                <Item key="Code">
                    <values::ShortCode short_code=participant.short_code />
                </Item>
                <Item key="Last Name">
                    {participant.last_name}
                </Item>
                <Item key="First Name">
                    {participant.first_name}
                </Item>
                <Item key="Gender">
                    <values::Gender gender=participant.gender />
                </Item>
                <Item key="Birthday">
                    <values::Date date=participant.birthday />
                </Item>
                <Item key="Age">
                    {participant.age}
                </Item>
                {/* TODO: Add group name for participant */}
            </Table>
        }
    }
}
