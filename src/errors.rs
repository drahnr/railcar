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

    // TODO extract into sub-error
    #[error("Create cgroup dir failed")]
    CreateCGroupDirFailed(#[source] ::std::io::Error),

    #[error("Create cgroup dir failed")]
    RemoveCGroupDirFailed(#[source] ::std::io::Error),

    #[error("Failed to find file {0} in parent cgroups")]
    CGroupParentsNotIncludingFile(String),

    #[error("Failed to copy parent cgroup")]
    CGroupFailedToCopyParent(#[source] ::std::io::Error),

    #[error("Mount failed to mask {1}")]
    MountFailedToMask(#[source] ::nix::Error, String),

    #[error("Mount failed to make path read only {1}")]
    MountMakeReadOnlyFailed(#[source] ::nix::Error, String),

    #[error("Mount failed {1}")]
    MountFailed(#[source] ::nix::Error, String),

    #[error("Mount removing /dev/ptmx failed")]
    MountRemovePtmxDeviceFailed(#[source] ::std::io::Error),

    #[error("Mount remount of {1} failed")]
    MountRemountFailed(#[source] ::nix::Error, String),

    #[error("Attaching action to signal failed")]
    SigAddingActionFailed(#[source] ::nix::Error),

    #[error("Raising signal {1:?} failed")]
    SigRaiseFailed(#[source] ::nix::Error, ::nix::sys::signal::Signal),

    #[error("Unblocking signal failed")]
    SigUnblockFailed(#[source] ::nix::Error),

    #[error("Dumpable returned {0:?}")]
    SetDumpableFailed(i32),
}
