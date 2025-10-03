// SPDX-License-Identifier: MPL-2.0

//! OC (Original Character) Generator Page
//!
//! A fun tool for creating random character concepts for furries, gamers, and creative folks!

use crate::fl;
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::Length;
use cosmic::widget::{self, column, text};
use cosmic::{cosmic_theme, prelude::*, theme};

/// Messages that the OC Generator page can emit
#[derive(Debug, Clone)]
pub enum Message {
    // Think in terms of current tense actions
    GenerateButtonClicked,
}

impl Default for Message {
    fn default() -> Self {
        Self::GenerateButtonClicked
    }
}

/// State for the OC Generator page
pub struct OcGeneratorPage {
    // Here is where state lives
    pub oc_text: Option<String>,
}

impl Default for OcGeneratorPage {
    fn default() -> Self {
        Self { oc_text: None }
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

        // Set up spacing and ui sections elements
        let cosmic_theme::Spacing {
            space_xxs, space_m, ..
        } = theme::active().cosmic().spacing;
        let title_section = widget::text::title1(fl!("oc-generator"))
            .apply(widget::container)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .padding([space_xxs, 0, 0, 0]);

        let content_section = column::column()
            .spacing(space_m)
            .push(
                text(self.oc_text.clone().unwrap_or_default())
                    .apply(widget::container)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
            )
            .push(
                widget::button::standard(fl!("generate-button"))
                    .on_press(Message::GenerateButtonClicked)
                    .apply(widget::container)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
            )
            .apply(widget::container)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        // Build widget
        column::column()
            .push(title_section)
            .push(content_section)
            .padding(space_xxs)
            .width(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    /// Handle messages for this page
    pub fn update(&mut self, message: Message) -> cosmic::Task<cosmic::Action<Message>> {
        match message {
            Message::GenerateButtonClicked => {
                self.oc_text = Some(self.generate())
            }
        }

        cosmic::Task::none()
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
