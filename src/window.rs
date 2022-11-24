use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::application::EasierTagApplication;
use crate::config::{APP_ID, PROFILE};
use crate::filecolumnview::FileColumnView;
use crate::folderbrowser::FolderBrowser;

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/net/fhannenheim/EasierTag/ui/window.ui")]
    pub struct EasierTagApplicationWindow {
        #[template_child]
        pub folderbrowser: TemplateChild<FolderBrowser>,
        #[template_child]
        pub filecolumnview: TemplateChild<FileColumnView>,
        pub settings: gio::Settings,
    }

    impl Default for EasierTagApplicationWindow {
        fn default() -> Self {
            Self {
                folderbrowser: TemplateChild::default(),
                filecolumnview: TemplateChild::default(),
                settings: gio::Settings::new(APP_ID),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EasierTagApplicationWindow {
        const NAME: &'static str = "ExampleApplicationWindow";
        type Type = super::EasierTagApplicationWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for EasierTagApplicationWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.instance();

            // Devel Profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            // Load latest window state
            obj.load_window_size();
        }
    }

    impl WidgetImpl for EasierTagApplicationWindow {}
    impl WindowImpl for EasierTagApplicationWindow {
        // Save window state on delete event
        fn close_request(&self) -> gtk::Inhibit {
            if let Err(err) = self.instance().save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }

            // Pass close request on to the parent
            self.parent_close_request()
        }
    }

    impl ApplicationWindowImpl for EasierTagApplicationWindow {}
}

glib::wrapper! {
    pub struct EasierTagApplicationWindow(ObjectSubclass<imp::EasierTagApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

impl EasierTagApplicationWindow {
    pub fn new(app: &EasierTagApplication) -> Self {
        glib::Object::new(&[("application", app)])
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();

        let (width, height) = self.default_size();

        imp.settings.set_int("window-width", width)?;
        imp.settings.set_int("window-height", height)?;

        imp.settings
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let imp = self.imp();

        let width = imp.settings.int("window-width");
        let height = imp.settings.int("window-height");
        let is_maximized = imp.settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }

    pub fn filecolumnview(&self) -> FileColumnView {
        self.imp().filecolumnview.clone()
    }

    pub fn init(&self) {
        self.imp().folderbrowser.init(self);
        self.imp().filecolumnview.init();
    }
}
