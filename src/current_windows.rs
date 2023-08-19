use gtk::gdk_pixbuf::Pixbuf;
use gtk::traits::IconThemeExt;
use hyprland::data::Client;

use hyprland::prelude::*;

use crate::window::CycleWindow;

pub struct WindowGroup {
    pub title: String,
    pub icon: Option<Pixbuf>,
    pub clients: Vec<Client>,
}

impl WindowGroup {
    pub fn get_list_of_window_groups(cycw_win: &CycleWindow) -> Vec<Self> {
        let clients = hyprland::data::Clients::get().expect("Hyprland doesn't seem to be working?");
        let mut window_groups: Vec<Self> = Vec::new();
        for client in clients {
            let mut found = false;
            for window_group in window_groups.iter_mut() {
                if window_group.title == client.initial_class {
                    window_group.clients.push(client.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                let icon = match cycw_win.icon_theme.load_icon(
                    &client.initial_class.to_lowercase(),
                    64,
                    gtk::IconLookupFlags::empty()
                ) {
                    Ok(icon) => icon,
                    Err(_) => {
                        println!("No icon found for {:?}", &client.initial_class.to_lowercase());
                        None
                    }
                };
                window_groups.push(Self {
                    title: client.clone().initial_class,
                    icon: icon,
                    clients: vec![client.clone()],
                });
            }
        }
        window_groups
    }
}
