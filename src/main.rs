use iced::executor;
use iced::widget::{column, row, container, pick_list, text, text_input, text_editor};
use iced::alignment::Vertical;
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};

struct Cipherator {
    cryptographic_times_options: Vec<i32>,
    cryptographic_times_selection: i32,
    cipher_input_data: text_editor::Content,
    password_inputs: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    CipherDataChanged(text_editor::Action),
    CryptoNumberChanged(i32),
    PasswordInputChange(i32, String),
}

impl Application for Cipherator {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Cipherator, Command<Self::Message>) {
        let cryptographic_times_options = vec![1, 2, 3];
        let cryptographic_times_selection = 1;
        let password_inputs = vec!["".to_string(); cryptographic_times_selection as usize];
        (
            Self {
                cryptographic_times_options,
                cipher_input_data: text_editor::Content::new(),
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

                return Command::none()
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
            text("Number of passwords: ").width(200).vertical_alignment(Vertical::Center).size(20),
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
            println!(
                "Index: {}, Value: {}",
                index, self.cryptographic_times_options[index]
            );
            let password_input = text_input(
                &format!(
                    "Password number {}",
                    self.cryptographic_times_options[index].to_string()
                ) as &str,
                value,
            )
            .on_input(move |value| Message::PasswordInputChange(index as i32, value))
            .size(30)
            .padding(15);
            content = content.push(password_input);
        }

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
