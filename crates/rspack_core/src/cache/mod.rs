use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc,
};

use crate::CompilerOptions;

mod local;
mod occasion;
mod snapshot;
mod storage;
pub use local::*;
use occasion::{
  BuildModuleOccasion, CodeGenerateOccasion, CreateChunkAssetsOccasion, ResolveModuleOccasion,
};
use snapshot::SnapshotManager;
use storage::new_storage;

#[derive(Debug)]
pub struct Cache {
  is_idle: AtomicBool,
  snapshot_manager: Arc<SnapshotManager>,
  pub resolve_module_occasion: ResolveModuleOccasion,
  pub build_module_occasion: BuildModuleOccasion,
  pub code_generate_occasion: CodeGenerateOccasion,
  pub create_chunk_assets_occasion: CreateChunkAssetsOccasion,
}

impl Cache {
  pub fn new(options: Arc<CompilerOptions>) -> Self {
    let snapshot_manager = Arc::new(SnapshotManager::new(options.snapshot.clone()));
    Self {
      is_idle: true.into(),
      snapshot_manager: snapshot_manager.clone(),
      resolve_module_occasion: ResolveModuleOccasion::new(
        new_storage(&options.cache),
        snapshot_manager.clone(),
      ),
      build_module_occasion: BuildModuleOccasion::new(
        new_storage(&options.cache),
        snapshot_manager,
      ),
      code_generate_occasion: CodeGenerateOccasion::new(new_storage(&options.cache)),
      create_chunk_assets_occasion: CreateChunkAssetsOccasion::new(new_storage(&options.cache)),
    }
  }

  pub fn begin_idle(&self) {
    if self
      .is_idle
      .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
      .is_ok()
    {
      self.snapshot_manager.clear();
    }
  }

  pub fn end_idle(&self) {
    self.is_idle.store(false, Ordering::Release);
  }
}
