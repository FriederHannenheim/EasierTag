use gtk::{
    gio, glib, glib::clone, glib::closure, glib::Object, prelude::*, subclass::prelude::*,
    BitsetIter, CompositeTemplate, ConstantExpression, CustomSorter, DirectoryList, FileFilter,
    FilterChange, FilterListModel, ListItem, ListView, MultiSelection, PropertyExpression,
    SignalListItemFactory, SingleSelection, SortListModel, TreeListModel, TreeListRow, Widget,
};

use crate::folderbrowser::folderitem::FolderItem;
use crate::taggablefile::taggablefilelist::TaggableFileListModel;
use crate::window::EasierTagApplicationWindow;
mod folderitem;

mod imp {
    use super::*;

    #[derive(CompositeTemplate)]
    #[template(resource = "/net/fhannenheim/EasierTag/ui/folderbrowser.ui")]
    pub struct FolderBrowser {
        #[template_child]
        pub primary_listview: TemplateChild<ListView>,
        pub primary_dirlist: DirectoryList,
    }

    impl Default for FolderBrowser {
        fn default() -> Self {
            let primary_dirlist =
                DirectoryList::new(Some("standard::*"), None as Option<&gio::File>);
            primary_dirlist.set_monitored(true);

            Self {
                primary_listview: TemplateChild::<ListView>::default(),
                primary_dirlist,
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FolderBrowser {
        const NAME: &'static str = "FolderBrowser";
        type Type = super::FolderBrowser;
        type ParentType = Widget;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FolderBrowser {
        fn constructed(&self) {
            self.parent_constructed();
        }
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for FolderBrowser {}
}

glib::wrapper! {
    pub struct FolderBrowser(ObjectSubclass<imp::FolderBrowser>)
        @extends gtk::Widget;
}

impl Default for FolderBrowser {
    fn default() -> Self {
        Self::new()
    }
}

impl FolderBrowser {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn primary_dirlist(&self) -> DirectoryList {
        self.imp().primary_dirlist.clone()
    }

    pub fn primary_listview(&self) -> ListView {
        self.imp().primary_listview.clone()
    }

    pub fn init(&self, window: &EasierTagApplicationWindow) {
        let primary_list_factory = SignalListItemFactory::new();

        primary_list_factory.connect_setup(move |_, list_item| {
            let folderitem = FolderItem::new();

            list_item.set_child(Some(&folderitem));

            let list_item_expr = ConstantExpression::new(list_item);
            let fileinfo_expr =
                PropertyExpression::new(ListItem::static_type(), Some(&list_item_expr), "item");

            let icon_name_expr =
                fileinfo_expr.chain_closure::<gio::ThemedIcon>(closure!(|_: Option<
                    glib::Object,
                >,
                                                                         fileinfo_obj: Option<
                    glib::Object,
                >| {
                    if let Some(fileinfo_obj) = fileinfo_obj {
                        if let Some(themed_icon) = fileinfo_obj
                            .downcast::<TreeListRow>()
                            .unwrap()
                            .item()
                            .unwrap()
                            .downcast::<gio::FileInfo>()
                            .unwrap()
                            .attribute_object("standard::symbolic-icon")
                        {
                            return themed_icon.downcast::<gio::ThemedIcon>().unwrap();
                        }
                    }

                    gio::ThemedIcon::from_names(&[
                        "workspace-folder-symbolic",
                        "folder-documents-symbolic",
                    ])
                }));

            let basename_expr =
                fileinfo_expr.chain_closure::<String>(closure!(|_: Option<glib::Object>,
                                                                fileinfo_obj: Option<
                    glib::Object,
                >| {
                    if let Some(fileinfo_obj) = fileinfo_obj {
                        if let Some(file) = fileinfo_obj
                            .downcast::<TreeListRow>()
                            .unwrap()
                            .item()
                            .unwrap()
                            .downcast::<gio::FileInfo>()
                            .unwrap()
                            .attribute_object("standard::file")
                        {
                            let file = file
                                .downcast::<gio::File>()
                                .expect("failed to downcast::<gio::File>() from file GObject");

                            return String::from(
                                file.basename()
                                    .expect("failed to get file.basename()")
                                    .to_string_lossy(),
                            );
                        }
                    }

                    String::from("")
                }));

            let treeexpander_expr = fileinfo_expr.chain_closure::<Option<TreeListRow>>(closure!(
                |_: Option<Object>, fileinfo_obj: Option<Object>| {
                    if let Some(fileinfo_obj) = fileinfo_obj {
                        if let Ok(tree_list_row) = fileinfo_obj.downcast::<TreeListRow>() {
                            return Some(tree_list_row);
                        }
                    }
                    None
                }
            ));

            basename_expr.bind(&folderitem.file_label(), "label", Widget::NONE);
            icon_name_expr.bind(&folderitem.file_image(), "gicon", Widget::NONE);
            treeexpander_expr.bind(&folderitem.tree_expander(), "list_row", Widget::NONE);
        });

        let filefilter = FileFilter::new();
        filefilter.add_mime_type("inode/directory");
        let filefilter_model =
            FilterListModel::new(Some(&self.imp().primary_dirlist), Some(&filefilter));

        let alphanumeric_sorter = CustomSorter::new(move |obj1, obj2| {
            let first_fileinfo = obj1
                .clone()
                .downcast::<gio::FileInfo>()
                .expect("failed to downcast obj1");
            let first_file = first_fileinfo.attribute_object("standard::file").unwrap();
            let first_file = first_file.downcast::<gio::File>().unwrap();
            let first_display_name = first_file.basename().unwrap();
            let first_display_name = first_display_name.to_str().unwrap();

            let second_fileinfo = obj2
                .clone()
                .downcast::<gio::FileInfo>()
                .expect("failed to downcast obj2");
            let second_file = second_fileinfo.attribute_object("standard::file").unwrap();
            let second_file = second_file.downcast::<gio::File>().unwrap();
            let second_display_name = second_file.basename().unwrap();
            let second_display_name = second_display_name.to_str().unwrap();

            first_display_name.cmp(second_display_name).into()
        });
        let sort_list_model =
            SortListModel::new(Some(&filefilter_model), Some(&alphanumeric_sorter));

        let treelist_model = TreeListModel::new(
            &sort_list_model,
            false,
            false,
            clone!(@weak filefilter, @weak alphanumeric_sorter => @default-return None, move |obj| {
                    let fileinfo = obj
                        .clone()
                        .downcast::<gio::FileInfo>()
                        .unwrap()
                        .attribute_object("standard::file")
                        .unwrap();
                    if let Ok(file) = fileinfo.downcast::<gio::File>() {
                        let secondary_dirlist = DirectoryList::new(Some("standard::*"), Some(&file));
                        secondary_dirlist.set_monitored(true);
                        let secondary_filefiltermodel = FilterListModel::new(Some(&secondary_dirlist), Some(&filefilter));
                        return Some(SortListModel::new(Some(&secondary_filefiltermodel), Some(&alphanumeric_sorter)).into());
                    };
                    None
            }),
        );

        let primary_selection_model = MultiSelection::new(Some(&treelist_model));

        self.imp()
            .primary_listview
            .get()
            .set_factory(Some(&primary_list_factory));
        self.imp()
            .primary_listview
            .get()
            .set_model(Some(&primary_selection_model));

        self.primary_dirlist().connect_items_changed(
            clone!(@weak filefilter => move |_primary_dirlist, _position, _removed, _added| {
                filefilter.changed(FilterChange::Different);
            }),
        );
        primary_selection_model.connect_selection_changed(clone!(@weak window => move |_model, _position, _n| {
            let filelist = window.filecolumnview().column_view().model().unwrap().downcast::<MultiSelection>().unwrap().model().unwrap().downcast::<TaggableFileListModel>().unwrap();
            filelist.clear_folders();
            if let Some(model) = _model.model() {
                for index in BitsetIter::init_first(&_model.selection()) {
                    let item = _model.item(index.1);
                    if let Some(fileinfo_obj) = item {
                        if let Some(file) = fileinfo_obj
                            .downcast::<TreeListRow>()
                            .unwrap()
                            .item()
                            .unwrap()
                            .downcast::<gio::FileInfo>()
                            .unwrap()
                            .attribute_object("standard::file")
                            {
                                let file = file
                                    .downcast::<gio::File>()
                                    .expect("failed to downcast::<gio::File>() from file GObject");
                                println!("adding {}", file.basename().unwrap().display());
                                filelist.add_folder(&file);
                            }
                    }
                }
                filelist.rebuild_taglist();
            }
        }));

        self.primary_dirlist()
            .set_file(Some(&gio::File::for_path("/home/fried/")));
        println!("{:?}", self.imp().primary_dirlist.is_loading());
    }
}
