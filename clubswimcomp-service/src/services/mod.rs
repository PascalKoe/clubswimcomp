mod competition;
mod group;
mod participant;
mod registration;
mod registration_card;
mod score;

pub use competition::*;
pub use group::*;
pub use participant::*;
pub use registration::*;
pub use registration_card::*;
pub use score::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceRepositoryError {
    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}
