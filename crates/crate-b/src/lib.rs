uniffi::setup_scaffolding!();

#[derive(uniffi::Error)]
pub enum CrateBError {
    CrateBScrewedUp(u64),
}
