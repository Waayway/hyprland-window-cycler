use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk::cairo::FontOptions;
use gtk::ffi::gtk_icon_theme_choose_icon;
use gtk::{ gdk, ApplicationWindow, traits::WidgetExt };
use gtk::glib::{ clone };

use hyprland::dispatch::{
    Dispatch,
    DispatchType,
    WorkspaceIdentifierWithSpecial,
    WindowIdentifier,
};
use hyprland::prelude::*;
use gtk::{ prelude::*, IconLookupFlags };
use gtk::glib;

use crate::current_windows::WindowGroup;
use crate::dbus::Message;

#[derive(Clone)]
pub struct CycleWindow {
    pub window: ApplicationWindow,
    pub display: gdk::Display,
    pub monitor: gdk::Monitor,
    timeout_id: Rc<RefCell<Option<glib::SourceId>>>,
    containers: Rc<RefCell<Vec<gtk::Box>>>,
    main_container: gtk::Box,
    pub icon_theme: gtk::IconTheme,
}

impl CycleWindow {
    pub fn new(app: &gtk::Application, display: &gdk::Display, monitor: &gdk::Monitor) -> Self {
        let window = gtk::ApplicationWindow
            ::builder()
            .application(app)
            .default_width(500)
            .default_height(300)
            .title("Cycle Window")
            .build();
        window.style_context().add_class(&gtk::STYLE_CLASS_OSD.to_string());

        gtk_layer_shell::init_for_window(&window);
        gtk_layer_shell::set_monitor(&window, monitor);
        gtk_layer_shell::set_namespace(&window, "window-cycler");

        gtk_layer_shell::set_exclusive_zone(&window, -1);
        gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);

        window.connect_map(
            clone!(@strong monitor => move |win| {
            let bottom = monitor.workarea().height() - win.allocated_height();
            let margin = (bottom as f32 * 0.5).round() as i32;
            gtk_layer_shell::set_margin(win, gtk_layer_shell::Edge::Top, margin);
        })
        );

        let icon_theme = match gtk::IconTheme::default() {
            Some(icon_theme) => icon_theme,
            None => { gtk::IconTheme::new() }
        };

        Self {
            main_container: Self::create_initial_widgets(&window),
            timeout_id: Rc::new(RefCell::new(None)),
            containers: Rc::new(RefCell::new(Vec::new())),
            display: display.clone(),
            monitor: monitor.clone(),
            window,
            icon_theme,
        }
    }

    fn create_initial_widgets(window: &ApplicationWindow) -> gtk::Box {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        container.set_margin(10);
        window.add(&container);
        container
    }

    fn create_window_widgets(&self) {
        let group_of_window_groups = WindowGroup::get_list_of_window_groups(&self);
        for group in group_of_window_groups {
            let container = gtk::Box::new(gtk::Orientation::Vertical, 10);
            container.set_margin(10);

            let image = match group.icon {
                Some(icon) => gtk::Image::from_pixbuf(Some(&icon)),
                None => gtk::Image::new(),
            };
            let label = gtk::Label
                ::builder()
                .label(format!("<span size='large'>{}</span>", &group.title))
                .use_markup(true)
                .build();

            let client_container = gtk::Box::new(gtk::Orientation::Vertical, 10);
            client_container.set_margin(10);

            for client in group.clients {
                let title = format!(
                    "{}",
                    &client.title
                        [..(if client.title.len() > 20 { 20 } else { client.title.len() })]
                );
                let client_btn = gtk::Button::new();
                let client_label = gtk::Label
                    ::builder()
                    .label(format!("{}", title))
                    .use_markup(true)
                    .build();
                client_btn.add(&client_label);
                client_container.add(&client_btn);
            }

            container.add(&label);
            container.add(&image);
            container.add(&client_container);

            self.main_container.add(&container);
        }
    }

    fn clear_container(&self) {
        for widget in self.main_container.children() {
            self.main_container.remove(&widget);
        }
    }

    pub fn open_window(&self, msg: Message) {
        self.clear_container();
        self.create_window_widgets();
        self.run_timeout();
    }

    pub fn run_timeout(&self) {
        // Hide window after timeout
        if let Some(timeout_id) = self.timeout_id.take() {
            timeout_id.remove();
        }
        let s = self.clone();
        self.timeout_id.replace(
            Some(
                glib::timeout_add_local_once(Duration::from_millis(3000), move || {
                    s.window.hide();
                    s.timeout_id.replace(None);
                })
            )
        );

        self.window.show_all();
    }
}
