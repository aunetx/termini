use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};
use log::{debug, info};

use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::window::TerminiApplicationWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct TerminiApplication {
        pub window: OnceCell<WeakRef<TerminiApplicationWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TerminiApplication {
        const NAME: &'static str = "TerminiApplication";
        type Type = super::TerminiApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for TerminiApplication {}

    impl ApplicationImpl for TerminiApplication {
        fn activate(&self, app: &Self::Type) {
            debug!("GtkApplication<TerminiApplication>::activate");

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.show();
                window.present();
                return;
            }

            let window = TerminiApplicationWindow::new(app);
            self.window
                .set(window.downgrade())
                .expect("Window already set");

            app.main_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("GtkApplication<TerminiApplication>::startup");
            self.parent_startup(app);

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for TerminiApplication {}
}

glib::wrapper! {
    pub struct TerminiApplication(ObjectSubclass<imp::TerminiApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl TerminiApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(APP_ID)),
            ("flags", &gio::ApplicationFlags::empty()),
            ("resource-base-path", &Some("/com/github/aunetx/Termini/")),
        ])
        .expect("Application initialization failed...")
    }

    fn main_window(&self) -> TerminiApplicationWindow {
        let imp = imp::TerminiApplication::from_instance(self);
        imp.window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak self as app => move |_, _| {
            // This is needed to trigger the delete event and saving the window state
            app.main_window().close();
            app.quit();
        }));
        self.add_action(&action_quit);

        // About
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about_dialog();
        }));
        self.add_action(&action_about);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<primary>q"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/com/github/aunetx/Termini/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn show_about_dialog(&self) {
        let dialog = gtk::AboutDialogBuilder::new()
            .program_name("Termini")
            .logo_icon_name(APP_ID)
            .license_type(gtk::License::MitX11)
            .website("https://github.com/aunetx/termini/")
            .version(VERSION)
            .transient_for(&self.main_window())
            .modal(true)
            .authors(vec!["Aurélien Hamy".into()])
            .artists(vec!["Aurélien Hamy".into()])
            .build();

        dialog.show();
    }

    pub fn run(&self) {
        info!("Termini ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self);
    }
}
