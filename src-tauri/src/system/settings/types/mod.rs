use std::sync::Arc;
use std::sync::Mutex;

mod app_settings;
mod runtime_settings;

pub use app_settings::AppSettings;
pub use runtime_settings::RuntimeSettings;

pub type WrappedRuntimeSettings = Arc<Mutex<RuntimeSettings>>;
