use crate::{application::ContextPage, config::Config, database::SqliteDatabase, pages::*, pages::context_pages::NewProjectMessage};

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    OcGeneratorPage(OCPageMessage),
    ProjectManagerPage(ProjectManagerPageMessage),
    NewProjectPage(NewProjectMessage),
    DatabaseInitialized(Result<SqliteDatabase, String>),
    OpenRepositoryUrl,
    SubscriptionChannel,
    ToggleContextPage(ContextPage),
    UpdateConfig(Config),
    LaunchUrl(String),
}