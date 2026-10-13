use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_bridge_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("resourceType").and_then(|x| x.as_str()) != Some("Patient") {
        r.push(ValidationIssue::error("resourceType", "ADT A04 bridge target is Patient", "bridge-patient"));
    }
    if v.get("identifier").and_then(|i| i.as_array()).is_none_or(|a| a.is_empty()) {
        r.push(ValidationIssue::error("identifier", "v2 PID segment should map to identifier", "pid-identifier"));
    }
    Ok(r)
}
