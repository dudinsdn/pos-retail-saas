mod bootstrap;

pub use bootstrap::run;

pub mod config;
pub mod error;
pub mod shared;
pub mod state;
pub mod telemetry;

pub mod application;
pub mod domain;
pub mod infrastructure;

pub mod api;
pub mod middleware;
pub mod router;
