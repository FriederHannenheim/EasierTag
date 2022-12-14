use gettextrs::gettext;
use log::{debug, info};

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};

use adw::subclass::prelude::*;

use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::window::EasierTagApplicationWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct EasierTagApplication {
        pub window: OnceCell<WeakRef<EasierTagApplicationWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EasierTagApplication {
        const NAME: &'static str = "EasierTagApplication";
        type Type = super::EasierTagApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for EasierTagApplication {}

    impl ApplicationImpl for EasierTagApplication {
        fn activate(&self) {
            debug!("AdwApplication<EasierTagApplication>::activate");
            self.parent_activate();
            let app = self.instance();

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = EasierTagApplicationWindow::new(&*app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();

            window.init();
        }

        fn startup(&self) {
            debug!("GtkApplication<EasierTagApplication>::startup");
            self.parent_startup();
            let app = self.instance();

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for EasierTagApplication {}

    impl AdwApplicationImpl for EasierTagApplication {}
}

glib::wrapper! {
    pub struct EasierTagApplication(ObjectSubclass<imp::EasierTagApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl EasierTagApplication {
    fn main_window(&self) -> EasierTagApplicationWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| {
                // This is needed to trigger the delete event and saving the window state
                app.main_window().close();
                app.quit();
            })
            .build();

        // About
        let action_about = gio::ActionEntry::builder("about")
            .activate(|app: &Self, _, _| {
                app.show_about_dialog();
            })
            .build();
        self.add_action_entries([action_quit, action_about])
            .unwrap();
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
        self.set_accels_for_action("window.close", &["<Control>w"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/net/fhannenheim/EasierTag/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn show_about_dialog(&self) {
        let dialog = gtk::AboutDialog::builder()
            .logo_icon_name(APP_ID)
            // Insert your license of choice here
            // .license_type(gtk::License::MitX11)
            // Insert your website here
            // .website("https://gitlab.gnome.org/bilelmoussaoui/easier-tag/")
            .version(VERSION)
            .transient_for(&self.main_window())
            .translator_credits(&gettext("translator-credits"))
            .modal(true)
            .authors(vec!["Frieder Hannenheim".into()])
            .artists(vec!["Frieder Hannenheim".into()])
            .build();

        dialog.present();
    }

    pub fn run(&self) {
        info!("EasierTag ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self);
    }
}

impl Default for EasierTagApplication {
    fn default() -> Self {
        glib::Object::new::<Self>(&[
            ("application-id", &APP_ID),
            ("flags", &gio::ApplicationFlags::empty()),
            ("resource-base-path", &"/net/fhannenheim/EasierTag/"),
        ])
    }
}
