//! Keeps `route_properties.track_conversions` in step with conversions.
//!
//! Route writes share a lock on their workspace and conversion writes take it
//! exclusively, so a route saved while a conversion is activated is either
//! seen by the conversion or sees it. Take the lock before any row lock.

use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use super::conversion_repo::db_err;
use crate::domain::entities::Result;

const COVERED: &str = "(NOT p.conversion_opt_out AND EXISTS ( \
     SELECT 1 FROM conversions c \
     WHERE c.workspace_id::text = LOWER(p.workspace_id) \
       AND c.status IN ('Activating', 'Active') \
       AND (c.all_routes OR EXISTS ( \
           SELECT 1 FROM conversion_routes cr \
           WHERE cr.conversion_id = c.id AND cr.route_id = p.route_id))))";

async fn lock(
    tx: &mut Transaction<'_, Postgres>,
    function: &str,
    workspace_id: &str,
) -> Result<()> {
    sqlx::query(&format!(
        "SELECT {function}(hashtextextended('conversion-coverage:' || LOWER($1), 0))"
    ))
    .bind(workspace_id)
    .execute(&mut **tx)
    .await
    .map_err(db_err)?;
    Ok(())
}

pub(super) async fn lock_for_route(
    tx: &mut Transaction<'_, Postgres>,
    workspace_id: Option<&str>,
) -> Result<()> {
    match workspace_id {
        Some(id) => lock(tx, "pg_advisory_xact_lock_shared", id).await,
        None => Ok(()),
    }
}

pub(super) async fn lock_for_conversions(
    tx: &mut Transaction<'_, Postgres>,
    workspace_id: Uuid,
) -> Result<()> {
    lock(tx, "pg_advisory_xact_lock", &workspace_id.to_string()).await
}

pub(super) async fn refresh_route(
    tx: &mut Transaction<'_, Postgres>,
    route_id: Uuid,
) -> Result<()> {
    sqlx::query(&format!(
        "UPDATE route_properties p SET track_conversions = {COVERED} WHERE p.route_id = $1"
    ))
    .bind(route_id)
    .execute(&mut **tx)
    .await
    .map_err(db_err)?;
    Ok(())
}

pub(super) async fn refresh_workspace(
    tx: &mut Transaction<'_, Postgres>,
    workspace_id: Uuid,
) -> Result<()> {
    sqlx::query(&format!(
        "UPDATE route_properties p SET track_conversions = {COVERED} \
         WHERE LOWER(p.workspace_id) = $1 AND p.track_conversions <> {COVERED}"
    ))
    .bind(workspace_id.to_string())
    .execute(&mut **tx)
    .await
    .map_err(db_err)?;
    Ok(())
}
