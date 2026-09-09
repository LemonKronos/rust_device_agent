
//! # Agent filesystem paths
//!
//! Provides the platform-specific filesystem locations used by the agent.
//!
//! [`AgentPath`] is exposed as a common interface to the rest of the workspace.
//! The concrete implementation is selected at compile time:
//!
//! - Linux -> [`linux::AgentPath`]
//! - Windows -> [`windows::AgentPath`]
//!
//! This keeps installation/runtime paths out of the individual crates.
//! Components such as `launcher` and `main_worker` should use `AgentPath`
//! instead of hard-coding paths themselves.
//!
//! ## Important
//!
//! Some paths depend on the `local_workspace` feature and therefore differ
//! between development and installed deployments. When changing these paths,
//! also check the platform-specific installation scripts and packaging logic.

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::AgentPath;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::AgentPath;