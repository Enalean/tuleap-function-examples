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
use serde_json::{json, Value};
use std::error::Error;
use std::io::stdin;

fn is_odd(number: i64) -> bool {
    number & 1 == 1
}

fn main() -> Result<(), Box<dyn Error>> {
    let json: Value = serde_json::from_reader(stdin()).map_err(|e| {
        eprintln!("ser: {e}");
        e
    })?;

    let values = &json["current"]["values"].as_array();

    let field_a_value = values
        .and_then(|fields| {
            fields
                .into_iter()
                .find(|&field| field["label"] == "field_a")
        })
        .and_then(|field| field["value"].as_i64());
    let field_b_value = values
        .and_then(|fields| {
            fields
                .into_iter()
                .find(|&field| field["label"] == "field_b")
        })
        .and_then(|field| field["value"].as_i64());
    let field_sum = values.and_then(|fields| {
        fields
            .into_iter()
            .find(|&field| field["label"] == "field_sum")
    });

    if field_a_value.is_none() {
        return Err("Cannot find field_a")?;
    } else if field_b_value.is_none() {
        return Err("Cannot find field_b")?;
    } else if field_sum.is_none() {
        return Err("Cannot find field_sum")?;
    } else {
        let field_sum_id = field_sum.unwrap()["field_id"].as_i64().unwrap_or(0);
        let value_a = field_a_value.unwrap();
        let value_b = field_b_value.unwrap();

        let sum = value_a + value_b;

        if is_odd(sum) {
            println!("{}", json!({
                "values": [{
                    "field_id": field_sum_id,
                    "value": "odd"
                }],
                "comment": {
                    "body": format!("Sum of field_a and field_b is odd -> {value_a} + {value_b} = {sum}"),
                    "format": "text"
                }
            }).to_string());
        } else {
            println!("{}", json!({
                "values": [{
                    "field_id": field_sum_id,
                    "value": "even"
                }],
                "comment": {
                    "body": format!("Sum of field_a and field_b is even -> {value_a} + {value_b} = {sum}"),
                    "format": "text"
                }
            }).to_string());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::is_odd;

    #[test]
    fn test_is_odd() {
        assert_eq!(true, is_odd(1));
        assert_eq!(false, is_odd(2));
    }
}
