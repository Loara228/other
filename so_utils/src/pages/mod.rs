use iced::{alignment::Horizontal, widget::{button, text}, Length};

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
    input_text: String,
    model: ModalModel
}

impl TReportPage {
    pub fn new(t: TReportPageType) -> Self {
        Self {
            t,
            input_text: String::new(),
            model: ModalModel::default()
        }
    }
}

impl Page for TReportPage {
    fn update(&mut self, event: crate::AppEvent) {
        match event {
            AppEvent::TReportOpen(text) => {
                let screenshot_path = crate::get_screenshot_path();
                let file_path = screenshot_path.parent().unwrap().join("tg-report").to_string_lossy().into_owned();
                let mut command = std::process::Command::new(file_path);

                command.arg("open").arg(screenshot_path.to_string_lossy().into_owned());
                if text.is_some() {
                    command.arg("-t").arg(format!("\"{}\"", text.unwrap()));
                }
                command.output().unwrap();
            },
            AppEvent::TReportClose => {
                
            }

            AppEvent::ModalInput1Changed(text) => {
                self.model.input1 = text;
            }

            _ => ()
        }
    }
    
    fn view(&self) -> iced::widget::Column<AppEvent> {
        iced::widget::column![
            iced::widget::text_input("Type something...", &self.model.input1)
                .on_input(AppEvent::ModalInput1Changed)
        ]
    }

    fn label(&self) -> &str {
        "Modal"
    }

    fn hidden(&self) -> bool {
        true
    }
}