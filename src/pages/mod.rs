// SPDX-License-Identifier: MPL-2.0

//! Page modules for the application
//! 
//! Each page is a self-contained module with its own:
//! - State management
//! - Message handling  
//! - View rendering
//! - Business logic
//! - Context drawer (if needed)

use crate::{database::*};
use anyhow::Error;
use std::sync::Arc;

pub mod oc_generator;
pub mod project_manager;
//pub mod dice_roller;
pub mod context_pages;

#[derive(Debug, Clone)]
pub enum OCPageMessage {
    LoadData,
    GenerateButtonClicked,
    SaveButtonClicked,
    DeleteCharacter(usize)
}
#[derive(Debug, Clone)]
pub enum ProjectManagerPageMessage {
    LoadData,
    DataLoaded(Arc<Result<Vec<ProjectJoin>, Error>>),
    ToggleCreateProject,
    EditProject(i64),  // Open edit form for project
    ProjectLoadedForEdit(Arc<Result<ProjectJoin, Error>>),  // Project data loaded for editing
    DeleteProject(i64),
    ToggleFeatureCompleted(i64),  // Toggle feature completion status by feature ID
    FeatureToggled(Arc<Result<bool, Error>>),  // Feature toggle result
    ProjectCreated(Arc<Result<Project, Error>>),
    ProjectDeleted(Arc<Result<(), Error>>),
    TagCreated(Arc<Result<Vec<Tag>, Error>>),
    CloseToast(cosmic::widget::ToastId),  // Close a specific toast
}

pub use oc_generator::OcGeneratorPage;
pub use project_manager::ProjectManagerPage;
//pub use dice_roller::DiceRollerPage;