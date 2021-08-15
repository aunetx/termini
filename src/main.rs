mod application;
#[rustfmt::skip]
mod config;
mod error;
mod window;

use application::TerminiApplication;
use config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};
use error::*;
use gettextrs::{gettext, LocaleCategory};
use gtk::{gio, glib};

fn main() -> Result<()> {
    // Initialize logger
    pretty_env_logger::init();
    init_i18n()?;
    init_app()?;

    glib::set_application_name(&gettext("Termini"));

    let app = TerminiApplication::new();
    app.run();

    Ok(())
}

fn init_i18n() -> Result<()> {
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR)?;
    gettextrs::textdomain(GETTEXT_PACKAGE)?;

    Ok(())
}

fn init_app() -> Result<()> {
    gtk::init()?;
    gio::resources_register(&gio::Resource::load(RESOURCES_FILE)?);

    Ok(())
}
