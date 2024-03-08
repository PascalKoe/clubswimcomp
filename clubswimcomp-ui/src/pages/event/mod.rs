use leptos::*;
use leptos_router::*;

use crate::components::*;

#[component]
pub fn Event() -> impl IntoView {
    let start_card_link = format!(
        "http://localhost:3000/event/cards",
    );
    view! {
        <PageLayout>
            <PageTitle
                title="Event".to_string()
                subtitle="Do actions that are relevant to the whole event.".to_string().into()
            />
            <div class="mb-8">

                <A href="http://localhost:3000/event/cards" class="btn btn-sm btn-primary rounded-full mr-4">
                    <phosphor_leptos::Printer />
                    Print Registration Cards
                </A>
                <A href="/competitions/add" class="btn btn-sm btn-primary rounded-full mr-4">
                    <phosphor_leptos::Printer />
                    Print Certificates
                </A>
            </div>

            <div>
            </div>
        </PageLayout>
    }
}
