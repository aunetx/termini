use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::application::TerminiApplication;
use crate::config::{APP_ID, PROFILE};

mod imp {
    use super::*;

    use gtk::CompositeTemplate;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/com/github/aunetx/Termini/ui/window.ui")]
    pub struct TerminiApplicationWindow {
        #[template_child]
        pub headerbar: TemplateChild<gtk::HeaderBar>,
        pub settings: gio::Settings,
    }

    impl Default for TerminiApplicationWindow {
        fn default() -> Self {
            Self {
                headerbar: TemplateChild::default(),
                settings: gio::Settings::new(APP_ID),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TerminiApplicationWindow {
        const NAME: &'static str = "TerminiApplicationWindow";
        type Type = super::TerminiApplicationWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TerminiApplicationWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            // Devel Profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            // Load latest window state
            obj.load_window_size();
        }
    }

    impl WidgetImpl for TerminiApplicationWindow {}
    impl WindowImpl for TerminiApplicationWindow {
        // Save window state on delete event
        fn close_request(&self, window: &Self::Type) -> gtk::Inhibit {
            if let Err(err) = window.save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }

            // Pass close request on to the parent
            self.parent_close_request(window)
        }
    }

    impl ApplicationWindowImpl for TerminiApplicationWindow {}
}

glib::wrapper! {
    pub struct TerminiApplicationWindow(ObjectSubclass<imp::TerminiApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl TerminiApplicationWindow {
    pub fn new(app: &TerminiApplication) -> Self {
        glib::Object::new(&[("application", app)])
            .expect("Failed to create TerminiApplicationWindow")
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let self_ = imp::TerminiApplicationWindow::from_instance(self);

        let (width, height) = self.default_size();

        self_.settings.set_int("window-width", width)?;
        self_.settings.set_int("window-height", height)?;

        self_
            .settings
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let self_ = imp::TerminiApplicationWindow::from_instance(self);

        let width = self_.settings.int("window-width");
        let height = self_.settings.int("window-height");
        let is_maximized = self_.settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}
