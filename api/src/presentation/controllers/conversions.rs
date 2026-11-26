//! Conversions controller, mounted under a workspace.

use salvo::prelude::*;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::application::dto::{
    ConversionDomainDto, ConversionDto, ConversionInputDto, ConversionWithSecretDto, UpdateConversionDto,
};
use crate::application::services::ConversionService;
use crate::domain::entities::{ApiError, Conversion, Result};
use crate::presentation::middleware::{
    render_created, render_error, render_success, DepotExt, UserExt,
};

use crate::infrastructure::http_clients::ConversionLog;

use super::cors_preflight;

/// Build conversions controller router.
pub fn conversions_controller() -> Router {
    let action = |path: &str| Router::with_path(path).options(cors_preflight);

    Router::with_path("conversions")
        .get(list_conversions)
        .post(create_conversion)
        .options(cors_preflight)
        .push(action("domains").get(list_conversion_domains))
        .push(
            Router::with_path("{conversion_id}")
                .get(get_conversion)
                .put(update_conversion)
                .options(cors_preflight)
                .push(action("events").get(list_conversion_events))
                .push(action("activate").post(activate_conversion))
                .push(action("pause").post(pause_conversion))
                .push(action("archive").post(archive_conversion))
                .push(action("rotate-secret").post(rotate_conversion_secret)),
        )
}

struct Context<'a> {
    service: &'a ConversionService,
    user_id: String,
    workspace_id: Uuid,
}

fn context<'a>(req: &Request, depot: &'a Depot) -> Result<Context<'a>> {
    Ok(Context {
        service: &depot
            .app_state()
            .map_err(|e| ApiError::internal(e.to_string()))?
            .conversion_service,
        user_id: depot
            .user_id()
            .map_err(|e| ApiError::unauthorized(e.to_string()))?,
        workspace_id: uuid_param(req, "id", "workspace")?,
    })
}

fn uuid_param(req: &Request, name: &str, entity: &str) -> Result<Uuid> {
    req.param::<Uuid>(name)
        .ok_or_else(|| ApiError::validation(format!("Invalid {entity} ID")))
}

async fn body<T: DeserializeOwned>(req: &mut Request) -> Result<T> {
    req.parse_json()
        .await
        .map_err(|e| ApiError::validation(e.to_string()))
}

fn conversion_id(req: &Request) -> Result<Uuid> {
    uuid_param(req, "conversion_id", "conversion")
}

fn render(res: &mut Response, result: Result<Conversion>) {
    match result {
        Ok(c) => render_success(res, ConversionDto::from(c)),
        Err(e) => render_error(res, e),
    }
}

fn with_secret((conversion, secret_key): (Conversion, String)) -> ConversionWithSecretDto {
    ConversionWithSecretDto {
        secret_key,
        conversion: conversion.into(),
    }
}

/// List a workspace's conversions.
#[endpoint(
    operation_id = "list_conversions",
    summary = "List conversions",
    tags("Conversions"),
    parameters(("id" = String, Path, description = "Workspace ID")),
    responses((status_code = 200, description = "Conversions, newest first", body = Vec<ConversionDto>))
)]
pub async fn list_conversions(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = async {
        let c = context(req, depot)?;
        c.service.list(c.workspace_id, &c.user_id).await
    };
    match result.await {
        Ok(list) => render_success(
            res,
            list.into_iter().map(ConversionDto::from).collect::<Vec<_>>(),
        ),
        Err(e) => render_error(res, e),
    }
}

/// Create a draft conversion.
#[endpoint(
    operation_id = "create_conversion",
    summary = "Create conversion",
    description = "Create a draft. The secret key is returned only in this response.",
    tags("Conversions"),
    parameters(("id" = String, Path, description = "Workspace ID")),
    request_body(content = ConversionInputDto, description = "Definition"),
    responses(
        (status_code = 201, description = "Created", body = ConversionWithSecretDto),
        (status_code = 400, description = "Invalid definition; details list the fields")
    )
)]
pub async fn create_conversion(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = async {
        let c = context(req, depot)?;
        let dto: ConversionInputDto = body(req).await?;
        c.service.create(c.workspace_id, dto.into(), &c.user_id).await
    };
    match result.await {
        Ok(created) => render_created(res, with_secret(created)),
        Err(e) => render_error(res, e),
    }
}

/// Get a conversion.
#[endpoint(
    operation_id = "get_conversion",
    summary = "Get conversion",
    tags("Conversions"),
    parameters(
        ("id" = String, Path, description = "Workspace ID"),
        ("conversion_id" = String, Path, description = "Conversion ID")
    ),
    responses((status_code = 200, description = "Conversion", body = ConversionDto))
)]
pub async fn get_conversion(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = async {
        let c = context(req, depot)?;
        c.service.get(c.workspace_id, conversion_id(req)?, &c.user_id).await
    };
    render(res, result.await);
}

/// Replace a conversion's definition.
#[endpoint(
    operation_id = "update_conversion",
    summary = "Update conversion",
    tags("Conversions"),
    parameters(
        ("id" = String, Path, description = "Workspace ID"),
        ("conversion_id" = String, Path, description = "Conversion ID")
    ),
    request_body(content = UpdateConversionDto, description = "Definition and the version being edited"),
    responses(
        (status_code = 200, description = "Updated", body = ConversionDto),
        (status_code = 409, description = "Changed by someone else, or archived")
    )
)]
pub async fn update_conversion(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = async {
        let c = context(req, depot)?;
        let id = conversion_id(req)?;
        let dto: UpdateConversionDto = body(req).await?;
        c.service
            .update(c.workspace_id, id, dto.definition.into(), dto.version, &c.user_id)
            .await
    };
    render(res, result.await);
}

/// Start tracking.
#[endpoint(
    operation_id = "activate_conversion",
    summary = "Activate conversion",
    tags("Conversions"),
    parameters(
        ("id" = String, Path, description = "Workspace ID"),
        ("conversion_id" = String, Path, description = "Conversion ID")
    ),
    responses(
        (status_code = 200, description = "Activating", body = ConversionDto),
        (status_code = 422, description = "No eligible domain; details name the missing condition")
    )
)]
pub async fn activate_conversion(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = async {
        let c = context(req, depot)?;
        c.service.activate(c.workspace_id, conversion_id(req)?, &c.user_id).await
    };
    render(res, result.await);
}

/// Stop tracking and keep stored events.
#[endpoint(
    operation_id = "pause_conversion",
    summary = "Pause conversion",
    tags("Conversions"),
    parameters(
        ("id" = String, Path, description = "Workspace ID"),
        ("conversion_id" = String, Path, description = "Conversion ID")
    ),
    responses((status_code = 200, description = "Paused", body = ConversionDto))
)]
pub async fn pause_conversion(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = async {
        let c = context(req, depot)?;
        c.service.pause(c.workspace_id, conversion_id(req)?, &c.user_id).await
    };
    render(res, result.await);
}

/// Retire a conversion for good and keep stored events.
#[endpoint(
    operation_id = "archive_conversion",
    summary = "Archive conversion",
    tags("Conversions"),
    parameters(
        ("id" = String, Path, description = "Workspace ID"),
        ("conversion_id" = String, Path, description = "Conversion ID")
    ),
    responses((status_code = 200, description = "Archived", body = ConversionDto))
)]
pub async fn archive_conversion(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = async {
        let c = context(req, depot)?;
        c.service.archive(c.workspace_id, conversion_id(req)?, &c.user_id).await
    };
    render(res, result.await);
}

/// Replace the secret key.
#[endpoint(
    operation_id = "rotate_conversion_secret",
    summary = "Rotate secret key",
    description = "The new key is returned only in this response, and the old one stops working.",
    tags("Conversions"),
    parameters(
        ("id" = String, Path, description = "Workspace ID"),
        ("conversion_id" = String, Path, description = "Conversion ID")
    ),
    responses((status_code = 200, description = "Rotated", body = ConversionWithSecretDto))
)]
pub async fn rotate_conversion_secret(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = async {
        let c = context(req, depot)?;
        c.service.rotate_secret(c.workspace_id, conversion_id(req)?, &c.user_id).await
    };
    match result.await {
        Ok(rotated) => render_success(res, with_secret(rotated)),
        Err(e) => render_error(res, e),
    }
}

const DEFAULT_EVENTS: u32 = 50;
const MAX_EVENTS: u32 = 100;
const MAX_EVENTS_OFFSET: u32 = 10_000;

/// Recent events, including rejected ones.
#[endpoint(
    operation_id = "list_conversion_events",
    summary = "List conversion events",
    description = "Newest first. Rejected events are kept for 30 days.",
    tags("Conversions"),
    parameters(
        ("id" = String, Path, description = "Workspace ID"),
        ("conversion_id" = String, Path, description = "Conversion ID"),
        ("limit" = Option<u32>, Query, description = "Up to 100 (default: 50)"),
        ("offset" = Option<u32>, Query, description = "Up to 10000 (default: 0)")
    ),
    responses(
        (status_code = 200, description = "Events", body = ConversionLog),
        (status_code = 404, description = "Not a conversion of a workspace of yours")
    )
)]
pub async fn list_conversion_events(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = async {
        let c = context(req, depot)?;
        let conversion = c
            .service
            .get(c.workspace_id, conversion_id(req)?, &c.user_id)
            .await?;
        let limit = req.query("limit").unwrap_or(DEFAULT_EVENTS).clamp(1, MAX_EVENTS);
        let offset = req.query("offset").unwrap_or(0).min(MAX_EVENTS_OFFSET);
        depot
            .app_state()
            .map_err(|e| ApiError::internal(e.to_string()))?
            .click_aggregator
            .get_conversion_events(c.workspace_id, conversion.id, limit, offset)
            .await
            .map_err(|e| {
                tracing::error!("Reading conversion events failed: {e:#}");
                ApiError::external_service("Conversion events are unavailable")
            })
    };
    match result.await {
        Ok(log) => render_success(res, log),
        Err(e) => render_error(res, e),
    }
}

/// Domains for a workspace's conversions.
#[endpoint(
    operation_id = "list_conversion_domains",
    summary = "List conversion domains",
    description = "Admins get every domain they can choose, Members those in use.",
    tags("Conversions"),
    parameters(("id" = String, Path, description = "Workspace ID")),
    responses((status_code = 200, description = "Domains", body = Vec<ConversionDomainDto>))
)]
pub async fn list_conversion_domains(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = async {
        let c = context(req, depot)?;
        c.service.domains(c.workspace_id, &c.user_id).await
    };
    match result.await {
        Ok(list) => render_success(
            res,
            list.into_iter().map(ConversionDomainDto::from).collect::<Vec<_>>(),
        ),
        Err(e) => render_error(res, e),
    }
}

/// Conversions that removing a domain would stop, for its owner.
#[endpoint(
    operation_id = "list_domain_conversions",
    summary = "List conversions on a domain",
    tags("Domains"),
    parameters(("id" = String, Path, description = "Domain ID")),
    responses(
        (status_code = 200, description = "Activating and active conversions", body = Vec<ConversionDto>),
        (status_code = 404, description = "Not a domain of yours")
    )
)]
pub async fn list_domain_conversions(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let result = async {
        let id = uuid_param(req, "id", "domain")?;
        let user_id = depot
            .user_id()
            .map_err(|e| ApiError::unauthorized(e.to_string()))?;
        depot
            .app_state()
            .map_err(|e| ApiError::internal(e.to_string()))?
            .conversion_service
            .list_on_domain(id, &user_id)
            .await
    };
    match result.await {
        Ok(list) => render_success(
            res,
            list.into_iter().map(ConversionDto::from).collect::<Vec<_>>(),
        ),
        Err(e) => render_error(res, e),
    }
}
