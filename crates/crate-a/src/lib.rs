uniffi::setup_scaffolding!();

#[derive(uniffi::Error)]
pub enum TestError {
    BadThingsHappened(String),
    ErrorFromCrateB(crate_b::CrateBError),
}

#[uniffi::export]
fn add(a: u32, b: u32) -> u32 {
    a + b
}

/* Uncomment this and it fixes the problem, the emitted code now does use Exception
#[uniffi::export]
fn test() -> Result<(), crate_b::CrateBError> {
    Err(crate_b::CrateBError::CrateBScrewedUp(1))
}
*/