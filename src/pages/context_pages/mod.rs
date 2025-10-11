pub mod new_project;

#[derive(Debug, Clone)]
pub enum NewProjectMessage {
    UpdateName(String),
    UpdateDescription(String),
    CreateProject(String, Option<String>),
    Cancel,
}