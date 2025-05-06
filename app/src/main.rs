use std::{io};
use crate::application::Application;

mod application;

fn main() -> io::Result<()> {
    main_impl()
}

#[tokio::main]
async fn main_impl() -> io::Result<()> {
    let mut app = Application::new();
    app.run().await;
    Ok(())


}

