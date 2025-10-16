// SPDX-License-Identifier: MPL-2.0

//! OC (Original Character) Generator Page
//!
//! A fun tool for creating random character concepts for furries, gamers, and creative folks!

use crate::application::app_data::SavedOC;
use crate::fl;
use cosmic::cosmic_config::{Config, ConfigGet, ConfigSet, CosmicConfigEntry};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::Length;
use cosmic::widget::{self, column, icon, row, text};
use cosmic::{cosmic_theme, prelude::*, theme, Action, Element, Task};

use crate::config::{SavedCharactersConfig, CONFIG_KEY};
use crate::pages::OCPageMessage as Message;

// impl Default for Message {
//     fn default() -> Self {
//         Self::LoadData
//     }
// }

/// State for the OC Generator page
pub struct OcGeneratorPage {
    // Here is where state lives
    pub oc_text: Option<String>,
    pub saved_characters: Vec<SavedOC>,
    pub is_loaded: bool,
}

impl Default for OcGeneratorPage {
    fn default() -> Self {
        Self {
            oc_text: None,
            saved_characters: vec![],
            is_loaded: false,
        }
    }
}

impl OcGeneratorPage {

    /// Create the view for this page
    pub fn view(&'_ self) -> Element<'_, Message> {
        // Build the UI here!
        // Documentation is sparse so this is very fly by the seat of my lack of pants
        // Pattern I've decided to follow to reduce monolithic chains:
        // 1. Get spacing from theme
        // 2. Create individual UI sections (header, form, buttons)
        // 3. Combine sections into single element with padding
        // 4. Wrap in container with padding
        // 5. Convert to Element with .into()
        // 6. Use helper methods for logical separation or repeated elements
        let cosmic_theme::Spacing {
            space_xxs, space_s, space_m, ..
        } = theme::active().cosmic().spacing;

        let save_button = if self.oc_text.is_some() { 
            widget::button::standard(fl!("save-button"))
            .on_press(Message::SaveButtonClicked)
        } else { 
             widget::button::standard(fl!("save-button"))
        };
        let save_button = save_button
            .class(theme::Button::Suggested);
        

        let button_row = row()
            .spacing(space_m)
            .align_y(Vertical::Center)
            .push(widget::horizontal_space()) // Centers the buttons
            .push(
                widget::button::standard(fl!("generate-button"))
                    .on_press(Message::GenerateButtonClicked)
            )
            .push(save_button)
            .push(widget::horizontal_space()); // Centers the buttons

        let content_section = column::column()
            .spacing(space_m)
            .push(
                text::title4(self.oc_text.clone().unwrap_or_default())
                    .apply(widget::container)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
            )
            .push(button_row)
            .apply(widget::container)
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        // Build widget from sections
        column::column()
            .spacing(space_s) // Use consistent small spacing
            .push(self.view_header(space_xxs.into()))
            .push(content_section)
            .push(self.favorite_section())
            .padding(space_xxs)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// Handle messages for this page
    pub fn update(&mut self, message: Message) -> Task<Action<Message>> {
        match message {
            Message::LoadData => {
                if !self.is_loaded {
                    self.load_characters().unwrap_or_else(|e| {
                        eprintln!("Unexpected error loading characters: {:?}", e);
                    });
                }
            }
            Message::GenerateButtonClicked => self.oc_text = Some(self.generate()),
            Message::SaveButtonClicked => {
                if let Some(oc) = &self.oc_text {
                    let new_saved = SavedOC::new(oc.clone());
                    self.saved_characters.push(new_saved);
                    if let Err(e) = self.save_characters() {
                        eprintln!("Error saving characters: {:?}", e);
                    } else {
                        //Todo: show message to user
                        println!("Saved characters successfully!");
                    }
                }
            }
            Message::DeleteCharacter(index) => {
                if let Err(e) = self.delete_character_at_index(index) {
                    eprintln!("Error saving characters: {:?}", e);
                    let _ = self.load_characters();
                } else {
                    // Todo: show message to user
                    print!("Character removed");
                }
            }
        }

        cosmic::Task::none()
    }

    fn view_header(&self, space_xxs: f32) -> Element<'_, Message> {
        widget::header_bar()
            .title(fl!("oc-generator"))
            .apply(widget::container)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .padding([space_xxs, 0.0, 0.0, 0.0])
            .into()
    }

    fn favorite_section(&self) -> Element<'_, Message> {
        column::column()
            .push(
                widget::text::title2(fl!("favorites"))
                    .apply(widget::container)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
            )
            .push(
                self.character_card_list()
                    .apply(widget::container)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Center),
            )
            .apply(widget::container)
            .width(Length::Fill)
            .height(Length::FillPortion(3))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Top)
            // .padding([space_xxs, 0, 0, 0])
            .into()
    }

    fn character_card_list(&self) -> Element<'_, Message> {
        // create a scrollable list of character cards
        let cosmic_theme::Spacing {
            space_xxs,
            space_xs,
            ..
        } = theme::active().cosmic().spacing;
        let mut list = widget::list::list_column();
        for (index, _character) in self.saved_characters.iter().enumerate() {
            list = list.add(self.character_card(index)).spacing(space_xs);
        }

        widget::scrollable(
            list.apply(widget::container)
                .padding([space_xxs, 0, space_xxs, 0])
                .width(Length::Fill),
        )
        .into()
    }
    fn character_card(&self, index: usize) -> Element<'_, Message> {
        let cosmic_theme::Spacing {
            space_xs, ..
        } = theme::active().cosmic().spacing;

        row::row()
            .spacing(space_xs)
            .align_y(Vertical::Center)
            .padding(space_xs)
            .push(
                // Character text - consistent styling and proper fill
                widget::text::body(&self.saved_characters[index].text)
                    .width(Length::Fill)
                    .apply(widget::container)
                    .align_y(Vertical::Center),
            )
            .push(
                // Delete button - consistent positioning
                widget::button::icon(icon::from_name("user-trash-symbolic"))
                    .on_press(Message::DeleteCharacter(index))
                    .class(theme::Button::Destructive)
                    .apply(widget::container)
                    .align_y(Vertical::Center),
            )
            .apply(widget::container)
            .width(Length::Fill)
            // .padding(space_xs)
            .into()
    }

    /// Save the currently generated OC to favorites
    fn save_characters(&self) -> Result<(), cosmic::cosmic_config::Error> {
        // Note, the ? operator will return early if there's an error, it's a nice shorthand
        let config = Config::new(CONFIG_KEY, SavedCharactersConfig::VERSION)?;

        let characters_config = SavedCharactersConfig {
            characters: self.saved_characters.clone(),
        };

        // Store the entire list under one key
        config.set("characters", &characters_config)?;
        Ok(())
    }

    /// Load saved characters from config
    fn load_characters(&mut self) -> Result<(), cosmic::cosmic_config::Error> {
        //Note, the ? operator will return early if there's an error, it's a nice shorthand
        let config = Config::new(CONFIG_KEY, SavedCharactersConfig::VERSION)?;

        match config.get::<SavedCharactersConfig>("characters") {
            Ok(characters_config) => {
                self.saved_characters = characters_config.characters;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Delete a character at a given index
    fn delete_character_at_index(
        &mut self,
        index: usize,
    ) -> Result<(), cosmic::cosmic_config::Error> {
        self.saved_characters.remove(index);
        self.save_characters()
    }
    /// Generate a string from random string elements
    fn generate(&self) -> String {
        "A ".to_string()
            + self.generate_oc_attribute().as_str()
            + " "
            + self.generate_oc_species().as_str()
            + " "
            + self.generate_characteristic().as_str()
    }

    fn generate_oc_attribute(&self) -> String {
        let attributes = [
            fl!("attribute-short"),
            fl!("attribute-tall"),
            fl!("attribute-fat"),
            fl!("attribute-nervous"),
            fl!("attribute-brave"),
            fl!("attribute-shy"),
            fl!("attribute-curious"),
            fl!("attribute-friendly"),
            fl!("attribute-aloof"),
            fl!("attribute-clever"),
            fl!("attribute-clumsy"),
            fl!("attribute-energetic"),
            fl!("attribute-sleepy"),
            fl!("attribute-grumpy"),
            fl!("attribute-optimistic"),
            fl!("attribute-pessimistic"),
            fl!("attribute-cunning"),
            fl!("attribute-kind"),
            fl!("attribute-sarcastic"),
            fl!("attribute-micro"),
            fl!("attribute-macro"),
        ];

        attributes[fastrand::usize(0..attributes.len())].to_string()
    }

    fn generate_oc_species(&self) -> String {
        // unfortuantely must pass fl! a string literal so can't just call it on the computed value
        let species = [
            fl!("species-cat"),
            fl!("species-dog"),
            fl!("species-fox"),
            fl!("species-wolf"),
            fl!("species-rabbit"),
            fl!("species-horse"),
            fl!("species-dragon"),
            fl!("species-lion"),
            fl!("species-tiger"),
            fl!("species-deer"),
            fl!("species-bat"),
            fl!("species-snake"),
        ];

        species[fastrand::usize(0..species.len())].to_string()
    }

    fn generate_characteristic(&self) -> String {
        let characteristics = [
            fl!("characteristic-mokawk"),
            fl!("characteristic-no-pants"),
            fl!("characteristic-constant-waffles"),
            fl!("characteristic-earrings"),
            fl!("characteristic-always-cape"),
            fl!("characteristic-tiny-squeak"),
            fl!("characteristic-overdramatic"),
            fl!("characteristic-secret-nerd"),
            fl!("characteristic-philosopher"),
            fl!("characteristic-sings-everything"),
            fl!("characteristic-hat-collection"),
            fl!("characteristic-uses-emoji"),
            fl!("characteristic-collects-bad-jokes"),
            fl!("characteristic-vore"),
            fl!("characteristic-ponytail"),
            fl!("characteristic-sparkle"),
        ];

        characteristics[fastrand::usize(0..characteristics.len())].to_string()
    }
}