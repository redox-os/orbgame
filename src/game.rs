use std::sync::Arc;
use std::cell::RefCell;
use std::time;

use orbtk::{Widget, Window, Label, Text};
use fps_counter::FPSCounter;

pub trait UpdatableWidget: Widget {
    fn update(&self);
}

pub struct Game {
    window: Window,
    updateable_widgets: RefCell<Vec<Arc<UpdatableWidget>>>,
    last_tick_time: time::Instant,
    fps_in_nanos: f32,
    fps_counter: FPSCounter,
    fps_label: RefCell<Option<Arc<Label>>>,
}

impl Game {
    pub fn new(window: Window) -> Self {
        Game {
            window,
            updateable_widgets: RefCell::new(Vec::new()),
            last_tick_time: time::Instant::now(),
            fps_in_nanos: (1. / 60.) * 1_000_000_000.,
            fps_counter: FPSCounter::new(),
            fps_label: RefCell::new(None),
        }
    }

    pub fn elapsed(&self) -> f32 {
        let time = self.last_tick_time.elapsed();
        let total_nanos = time.as_secs() * 1_000_000_000 + time.subsec_nanos() as u64;
        self.fps_in_nanos - (total_nanos as f32)
    }

    pub fn update(&mut self) {
        if self.elapsed() > 0. {
            return
        }

        self.last_tick_time = time::Instant::now();

        for i in 0..self.updateable_widgets.borrow().len() {
            if let Some(widget) = self.updateable_widgets.borrow().get(i) {
                widget.update();
            }
        }
    }

    pub fn add<T: UpdatableWidget>(&self, widget: &Arc<T>) -> usize {
        let mut widgets = self.updateable_widgets.borrow_mut();
        let id = widgets.len();
        widgets.push(widget.clone());
        id
    }


    pub fn fps_label(&self, fps_label: &Arc<Label>) -> &Self {
        (*self.fps_label.borrow_mut()) = Some(fps_label.clone());
        self
    }

    pub fn exec(&mut self) {
        'event: while self.window.running.get() {
            self.window.drain_events();
            self.update();
            self.window.draw();
            self.window.drain_orbital_events();
         
            if let Some(ref label) = *self.fps_label.borrow() {
                label.text(format!("{}", self.fps_counter.tick()));
            }      
        }
    }
}
