// SPDX-License-Identifier: MPL-2.0

//! Project Manager Page
//! 
//! Track development projects, tasks, and progress with a simple kanban-style interface

use cosmic::prelude::*;
use cosmic::widget::{self, container, text};
use cosmic::iced::Length;
use crate::fl;

use crate::database::{Project, Tag, Feature, ProjectTags, ProjectFeatures};

/// Messages that the Project Manager page can emit, breaking down async vs UI state
#[derive(Debug, Clone)]
pub enum Message {
    // Trigger async operations
    LoadData,
    
    // Results from async operations
    ProjectsLoaded(Result<Vec<(Project, ProjectTags, ProjectFeatures)>, String>),
    ProjectCreated(Result<Project, String>),
    ProjectDeleted(Result<(), String>),
}

/// State for the Project Manager page
pub struct ProjectManagerPage {
    // Data from database
    projects: Vec<(Project, ProjectTags, ProjectFeatures)>,
    
    // UI state
    new_project_name: String,
    new_project_description: String,
    show_add_project_dialog: bool,
    is_loading: bool,
    error_message: Option<String>,
}

impl Default for ProjectManagerPage {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            new_project_name: String::new(),
            new_project_description: String::new(),
            show_add_project_dialog: false,
            is_loading: false,
            error_message: None,
        }
    }
}

impl ProjectManagerPage {
    /// Create the view for this page
    pub fn view(&self) -> Element<Message> {
        // TODO: Build your full UI
        widget::text::title1("Project Manager - Loading...")
            .apply(widget::container)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// Handle messages for this page
    /// 
    /// This method ONLY handles UI state updates.
    /// Data operations (LoadData, CreateProject, etc.) are handled by AppModel.
    pub fn update(&mut self, message: Message) {
        match message {
            // Data request messages - set loading state, AppModel will handle actual work
            Message::LoadData => {
                self.is_loading = true;
                // AppModel will trigger the actual async load
            }
            
            // Data result messages - update UI state with results
            Message::ProjectsLoaded(result) => {
                self.is_loading = false;
                match result {
                    Ok(projects) => {
                        self.projects = projects;
                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to load projects: {}", e));
                    }
                }
            }
            
            Message::ProjectCreated(result) => {
                self.is_loading = false;
                match result {
                    Ok(_project) => {
                        self.new_project_name.clear();
                        self.new_project_description.clear();
                        self.show_add_project_dialog = false;
                        // AppModel will reload projects automatically
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to create project: {}", e));
                    }
                }
            }
            
            Message::ProjectDeleted(result) => {
                self.is_loading = false;
                match result {
                    Ok(()) => {
                        self.error_message = None;
                        // AppModel will reload projects automatically
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to delete project: {}", e));
                    }
                }
            }
        }
    }
    
    // Helper methods for UI logic
    
    /// Get total number of projects
    pub fn total_projects(&self) -> usize {
        self.projects.len()
    }
    
    /// Check if a project name is valid (not empty, reasonable length)
    pub fn is_valid_project_name(name: &str) -> bool {
        !name.trim().is_empty() && name.len() <= 100
    }
    
    // TODO: Add more helper methods as needed
    // - fn completed_features_count(&self, project_id: i64) -> usize
    // - fn project_by_id(&self, id: i64) -> Option<&Project>
    // - fn format_project_summary(&self, project: &Project) -> String
}