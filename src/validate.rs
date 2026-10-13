
use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

const COMPARTMENT: &[&str] = &["Patient", "Observation", "Condition", "MedicationRequest", "AllergyIntolerance", "DocumentReference", "DiagnosticReport", "Immunization", "CarePlan"];

pub fn validate_everything_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("resourceType").and_then(|x| x.as_str()) != Some("Bundle") {
        r.push(ValidationIssue::error("resourceType", "$everything output is a Bundle", "bundle"));
        return Ok(r);
    }
    let entries = v.get("entry").and_then(|e| e.as_array());
    if entries.is_none_or(|a| a.is_empty()) {
        r.push(ValidationIssue::error("entry", "bundle needs entries", "entries"));
        return Ok(r);
    }
    let patient_ref = v.get("patientRef").and_then(|x| x.as_str()).unwrap_or("Patient/1");
    for (i, e) in entries.unwrap().iter().enumerate() {
        if e.get("fullUrl").and_then(|u| u.as_str()).is_none_or(|s| s.is_empty()) {
            r.push(ValidationIssue::error(format!("entry[{}].fullUrl", i), "fullUrl required in $everything bundle", "fullurl"));
        }
        let rt = e.get("resource").and_then(|res| res.get("resourceType")).and_then(|x| x.as_str());
        if let Some(rt) = rt {
            if !COMPARTMENT.contains(&rt) {
                r.push(ValidationIssue::warning(format!("entry[{}]", i), format!("{rt} is outside typical patient compartment"), "compartment"));
            }
            if rt == "Observation" {
                let subj = e.pointer("/resource/subject/reference").and_then(|x| x.as_str());
                if subj.is_some_and(|s| !s.starts_with(patient_ref)) {
                    r.push(ValidationIssue::error(format!("entry[{}]", i), "resource subject must stay in patient compartment", "foreign-subject"));
                }
            }
        }
    }
    Ok(r)
}
