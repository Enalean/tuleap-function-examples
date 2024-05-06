/**
 * MIT License
 *
 * Copyright (c) 2024 Enalean
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::error::Error;
use std::io::stdin;
use std::str::FromStr;

#[derive(Serialize, Debug)]
struct FieldValueBinding {
    field_id: i64,
    bind_value_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListValue {
    id: Option<i64>,
    label: Option<String>,
    color: Option<String>,
    tlp_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum FieldValue {
    #[serde(rename = "string")]
    String {
        field_id: u32,
    },
    #[serde(rename = "text")]
    Text {
        field_id: u32,
    },
    #[serde(rename = "art_link")]
    Link {
        field_id: u32,
    },
    #[serde(rename = "sb")]
    SelectBox {
        field_id: u32,
        label: String,
        values: Vec<ListValue>,
        bind_value_ids: Vec<u32>,
    },
    #[serde(rename = "aid")]
    ArtifactId {
        field_id: u32,
        label: String,
    },
    #[serde(rename = "rb")]
    RadioButton {
        field_id: u32,
        label: String,
    },
}

#[derive(Serialize, Deserialize)]
struct Changeset {
    id: i64,
    values: Vec<FieldValue>,
}

#[derive(Serialize, Deserialize)]
struct TrackerFieldValue {
    id: i64,
    label: String,
}

#[derive(Serialize, Deserialize)]
struct TrackerField {
    field_id: i64,
    label: String,
    values: Option<Vec<TrackerFieldValue>>,
}

#[derive(Serialize, Deserialize)]
struct Tracker {
    id: i64,
    fields: Vec<TrackerField>
}

#[derive(Serialize, Deserialize)]
struct Artifact {
    id: i64,
    current: Changeset,
    tracker: Tracker,
}

fn convert_label_to_integer(label: &str) -> Result<i64, Box<dyn Error>> {
    i64::from_str(label).map_err(Into::into)
}

fn find_select_box_by_label<'a>(changeset: &'a Changeset, target_label: &str) -> Option<&'a FieldValue> {
    changeset.values.iter().find(|field_value| match field_value {
        FieldValue::SelectBox { label, .. } if label == target_label => true,
        _ => false,
    })
}

fn find_select_box_value_by_label(artifact: &Artifact, target_label: &str) -> Result<Option<i64>, Box<dyn Error>> {
    find_select_box_by_label(&artifact.current, target_label)
    .and_then(|field_value| match field_value {
        FieldValue::SelectBox { values, .. } => values.first().and_then(|first_value| first_value.label.as_deref()),
        _ => None,
    })
    .map(|label| convert_label_to_integer(label))
    .transpose()
}

fn process_risk_values(artifact: &Artifact, severity_field_label: &str, probability_field_label: &str, risk_field_label: &str) -> Result<Option<FieldValueBinding>, Box<dyn Error>> {
    
    let severity = find_select_box_value_by_label(&artifact, severity_field_label)?;
    if severity.is_none() {
        return Ok(None);
    }
    let severity_value = severity.unwrap();

    let probability = find_select_box_value_by_label(&artifact, probability_field_label)?;
    if probability.is_none() {
        return Ok(None);
    }
    let probability_value = probability.unwrap();
    

    let risk_field_option = artifact.tracker.fields.iter().find(|field| field.label == risk_field_label);

    if risk_field_option.is_none() {
        return Err(format!("Cannot find field {}", risk_field_label).into());
    }
    let risk_field = risk_field_option.unwrap();

    if risk_field.values.is_none() {
        return Err("Cannot find risk field values".into());
    }
    let risk_values = risk_field.values.as_ref().unwrap();

    let product = severity_value * probability_value;

    let matching_value = risk_values.iter()
        .find(|value| {
            value.label.as_str() == product.to_string()
        });

    if let Some(matching_value) = matching_value {
        Ok(Some(FieldValueBinding {
            field_id: risk_field.field_id,
            bind_value_ids: vec![matching_value.id],
        }))
    } else {
        Err("Cannot find matching Risk value".into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let artifact: Artifact = serde_json::from_reader(stdin()).map_err(|e| {
        eprintln!("Serde unserialize error: {e}");
        e
    })?;

    let mut bindings: Vec<FieldValueBinding> = Vec::new();
    match process_risk_values(&artifact, "Severity", "Probability", "Risk") {
        Ok(possible_binding) => {
            match possible_binding {
                Some(binding) => bindings.push(binding),
                None => ()
            }
        },
        Err(e) => return Err(e),
    }
    match process_risk_values(&artifact, "Residual severity", "Residual probability", "Residual risk level") {
        Ok(possible_binding) => {
            match possible_binding {
                Some(binding) => bindings.push(binding),
                None => ()
            }
        },
        Err(e) => return Err(e),
    }
    println!("{}", json!({
        "values": bindings
    }));

    Ok(())
}
