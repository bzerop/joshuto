use termion::event::{Event, Key};

use tui::layout::Rect;

use crate::context::AppContext;
use crate::ui::widgets::{TuiTopBar, TuiWorker};
use crate::ui::TuiBackend;
use crate::util::event::AppEvent;
use crate::util::input;

pub struct TuiWorkerView {}

impl TuiWorkerView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn display(&self, context: &mut AppContext, backend: &mut TuiBackend) {
        let terminal = backend.terminal_mut();

        loop {
            let _ = terminal.draw(|frame| {
                let area: Rect = frame.size();
                if area.height == 0 {
                    return;
                }

                let rect = Rect { height: 1, ..area };
                let curr_tab = context.tab_context_ref().curr_tab_ref();
                let view = TuiTopBar::new(context, curr_tab.cwd());
                frame.render_widget(view, rect);

                let rect = Rect {
                    x: 0,
                    y: 1,
                    width: area.width,
                    height: area.height - 1,
                };
                let view = TuiWorker::new(&context);
                frame.render_widget(view, rect);
            });

            if let Ok(event) = context.poll_event() {
                match event {
                    AppEvent::Termion(event) => {
                        if let Event::Key(Key::Esc) = event {
                            break;
                        }
                        context.flush_event();
                    }
                    event => input::process_noninteractive(event, context),
                };
            }
        }
    }
}
