use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::{core::base_flow_router::FlowRouterContext, model::expression::Expression};

pub trait BaseExpressionEvaluator: DynClone {
    fn eval(&self, router_context: &FlowRouterContext, expr: &Expression) -> Result<bool>;
}

clone_trait_object!(BaseExpressionEvaluator);
