use nalgebra::DMatrix;

use crate::executor::algorithm::{digits::SystemDigitsEnum, models::{Norms, WorkerError}, systems::{GenericSystem, SystemEnum}};

pub struct MatcherContext<'a> {
  pub base: &'a DMatrix<f64>
}

pub struct BuilderContext<'a> {
  pub base: &'a DMatrix<f64>,
  pub digits: SystemDigitsEnum,
  pub norm: Norms
}

pub trait SystemFactory {
  fn matches(&self, ctx: &MatcherContext) -> bool;
  fn create(&self, ctx: BuilderContext) -> Result<SystemEnum, WorkerError>;
}

pub struct GenericFactory;

impl SystemFactory for GenericFactory {
  fn matches(&self, _ctx: &MatcherContext) -> bool {
    GenericSystem::valid_for()
  }

  fn create(&self, ctx: BuilderContext) -> Result<SystemEnum, WorkerError> {
    Ok(SystemEnum::Generic(GenericSystem::new(ctx.base, ctx.digits, ctx.norm)?))
  }
}

pub fn system_factories() -> Vec<Box<dyn SystemFactory>> {
  vec![
    Box::new(GenericFactory)
  ]
}