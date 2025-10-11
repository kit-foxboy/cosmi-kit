use cosmic::iced::{Length};
use cosmic::{cosmic_theme, widget, Element};
use crate::fl;

// use crate::application::Message;
use crate::pages::context_pages::NewProjectMessage as Message;

pub struct NewProjectPage {
    // State for the new project form
    pub title: String,
    name: String,
    description: String,
}

impl Default for NewProjectPage {
    fn default() -> Self {
        Self {
            title: fl!("new-project"),
            name: String::new(),
            description: String::new(),
        }
    }
}

impl NewProjectPage { 

   pub fn view(&'_ self) -> Element<'_, Message> {
        // Build the UI for the new project form
        let cosmic_theme::Spacing { space_xxs, space_s, .. } = cosmic::theme::active().cosmic().spacing;
        
        // Create a row for the buttons
        let button_row = widget::row::with_capacity(2)
            .push(
                widget::button::suggested(fl!("save-button"))
                    .on_press(Message::CreateProject(
                        self.name.clone(),
                        if self.description.is_empty() {
                            None
                        } else {
                            Some(self.description.clone())
                        }
                    ))
            )
            .push(widget::horizontal_space())
            .push(
                widget::button::standard(fl!("cancel"))
                    .on_press(Message::Cancel)
            )
            .spacing(space_s);

        // Main form column
        widget::column::with_capacity(4)
            .push(
                widget::text_input(fl!("project-name"), &self.name)
                    .on_input(Message::UpdateName)
                    .padding(space_xxs)
            )
            .push(
                widget::text_input(fl!("project-description"), &self.description)
                    .on_input(Message::UpdateDescription)
                    .padding(space_xxs)
            )
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
            Message::Cancel => {
                self.name.clear();
                self.description.clear();
            }
            // All other messages are handled by the parent page/app
            _ => {}
        }
        cosmic::Task::none()
    }
}