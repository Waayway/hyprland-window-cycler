use std::future::pending;

use gtk::glib::Sender;
use zbus::{dbus_proxy, dbus_interface, ConnectionBuilder};


pub struct ServerBus {
    sender: Sender<Message>
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, zbus::zvariant::Type, PartialEq, Eq)]
pub enum Message {
    Prev,
    Next,
    Invalid
}
impl ServerBus {
    pub async fn new(sender: Sender<Message>) -> zbus::Result<()> {
		let _connection = ConnectionBuilder::session()?
			.name("org.waayway.windowcyclerserver")?
			.serve_at("/org/waayway/windowcycler", Self { sender })?
			.build()
			.await?;
		pending::<()>().await;
		Ok(())
	}
}

#[dbus_interface(name = "org.waayway.windowcycler")]
impl ServerBus {
    pub fn message(&self, message: &str) -> bool {
        let msg = match message {
            "prev" => Message::Prev,
            "next" => Message::Next,
            _ => Message::Invalid
        };
        let ok = self.sender.send(msg.clone()).is_ok();
        ok && msg != Message::Invalid
    }
}

#[dbus_proxy(
    interface = "org.waayway.windowcycler",
    default_service = "org.waayway.windowcyclerserver",
    default_path = "/org/waayway/windowcycler"
)]
trait Server {
    async fn message(&self, message: &str) -> zbus::Result<bool>;
}