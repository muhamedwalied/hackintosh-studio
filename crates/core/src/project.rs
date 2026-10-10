use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::identifiers::ProjectId;

/// Project manifest stored on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub id: ProjectId,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub version: u32,
    pub hardware_report_path: Option<PathBuf>,
    pub target_macos: Option<String>,
    pub target_opencore: Option<String>,
    pub smbios_profile_path: Option<PathBuf>,
    pub releases_dir: PathBuf,
    pub metadata: HashMap<String, String>,
}

impl ProjectManifest {
    pub fn new(name: String, project_dir: PathBuf) -> Self {
        use chrono::Utc;
        let now = Utc::now().to_rfc3339();
        Self {
            id: ProjectId::new(),
            name,
            created_at: now.clone(),
            updated_at: now,
            version: 1,
            hardware_report_path: None,
            target_macos: None,
            target_opencore: None,
            smbios_profile_path: None,
            releases_dir: project_dir.join("Releases"),
            metadata: HashMap::new(),
        }
    }
}

/// SMBIOS profile stored per project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbiosProfile {
    pub project_id: ProjectId,
    pub hardware_fingerprint: crate::identifiers::HardwareFingerprint,
    pub smbios_data: SmbiosData,
    pub created_at: String,
    pub updated_at: String,
    pub generation_count: u32,
}

/// SMBIOS data values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbiosData {
    pub system_product_name: String,
    pub system_serial_number: String,
    pub system_uuid: String,
    pub board_serial_number: String,
    pub board_asset_tag: String,
    pub board_location_in_chassis: String,
    pub chassis_serial_number: String,
    pub chassis_asset_tag: String,
    pub platform_feature: String,
    pub processor_type: u8,
    pub memory_form_factor: u8,
}
