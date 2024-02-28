mod competition;
mod participant;
mod registration;
mod registration_card;

pub use competition::*;
pub use participant::*;
pub use registration::*;
pub use registration_card::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceRepositoryError {
    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}
