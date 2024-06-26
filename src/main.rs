use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use futures::{FutureExt, StreamExt};
use tokio::{sync::mpsc, task::JoinHandle};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Error,
    Tick,
    Key(KeyEvent),
}

#[derive(Debug)]
pub struct EventHandler {
    _tx: mpsc::UnboundedSender<Event>,
    rx: mpsc::UnboundedReceiver<Event>,
    task: Option<JoinHandle<()>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // setup terminal
    // startup()?;
    //
    // let result = run();
    //
    // // teardown terminal before unwrapping Result of app run
    // shutdown()?;
    //
    // result?;

    Ok(())
}
