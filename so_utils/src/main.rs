#![windows_subsystem = "windows"]

pub mod pages;
pub mod models;

use std::{collections::HashMap, env, path::PathBuf};

use iced::{widget::*, Alignment::Center, Element, Length::{self, Fill}, Size, Theme};
use pages::Page;
use xcap::image::{DynamicImage, ImageFormat};

pub fn main() -> iced::Result {
    let screenshot = xcap::Monitor::all().unwrap()
        .get(0).unwrap()
        .capture_image().unwrap();

    DynamicImage::from(screenshot)
        .crop(240, 160, 1580, 820)
        .save_with_format(get_screenshot_path(), ImageFormat::Png)
        .unwrap();

    iced::application("SvyazON utils", App::update, App::view)
        .window_size(Size::new(250f32, 400f32))
        .theme(App::theme)
        .run()
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    SwitchPage(String),

    TReportClose,
    TReportOpen,

    ModalInput1Changed(String),
    ModalInput2Changed(String),
    ModalInput3Changed(String),
    ModalInput4Changed(String),
    ModalInput5Changed(String),
    ModalInput6Changed(String),
}

struct App {
    pages: HashMap<String, Box<dyn Page>>,
    cur_page: String
}

impl Default for App {  
    fn default() -> Self {
        let mut pages: HashMap<String, Box<dyn Page>> = HashMap::new();
        pages.insert("HomePage".to_owned(), Box::new(pages::HomePage::default()));
        pages.insert("TRModal1".to_owned(), Box::new(pages::TReportPage::new(pages::TReportPageType::Purchase)));
        pages.insert("TRModal2".to_owned(), Box::new(pages::TReportPage::new(pages::TReportPageType::Close)));
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
        Theme::Light
    }

    fn view_menu(&self) -> Container<AppEvent> {
        
        let mut buttons: Row<_> = Row::new(); // Предполагаем, что Row<_> можно создать таким образом

        for page in self.pages.iter() {
            if !page.1.hidden() {
                let button: Button<AppEvent, Theme> = button(text(page.1.label()))
                    .on_press(AppEvent::SwitchPage(page.0.to_owned()))
                    .style(button::text)
                    .width(Length::Shrink);        
                buttons = buttons.push(button); // Добавляем кнопку в Row
            }
        }
        container(buttons).width(Fill).align_x(Center).style(container::bordered_box)
    }
}

pub fn get_screenshot_path() -> PathBuf {
    env::current_exe().unwrap().parent().unwrap().join("screenshot.png")
}