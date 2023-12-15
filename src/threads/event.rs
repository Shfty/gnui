use crossbeam_channel::Receiver;
use crossterm::event::Event;

pub type CrosstermEventReceiver = Receiver<Event>;

pub fn event_thread() -> CrosstermEventReceiver {
    let (event_tx, event_rx) = crossbeam_channel::unbounded();

    std::thread::spawn(move || loop {
        match crossterm::event::read() {
            Ok(event) => event_tx.send(event).unwrap(),
            Err(e) => panic!("{e:}"),
        }
    });

    event_rx
}
