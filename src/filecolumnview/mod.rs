use crate::taggablefile::TaggableFile;
use crate::taggablefile::taggablefilelist::TaggableFileListModel;
use gtk::{
    gio, glib, glib::closure, prelude::*, subclass::prelude::*, ColumnView, ColumnViewColumn,
    CompositeTemplate, ConstantExpression, Label, ListItem, MultiSelection, PropertyExpression,
    SignalListItemFactory, Widget,
};

mod imp {
    use super::*;

    #[derive(Debug, CompositeTemplate, Default)]
    #[template(resource = "/net/fhannenheim/EasierTag/ui/filecolumnview.ui")]
    pub struct FileColumnView {
        #[template_child]
        pub file_column_view: TemplateChild<ColumnView>,
        #[template_child]
        pub filename_column: TemplateChild<ColumnViewColumn>,
        #[template_child]
        pub title_column: TemplateChild<ColumnViewColumn>,
        #[template_child]
        pub artist_column: TemplateChild<ColumnViewColumn>,
        #[template_child]
        pub album_column: TemplateChild<ColumnViewColumn>,
        #[template_child]
        pub year_column: TemplateChild<ColumnViewColumn>,
        #[template_child]
        pub disc_column: TemplateChild<ColumnViewColumn>,
        #[template_child]
        pub track_column: TemplateChild<ColumnViewColumn>,
        #[template_child]
        pub genre_column: TemplateChild<ColumnViewColumn>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FileColumnView {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "FileColumnView";
        type Type = super::FileColumnView;
        type ParentType = Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FileColumnView {
        fn constructed(&self) {
            self.parent_constructed();
        }
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for FileColumnView {}
}

glib::wrapper! {
    pub struct FileColumnView(ObjectSubclass<imp::FileColumnView>)
        @extends Widget;
}

impl FileColumnView {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
    pub fn column_view(&self) -> ColumnView {
        self.imp().file_column_view.clone()
    }
    pub fn columns(&self) -> Vec<ColumnViewColumn> {
        vec![
            self.imp().filename_column.clone(),
            self.imp().title_column.clone(),
            self.imp().artist_column.clone(),
            self.imp().album_column.clone(),
            self.imp().year_column.clone(),
            self.imp().disc_column.clone(),
            self.imp().track_column.clone(),
            self.imp().genre_column.clone(),
        ]
    }
    pub fn init(&self) {
        for column in self.columns() {
            let column_list_factory = SignalListItemFactory::new();

            let column_title = column.title().unwrap_or(String::from("").into());
            column_list_factory.connect_setup(move |_, list_item| {
                let label = Label::new(Some(""));
                list_item.set_child(Some(&label));

                let list_item_expr = ConstantExpression::new(list_item);
                let taggablefile_expr =
                    PropertyExpression::new(ListItem::static_type(), Some(&list_item_expr), "item");

                let _column_title = column_title.clone();
                let label_expr =
                    taggablefile_expr.chain_closure::<String>(closure!(|_: Option<
                        glib::Object,
                    >,
                                                                        taggable_file: Option<
                        glib::Object,
                    >| {
                        if let Some(taggable_file) = taggable_file {
                            if let Ok(taggable_file) = taggable_file.downcast::<TaggableFile>() {
                                return match _column_title.as_str() {
                                    "Filename" => taggable_file.property::<String>("filename"),
                                    "Title" => taggable_file.property::<String>("title"),
                                    "Artist" => {
                                        taggable_file.property::<Vec<String>>("artists").join(", ")
                                    }
                                    "Album" => taggable_file.property::<String>("album"),
                                    "Year" => taggable_file.property::<String>("year"),
                                    "Disc" => taggable_file.property::<String>("disc"),
                                    "Track" => taggable_file.property::<String>("track"),
                                    "Genre" => taggable_file.property::<String>("genre"),
                                    &_ => String::new(),
                                };
                            }
                        }
                        String::new()
                    }));
                label_expr.bind(&label, "label", Widget::NONE);
            });
            column.set_factory(Some(&column_list_factory));
        }
        let model = TaggableFileListModel::new();

        let multi_selection = MultiSelection::new(Some(&model));
        self.column_view().set_model(Some(&multi_selection));
    }
}

impl Default for FileColumnView {
    fn default() -> Self {
        Self::new()
    }
}
