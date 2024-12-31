use std::{
  cell::{Cell, RefCell},
  path::PathBuf,
  sync::Arc,
};

#[derive(Clone)]
pub struct ContextInner {
  path: PathBuf,
  is_singleplayer: bool,
}

impl ContextInner {
  pub fn new() -> Self {
    ContextInner {
      path: PathBuf::new(),
      is_singleplayer: false,
    }
  }
}

#[derive(Clone)]
pub struct Context {
  inner: RefCell<ContextInner>,
}

impl Context {
  pub fn new() -> Self {
    Context {
      inner: RefCell::new(ContextInner::new()),
    }
  }

  pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
    self.inner.get_mut().path = path.into();
    self
  }

  pub fn with_is_singleplayer(mut self, is_singleplayer: bool) -> Self {
    self.inner.get_mut().is_singleplayer = is_singleplayer;
    self
  }

  pub fn path(&self) -> PathBuf {
    self.inner.borrow().path.clone()
  }

  pub fn is_singleplayer(&self) -> bool {
    self.inner.borrow().is_singleplayer
  }
}
