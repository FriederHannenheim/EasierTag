use crate::taggablefile::TaggableFile;
use audiotags::{Tag, TagType};
use gtk::{gio, glib, prelude::*, subclass::prelude::*, DirectoryList};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TaggableFileListModel {
        pub directory_list: DirectoryList,
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
                    let path = fileinfo.get_name().unwrap();
                    if let Ok(tag) = Tag::new().read_from_path(path) {
                        return Some(
                            TaggableFile::new(
                                "",
                                tag.title().unwrap_or(""),
                                tag.artists()
                                    .unwrap_or(vec![])
                                    .into_iter()
                                    .map(String::from)
                                    .collect(),
                                tag.album_title().unwrap_or(""),
                                tag.year(),
                                tag.disc_number(),
                                tag.total_discs(),
                                tag.track_number(),
                                tag.total_tracks(),
                                tag.genre().unwrap_or(""),
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
