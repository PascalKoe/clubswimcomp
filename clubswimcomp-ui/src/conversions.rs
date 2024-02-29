use clubswimcomp_types::model;

pub fn gender_long(gender: &model::Gender) -> String {
    match gender {
        model::Gender::Female => "Female",
        model::Gender::Male => "Male",
    }
    .to_string()
}

pub fn gender_short(gender: &model::Gender) -> String {
    match gender {
        model::Gender::Female => "F",
        model::Gender::Male => "M",
    }
    .to_string()
}

pub fn stroke(stroke: &model::Stroke) -> String {
    match stroke {
        model::Stroke::Butterfly => "Butterfly",
        model::Stroke::Back => "Back",
        model::Stroke::Breast => "Breast",
        model::Stroke::Freestyle => "Freestyle",
    }
    .to_string()
}
