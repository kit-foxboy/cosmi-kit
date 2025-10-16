// SPDX-License-Identifier: MPL-2.0

//! Project Manager Page
//!
//! Track development projects, tasks, and progress with a simple kanban-style interface

// use slotmap::SlotMap;
use cosmic::iced::Length;
use cosmic::theme;
use cosmic::widget::{self, icon, Toast, Toasts};
use cosmic::{Element, Task};

use crate::database::{Feature, Project, ProjectJoin, Tag};
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
    /// Create the header bar with add button
    fn build_header(&self) -> widget::HeaderBar<'_, Message> {
        widget::header_bar()
            .title("Project Manager")
            .end(
                widget::button::icon(icon::from_name("list-add-symbolic"))
                    .on_press(Message::ToggleCreateProject)
                    .tooltip("Add New Project")
                    .class(theme::Button::Suggested)
            )
    }

    /// Build loading state
    fn build_loading_state(&self) -> Element<'_, Message> {
        widget::container(
            widget::text::body("Loading projects...")
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }

    /// Build empty state message
    fn build_empty_state(&self) -> Element<'_, Message> {
        let theme = theme::active();
        let spacing = theme.cosmic().spacing;

        widget::container(
            widget::column::with_capacity(2)
                .push(
                    widget::text::title3("No projects yet")
                )
                .push(
                    widget::text::body("Click the + button to create your first project!")
                )
                .spacing(spacing.space_s)
                .align_x(cosmic::iced::Alignment::Center)
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }

    /// Build a colored tag chip with icon
    fn build_tag_chip<'a>(&self, tag: &'a crate::database::Tag) -> Element<'a, Message> {
        let theme = theme::active();
        let spacing = theme.cosmic().spacing;

        // Parse the tag color, default to theme text color if not available
        let tag_color = tag.color.as_ref()
            .and_then(|hex| Self::parse_hex_color(hex).ok())
            .unwrap_or_else(|| theme.cosmic().on_bg_color().into());

        // Create tag chip with icon and colored text
        // Wrap in a container to apply text color styling
        // Icon size will match the text size automatically
        widget::container(
            widget::row::with_capacity(2)
                .push(
                    widget::icon::from_name("tag-symbolic")
                )
                .push(
                    widget::text::body(&tag.name)
                )
                .spacing(spacing.space_xxxs)
                .align_y(cosmic::iced::Alignment::Center)
        )
        .style(move |_theme: &cosmic::Theme| {
            widget::container::Style {
                text_color: Some(tag_color),
                icon_color: Some(tag_color),
                ..Default::default()
            }
        })
        .into()
    }

    /// Build a single project card
    fn build_project_card<'a>(&self, project: &'a Project, tags: &'a [Tag], features: &'a [Feature]) -> Element<'a, Message> {
        let theme = theme::active();
        let spacing = theme.cosmic().spacing;

        // Create tag chips
        let tag_elements: Vec<Element<'_, Message>> = tags.iter()
            .map(|tag| self.build_tag_chip(tag))
            .collect();

        let tags_row = widget::flex_row(tag_elements)
            .row_spacing(spacing.space_xs)
            .column_spacing(spacing.space_xs);

        // Build action buttons (edit and delete) - using flex_row so they wrap on small screens
        let action_buttons = widget::flex_row(vec![
            widget::button::icon(icon::from_name("document-edit-symbolic"))
                .on_press(Message::EditProject(project.id))
                .tooltip("Edit Project")
                .class(theme::Button::Standard)
                .into(),
            widget::button::icon(icon::from_name("user-trash-symbolic"))
                .on_press(Message::DeleteProject(project.id))
                .tooltip("Delete Project")
                .class(theme::Button::Destructive)
                .into(),
        ])
        .row_spacing(spacing.space_xs)
        .column_spacing(spacing.space_xs);

        // Build title row with project name and action buttons
        let title_row = widget::row::with_capacity(2)
            .push(
                widget::text::title4(&project.name)
            )
            .push(widget::horizontal_space())
            .push(action_buttons)
            .align_y(cosmic::iced::Alignment::Center)
            .width(Length::Fill);

        // Build the card content
        let mut card_content = widget::column::with_capacity(4)
            .push(title_row)
            .spacing(spacing.space_s);

        // Add description if present
        if let Some(desc) = &project.description {
            card_content = card_content.push(widget::text::body(desc));
        }

        // Add tags if present
        if !tags.is_empty() {
            card_content = card_content.push(tags_row);
        }

        // Add features as checkboxes if present
        if !features.is_empty() {
            let mut features_column = widget::column::with_capacity(features.len())
                .spacing(spacing.space_xxs);
            
            for feature in features {
                let checkbox = widget::checkbox(&feature.description, feature.completed)
                    .on_toggle(move |_| Message::ToggleFeatureCompleted(feature.id));
                
                features_column = features_column.push(checkbox);
            }
            
            card_content = card_content.push(features_column);
        }

        widget::container(card_content)
            .padding(spacing.space_m)
            .class(theme::Container::Card)
            .width(Length::Fill)
            .into()
    }

    /// Build the scrollable project list
    fn build_project_list(&self) -> Element<'_, Message> {
        let theme = theme::active();
        let spacing = theme.cosmic().spacing;

        let mut projects_column = widget::column::with_capacity(self.projects.len())
            .spacing(spacing.space_m)
            .padding([spacing.space_m, spacing.space_m]);

        // ProjectJoin is a Vec<HashMap>, flatten and sort by project ID
        let mut sorted_projects: Vec<_> = self.projects.iter()
            .flat_map(|project_map| project_map.iter())
            .collect();
        sorted_projects.sort_by(|a, b| b.0.cmp(a.0));

        // Build cards for each project
        for (_project_id, (project, tags, features)) in sorted_projects {
            let card = self.build_project_card(project, tags, features);
            projects_column = projects_column.push(card);
        }

        // Make scrollable
        widget::container(
            widget::scrollable(projects_column)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    /// Create the view for this page
    pub fn view(&'_ self) -> Element<'_, Message> {
        let header = self.build_header();

        // Choose which content to display
        let projects_content = if self.is_loading {
            self.build_loading_state()
        } else if self.projects.is_empty() {
            self.build_empty_state()
        } else {
            self.build_project_list()
        };

        let content = widget::column::with_capacity(2)
            .push(header)
            .push(projects_content)
            .width(Length::Fill)
            .height(Length::Fill);

        // Wrap with toaster to show toast notifications
        widget::toaster(&self.toasts, content).into()
    }

    /// Parse hex color string (#RRGGBB) to iced Color
    fn parse_hex_color(hex: &str) -> Result<cosmic::iced::Color, ()> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(());
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ())?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ())?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ())?;
        
        Ok(cosmic::iced::Color::from_rgb8(r, g, b))
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
            Message::EditProject(_id) => {
                // AppModel handles loading project data and opening the edit form
            }
            Message::ProjectLoadedForEdit(_result) => {
                // AppModel handles populating the form and opening context drawer
            }
            Message::ToggleFeatureCompleted(_feature_id) => {
                // AppModel handles the async toggle
            }
            Message::FeatureToggled(result) => {
                match result.as_ref() {
                    Ok(_completed) => {
                        // Feature toggled successfully - reload to show updated state
                        return Task::done(cosmic::Action::App(Message::LoadData));
                    }
                    Err(e) => {
                        let _ = self.toasts.push(
                            Toast::new(format!("Failed to toggle feature: {}", e))
                        );
                    }
                }
            }
            Message::DeleteProject(_id) => {
                self.is_loading = true;
                // AppModel handles the actual async deletion
            }
            Message::ProjectDeleted(result) => {
                self.is_loading = false;
                match result.as_ref() {
                    Ok(_) => {
                        // Show success toast
                        // App.rs will handle reloading the project list
                        let _ = self.toasts.push(
                            Toast::new(crate::fl!("project-deleted"))
                        );
                    }
                    Err(e) => {
                        // Show error toast
                        let _ = self.toasts.push(
                            Toast::new(format!("{}: {}", crate::fl!("error"), e))
                        );
                    }
                }
            }
            Message::ToggleCreateProject => {
                // Handled by app.rs to toggle the context drawer
            }
            Message::CloseToast(id) => {
                // Close the toast with the given ID
                self.toasts.remove(id);
            }
            Message::TagCreated(result) => {
                // Tags have been created and linked to project
                // The app.rs handler already triggers a data reload
                match result.as_ref() {
                    Ok(tags) => {
                        // Optional: Show toast for tags created
                        eprintln!("Tags created: {}", tags.len());
                    }
                    Err(e) => {
                        // Show error toast
                        let _ = self.toasts.push(
                            Toast::new(format!("{}: {}", crate::fl!("error"), e))
                        );
                    }
                }
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
