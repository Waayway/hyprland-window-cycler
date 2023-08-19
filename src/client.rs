use zbus::Connection;

use crate::dbus::{Message, ServerProxy};



pub async fn main(oth_args: Vec<String>) {
    let connection = match Connection::session().await {
        Ok(connection) => connection,
        Err(err) => {
            println!("Failed to connect to session bus: {}", err);
            return;
        }
    };
    let proxy = match ServerProxy::new(&connection).await {
        Ok(proxy) => proxy,
        Err(err) => {
            println!("Failed to create proxy: {}", err);
            return;
        }
    };
    let msg = match oth_args.first() {
        Some(msg) => msg,
        None => {
            println!("No message specified");
            return;
        }
    };
    match proxy.message(msg).await {
        Ok(ok) => match ok {
            true => println!("Message sent"),
            false => println!("Incorrect command: {}", msg),
        } 
        Err(_) => {
            println!("Failed to send message");
        },
    };
}