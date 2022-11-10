use gtk::{
    gdk, gio, glib, glib::clone, glib::closure, prelude::*, subclass::prelude::*, Button,
    CompositeTemplate, ConstantExpression, CustomSorter, DirectoryList, Entry, FileFilter,
    FilterChange, FilterListModel, ListItem, ListView, MultiSorter, PropertyExpression,
    SignalListItemFactory, SingleSelection, SortListModel, SorterChange, Widget, TreeListModel,
    TreeListRow,
};

use crate::folderbrowser::folderitem::FolderItem;

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

    pub fn init(&self) {
        let primary_list_factory = SignalListItemFactory::new();

        primary_list_factory.connect_setup(move |_, list_item| {
            let folderitem = FolderItem::new();

            list_item.set_child(Some(&folderitem));

            let list_item_expr = ConstantExpression::new(list_item);
            let fileinfo_expr =
                PropertyExpression::new(ListItem::static_type(), Some(&list_item_expr), "item");

            let file_expr = fileinfo_expr.chain_closure::<Option<gio::File>>(closure!(
                |_: Option<glib::Object>, fileinfo_obj: Option<glib::Object>| {
                    fileinfo_obj
                        .map(|fileinfo_obj| {
                            fileinfo_obj
                                .downcast::<TreeListRow>()
                                .unwrap()
                                .item()
                                .unwrap()
                                .downcast::<gio::FileInfo>()
                                .unwrap()
                                .attribute_object("standard::file")
                                .unwrap()
                                .downcast::<gio::File>()
                                .unwrap()
                        })
                        .to_value()
                }
            ));

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

            // file_expr.bind(&folderitem, "current-file", Widget::NONE);
            basename_expr.bind(&folderitem.file_label(), "label", Widget::NONE);
            icon_name_expr.bind(&folderitem.file_image(), "gicon", Widget::NONE);
            println!("Alright so the list init is working.");
        });

        let filefilter = FileFilter::new();
        filefilter.add_mime_type("inode/directory");
        let filefilter_model =
            FilterListModel::new(Some(&self.imp().primary_dirlist), Some(&filefilter));

        let treelist_model = TreeListModel::new(&filefilter_model, false, false, |obj| {
            let fileinfo = obj.clone().downcast::<gio::FileInfo>()
                            .unwrap().attribute_object("standard::file").unwrap();
            if let Ok(file) = fileinfo.downcast::<gio::File>() {
                let secondary_filefilter = FileFilter::new();
                secondary_filefilter.add_mime_type("inode/directory");
                let secondary_dirlist = DirectoryList::new(Some("standard::*"), Some(&file));
                secondary_dirlist.set_monitored(true);
                return Some(FilterListModel::new(Some(&secondary_dirlist), Some(&secondary_filefilter)).into());
            }
            None
        });

        let primary_selection_model = SingleSelection::new(Some(&treelist_model));

        self.imp()
            .primary_listview
            .get()
            .set_factory(Some(&primary_list_factory));
        self.imp()
            .primary_listview
            .get()
            .set_model(Some(&primary_selection_model));
        self.primary_listview().connect_activate(|primary_listview, position| {
            let model = primary_listview.model().expect("Listview has no SelectionModel")
                .downcast::<SingleSelection>()
                .expect("SelectionModel is not a SingleSelectionModel")
                .model()
                .expect("SelectionModel has no ListModel");
            let treelist_model = model.downcast::<TreeListModel>().expect("Model isn't a TreeListModel'");
            if let Some(child_row) = treelist_model.child_row(position) {
                child_row.set_expanded(true);
            }
        });

        self.primary_dirlist().connect_items_changed(clone!(@weak filefilter => move |_primary_dirlist, _position, _removed, _added| {
            filefilter.changed(FilterChange::Different);
            println!("cat kitty cat cat kitty cat cat");
        }));
        self.primary_dirlist().set_file(Some(&gio::File::for_path("/home/fried/")));
        println!("{:?}", self.imp().primary_dirlist.is_loading());
    }
}

