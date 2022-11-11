use gtk::{
    glib, prelude::*, subclass::prelude::*, CompositeTemplate, Image, Label, TreeExpander, Widget,
};

mod imp {
    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/net/fhannenheim/EasierTag/ui/folderitem.ui")]
    pub struct FolderItem {
        #[template_child]
        pub expander: TemplateChild<TreeExpander>,
        #[template_child]
        pub icon: TemplateChild<Image>,
        #[template_child]
        pub label: TemplateChild<Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FolderItem {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "FolderItem";
        type Type = super::FolderItem;
        type ParentType = Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FolderItem {
        fn constructed(&self) {
            self.parent_constructed();
        }
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for FolderItem {}
}

glib::wrapper! {
    pub struct FolderItem(ObjectSubclass<imp::FolderItem>)
        @extends Widget;
}

impl FolderItem {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn tree_expander(&self) -> TreeExpander {
        self.imp().expander.clone()
    }

    pub fn file_image(&self) -> Image {
        self.imp().icon.clone()
    }

    pub fn file_label(&self) -> Label {
        self.imp().label.clone()
    }
}

impl Default for FolderItem {
    fn default() -> Self {
        Self::new()
    }
}
