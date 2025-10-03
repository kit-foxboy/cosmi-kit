// SPDX-License-Identifier: MPL-2.0

//! Page modules for the application
//! 
//! Each page is a self-contained module with its own:
//! - State management
//! - Message handling  
//! - View rendering
//! - Business logic

pub mod oc_generator;
//pub mod project_manager;
//pub mod dice_roller;

// Re-export the main types for convenience
pub use oc_generator::OcGeneratorPage;
//pub use project_manager::ProjectManagerPage;
//pub use dice_roller::DiceRollerPage;