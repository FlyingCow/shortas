use anyhow::Result;
use dyn_clone::{clone_trait_object, DynClone};

use crate::{core::flow_router::FlowRouterContext, model::{expression::Expression, route::ConditionalRouting}};

pub trait BaseExpressionEvaluator: DynClone {
    fn eval(&self, router_context: &FlowRouterContext, expr: &Expression) -> Result<bool>;
    fn find<'a>(&self, router_context: &FlowRouterContext,  conditions: &'a Vec<ConditionalRouting>) -> Option<&'a ConditionalRouting>;
}

clone_trait_object!(BaseExpressionEvaluator);
