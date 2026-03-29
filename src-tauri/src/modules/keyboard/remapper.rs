use super::platform;
use crate::config;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RemapStatus {
    pub id: String,
    pub source: String,
    pub target: String,
    pub active: bool,
    pub icon: Option<String>,
    pub label: String,
}

/// Gets the live status of all configured remaps by checking system state.
pub fn get_all_statuses() -> Vec<RemapStatus> {
    let cfg = config::load();
    cfg.keyboard
        .remaps
        .iter()
        .map(|remap| {
            let active = platform::remap_to_xkb_option(&remap.source, &remap.target)
                .and_then(|opt| platform::is_option_active(&opt).ok())
                .unwrap_or(false);

            RemapStatus {
                id: remap.id.clone(),
                source: remap.source.clone(),
                target: remap.target.clone(),
                active,
                icon: remap.icon.clone(),
                label: if active {
                    remap.label_on.clone()
                } else {
                    remap.label_off.clone()
                },
            }
        })
        .collect()
}

/// Toggles a remap by ID. Returns the new status.
pub fn toggle(remap_id: &str) -> Result<RemapStatus, String> {
    let cfg = config::load();
    let remap = cfg
        .keyboard
        .remaps
        .iter()
        .find(|r| r.id == remap_id)
        .ok_or_else(|| format!("Remap '{remap_id}' not found"))?
        .clone();

    let xkb_option = platform::remap_to_xkb_option(&remap.source, &remap.target)
        .ok_or_else(|| format!("No XKB option for {} → {}", remap.source, remap.target))?;

    let now_active = platform::toggle_option(&xkb_option)?;

    Ok(RemapStatus {
        id: remap.id,
        source: remap.source,
        target: remap.target,
        active: now_active,
        icon: remap.icon,
        label: if now_active {
            remap.label_on
        } else {
            remap.label_off
        },
    })
}
