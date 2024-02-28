use leptos::*;
use phosphor_leptos::Plus;

use crate::Page;

pub fn AddCompetition() -> impl IntoView {
    view! {
        <Page title="Add new Competition".to_string()>
            <div class="mt-2 mb-8 text-sm font-light text-gray-800 max-w-2xl">
                <p>Add a new competition to the event. Only participants with the same gender can participate in the competition. There must not be any competition with exactly the same parameters.</p>
            </div>

            <label class="form-control w-full max-w-2xl">
                <div class="label">
                    <span class="label-text">Gender</span>
                </div>
                <select class="select select-bordered">
                    <option value="Female">Female</option>
                    <option value="Male">Male</option>
                </select>
            </label>

            <label class="form-control w-full max-w-2xl">
                <div class="label">
                    <span class="label-text">Stroke</span>
                </div>
                <select class="select select-bordered">
                    <option value="Butterfly">Butterfly</option>
                    <option value="Back">Back</option>
                    <option value="Breast">Breast</option>
                    <option value="Freestyle">Freestyle</option>
                </select>
            </label>

            <label class="form-control w-full max-w-2xl">
                <div class="label">
                    <span class="label-text">Distance</span>
                </div>
                <select class="select select-bordered">
                    <option value="25">25 Meters</option>
                    <option value="50">50 Meters</option>
                </select>
            </label>

            <div class="form-control w-full max-w-2xl mt-4">
                <button class="btn btn-sm btn-primary">
                    Add Competition
                </button>
            </div>
            <div class="form-control w-full max-w-2xl mt-4">
                <a href="/competitions" class="btn btn-sm btn-neutral">
                    Cancel
                </a>
            </div>
        </Page>
    }
}
