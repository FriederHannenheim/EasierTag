use crate::taggablefile::TaggableFile;
use audiotags::{Tag, TagType};
use gtk::{gio, glib, prelude::*, subclass::prelude::*, DirectoryList};

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct TaggableFileListModel {
        pub directory_list: DirectoryList,
    }

    impl Default for TaggableFileListModel {
        fn default() -> Self {
            let directory_list =
                DirectoryList::new(Some("standard::*"), None as Option<&gio::File>);
            directory_list.set_monitored(true);
            Self { directory_list }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TaggableFileListModel {
        const NAME: &'static str = "TaggableFileListModel";
        type Type = super::TaggableFileListModel;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for TaggableFileListModel {}

    impl ListModelImpl for TaggableFileListModel {
        fn item_type(&self) -> glib::Type {
            TaggableFile::static_type()
        }
        fn n_items(&self) -> u32 {
            self.directory_list.n_items()
        }
        fn item(&self, position: u32) -> Option<glib::Object> {
            if let Some(file) = self.directory_list.item(position) {
                if let Ok(fileinfo) = file.downcast::<gio::FileInfo>() {
                    let path = fileinfo.name();
                    if let Ok(tag) = Tag::new().read_from_path(&path) {
                        return Some(
                            TaggableFile::new(
                                path.to_str().expect("filepath is not valid utf-8"),
                                tag.title().unwrap_or(""),
                                tag.album_title().unwrap_or(""),
                                tag.composer().unwrap_or(""),
                                tag.genre().unwrap_or(""),
                                tag.duration(),
                                tag.year(),
                                tag.disc_number(),
                                tag.total_discs(),
                                tag.track_number(),
                                tag.total_tracks(),
                                tag.artists()
                                    .unwrap_or(vec![])
                                    .into_iter()
                                    .map(|artist| artist.to_owned())
                                    .collect(),
                                tag.album_artists()
                                    .unwrap_or(vec![])
                                    .into_iter()
                                    .map(|artist| artist.to_owned())
                                    .collect(),
                                tag.album_cover(),
                            )
                            .into(),
                        );
                    }
                }
            }
            None
        }
    }
}

glib::wrapper! {
    pub struct TaggableFileListModel(ObjectSubclass<imp::TaggableFileListModel>)
        @implements gio::ListModel;
}

impl TaggableFileListModel {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}

impl Default for TaggableFileListModel {
    fn default() -> Self {
        Self::new()
    }
}
