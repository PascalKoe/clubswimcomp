mod participant;
mod results;

pub use participant::*;
pub use results::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceRepositoryError {
    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}
