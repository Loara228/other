use iced::{alignment::Horizontal, widget::{button, horizontal_space, text, text_input}, Alignment::Center, Length};
use crate::{models::ModalModel, AppEvent};

pub trait Page {
    fn update(&mut self, event: crate::AppEvent);
    fn view(&self) -> iced::widget::Column<AppEvent>;
    fn label(&self) -> &str;
    fn hidden(&self) -> bool;
}

#[derive(Default)]
pub struct HomePage {

}

impl Page for HomePage {
    fn update(&mut self, event: crate::AppEvent) {
    }

    fn view(&self) -> iced::widget::Column<AppEvent> {
        iced::widget::column![
            text("Отчет в тг").align_x(Horizontal::Left).size(24).width(Length::Fill),
            iced::widget::row![
                button(text("открытие").align_x(Horizontal::Center)).width(Length::Fill).on_press(AppEvent::SwitchPage("TRModal1".to_owned())),
                button(text("закрытие").align_x(Horizontal::Center)).width(Length::Fill).on_press(AppEvent::SwitchPage("TRModal2".to_owned()))
            ].spacing(6),
            iced::widget::row![
                button(text("чек").align_x(Horizontal::Center)).width(Length::Fill).on_press(AppEvent::SwitchPage("TRModal1".to_owned()))
            ]
        ].spacing(6)
    }

    fn label(&self) -> &str {
        "Главная"
    }
    
    fn hidden(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy)]
pub enum TReportPageType {
    Close,
    Purchase
}

pub struct TReportPage {
    t: TReportPageType,
    modal: ModalModel,
    output: String
}

impl TReportPage {
    pub fn new(t: TReportPageType) -> Self {
        Self {
            t,
            output: "empty".to_owned(),
            modal: ModalModel::default()
        }
    }
}

impl Page for TReportPage {
    fn update(&mut self, event: crate::AppEvent) {
        match event {
            AppEvent::TReportOpen => {
                let screenshot_path = crate::get_screenshot_path();
                let file_path = screenshot_path.parent().unwrap().join("tg-report").to_string_lossy().into_owned();
                let mut command = std::process::Command::new(file_path);

                if !self.modal.input1.trim().is_empty() {
                    command.arg("-t");
                    command.arg(self.modal.input1.trim());
                }
                let output = command.arg("open")
                    .arg(screenshot_path.to_string_lossy().into_owned())
                    .output()
                    .unwrap();

                self.output = String::from_utf8(output.stdout).unwrap_or("stdout?".to_owned());
                self.modal = ModalModel::default();
            },
            AppEvent::TReportClose => {
                let screenshot_path = crate::get_screenshot_path();
                let file_path = screenshot_path.parent().unwrap().join("tg-report").to_string_lossy().into_owned();
                let mut command = std::process::Command::new(file_path);

                command.arg("close");
                command.arg("--credit-requests").arg(self.modal.input2.clone());
                command.arg("--credit-responses").arg(self.modal.input3.clone());
                command.arg("--registrations").arg(self.modal.input4.clone());
                command.arg("--adapter").arg(self.modal.input5.clone());
                command.arg("--cash").arg(self.modal.input6.clone());
                
                let output = command.arg(screenshot_path.to_string_lossy().into_owned())
                    .output()
                    .unwrap();

                self.output = String::from_utf8(output.stdout).unwrap_or("stdout?".to_owned());
                self.modal = ModalModel::default();
            },
            AppEvent::ModalInput1Changed(text) => {
                self.modal.input1 = text;
            },
            AppEvent::ModalInput2Changed(text) => {
                self.modal.input2 = text;
            },
            AppEvent::ModalInput3Changed(text) => {
                self.modal.input3 = text;
            },
            AppEvent::ModalInput4Changed(text) => {
                self.modal.input4 = text;
            },
            AppEvent::ModalInput5Changed(text) => {
                self.modal.input5 = text;
            },
            AppEvent::ModalInput6Changed(text) => {
                self.modal.input6 = text;
            },
            _ => ()
        }
    }
    
    fn view(&self) -> iced::widget::Column<AppEvent> {
        match self.t {
            TReportPageType::Purchase => {
                iced::widget::column![
                    text("Дополнительный текст"),
                    text_input("Можно оставить пустым", &self.modal.input1)
                        .on_input(AppEvent::ModalInput1Changed),
        
                    iced::widget::row![
                        button(text("Отправить").align_x(Horizontal::Center)).width(Length::Fill).on_press(AppEvent::TReportOpen)
                    ],
                    horizontal_space(),
                    text(&self.output).size(12).style(iced::widget::text::secondary)
                ].spacing(10)
            },
            TReportPageType::Close => {
                iced::widget::column![
                    iced::widget::row![
                        text("Заявки КД"),
                        text_input("integer", &self.modal.input2).on_input(AppEvent::ModalInput2Changed),
                        text("/"),
                        text_input("integer", &self.modal.input3).on_input(AppEvent::ModalInput3Changed),
                    ].spacing(6).align_y(Center),
                    iced::widget::row![
                        text("Регистраций 1C"),
                        text_input("integer", &self.modal.input4).on_input(AppEvent::ModalInput4Changed),
                    ].spacing(6).align_y(Center),
                    iced::widget::row![
                        text("Адаптер"),
                        text_input("integer", &self.modal.input5).on_input(AppEvent::ModalInput5Changed),
                    ].spacing(6).align_y(Center),
                    iced::widget::row![
                        text("ДС"),
                        text_input("integer", &self.modal.input6).on_input(AppEvent::ModalInput6Changed),
                    ].spacing(6).align_y(Center),
        
                    iced::widget::row![
                        button(text("Отправить").align_x(Horizontal::Center)).width(Length::Fill).on_press(AppEvent::TReportClose)
                    ],
                    iced::widget::horizontal_space(),
                    iced::widget::text(&self.output).size(12).style(iced::widget::text::secondary)
                ].spacing(10)
            }
        }
    }

    fn label(&self) -> &str {
        "Modal"
    }

    fn hidden(&self) -> bool {
        true
    }
}