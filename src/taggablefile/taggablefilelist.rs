use crate::taggablefile::TaggableFile;
use audiotags::{Tag, TagType};
use core::cell::RefCell;
use gtk::{
    builders::DirectoryListBuilder, gio, gio::File, glib, prelude::*, subclass::prelude::*,
    DirectoryList,
};

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct TaggableFileListModel {
        pub directory_lists: RefCell<Vec<DirectoryList>>,
        pub taggable_files: RefCell<Vec<TaggableFile>>,
    }

    impl Default for TaggableFileListModel {
        fn default() -> Self {
            Self {
                directory_lists: RefCell::new(vec![]),
                taggable_files: RefCell::new(vec![]),
            }
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
            self.taggable_files.borrow().len() as u32
        }
        fn item(&self, position: u32) -> Option<glib::Object> {
            if let Some(filetag) = self.taggable_files.borrow().get(position as usize) {
                return Some(filetag.clone().into());
            }
            None
        }
    }

    impl TaggableFileListModel {
        pub fn rebuild_taglist(&self) {
            for list in self.directory_lists.borrow().iter() {
                for i in 0..list.n_items() {
                    if let Some(item) = list.item(i) {
                        if let Some(filetag) = filetag_from_directorylist(&list, i) {
                            self.taggable_files.borrow_mut().push(filetag);
                        }
                    }
                }
            }
        }
    }

    fn filetag_from_directorylist(
        directory_list: &DirectoryList,
        position: u32,
    ) -> Option<TaggableFile> {
        if let Some(file) = directory_list.item(position) {
            if let Ok(fileinfo) = file.downcast::<gio::FileInfo>() {
                let path = fileinfo.name();
                if let Ok(tag) = Tag::new().read_from_path(&path) {
                    return Some(TaggableFile::new(
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
                    ));
                }
            }
        }
        None
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
    pub fn add_folder(&self, file: &impl IsA<File>) {
        let dirlist = DirectoryListBuilder::new()
            .file(file)
            .monitored(true)
            .build();
        //dirlist.connect_items_changed(|_,_,_,_| self.imp().rebuild_taglist());
        self.imp().directory_lists.borrow_mut().push(dirlist);
    }
    pub fn rebuild_taglist(&self) {
        self.imp().rebuild_taglist();
    }
    pub fn clear_folders(&self) {
        self.imp().directory_lists.borrow_mut().clear();
    }
}

impl Default for TaggableFileListModel {
    fn default() -> Self {
        Self::new()
    }
}
