/**
 * Copyright (c) Enalean, 2024-Present. All Rights Reserved.
 *
 * This file is a part of Tuleap.
 *
 * Tuleap is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * Tuleap is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Tuleap. If not, see <http://www.gnu.org/licenses/>.
 */
use serde_json::{json, Value};
use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    let json: Value = serde_json::from_reader(stdin()).map_err(|e| {
        eprintln!("ser: {e}");
        e
    })?;

    let values = &json["current"]["values"].as_array();

    let field_severity_value = values
        .and_then(|fields| {
            fields
                .into_iter()
                .find(|&field| field["label"] == "Severity")
        })
        .and_then(|field| {
            field["values"]
                .as_array()
                .and_then(|values| values.first())
                .and_then(|value| value["label"].as_str())
                .and_then(|severity_str| severity_str.parse::<i64>().ok())
        });

    let field_probability_value = values
        .and_then(|fields| {
            fields
                .into_iter()
                .find(|&field| field["label"] == "Probability")
        })
        .and_then(|field| {
            field["values"]
                .as_array()
                .and_then(|values| values.first())
                .and_then(|value| value["label"].as_str()) // Extracting the "Probability" value as a string
                .and_then(|probability_str| probability_str.parse::<i64>().ok()) // Converting the string to an integer
        });

    let field_risk = json["tracker"]["fields"]
        .as_array()
        .and_then(|fields| {
            fields
                .iter()
                .find(|&field| field["label"] == "Risk")
        });

    if field_risk.is_none() {
        return Err("Cannot find field_risk")?;
    }

    let risk_values = field_risk.unwrap()["values"].as_array();

    if risk_values.is_none() {
        return Err("Cannot find Risk values")?;
    }

    if let (Some(severity_value), Some(probability_value)) = (field_severity_value, field_probability_value) {
        let product = severity_value * probability_value;

        let matching_value = risk_values.unwrap().into_iter()
        .find(|&value| {
            let value_label = value["label"].as_str().unwrap_or_default();
            value_label == product.to_string()
        });

        if let Some(matching_value) = matching_value {
            let field_id = field_risk.unwrap()["field_id"].as_i64().unwrap_or(0);
            println!("{}", json!({
                "values": [{
                    "field_id": field_id,
                    "bind_value_ids": [
                        matching_value["id"]
                    ]
                }]
            }).to_string());

            Ok(())
        } else {
            return Err("Cannot find matching Risk value")?;
        }
    } else {
        return Err("Cannot find Severity or Probability field")?;
    }
}
