mod modules {
    pub mod crypto;
}

use crate::modules::crypto;
use iced::alignment::Vertical;
use iced::executor;
use iced::widget::{
    button, column, container, pick_list, row, text, text_editor, text_input, toggler,
};
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};
use std::fs::File;
use std::io::Write;


struct Cipherator {
    passwords_revealed: bool,
    cryptographic_times_options: Vec<i32>,
    cryptographic_times_selection: i32,
    cipher_input_data: text_editor::Content,
    cipher_output_data: text_editor::Content,
    password_inputs: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    CipherDataChanged(text_editor::Action),
    CryptoNumberChanged(i32),
    PasswordInputChange(i32, String),
    CopyEncryptedOutput,
    ToggleRevealPasswords(bool),
    EncryptData,
    DecryptData,
    ExportAsFile,
}

impl Application for Cipherator {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn new(_flags: ()) -> (Cipherator, Command<Self::Message>) {
        let cryptographic_times_options = vec![1, 2, 3];
        let cryptographic_times_selection = 1;
        let password_inputs = vec!["".to_string(); cryptographic_times_selection as usize];
        (
            Self {
                passwords_revealed: false,
                cryptographic_times_options,
                cipher_input_data: text_editor::Content::with_text("Seed phrase"),
                cipher_output_data: text_editor::Content::with_text("Encrypted seed phrase"),
                cryptographic_times_selection,
                password_inputs,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cipherator")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::CryptoNumberChanged(number) => {
                self.cryptographic_times_selection = number;
                self.password_inputs = vec!["".to_string(); number as usize];
            }
            Message::PasswordInputChange(index, value) => {
                if let Some(input) = self.password_inputs.get_mut(index as usize) {
                    *input = value;
                }
            }
            Message::CipherDataChanged(action) => {
                self.cipher_input_data.perform(action);

                return Command::none();
            }
            Message::CopyEncryptedOutput => {
                return iced::clipboard::write(self.cipher_output_data.text().clone());
            }
            Message::ToggleRevealPasswords(value) => {
                self.passwords_revealed = value;
                return Command::none();
            }
            Message::EncryptData => {
                self.cipher_output_data = text_editor::Content::with_text(
                    &self
                        .password_inputs
                        .iter()
                        .fold(self.cipher_input_data.text().clone(), |acc, password| {
                            match crypto::encrypt_data(&acc, &password) {
                                Ok(data) => data,
                                Err(_) => "Encryption failed".to_string(),
                            }
                        }),
                );
                return Command::none();
            }
            Message::DecryptData => {
                self.cipher_output_data = text_editor::Content::with_text(
                    &self
                        .password_inputs
                        .iter()
                        .rev()
                        .fold(self.cipher_input_data.text().clone(), |acc, password| {
                            if acc == "Decryption failed" {
                                return acc;
                            }

                            match crypto::decrypt_data(&acc, &password) {
                                Ok(data) => data,
                                Err(_) => "Decryption failed".to_string()
                            }
                        }),
                );
            }
            Message::ExportAsFile => {
                let maybe_path = rfd::FileDialog::new().add_filter("aes", &["aes"]).set_file_name(".aes").save_file();
                if let Some(path) = maybe_path {
                    let content = self.cipher_output_data.text().clone();
                    let mut file = File::create(path).expect("Unable to create file");
                    file.write_all(content.as_bytes())
                        .expect("Unable to write data");
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let title = text("Cipherator").size(70);

        let crypto_times_pick = pick_list(
            self.cryptographic_times_options.clone(),
            Some(&self.cryptographic_times_selection),
            Message::CryptoNumberChanged,
        )
        .placeholder("Number of cryptographic layered passwords to encrypt your seed phrase with")
        .padding(15);

        let first_row = row![
            text("Number of passwords: ")
                .width(200)
                .vertical_alignment(Vertical::Center)
                .size(20),
            crypto_times_pick
        ];

        let cipher_input = text_editor(&self.cipher_input_data)
            .on_action(Message::CipherDataChanged)
            .height(200)
            .padding(15);

        let mut content = column![title, first_row, cipher_input]
            .spacing(20)
            .align_items(Alignment::Center);

        for (index, value) in self.password_inputs.iter().enumerate() {
            let password_input = text_input(
                &format!(
                    "Password number {}",
                    self.cryptographic_times_options[index].to_string()
                ) as &str,
                value,
            )
            .on_input(move |value| Message::PasswordInputChange(index as i32, value))
            .size(15)
            .padding(15)
            .secure(!self.passwords_revealed);
            content = content.push(password_input);
        }

        let password_toggler = toggler(
            String::from("Reveal Passwords"),
            self.passwords_revealed,
            |value| Message::ToggleRevealPasswords(value),
        );

        let encrypt_button = button("Encrypt").on_press(Message::EncryptData).padding(15);
        let decrypt_button = button("Decrypt").on_press(Message::DecryptData).padding(15);

        content = content.push(
            row![password_toggler, encrypt_button, decrypt_button]
                .spacing(20)
                .align_items(Alignment::Center),
        );

        let cipher_output = text_editor(&self.cipher_output_data)
            .height(200)
            .padding(15);

        content = content.push(cipher_output);

        let copy_button = button("Copy")
            .on_press(Message::CopyEncryptedOutput)
            .padding(15);

        let export_as_file_button = button("Export as file").on_press(Message::ExportAsFile).padding(15);

        content = content.push(row![copy_button, export_as_file_button].spacing(20));

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn main() -> iced::Result {
    Cipherator::run(Settings::default())
}
