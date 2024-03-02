use leptos::*;
use leptos_router::*;
use uuid::Uuid;

use crate::{api_client, components::*};

#[component]
pub fn AddResult() -> impl IntoView {
    let params = use_params_map();
    let registration_id = move || {
        params()
            .get("registration_id")
            .map(|r| Uuid::parse_str(&r).unwrap())
            .unwrap()
    };

    let load_registration_details = move || {
        let registration_id = registration_id().clone();
        async move {
            api_client::registration_details(registration_id)
                .await
                .unwrap()
        }
    };

    let navigate_to_redirect = move |()| {
        let query = use_query_map();
        let redirect = query()
            .get("redirect")
            .map(|r| r.to_string())
            .unwrap_or("/participants".to_string());

        use_navigate()(&redirect, Default::default());
    };

    view! {
        <PageLayout>
            <PageTitle title="Add Result for Registration" subtitle="Add a result to a registration.".to_string().into() />
            <Await future=load_registration_details let:rd>
                 {/* Participant and Competition info next to each other */}
                 <div class="flex flex-row w-full">
                    <div class="">
                        <ParticipantInfoTable participant=rd.participant.clone() />
                    </div>
                    <div class="flex-1">
                        <CompetitionInfoTable competition=rd.competition.clone() />
                    </div>
                </div>

                {/* The form to add the result to the registration */}
                <SectionTitle title="Result for Registration"/>
                <AddResultForm
                    registration_id=rd.id
                    on_added=navigate_to_redirect
                    on_cancel=navigate_to_redirect
                />
            </Await>
        </PageLayout>
    }
}
