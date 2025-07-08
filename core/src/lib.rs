#[derive(serde::Serialize, serde::Deserialize)]
pub struct PublicJournalData {
    pub flag: bool,
    pub day: u32,
    pub month: u32,
    pub year: u32,
}
