//! Alula is a UI framework written in Rust.
//!
//! This crate provides a declarative UI framework inspired by Flutter and SwiftUI,
//! allowing you to build native applications with a widget-based architecture.
//! It uses WGPU for hardware-accelerated rendering and provides a flexible
//! layout system.

/// The main framework module containing all the core components.
pub mod framework;

pub use framework::*;