// ui/events.rs
use crossterm::event::{Event, EventStream, KeyCode};
use futures::StreamExt;
use tokio::sync::mpsc;

pub enum AppEvent {
    Tick,
    Key(crossterm::event::KeyEvent),
    Network(NetworkEvent),
    Message(MessageEvent),
}

pub async fn event_loop(tx: mpsc::Sender<AppEvent>) {
    let mut reader = EventStream::new();
    let mut tick = tokio::time::interval(Duration::from_millis(250));

    loop {
        tokio::select! {
            _ = tick.tick() => { tx.send(AppEvent::Tick).await.ok(); }
            Some(Ok(event)) = reader.next() => {
                if let Event::Key(key) = event {
                    tx.send(AppEvent::Key(key)).await.ok();
                }
            }
        }
    }
}
