use leptos::*;
use leptos_router::*;

use crate::components::*;

#[component]
pub fn Event() -> impl IntoView {
    view! {
        <PageLayout>
            <PageTitle
                title="Event".to_string()
                subtitle="Do actions that are relevant to the whole event.".to_string().into()
            />
            <div class="mb-8">
                <A href="/competitions/add" class="btn btn-sm btn-primary rounded-full mr-4">
                    <phosphor_leptos::Printer />
                    Print Registration Cards
                </A>
                <A href="/competitions/add" class="btn btn-sm btn-primary rounded-full mr-4">
                    <phosphor_leptos::Printer />
                    Print Certificates
                </A>
            </div>
        </PageLayout>
    }
}
