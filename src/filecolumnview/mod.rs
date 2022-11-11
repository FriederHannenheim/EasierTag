use gtk::{
    glib, gio, subclass::prelude::*, prelude::*, ColumnView, ColumnViewColumn,
    Widget, CompositeTemplate, ConstantExpression,
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
}

impl Default for FileColumnView {
    fn default() -> Self {
        Self::new()
    }
}
