use gtk::{
    glib,
    glib::{
        Object, ParamSpec, ParamSpecInt, ParamSpecString, ParamSpecUInt, ParamSpecValueArray, Value,
    },
    prelude::*,
    subclass::prelude::*,
};
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

mod taggablefilelist;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct TaggableFile {
        pub filename: RefCell<String>,
        pub title: RefCell<Option<String>>,
        pub artists: RefCell<Vec<String>>,
        pub duration: Cell<Option<f64>>,
        pub album: RefCell<Option<String>>,
        pub album_artists: RefCell<Vec<String>>,
        pub year: Cell<Option<i32>>,
        pub cover: RefCell<Vec<u8>>,
        pub cover_mime_type: RefCell<Option<String>>,
        pub composer: RefCell<Option<String>>,
        pub disc: Cell<Option<u16>>,
        pub total_discs: Cell<Option<u16>>,
        pub track: Cell<Option<u16>>,
        pub total_tracks: Cell<Option<u16>>,
        pub genre: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TaggableFile {
        const NAME: &'static str = "TaggableFile";
        type Type = super::TaggableFile;
    }

    impl ObjectImpl for TaggableFile {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::builder("filename").build(),
                    ParamSpecString::builder("title").build(),
                    ParamSpecValueArray::builder("artists").build(),
                    ParamSpecString::builder("album").build(),
                    ParamSpecInt::builder("year").build(),
                    ParamSpecUInt::builder("disc").build(),
                    ParamSpecUInt::builder("total_discs").build(),
                    ParamSpecUInt::builder("track").build(),
                    ParamSpecUInt::builder("total_tracks").build(),
                    ParamSpecString::builder("genre").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            if let Ok(value) = value.get::<String>() {
                match pspec.name() {
                    "filename" => self.filename.replace(Some(string_value.unwrap())),
                    "title" => self.title.replace(Some(string_value.unwrap())),
                    "album" => self.album.replace(Some(string_value.unwrap())),
                    "genre" => self.genre.replace(Some(string_value.unwrap())),
                    _ => unimplemented!(),
                }
            }
            if let Ok(value) = value.get::<Vec<String>>() {
                match pspec.name() {
                    "artists" => self.artists.replace(string_vec_value.unwrap()),
                    _ => unimplemented!(),
                }
            }
            if let Ok(value) = value.get::<u32>() {
                match pspec.name() {
                    "disc" => self.disc.replace(Some(uint_value.unwrap() as u16)),
                    "total_discs" => self.total_discs.replace(Some(uint_value.unwrap() as u16)),
                    "track" => self.track.replace(Some(uint_value.unwrap() as u16)),
                    "total_tracks" => self.total_tracks.replace(Some(uint_value.unwrap() as u16)),
                    _ => unimplemented!(),
                }
            }
            if let Ok(value) = value.get::<i32>() {
                match pspec.name() {
                    "year" => self.year.replace(Some(int_value.unwrap())),
                    _ => unimplemented!(),
                };
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "filename" => self.filename.borrow().to_value(),
                "title" => self.title.borrow().to_value(),
                "artists" => self.artists.borrow().to_value(),
                "album" => self.album.borrow().to_value(),
                "year" => self.year.get().to_value(),
                "disc" => self.disc.get().map(|inner| inner as u32).to_value(),
                "total_discs" => self.total_discs.get().to_value(),
                "track" => self.track.get().to_value(),
                "total_tracks" => self.total_tracks.get().to_value(),
                "genre" => self.genre.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}

glib::wrapper! {
    pub struct TaggableFile(ObjectSubclass<imp::TaggableFile>);
}

impl TaggableFile {
    pub fn new(
        filename: &str,
        title: &str,
        artists: Vec<String>,
        album: &str,
        year: Option<i32>,
        disc: Option<u16>,
        total_discs: Option<u16>,
        track: Option<u16>,
        total_tracks: Option<u16>,
        genre: &str,
    ) -> Self {
        Object::builder()
            .property("filename", filename)
            .property("title", title)
            .property("artists", artists)
            .property("album", album)
            .property("year", year)
            .property("disc", disc.map(|inner| inner as u32))
            .property("total_discs", total_discs.map(|inner| inner as u32))
            .property("track", track.map(|inner| inner as u32))
            .property("total_tracks", total_tracks.map(|inner| inner as u32))
            .property("genre", genre)
            .build()
    }
}
