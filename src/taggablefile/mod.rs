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
        pub filename: RefCell<Option<String>>,
        pub title: RefCell<Option<String>>,
        pub album: RefCell<Option<String>>,
        pub composer: RefCell<Option<String>>,
        pub genre: RefCell<Option<String>>,
        pub artists: RefCell<Vec<String>>,
        pub album_artists: RefCell<Vec<String>>,

        pub duration: Cell<Option<f64>>,
        pub year: Cell<Option<i32>>,
        pub disc: Cell<Option<u32>>,
        pub total_discs: Cell<Option<u32>>,
        pub track: Cell<Option<u32>>,
        pub total_tracks: Cell<Option<u32>>,

        pub cover: RefCell<Option<Texture>>,
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
                    ParamSpecString::builder("album").build(),
                    ParamSpecString::builder("composer").build(),
                    ParamSpecString::builder("genre").build(),
                    ParamSpecValueArray::builder("artists").build(),
                    ParamSpecValueArray::builder("album_artists").build(),
                    ParamSpecFloat::builder("duration").build(),
                    ParamSpecInt::builder("year").build(),
                    ParamSpecUInt::builder("disc").build(),
                    ParamSpecUInt::builder("total_discs").build(),
                    ParamSpecUInt::builder("track").build(),
                    ParamSpecUInt::builder("total_tracks").build(),
                    ParamSpecObject::builder::<Texture>("cover").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            if let Ok(value) = value.get::<String>() {
                match pspec.name() {
                    "filename" => self.filename.replace(Some(value)),
                    "title" => self.title.replace(Some(value)),
                    "album" => self.album.replace(Some(value)),
                    "composer" => self.composer.replace(Some(value)),
                    "genre" => self.genre.replace(Some(value)),
                    "cover_mime_type" => self.cover_mime_type.replace(Some(value)),
                    _ => unimplemented!(),
                }
            }
            if let Ok(value) = value.get::<Vec<String>>() {
                match pspec.name() {
                    "artists" => self.artists.replace(value),
                    "album_artists" => self.album_artists.replace(value),
                    _ => unimplemented!(),
                }
            }
            if let Ok(value) = value.get::<u32>() {
                match pspec.name() {
                    "disc" => self.disc.replace(Some(value)),
                    "total_discs" => self.total_discs.replace(Some(value)),
                    "track" => self.track.replace(Some(value)),
                    "total_tracks" => self.total_tracks.replace(Some(value)),
                    _ => unimplemented!(),
                }
            }
            if let Ok(value) = value.get::<i32>() {
                match pspec.name() {
                    "year" => self.year.replace(Some(value)),
                    _ => unimplemented!(),
                };
            }
            if let Ok(value) = value.get::<f64>() {
                match pspec.name() {
                    "duration" => self.duration.replace(Some(value)),
                    _ => unimplemented!(),
                };
            }
            if let Ok(value) = value.get::<Texture>() {
                match pspec.name() {
                    "cover" => self.cover.replace(Some(value)),
                    _ => unimplemented!(),
                }
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "filename" => self.filename.borrow().to_value(),
                "title" => self.title.borrow().to_value(),
                "album" => self.album.borrow().to_value(),
                "composer" => self.composer.borrow().to_value(),
                "genre" => self.genre.borrow().to_value(),
                "artists" => self.artists.borrow().to_value(),
                "album_artists" => self.album_artists.borrow().to_value(),

                "duration" => self.duration.get().to_value(),
                "year" => self.year.get().to_value(),
                "disc" => self.disc.get().map().to_value(),
                "total_discs" => self.total_discs.get().to_value(),
                "track" => self.track.get().to_value(),
                "total_tracks" => self.total_tracks.get().to_value(),

                "cover" => self.cover.borrow().to_value(),
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
        album: &str,
        composer: &str,
        genre: &str,

        artists: Vec<String>,
        album_artists: Vec<String>,

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
