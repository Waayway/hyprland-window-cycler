use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::{ clone, OptionFlags };

use gtk::{Application, gdk};
use gtk::prelude::*;
use gtk::glib::{MainContext, Priority, Receiver};

use crate::dbus::{Message, ServerBus};
use crate::window::CycleWindow;


pub fn main() {
    let (sender, receiver) = MainContext::channel::<Message>(Priority::default());
    
    async_std::task::spawn(ServerBus::new(sender));
    
    let mut cycw_app = CycleWindowApplication::new(receiver);
        
    cycw_app.start();
}

#[derive(Clone)]
pub struct CycleWindowApplication {
    app: Application,
    windows: Rc<RefCell<Vec<CycleWindow>>>,
}


impl CycleWindowApplication {
    pub fn new(receiver: Receiver<Message>) -> Self {
        let app = Application::builder()
            .application_id("org.waayway.cycle-window-hyprland")
            .build();

        app.add_main_option("-daemon", gtk::glib::Char::try_from('d').unwrap(), OptionFlags::empty(), gtk::glib::OptionArg::None, "Default mode, needed to resolve error", None);

        let cycw_app = Self {
            app,
            windows: Rc::new(RefCell::new(Vec::new())),
        };

        receiver.attach(
			None,
			clone!(@strong cycw_app => @default-return Continue(false), move |msg| {
				Self::action_activated(&cycw_app, msg);
				Continue(true)
			}),
		);

        cycw_app
    }
    pub fn initialize(&self) {
        let display: gdk::Display = match gdk::Display::default() {
            Some(display) => display,
            None => {
                println!("No display found");
                return;
            }
        };
        for i in 0..display.n_monitors() {
            let monitor = display.monitor(i).unwrap();
            let window = CycleWindow::new(&self.app, &display, &monitor);
            self.windows.borrow_mut().push(window);
        }
    }

    pub fn start(&mut self) {
        gtk::init().expect("Failed to initialize GTK.");
        let s = self.clone();
        self.app.connect_activate(move |_| {
            s.initialize();
        });
        self.app.run();
    }
    pub fn action_activated(cycw_app: &Self, msg: Message) {
        if msg == Message::Invalid {
            return;
        }
        cycw_app.windows.borrow_mut().iter_mut().for_each(move |window| {
            window.open_window(msg.clone());
        });
    }
}