pub mod new_project;

use crate::database::Tag;

#[derive(Debug, Clone)]
pub enum NewProjectMessage {
    UpdateName(String),
    UpdateDescription(String),
    UpdateTagInput(String),
    AcceptAutocomplete,      // Accept the autocomplete suggestion (button click)
    AddTag,                  // Add current tag (Enter key)
    RemoveTag(usize),
    UpdateFeatureInput(String), // Update feature input field
    AddFeature,              // Add current feature (Enter key or Space)
    RemoveFeature(usize),    // Remove feature at index
    LoadExistingTags(Vec<Tag>), // Load all tags from database for autocomplete
    CreateProject(String, Option<String>, Vec<String>, Vec<String>), // Name, Description, Tags, Features
    UpdateProject(i64, String, Option<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>), // ID, Name, Desc, Tags, Features, RemovedTags, RemovedFeatures
    Cancel,
}