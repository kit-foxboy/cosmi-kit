// SPDX-License-Identifier: MPL-2.0

//! Project Manager Page
//!
//! Track development projects, tasks, and progress with a simple kanban-style interface

// use slotmap::SlotMap;
use cosmic::iced::Length;
use cosmic::theme;
use cosmic::widget::{self, icon, Toast, Toasts};
use cosmic::{Element, Task};

use crate::database::ProjectJoin;
use crate::pages::ProjectManagerPageMessage as Message;
use crate::pages::context_pages::new_project;


/// State for the Project Manager page
pub struct ProjectManagerPage {
    // Data from database
    projects: Vec<ProjectJoin>,

    // UI state (public so AppModel can read for context drawer form)
    // pub context_page: Option<ContextPage>,
    pub new_project_form: new_project::NewProjectPage,
    is_loading: bool,
    toasts: Toasts<Message>,
    error_message: Option<String>,
}

impl Default for ProjectManagerPage {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            // context_page: None,
            new_project_form: new_project::NewProjectPage::default(),
            is_loading: true,
            toasts: Toasts::new(Message::CloseToast),
            error_message: None,
        }
    }
}

impl ProjectManagerPage {

    /// Create the view for this page
    pub fn view(&'_ self) -> Element<'_, Message> {
        println!("{:?}", self.projects);

        // TODO: Build full UI
        let header = widget::header_bar()
            .title("Project Manager")
            .end(
                widget::button::icon(icon::from_name("list-add-symbolic"))
                    .on_press(Message::ToggleCreateProject)
                    .tooltip("Add New Project")
                    .class(theme::Button::Suggested)
            );

        let content = widget::container(header)
            .width(Length::Fill)
            .height(Length::Fill);

        // Wrap with toaster to show toast notifications
        widget::toaster(&self.toasts, content).into()
    }

    /// Handle messages for this page
    ///
    /// This method ONLY handles UI state updates.
    /// Data operations (LoadData, CreateProject, etc.) are handled by AppModel.
    pub fn update(&mut self, message: Message) -> cosmic::Task<cosmic::Action<Message>> {
        match message {
            Message::LoadData => {
                self.is_loading = true;
                // AppModel will trigger the actual async load
            }
            Message::DataLoaded(result) => {
                self.is_loading = false;
                match result.as_ref() {
                    Ok(projects) => {
                        self.projects = projects.to_vec();
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to load projects: {}", e));
                    }
                }
            }
            // Message::ProjectCreated(result) => {
            //     self.is_loading = false;
            //     match result {
            //         Ok(projects) => {
            //             self.projects = projects;
            //             self.error_message = None;
            //         }
            //         Err(e) => {
            //             self.error_message = Some(format!("Failed to load projects: {}", e));
            //         }
            //     }
            // }
            Message::ProjectCreated(result) => {
                self.is_loading = false;
                match result.as_ref() {
                    Ok(project) => {
                        // Show success toast
                        let _ = self.toasts.push(
                            Toast::new(format!("{}: {}", crate::fl!("project-created"), project.name))
                        );
                        return Task::done(cosmic::Action::App(Message::LoadData));
                    }
                    Err(e) => {
                        // Show error toast
                        let _ = self.toasts.push(
                            Toast::new(format!("{}: {}", crate::fl!("error"), e))
                        );
                    }
                }
            }
            Message::CreateProject(_name, _description) => {
                self.is_loading = true;
                // AppModel handles the actual async creation
            }
            Message::DeleteProject(_id) => {
                self.is_loading = true;
                // AppModel handles the actual async deletion
            }
            Message::ToggleCreateProject => {
                // Handled by app.rs to toggle the context drawer
            }
            Message::CloseToast(id) => {
                // Close the toast with the given ID
                self.toasts.remove(id);
            }
        }

        Task::none()
    }
}

// impl ProjectManagerPage {
//     pub fn context_drawer(&'_ self) -> Option<ContextDrawer<'_, Message>> {
//         if let Some(context_page) = &self.context_page {
//             let content = match context_page {
//                 ContextPage::NewProject => self.new_project_form.view(),
//                 // Add other context pages here as needed
//                 _ => return None,
//             };
//             Some(context_drawer::context_drawer(
//                 content,
//                 Message::ToggleContextDrawer(None), // Close drawer on outside click
//             ).title(self.new_project_form.title.clone())
//             )
//         } else {
//             None
//         }
//     }
//     // Helper methods for UI logic

//     // TODO: Add more helper methods as needed
//     // - fn completed_features_count(&self, project_id: i64) -> usize
//     // - fn project_by_id(&self, id: i64) -> Option<&Project>
//     // - fn format_project_summary(&self, project: &Project) -> String
// }
