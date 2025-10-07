// SPDX-License-Identifier: MPL-2.0

use crate::config::Config;
use crate::database::SqliteDatabase;
use crate::fl;
use cosmic::app::context_drawer;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::prelude::*;
use cosmic::widget::{self, icon, menu, nav_bar};
use cosmic::{cosmic_theme, theme};
use futures_util::SinkExt;
use std::collections::HashMap;
use anyhow::Result;

use crate::pages::{oc_generator, project_manager, ProjectManagerPage, OcGeneratorPage};

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/icon.svg");

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
    app_data: crate::app_data::AppData,
    // pages (just UI and state management)
    oc_generator_page: oc_generator::OcGeneratorPage,
    project_manager_page: ProjectManagerPage,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    OcGeneratorPage(oc_generator::Message),
    ProjectManagerPage(crate::pages::project_manager::Message),
    DatabaseInitialized(Result<SqliteDatabase, String>),
    OpenRepositoryUrl,
    SubscriptionChannel,
    ToggleContextPage(ContextPage),
    UpdateConfig(Config),
    LaunchUrl(String),
}

/// Hook the page messages into the app's
impl From<oc_generator::Message> for Message {
    fn from(message: oc_generator::Message) -> Self {
        Self::OcGeneratorPage(message)
    }
}

impl From<crate::pages::project_manager::Message> for Message {
    fn from(message: crate::pages::project_manager::Message) -> Self {
        Self::ProjectManagerPage(message)
    }
}

/// Create a COSMIC application from the app model
impl cosmic::Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.github.kitfoxboy.cosmi-kit";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // Create a nav bar with three page items.
        let mut nav = nav_bar::Model::default();

        nav.insert()
            .text(fl!("oc-generator"))
            .data::<Page>(Page::OCGenerator)
            .icon(icon::from_name("applications-science-symbolic"))
            .activate();

        nav.insert()
            .text(fl!("project-manager"))
            .data::<Page>(Page::ProjectManager)
            .icon(icon::from_name("applications-system-symbolic"));

        nav.insert()
            .text(fl!("dice-roller"))
            .data::<Page>(Page::DiceRoller)
            .icon(icon::from_name("applications-games-symbolic"));

        // Construct the app model with the runtime's core.
        let mut app = AppModel {
            core,
            context_page: ContextPage::default(),
            oc_generator_page: OcGeneratorPage::default(),
            project_manager_page: ProjectManagerPage::default(),
            app_data: crate::app_data::AppData::new(),  // Data layer initialized (no database yet)
            nav,
            key_binds: HashMap::new(),
            // Optional configuration file for an application.
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => {
                        // for why in errors {
                        //     tracing::error!(%why, "error loading app config");
                        // }

                        config
                    }
                })
                .unwrap_or_default(),
        };

        // Create a startup command that sets the window title.
        let command = app.update_title();

        // Create a batch of tasks to run at startup.
        let mut tasks = vec![command];

        // Initialize database asynchronously - this runs ONCE at startup
        // The connection pool will be stored in AppModel and reused!
        tasks.push(Task::perform(
            async move {
                SqliteDatabase::new().await
                    .map_err(|e| e.to_string())
            },
            |result| cosmic::Action::App(Message::DatabaseInitialized(result))
        ));

        // Note: We DON'T load page data here because database isn't ready yet!
        // Page data loading happens AFTER DatabaseInitialized message.

        (app, Task::batch(tasks))
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&'_ self) -> Vec<Element<'_, Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")).apply(Element::from),
            menu::items(
                &self.key_binds,
                vec![menu::Item::Button(fl!("about"), None, MenuAction::About)],
            ),
        )]);

        vec![menu_bar.into()]
    }

    /// Enables the COSMIC application to create a nav bar with this model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    /// Nav item selected handler, called when a nav item is selected as a lifecycle event.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<cosmic::Action<Self::Message>> {
        // Activate the page in the model.
        self.nav.activate(id);

        // Create a command to update the window title,
        // we'll also load data for the page if needed as a batch of tasks
        let mut tasks = vec![self.update_title()];

        // load data for the page if needed
        tasks.push(self.load_page_data());

        Task::batch(tasks)
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&'_ self) -> Option<context_drawer::ContextDrawer<'_, Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => context_drawer::context_drawer(
                self.about(),
                Message::ToggleContextPage(ContextPage::About),
            )
            .title(fl!("about")),
        })
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// Application events will be processed through the view. Any messages emitted by
    /// events received by widgets will be passed to the update method.
    fn view(&'_ self) -> Element<'_, Self::Message> {
        
        match self.active_page() {
            Some(Page::OCGenerator) => self.oc_generator_page.view().map(Message::OcGeneratorPage),
            Some(Page::ProjectManager) => self.project_manager_page.view().map(Message::ProjectManagerPage),
            Some(Page::DiceRoller) => {
                 widget::text::title1(fl!("dice-roller"))
                .apply(widget::container)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into()
            },
            None => panic!("Invalid Page, if this happens you borked it real bad")
        }
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    fn subscription(&self) -> Subscription<Self::Message> {
        struct MySubscription;

        Subscription::batch(vec![
            // Create a subscription which emits updates through a channel.
            Subscription::run_with_id(
                std::any::TypeId::of::<MySubscription>(),
                cosmic::iced::stream::channel(4, move |mut channel| async move {
                    _ = channel.send(Message::SubscriptionChannel).await;

                    futures_util::future::pending().await
                }),
            ),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| {
                    // for why in update.errors {
                    //     tracing::error!(?why, "app config error");
                    // }

                    Message::UpdateConfig(update.config)
                }),
        ])
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::DatabaseInitialized(result) => {
                match result {
                    Ok(db) => {
                        eprintln!("Database initialized successfully!");
                        self.app_data.set_database(db);
                        // Now that database is ready, load the active page's data
                        return self.load_page_data();
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize database: {}", e);
                        // Continue without database - pages will handle the None case
                    }
                }
            }

            Message::OcGeneratorPage(page_message) => {
                let _ = self.oc_generator_page.update(page_message);
            }

            Message::ProjectManagerPage(page_message) => {
                // Handle data operations here in app.rs, then update page state
                // This keeps the page decoupled from data layer
                return self.handle_project_manager_message(page_message);
            }

            Message::OpenRepositoryUrl => {
                _ = open::that_detached(REPOSITORY);
            }

            Message::SubscriptionChannel => {
                // For example purposes only.
            }

            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    // Close the context drawer if the toggled context page is the same.
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    // Open the context drawer to display the requested context page.
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }
            }

            Message::UpdateConfig(config) => {
                self.config = config;
            }

            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("failed to open {url:?}: {err}");
                }
            },
        }
        Task::none()
    }
}

impl AppModel {
    /// The about page for this app.
    pub fn about(&'_ self) -> Element<'_, Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(APP_ICON));

        let title = widget::text::title3(fl!("app-title"));

        let hash = env!("VERGEN_GIT_SHA");
        let short_hash: String = hash.chars().take(7).collect();
        let date = env!("VERGEN_GIT_COMMIT_DATE");

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::OpenRepositoryUrl)
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .push(
                widget::button::link(fl!(
                    "git-description",
                    hash = short_hash.as_str(),
                    date = date
                ))
                .on_press(Message::LaunchUrl(format!("{REPOSITORY}/commits/{hash}")))
                .padding(0),
            )
            .align_x(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    /// Updates the header and window titles.
    pub fn update_title(&mut self) -> Task<cosmic::Action<Message>> {
        let mut window_title = fl!("app-title");

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
        }

        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }

    pub fn active_page(&self) -> Option<Page> {
        self.nav.data::<Page>(self.nav.active()).cloned()
    }

    // Helper method for loading page data if needed
    fn load_page_data(&self) -> Task<cosmic::Action<Message>> {
        // Note: Match statements are more verbose but easier to extend later than chain if lets
        match self.active_page() {
            Some(Page::OCGenerator) => {
                // Convert the page message to app message and trigger loading
                Task::done(cosmic::Action::App(Message::OcGeneratorPage(oc_generator::Message::LoadData)))
            }
            Some(Page::ProjectManager) => {
                // Project manager needs to load projects from database
                Task::done(cosmic::Action::App(Message::ProjectManagerPage(project_manager::Message::LoadData)))
            }
            _ => Task::none()
        }
    }

    // Handle Project Manager messages - this is where data operations happen!
    fn handle_project_manager_message(
        &mut self,
        message: project_manager::Message,
    ) -> Task<cosmic::Action<Message>> {
        use project_manager::Message as PM; //Note: Useful to avoid naming conflicts. I dislike managing crates thus far tbh

        match message {
            PM::LoadData => {
                // Trigger async data load from AppData
                let app_data = self.app_data.clone();  // TODO: We need to make AppData cloneable
                Task::perform(
                    async move {
                        app_data.load_projects().await
                            .map_err(|e| e.to_string())
                    },
                    |result| cosmic::Action::App(Message::ProjectManagerPage(PM::ProjectsLoaded(result)))
                )
            }

            // PM::CreateProject(name, description) => {
            //     // Trigger async project creation
            //     let app_data = self.app_data.clone();
            //     Task::perform(
            //         async move {
            //             app_data.create_project(name, description).await
            //                 .map_err(|e| e.to_string())
            //         },
            //         |result| cosmic::Action::App(Message::ProjectManagerPage(PM::ProjectCreated(result)))
            //     )
            // }

            // PM::DeleteProject(id) => {
            //     // Trigger async project deletion
            //     let app_data = self.app_data.clone();
            //     Task::perform(
            //         async move {
            //             app_data.delete_project(id).await
            //                 .map_err(|e| e.to_string())
            //         },
            //         |result| cosmic::Action::App(Message::ProjectManagerPage(PM::ProjectDeleted(result)))
            //     )
            // }

            // All other messages just update UI state - delegate to page
            _ => {
                self.project_manager_page.update(message);
                Task::none()
            }
        }
    }
}

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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
        }
    }
}
