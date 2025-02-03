pub mod pages;

use std::collections::HashMap;

use iced::{widget::*, Alignment::Center, Element, Length::{self, Fill}, Theme};
use pages::Page;

pub fn main() -> iced::Result {
    iced::application("Svyaz.ON Utils", App::update, App::view).theme(App::theme).run()
}

struct App {
    pages: HashMap<String, Box<dyn Page>>,
    cur_page: String
}

impl Default for App {
    fn default() -> Self {
        let mut pages: HashMap<String, Box<dyn Page>> = HashMap::new();
        pages.insert("HomePage".to_owned(), Box::new(pages::HomePage::default()));
        let mut app = Self { 
            pages,
            cur_page: Default::default()
        };
        app.set_page("HomePage");
        app
    }
}

impl App {
    fn update(&mut self, event: AppEvent) {
        match &event {
            AppEvent::SwitchPage(name) => self.set_page(&name),
            _ => self.current_page_mut().update(event),
        }
    }

    fn view(&self) -> Element<AppEvent> {
        // self.view_menu().wrap();
        iced::widget::column![self.view_menu(), self.current_page().view().padding(10).align_x(Center)].into()
    }

    fn set_page(&mut self, name: &str) {
        self.cur_page = name.to_owned();
    }

    fn current_page_mut(&mut self) -> &mut Box<dyn Page> {
        self.pages.get_mut(&self.cur_page).expect("unknown page")
    }

    fn current_page(&self) -> &Box<dyn Page> {
        self.pages.get(&self.cur_page).expect("unknown page")
    }

    fn theme(&self) -> Theme {
        Theme::SolarizedLight
    }

    fn view_menu(&self) -> Container<AppEvent> {
        let buttons: Row<_> = self.pages.iter().map(|page| {
            button(text(page.0)).on_press(AppEvent::SwitchPage(page.0.to_owned())).style(button::text).width(Length::Shrink).into()
        }).collect();
        
        container(buttons).width(Fill).align_x(Center).style(container::bordered_box)
    }
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    SwitchPage(String)
}