use salvo::oapi::endpoint;
use salvo::prelude::*;

use crate::adapters::api::{
    app_state::AppState, error_presenter::ErrorResponse as ErrorPresenter,
    openapi_schemas::ErrorResponse,
};
use crate::adapters::rabbitmq::messages::ConversionChangedMessage;
use crate::model::error::{ApiError, ValidationError};
use crate::model::Conversion;

pub fn api_routes() -> Router {
    Router::with_path("/conversions").push(
        Router::with_path("/{public_key}")
            .put(put_conversion)
            .delete(delete_conversion),
    )
}

fn invalid(field: &str, message: String) -> ErrorPresenter {
    ErrorPresenter::from_api_error(&ApiError::Validation(ValidationError::InvalidInput {
        field: field.to_string(),
        message,
    }))
}

async fn store(app_state: &AppState, conversion: &Conversion) -> anyhow::Result<()> {
    app_state.conversion_store.put_conversion(conversion).await?;

    // Also after a repeated delivery: the first one may have been stored
    // without reaching the routers.
    if let Some(ref publisher) = app_state.rabbitmq_publisher {
        publisher
            .publish_conversion_changed(&ConversionChangedMessage {
                public_key: conversion.public_key.clone(),
            })
            .await;
    }
    Ok(())
}

/// Store a conversion definition
#[endpoint(
    operation_id = "put_conversion",
    summary = "Store a conversion definition",
    description = "Stores the definition unless a newer version is already stored, and tells routers to reload it.",
    parameters(("public_key" = String, Path, description = "The conversion's public key")),
    request_body = Conversion,
    responses(
        (status_code = 204, description = "Stored, or a newer version already is"),
        (status_code = 400, description = "Invalid definition", body = ErrorResponse)
    )
)]
pub async fn put_conversion(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let public_key = req.param::<String>("public_key").unwrap_or_default();
    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    let error_response = match req.parse_json::<Conversion>().await {
        Err(e) => invalid("body", format!("Invalid JSON: {e}")),
        Ok(c) if c.public_key != public_key => {
            invalid("publicKey", "Does not match the path".to_string())
        }
        Ok(conversion) => match store(app_state, &conversion).await {
            Ok(()) => {
                res.status_code(StatusCode::NO_CONTENT);
                return;
            }
            Err(e) => ErrorPresenter::map_error(e),
        },
    };
    res.status_code(error_response.status_code);
    res.render(error_response);
}

/// Stop a deleted conversion
#[endpoint(
    operation_id = "delete_conversion",
    summary = "Stop a deleted conversion",
    description = "Replaces the definition with an inactive one that later updates cannot replace.",
    parameters(("public_key" = String, Path, description = "The conversion's public key")),
    responses((status_code = 204, description = "Stopped"))
)]
pub async fn delete_conversion(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let public_key = req.param::<String>("public_key").unwrap_or_default();
    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    match store(app_state, &Conversion::tombstone(public_key)).await {
        Ok(()) => {
            res.status_code(StatusCode::NO_CONTENT);
        }
        Err(e) => {
            let error_response = ErrorPresenter::map_error(e);
            res.status_code(error_response.status_code);
            res.render(error_response);
        }
    }
}
