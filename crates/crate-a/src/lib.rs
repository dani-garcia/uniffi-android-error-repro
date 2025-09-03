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
