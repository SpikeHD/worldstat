use std::{cell::RefCell, path::PathBuf};

#[derive(Clone)]
pub struct ContextInner {
  path: PathBuf,
  is_singleplayer: bool,
}

impl Default for ContextInner {
  fn default() -> Self {
    Self::new()
  }
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
/// Struct used to specify where and how to look for data.
pub struct Context {
  inner: RefCell<ContextInner>,
}

impl Default for Context {
  fn default() -> Self {
    Self::new()
  }
}

impl Context {
  pub fn new() -> Self {
    Context {
      inner: RefCell::new(ContextInner::new()),
    }
  }

  /// Specify the path to the world.
  pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
    self.inner.get_mut().path = path.into();
    self
  }

  /// Specify whether to look for singleplayer or multiplayer data.
  ///
  /// For example, player data is either stored in the `level.dat` file (singleplayer) or in a `playerdata` folder (multiplayer).
  pub fn with_is_singleplayer(mut self, is_singleplayer: bool) -> Self {
    self.inner.get_mut().is_singleplayer = is_singleplayer;
    self
  }

  /// Get the path to the world.
  pub fn path(&self) -> PathBuf {
    self.inner.borrow().path.clone()
  }

  /// Get whether to look for singleplayer or multiplayer data.
  pub fn is_singleplayer(&self) -> bool {
    self.inner.borrow().is_singleplayer
  }
}
