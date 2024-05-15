/// This error exists only to be used a non-UniFFI variant in the flat errors
#[derive(Debug, thiserror::Error)]
pub enum NotUniffiErrorStd {
    #[error(transparent)]
    Whoah(#[from] std::str::Utf8Error),
    #[error(transparent)]
    Stop(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    Right(#[from] std::num::ParseIntError),
    #[error(transparent)]
    There(#[from] std::io::Error),
}

/// Flat error with non-UniFFI variants
#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum FlatErrorA {
    #[error("{0}")]
    CaseA(String),
    #[error(transparent)]
    CaseB(#[from] NotUniffiErrorStd),
}

/// This error is a copy of [`FlatErrorA`] for the purpose of testing different branches
/// in a non-flat error in another test
#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum FlatErrorB {
    #[error("{0}")]
    CaseA(String),
    #[error(transparent)]
    CaseB(#[from] NotUniffiErrorStd),
}

uniffi::setup_scaffolding!("flat_procmacro_errors");
