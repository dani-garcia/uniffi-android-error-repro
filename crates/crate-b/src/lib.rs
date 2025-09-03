use std::fmt::Display;

uniffi::setup_scaffolding!();

#[derive(uniffi::Error, Debug)]
pub enum CrateBError {
    CrateBScrewedUp(u64),
}

impl Display for CrateBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrateBError::CrateBScrewedUp(code) => write!(f, "CrateB screwed up with code {}", code),
        }
    }
}
