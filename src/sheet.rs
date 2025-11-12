use crate::error::Error;
use google_sheets4 as sheets4;
use google_sheets4::api::{AppendValuesResponse, BatchUpdateValuesRequest, BatchUpdateValuesResponse, ValueRange};
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde_json::Value;
use sheets4::{oauth2, oauth2::ServiceAccountAuthenticator, Sheets};
use std::result::Result;

pub async fn get(file: String) -> Result<Sheets<HttpsConnector<HttpConnector>>, Error> {
    let credentials = oauth2::read_service_account_key(file)
        .await
        .map_err(|e| Error::new("Cannot read credentials, an error occurred".to_string(), e))?;

    let authorization = ServiceAccountAuthenticator::builder(credentials)
        .build()
        .await
        .map_err(|e| {
            Error::new(
                "There was an error trying to build connection with authenticator".to_string(),
                e,
            )
        })?;

    let connector = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .build();

    let client = Client::builder().build(connector);

    Ok(Sheets::new(client, authorization))
}

pub async fn fetch(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    sheet_id: &str,
    range: &str,
) -> Result<ValueRange, Error> {
    let (_, response) = hub
        .spreadsheets()
        .values_get(sheet_id, range)
        .doit()
        .await
        .map_err(|e| Error::new("Could not fetch data".to_string(), e))?;

    Ok(response)
}

pub async fn append(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    data: Vec<Value>,
    sheet_id: &str,
    range: &str,
) -> Result<AppendValuesResponse, Error> {
    let request = ValueRange {
        major_dimension: None,
        range: None,
        values: Some(vec![data]),
    };

    let (_, response) = hub
        .spreadsheets()
        .values_append(request, sheet_id, range)
        .value_input_option("USER_ENTERED")
        .doit()
        .await
        .map_err(|e| Error::new("Could not populate data".to_string(), e))?;

    Ok(response)
}

pub async fn update(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    data: Vec<ValueRange>,
    sheet_id: &str,
) -> Result<BatchUpdateValuesResponse, Error> {
    let batch_request = BatchUpdateValuesRequest {
        data: Some(data),
        include_values_in_response: None,
        response_date_time_render_option: None,
        response_value_render_option: None,
        value_input_option: Some("USER_ENTERED".to_string()),
    };

    let (_, response) = hub
        .spreadsheets()
        .values_batch_update(batch_request, sheet_id)
        .doit()
        .await
        .map_err(|e| Error::new("Could not update data".to_string(), e))?;

    Ok(response)
}
