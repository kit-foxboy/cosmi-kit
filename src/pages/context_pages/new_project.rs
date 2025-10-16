use cosmic::iced::Length;
use cosmic::{cosmic_theme, widget, Element};
use crate::fl;

// use crate::application::Message;
use crate::pages::context_pages::NewProjectMessage as Message;

pub struct NewProjectPage {
    // State for the new project form
    pub title: String,
    name: String,
    description: String,
    tags: Vec<String>,          // List of added tags
    tag_input: String,           // Current tag being typed
    tag_input_id: widget::Id, // ID for maintaining focus on tag input
    existing_tags: Vec<String>,  // All tags in database for autocomplete
    autocomplete_suggestion: Option<String>, // Current autocomplete suggestion
    
    // Features support (no autocomplete)
    features: Vec<String>,       // List of added features
    feature_input: String,       // Current feature being typed
    feature_input_id: widget::Id, // ID for maintaining focus on feature input
    
    // Edit mode
    editing_project_id: Option<i64>, // If Some, we're editing an existing project
    original_tags: Vec<String>,      // Original tags when editing started (for tracking deletions)
    original_features: Vec<String>,  // Original features when editing started (for tracking deletions)
    removed_tags: Vec<String>,       // Tags that were removed during editing
    removed_features: Vec<String>,   // Features that were removed during editing
}

impl Default for NewProjectPage {
    fn default() -> Self {
        Self {
            title: fl!("new-project"),
            name: String::new(),
            description: String::new(),
            tags: Vec::new(),
            tag_input: String::new(),
            tag_input_id: widget::Id::unique(), // Create unique ID for tag input
            existing_tags: Vec::new(),
            autocomplete_suggestion: None,
            features: Vec::new(),
            feature_input: String::new(),
            feature_input_id: widget::Id::unique(), // Create unique ID for feature input
            editing_project_id: None, // Default to create mode
            original_tags: Vec::new(),
            original_features: Vec::new(),
            removed_tags: Vec::new(),
            removed_features: Vec::new(),
        }
    }
}

impl NewProjectPage {
    /// Get the tags (needed to pass to database after project creation)
    pub fn tags(&self) -> &[String] {
        &self.tags
    }
    
    /// Get the features (needed to pass to database after project creation)
    pub fn features(&self) -> &[String] {
        &self.features
    }
    
    /// Check if we're in edit mode
    pub fn is_editing(&self) -> bool {
        self.editing_project_id.is_some()
    }
    
    /// Get only the NEW tags (ones not in the original list)
    pub fn new_tags(&self) -> Vec<String> {
        self.tags.iter()
            .filter(|tag| !self.original_tags.contains(tag))
            .cloned()
            .collect()
    }
    
    /// Get only the NEW features (ones not in the original list)
    pub fn new_features(&self) -> Vec<String> {
        self.features.iter()
            .filter(|feature| !self.original_features.contains(feature))
            .cloned()
            .collect()
    }
    
    /// Enter edit mode with existing project data
    pub fn start_editing(&mut self, project_id: i64, name: String, description: Option<String>, tags: Vec<String>, features: Vec<String>) {
        self.editing_project_id = Some(project_id);
        self.title = "Edit Project".to_string();
        self.name = name;
        self.description = description.unwrap_or_default();
        
        // Store originals for tracking deletions
        self.original_tags = tags.clone();
        self.original_features = features.clone();
        
        // Set current state
        self.tags = tags;
        self.features = features;
        
        // Clear tracking lists (fresh edit session)
        self.removed_tags.clear();
        self.removed_features.clear();
        
        self.tag_input.clear();
        self.feature_input.clear();
        self.autocomplete_suggestion = None;
    }
    
    /// Reset the form to default state (called after successful creation/edit)
    pub fn reset(&mut self) {
        self.editing_project_id = None;
        self.title = fl!("new-project");
        self.name.clear();
        self.description.clear();
        self.tags.clear();
        self.tag_input.clear();
        self.autocomplete_suggestion = None;
        self.features.clear();
        self.feature_input.clear();
        
        // Clear edit tracking
        self.original_tags.clear();
        self.original_features.clear();
        self.removed_tags.clear();
        self.removed_features.clear();
        
        // Keep existing_tags for future autocomplete
    }
    
    /// Find autocomplete suggestion based on current input
    fn find_autocomplete(&self) -> Option<String> {
        if self.tag_input.is_empty() {
            return None;
        }
        
        let input_lower = self.tag_input.to_lowercase();
        
        // Find first tag that starts with the input (case-insensitive)
        // and isn't already added
        self.existing_tags
            .iter()
            .find(|tag| {
                let tag_lower = tag.to_lowercase();
                tag_lower.starts_with(&input_lower) 
                    && tag_lower != input_lower  // Don't suggest if exact match
                    && !self.tags.contains(tag)  // Don't suggest already added tags
            })
            .cloned()
    }

   pub fn view(&'_ self) -> Element<'_, Message> {
        // Build the UI for the new project form
        let cosmic_theme::Spacing { space_xxs, space_s, .. } = cosmic::theme::active().cosmic().spacing;
        
        // Create tag chips (visual representation of added tags)
        // Build chips as individual elements for wrapping
        let mut tag_elements = Vec::with_capacity(self.tags.len());
        
        for (index, tag) in self.tags.iter().enumerate() {
            tag_elements.push(
                widget::button::text(
                    format!("{} ✕", tag)  // Use × symbol instead of icon
                )
                .padding([space_xxs, space_s])
                .on_press(Message::RemoveTag(index))
                .into()
            );
        }
        
        // Use flex_row for wrapping behavior - important for cosmic's responsive screen layout
        let tag_chips = widget::flex_row(tag_elements)
            .row_spacing(space_xxs)
            .column_spacing(space_xxs)
            .padding(space_xxs);

        // Create a row for the buttons
        let button_row = widget::row::with_capacity(2)
            .push(
                widget::button::suggested(fl!("save-button"))
                    .on_press(
                        if let Some(project_id) = self.editing_project_id {
                            // Update existing project - only send NEW tags/features, not all of them
                            Message::UpdateProject(
                                project_id,
                                self.name.clone(),
                                if self.description.is_empty() {
                                    None
                                } else {
                                    Some(self.description.clone())
                                },
                                self.new_tags(),  // Only NEW tags
                                self.new_features(),  // Only NEW features
                                self.removed_tags.clone(),
                                self.removed_features.clone(),
                            )
                        } else {
                            // Create new project
                            Message::CreateProject(
                                self.name.clone(),
                                if self.description.is_empty() {
                                    None
                                } else {
                                    Some(self.description.clone())
                                },
                                self.tags.clone(),
                                self.features.clone()
                            )
                        }
                    )
            )
            .push(widget::horizontal_space())
            .push(
                widget::button::standard(fl!("cancel"))
                    .on_press(Message::Cancel)
            )
            .spacing(space_s);

        // Main form column
        widget::column::with_capacity(6)
            .push(
                widget::text_input(fl!("project-name"), &self.name)
                    .on_input(Message::UpdateName)
            )
            .push(
                widget::text_input(fl!("project-description"), &self.description)
                    .on_input(Message::UpdateDescription)
            )
            .push({
                // Tag input with autocomplete hint
                let tag_input = widget::text_input(fl!("tags-placeholder"), &self.tag_input)
                    .id(self.tag_input_id.clone())  // Set the ID to maintain focus
                    .on_input(Message::UpdateTagInput)
                    .on_submit(|_| Message::AddTag);  // Enter key adds tag (submit gets the string value)
                
                // ALWAYS render the hint row to prevent layout shifts
                // When no suggestion, show empty text to maintain consistent layout
                let hint_row = if let Some(suggestion) = &self.autocomplete_suggestion {
                    // Show clickable autocomplete suggestion
                    widget::row::with_capacity(2)
                        .push(
                            widget::text("💡 Complete as: ")
                                .size(12)
                        )
                        .push(
                            widget::button::link(suggestion.clone())
                                .on_press(Message::AcceptAutocomplete)
                                .padding([0, space_xxs])
                        )
                        .spacing(space_xxs)
                        .align_y(cosmic::iced::Alignment::Center)
                } else {
                    // Empty row with invisible text to maintain layout height
                    widget::row::with_capacity(1)
                        .push(
                            widget::text(" ")  // Single space to maintain height
                                .size(12)
                        )
                };
                
                // Create a column to show the input + hint (always present)
                widget::column::with_capacity(2)
                    .push(tag_input)
                    .push(hint_row)
                    .spacing(space_xxs)
            })
            .push(tag_chips)
            .push({
                // Features input (allows spaces, add with Enter key)
                let feature_input = widget::text_input("Add features (press enter to add)", &self.feature_input)
                    .id(self.feature_input_id.clone())
                    .on_input(Message::UpdateFeatureInput)
                    .on_submit(|_| Message::AddFeature);
                
                widget::column::with_capacity(1)
                    .push(feature_input)
                    .spacing(space_xxs)
            })
            .push({
                // Feature chips
                let mut feature_elements = Vec::with_capacity(self.features.len());
                
                for (index, feature) in self.features.iter().enumerate() {
                    feature_elements.push(
                        widget::button::text(
                            format!("{} ✕", feature)
                        )
                        .padding([space_xxs, space_s])
                        .on_press(Message::RemoveFeature(index))
                        .into()
                    );
                }
                
                widget::flex_row(feature_elements)
                    .row_spacing(space_xxs)
                    .column_spacing(space_xxs)
                    .padding(space_xxs)
            })
            .push(button_row)
            .spacing(space_s)
            .padding(space_s)
            .width(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: Message) -> cosmic::Task<cosmic::Action<Message>> {
        // Handle input messages to update local state
        match message {
            Message::UpdateName(name) => {
                self.name = name;
            }
            Message::UpdateDescription(description) => {
                self.description = description;
            }
            Message::UpdateTagInput(input) => {
                // Check if the user pressed space
                if input.ends_with(' ') {
                    // If we have an autocomplete suggestion, use that instead of the typed text
                    let tag = if let Some(suggestion) = &self.autocomplete_suggestion {
                        suggestion.clone()
                    } else {
                        // No suggestion - use what they typed
                        input.trim().to_string()
                    };
                    
                    // Only add non-empty, unique tags
                    if !tag.is_empty() && !self.tags.contains(&tag) {
                        self.tags.push(tag);
                    }
                    
                    // Clear the input field
                    self.tag_input.clear();
                    self.autocomplete_suggestion = None;
                } else {
                    // Normal character input - update and find autocomplete
                    self.tag_input = input;
                    self.autocomplete_suggestion = self.find_autocomplete();
                }
            }
            Message::AcceptAutocomplete => {
                // Accept the autocomplete suggestion (Tab key)
                if let Some(suggestion) = &self.autocomplete_suggestion {
                    self.tag_input = suggestion.clone();
                    self.autocomplete_suggestion = None;
                }
            }
            Message::AddTag => {
                // Manually add the current tag (e.g., on Enter key)
                // If we have an autocomplete suggestion, use that instead of the typed text
                let tag = if let Some(suggestion) = &self.autocomplete_suggestion {
                    suggestion.clone()
                } else {
                    self.tag_input.trim().to_string()
                };
                
                if !tag.is_empty() && !self.tags.contains(&tag) {
                    self.tags.push(tag);
                    self.tag_input.clear();
                    self.autocomplete_suggestion = None;
                }
            }
            Message::RemoveTag(index) => {
                // Remove tag at the specified index
                if index < self.tags.len() {
                    let removed_tag = self.tags.remove(index);
                    
                    // If we're editing and this tag was in the original list, track it for deletion
                    if self.is_editing() && self.original_tags.contains(&removed_tag) {
                        self.removed_tags.push(removed_tag);
                    }
                }
            }
            Message::UpdateFeatureInput(input) => {
                // Just update the input - no space detection since features can have spaces
                // Features are only added when Enter is pressed (via AddFeature message)
                self.feature_input = input;
            }
            Message::AddFeature => {
                // Manually add the current feature (e.g., on Enter key)
                let feature = self.feature_input.trim().to_string();
                
                if !feature.is_empty() && !self.features.contains(&feature) {
                    self.features.push(feature);
                    self.feature_input.clear();
                }
            }
            Message::RemoveFeature(index) => {
                // Remove feature at the specified index
                if index < self.features.len() {
                    let removed_feature = self.features.remove(index);
                    
                    // If we're editing and this feature was in the original list, track it for deletion
                    if self.is_editing() && self.original_features.contains(&removed_feature) {
                        self.removed_features.push(removed_feature);
                    }
                }
            }
            Message::LoadExistingTags(tags) => {
                // Load existing tag names for autocomplete
                self.existing_tags = tags.iter().map(|t| t.name.clone()).collect();
            }
            Message::Cancel => {
                self.name.clear();
                self.description.clear();
                self.tags.clear();
                self.tag_input.clear();
                self.features.clear();
                self.feature_input.clear();
            }
            // All other messages are handled by the parent page/app
            _ => {}
        }
        cosmic::Task::none()
    }
}