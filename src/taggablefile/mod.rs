use gtk::{
    gdk::Texture,
    glib,
    glib::{Bytes, Object, ParamSpec, ParamSpecBoxed, ParamSpecObject, ParamSpecString, Value},
    prelude::*,
    subclass::prelude::*,
};
use log::warn;
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

pub mod taggablefilelist;

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct TaggableFile {
        pub filename: RefCell<Option<String>>,
        pub title: RefCell<Option<String>>,
        pub album: RefCell<Option<String>>,
        pub composer: RefCell<Option<String>>,
        pub genre: RefCell<Option<String>>,
        pub duration: RefCell<Option<String>>,
        pub year: RefCell<Option<String>>,
        pub disc: RefCell<Option<String>>,
        pub total_discs: RefCell<Option<String>>,
        pub track: RefCell<Option<String>>,
        pub total_tracks: RefCell<Option<String>>,

        pub artists: RefCell<Vec<String>>,
        pub album_artists: RefCell<Vec<String>>,

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
                    ParamSpecString::builder("duration").build(),
                    ParamSpecString::builder("year").build(),
                    ParamSpecString::builder("disc").build(),
                    ParamSpecString::builder("total-discs").build(),
                    ParamSpecString::builder("track").build(),
                    ParamSpecString::builder("total-tracks").build(),
                    ParamSpecBoxed::builder::<Vec<String>>("artists").build(),
                    ParamSpecBoxed::builder::<Vec<String>>("album-artists").build(),
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
                    "duration" => self.duration.replace(Some(value)),
                    "year" => self.year.replace(Some(value)),
                    "disc" => self.disc.replace(Some(value)),
                    "total-discs" => self.total_discs.replace(Some(value)),
                    "track" => self.track.replace(Some(value)),
                    "total-tracks" => self.total_tracks.replace(Some(value)),
                    _ => unimplemented!(),
                };
            }
            if let Ok(value) = value.get::<Vec<String>>() {
                match pspec.name() {
                    "artists" => self.artists.replace(value),
                    "album-artists" => self.album_artists.replace(value),
                    _ => unimplemented!(),
                };
            }
            if let Ok(value) = value.get::<Texture>() {
                match pspec.name() {
                    "cover" => self.cover.replace(Some(value)),
                    _ => unimplemented!(),
                };
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "filename" => self.filename.borrow().to_value(),
                "title" => self.title.borrow().to_value(),
                "album" => self.album.borrow().to_value(),
                "composer" => self.composer.borrow().to_value(),
                "genre" => self.genre.borrow().to_value(),
                "duration" => self.duration.borrow().to_value(),
                "year" => self.year.borrow().to_value(),
                "disc" => self.disc.borrow().to_value(),
                "total-discs" => self.total_discs.borrow().to_value(),
                "track" => self.track.borrow().to_value(),
                "total-tracks" => self.total_tracks.borrow().to_value(),

                "artists" => self.artists.borrow().to_value(),
                "album-artists" => self.album_artists.borrow().to_value(),

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
        duration: Option<f64>,
        year: Option<i32>,
        disc: Option<u16>,
        total_discs: Option<u16>,
        track: Option<u16>,
        total_tracks: Option<u16>,

        artists: Vec<String>,
        album_artists: Vec<String>,

        cover: Option<audiotags::Picture>,
    ) -> Self {
        let mut filetag_obj = Object::builder()
            .property("filename", filename)
            .property("title", title)
            .property("album", album)
            .property("composer", composer)
            .property("genre", genre)
            .property("artists", artists)
            .property("album-artists", album_artists);

        if let Some(value) = duration {
            filetag_obj = filetag_obj.property("duration", value.to_string());
        }
        if let Some(value) = year {
            filetag_obj = filetag_obj.property("year", value.to_string());
        }
        if let Some(value) = disc {
            filetag_obj = filetag_obj.property("disc", value.to_string());
        }
        if let Some(value) = total_discs {
            filetag_obj = filetag_obj.property("total-discs", value.to_string());
        }
        if let Some(value) = track {
            filetag_obj = filetag_obj.property("track", value.to_string());
        }
        if let Some(value) = total_tracks {
            filetag_obj = filetag_obj.property("total-tracks", value.to_string());
        }

        if let Some(cover) = cover {
            if let Ok(texture) = Texture::from_bytes(&Bytes::from_owned(cover.data.to_vec())) {
                filetag_obj = filetag_obj.property("cover", texture);
            } else {
                warn!(
                    "{} has cover metadata but it is in an unsupported format.",
                    filename
                );
            }
        }

        filetag_obj.build()
    }
}
