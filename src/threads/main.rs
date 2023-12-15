use std::{cell::RefCell, rc::Rc};

use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::CrosstermBackend;

use super::{event::CrosstermEventReceiver, input::InputReceiver};
use crate::{backend::Backend, Result};

pub type Terminal = tui::Terminal<CrosstermBackend<Backend>>;
pub type Frame<'a> = tui::Frame<'a, CrosstermBackend<Backend>>;

pub type InputBuffer = Rc<RefCell<String>>;

pub trait FnDraw: FnMut(&mut Frame) {}
impl<T> FnDraw for T where T: FnMut(&mut Frame) {}

pub fn main_thread(
    terminal: &mut Terminal,
    stdin_rx: InputReceiver,
    event_rx: CrosstermEventReceiver,
    buf: InputBuffer,
    draw: impl FnDraw,
) -> Result {
    main_initialize(terminal)?;
    main_loop(terminal, stdin_rx, event_rx, buf, draw)?;
    main_finalize(terminal)?;
    Ok(())
}

fn main_initialize(terminal: &mut Terminal) -> Result {
    terminal.hide_cursor()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen,)?;
    enable_raw_mode()?;
    Ok(())
}

fn main_loop(
    terminal: &mut Terminal,
    stdin_rx: InputReceiver,
    event_rx: CrosstermEventReceiver,
    buf: InputBuffer,
    mut draw: impl FnDraw,
) -> Result {
    loop {
        terminal.draw(|f| draw(f))?;

        crossbeam_channel::select! {
            recv(stdin_rx) -> msg => {
                *buf.borrow_mut() = msg.unwrap();
            },
            recv(event_rx) -> msg => {
                match msg.unwrap() {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL
                    }) => {
                        break Ok(())
                    }
                    _ => ()
                }
            },
        };
    }
}

fn main_finalize(terminal: &mut Terminal) -> Result {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;
    Ok(())
}
