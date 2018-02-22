use std::sync::Arc;
use std::cell::RefCell;
use std::time;

//use fps_counter::FPSCounter;

use orbclient::WindowFlag;

use orbtk::{Rect, Widget, Window, WindowBuilder};
use orbtk::theme::Theme;

use super::{EventAction, Scene, SceneConfig, ScriptEngine};

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "Game")]
pub struct GameConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub target_fps: u32,
    pub theme: String,
    pub scenes: Vec<SceneConfig>,
}

pub struct Game {
    window: Window,
    target_fps: f64,
    target_fps_nanos: f32,
    script_engine: ScriptEngine,
    scenes: Vec<Arc<Scene>>,
    active_scene: RefCell<Arc<Scene>>,
    last_tick_time: time::Instant,
    actions: RefCell<Vec<EventAction>>,
    //fps_counter: FPSCounter,
}

impl Game {
    pub fn from_config(config: &GameConfig) -> Game {
        let theme = Theme::from_path(&config.theme).unwrap();

        let mut window_builder =
            WindowBuilder::new(Rect::new(0, 0, config.width, config.height), &config.title);
        window_builder = window_builder.flags(&[WindowFlag::Async]);
        window_builder = window_builder.theme(theme);
        let window = window_builder.build();

        let mut scenes = vec![];

        for scene in &config.scenes {
            let scene = Scene::from_config(scene);
            window.add(&scene);
            scenes.push(scene);
        }

        let active_scene = scenes[0].clone();
        active_scene.active().set(true);

        Game {
            window,
            target_fps: config.target_fps as f64,
            target_fps_nanos: (1. / config.target_fps as f32) * 1_000_000_000.,
            script_engine: ScriptEngine::new(),
            scenes,
            active_scene: RefCell::new(active_scene),
            last_tick_time: time::Instant::now(),
            actions: RefCell::new(vec![]),
            //fps_counter: FPSCounter::new(),
        }
    }

    pub fn elapsed(&self) -> f32 {
        let time = self.last_tick_time.elapsed();
        let total_nanos = time.as_secs() * 1_000_000_000 + time.subsec_nanos() as u64;
        self.target_fps_nanos - (total_nanos as f32)
    }

    pub fn show_scene(&self, id: &str, entity: &str, x: i32, y: i32) {
        if let Some(scene) = self.scenes.iter().find(|s| s.id() == id) {
            self.active_scene.borrow().active().set(false);
            *self.active_scene.borrow_mut() = scene.clone();
            self.active_scene.borrow().active().set(true);
            self.active_scene.borrow().place_entity(&entity, x, y);
        }
    }

    fn update(&mut self) {
        if self.elapsed() > 0. {
            return;
        }

        let delta = 1.0 / self.target_fps;

        self.active_scene
            .borrow()
            .update(&mut self.script_engine, delta, &self.actions);
        self.handle_actions();
        self.last_tick_time = time::Instant::now();
    }

    fn handle_actions(&self) {
        for action in self.actions.borrow_mut().pop() {
            match action {
                EventAction::SwitchScene { id, entity, x, y } => {
                    self.show_scene(&id, &entity, x, y);
                }
                _ => {}
            }
        }
    }

    pub fn exec(&mut self) {
        'event: while self.window.running.get() {
            self.window.drain_events();
            self.update();
            self.window.draw();
            self.window.drain_orbital_events();
        }
    }
}
