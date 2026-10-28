use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MinistryState {
    pub ministry_id: u32,
    pub ministry_name: String,
    pub project_name: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AuditRecord {
    pub record_id: String,
    pub timestamp: u64,
    pub state_hash: String,
    pub verified: bool,
}

pub fn get_metadata() -> MinistryState {
    MinistryState {
        ministry_id: 18,
        ministry_name: "Ministry of Environment, Forest and Climate Change".to_string(),
        project_name: "EcoLedger AI".to_string(),
        strategy: "Programmatic Utility ($ECO) -- Audited Sovereign Carbon Credits".to_string(),
        target_chain: "Internet Computer (ICP) + Algorand".to_string(),
        is_active: true,
    }
}
