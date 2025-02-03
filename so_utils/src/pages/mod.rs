use iced::widget::text;

use crate::AppEvent;

pub trait Page {
    fn update(&mut self, event: crate::AppEvent);
    fn view(&self) -> iced::widget::Column<AppEvent>;
}

#[derive(Default)]
pub struct HomePage {

}

impl Page for HomePage {
    fn update(&mut self, event: crate::AppEvent) {
        match event {
            _ => ()
        }
    }

    fn view(&self) -> iced::widget::Column<AppEvent> {
        iced::widget::column![
            text("hello world!")
        ]
    }
}