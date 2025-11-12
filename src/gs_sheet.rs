use crate::error::Error;
use crate::gs_args::PRData;
use crate::sheet;
use crate::sheet_model::SheetModel;
use google_sheets4::api::ValueRange;
use google_sheets4::Sheets;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use serde_json::Value;
use std::collections::HashMap;

pub async fn add(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    pr_data: PRData,
    sheet_id: &str,
    sheet_name: &str,
) -> Result<String, Error> {
    let info: Vec<Value> = pr_data
        .data
        .into_iter()
        .map(|s| {
            let value = if s.is_empty() {
                "n/a".to_string()
            } else {
                s.to_string().replace("\\\"", "\"").replace("\\`", "`")
            };
            Value::String(value)
        })
        .collect();

    let pr_number = if let Value::String(v) = &info[0] {
        v.to_string()
    } else {
        "".to_string()
    };

    sheet::append(&hub, info, sheet_id, format!("{}!A:Z", sheet_name).as_str()).await?;

    Ok(format!("PR [{}] added to sheet", pr_number))
}

pub async fn done(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    pr_titles: Vec<String>,
    branch: &str,
    sheet_id: &str,
    sheet_name: &str,
) -> Result<String, Error> {
    let sheet_data = sheet::fetch(&hub, sheet_id, format!("{}!A:Z", sheet_name).as_str())
        .await?
        .values;

    let value_ranges: Vec<ValueRange> = get_pr_data_by_pr_title(sheet_data, pr_titles.clone())
        .iter()
        .map(|map| {
            let mut updated_row: Vec<Value> = vec![];
            updated_row.push(Value::String(map["Number"].to_string()));
            updated_row.push(Value::String(map["Title"].to_string()));
            updated_row.push(Value::String(map["Description"].to_string()));
            updated_row.push(Value::String(map["Author"].to_string()));
            updated_row.push(Value::String(map["URL"].to_string()));
            updated_row.push(Value::String(map["Commit Hash"].to_string()));
            updated_row.push(Value::String(map["Merged Date"].to_string()));
            updated_row.push(Value::String(map["Deployable"].to_string()));

            let rc_value = if branch == "rc" {
                "TRUE".to_string()
            } else {
                map["RC"].to_string()
            };

            let master_value = if branch == "master" {
                "TRUE".to_string()
            } else {
                map["Production"].to_string()
            };

            updated_row.push(Value::String(rc_value));
            updated_row.push(Value::String(master_value));

            return ValueRange {
                major_dimension: None,
                range: Some(format!("{}!{}", sheet_name, &map["range"])),
                values: Some(vec![updated_row]),
            };
        })
        .collect();

    let _ = sheet::update(&hub, value_ranges, sheet_id).await?;

    return Ok(format!(
        "PR(s) with title(s) [{:?}] marked as done for [{}]",
        pr_titles, branch
    ));
}

pub async fn fetch(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    sheet_id: &str,
    sheet_name: &str,
) -> Result<String, Error> {
    let response = sheet::fetch(hub, sheet_id, format!("{}!A:Z", sheet_name).as_str()).await?;
    let range = response.range.unwrap_or_else(String::new);
    let sheet_model = SheetModel {
        range,
        values: process_sheet_data(response.values),
    };

    let serialized_sheet_model = serde_json::to_string(&sheet_model)
        .map_err(|e| Error::new("Could not serialize sheet data".to_string(), e))?;

    Ok(serialized_sheet_model)
}

fn get_pr_data_by_pr_title(
    raw_values: Option<Vec<Vec<Value>>>,
    titles: Vec<String>,
) -> Vec<HashMap<String, String>> {
    let mut data: Vec<HashMap<String, String>> = vec![];

    for (index, processed_value) in process_sheet_data(raw_values).iter().enumerate() {
        let title_from_sheet = &processed_value["Title"];

        let mut has_match = false;
        for title in titles.clone() {
            if !title.is_empty() && title.starts_with(title_from_sheet) {
                has_match = true;
                break;
            }
        }

        if has_match {
            let mut processed_value = processed_value.clone();
            processed_value.insert(
                "range".to_string(),
                format!("A{}:Z{}", index + 2, index + 2),
            );
            data.push(processed_value);
        }
    }

    data
}

fn process_sheet_data(raw_values: Option<Vec<Vec<Value>>>) -> Vec<HashMap<String, String>> {
    let mut values: Vec<HashMap<String, String>> = vec![];

    match raw_values {
        None => { /*no-op*/ }
        Some(raw_values) => {
            if raw_values.len() > 0 {
                let mut headers = raw_values[0].clone();

                // Remove empty items
                headers.retain(|x| {
                    if let Value::String(v) = x {
                        return !v.is_empty();
                    }
                    false
                });

                for (index, raw_value) in raw_values.iter().enumerate() {
                    // The first row contains the headers; so it can be skipped
                    if index == 0 {
                        continue;
                    }

                    let mut map = HashMap::<String, String>::new();
                    for i in 0..headers.len() {
                        let key = if let Value::String(v) = &headers[i] {
                            v.clone()
                        } else {
                            "".to_string()
                        };

                        let value = if let Value::String(v) = &raw_value[i] {
                            v.clone()
                        } else {
                            "".to_string()
                        };

                        map.insert(key, value);
                    }
                    values.push(map);
                }
            }
        }
    }

    values
}
