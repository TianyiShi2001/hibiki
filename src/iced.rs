use crate::components::{Album, Library, Song};
use crate::db::DB;
use iced::widget::image::Handle;
use iced::{Align, Column, Container, Element, Image, Length, Row, Sandbox, Settings, Svg};
use rusqlite::{params, Connection};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    MainView::run(Settings::default())?;
    Ok(())
    //hibiki::main();
}

#[derive(Debug)]
pub enum Message {
    Refresh,
}

struct MainView {}

impl Sandbox for MainView {
    type Message = Message;

    fn new() -> Self {
        Library::init();
        MainView {}
    }

    fn title(&self) -> String {
        String::from("Hibiki")
    }

    fn update(&mut self, _message: Message) {}

    fn view(&mut self) -> Element<Message> {
        let db = DB.lock().unwrap();
        let mut stmt = db
            .prepare("SELECT id, title, artist, cover FROM albums")
            .unwrap();
        let albums = stmt
            .query_map(params![], |row| {
                Ok(Album {
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    artist: row.get(2).unwrap(),
                    cover: row.get(3).unwrap(),
                })
            })
            .unwrap()
            .map(|x| x.unwrap());

        albums
            .fold(
                Row::new().padding(0).spacing(0).align_items(Align::Center),
                |row, album| {
                    println!("Found {:?}", &album.title);
                    row.push(Image::new(match &album.cover {
                        Some(data) => Handle::from_memory(data.clone()),
                        None => Handle::from_memory(
                            include_bytes!("../assets/folder-music.png").to_vec(),
                        ),
                    }))
                    //.width(Length::Units(100))
                    .height(Length::Units(100))
                },
            )
            .into()

        // Container::new(image)
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .padding(20)
        //     .center_x()
        //     .center_y()
        //     .into()
    }
}
