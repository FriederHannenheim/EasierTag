use gtk::{
    glib,
    glib::{Object, ParamSpec, ParamSpecString, Value},
    prelude::*,
    subclass::prelude::*,
};
use once_cell::sync::Lazy;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct TaggableFile {
        pub filename: RefCell<Option<String>>,
        pub title: RefCell<Option<String>>,
        pub artist: RefCell<Option<String>>,
        pub album: RefCell<Option<String>>,
        pub year: RefCell<Option<String>>,
        pub disc: RefCell<Option<String>>,
        pub track: RefCell<Option<String>>,
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
                    ParamSpecString::builder("artist").build(),
                    ParamSpecString::builder("album").build(),
                    ParamSpecString::builder("year").build(),
                    ParamSpecString::builder("disc").build(),
                    ParamSpecString::builder("track").build(),
                    ParamSpecString::builder("genre").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            let _value = value
                .get::<String>()
                .expect("The value needs to be of type `String`.");
            match pspec.name() {
                "filename" => self.filename.replace(Some(_value)),
                "title" => self.title.replace(Some(_value)),
                "artist" => self.artist.replace(Some(_value)),
                "album" => self.album.replace(Some(_value)),
                "year" => self.year.replace(Some(_value)),
                "disc" => self.disc.replace(Some(_value)),
                "track" => self.track.replace(Some(_value)),
                "genre" => self.genre.replace(Some(_value)),
                _ => unimplemented!(),
            };
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "filename" => self.filename.borrow(),
                "title" => self.title.borrow(),
                "artist" => self.artist.borrow(),
                "album" => self.album.borrow(),
                "year" => self.year.borrow(),
                "disc" => self.disc.borrow(),
                "track" => self.track.borrow(),
                "genre" => self.genre.borrow(),
                _ => unimplemented!(),
            }
            .to_value()
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
        artist: &str,
        album: &str,
        year: &str,
        disc: &str,
        track: &str,
        genre: &str,
    ) -> Self {
        Object::builder()
            .property("filename", filename)
            .property("title", title)
            .property("artist", artist)
            .property("album", album)
            .property("year", year)
            .property("disc", disc)
            .property("track", track)
            .property("genre", genre)
            .build()
    }
}
