pub use color_eyre::eyre::{anyhow, bail, Context, ContextCompat};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Nix(#[from] ::nix::Error),

    #[error(transparent)]
    Io(#[from] ::std::io::Error),

    #[error(transparent)]
    Ffi(#[from] ::std::ffi::NulError),

    #[error(transparent)]
    Caps(#[from] ::caps::errors::CapsError),

    #[error("invalid spec: '{0}'")]
    InvalidSpec(String),

    #[error("seccomp error: '{0}'")]
    SeccompError(String),

    #[error("timeout after {timeout} milliseconds")]
    Timeout { timeout: i32 },

    #[error("pipe closed: '{0}'")]
    PipeClosed(String),

    #[error("invalid value: '{0}'")]
    InvalidValue(String),

    #[error("invalid hook: '{0}'")]
    InvalidHook(String),

    #[error(transparent)]
    Oci(::oci::Error),

    #[error("{0} is not a valid signal")]
    InvalidSignal(String),

    #[error("Set name returned {0}")]
    SetNameFailed(i32),
}
