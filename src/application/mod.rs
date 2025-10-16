// SPDX-License-Identifier: MPL-2.0
pub mod app;
pub mod message;
pub mod app_data;

use crate::config::Config;
use cosmic::widget::{menu, nav_bar};
use std::collections::HashMap;

use crate::pages::*;

pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
pub const APP_ICON: &[u8] = include_bytes!("../../resources/icons/hicolor/scalable/apps/icon.svg");

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: cosmic::Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Contains items assigned to the nav bar panel.
    nav: nav_bar::Model,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    // Configuration data that persists between application runs.
    config: Config,
    // Data layer - handles all data operations (database, caching, etc.)
    // Pages don't touch the database directly - they go through AppData!
    app_data: app_data::AppData,
    // pages (just UI and state management)
    oc_generator_page: oc_generator::OcGeneratorPage,
    project_manager_page: project_manager::ProjectManagerPage,
}

pub use message::Message;

/// The page to display in the application.
#[derive(Clone)]
pub enum Page {
    OCGenerator,
    ProjectManager,
    DiceRoller,
}

/// The context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
    NewProject,  // Form for creating a new project
}

/// Actions that can be triggered from the menu bar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}