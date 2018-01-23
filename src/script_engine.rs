use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use rhai::{Engine, RegisterFn, Scope};

use regex::Regex;

// use sprite::Sprite;
use Entity;
use TileMap;
// use scene::Scene;

pub struct ScriptEngine {
    inner_engine: Engine,
    scope: Scope,
}

impl ScriptEngine {
    pub fn new() -> Self {
        let mut inner_engine = Engine::new();
        let scope = Scope::new();

        inner_engine.register_type::<Entity>();
        inner_engine.register_fn("mov", Entity::mov);
        inner_engine.register_fn("animation_step", Entity::animation_step);

        inner_engine.register_type::<TileMap>();

        ScriptEngine {
            inner_engine,
            scope,
        }
    }

    pub fn load_script(script: &str) -> String {
        let re = Regex::new(r".+(.rhai){1}").unwrap();

        if re.is_match(script) {
            let mut f = File::open(script).expect("script file not found");

            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("could not read script file");

            return contents;
        }

        String::from(script)
    }

    pub fn update(
        &mut self,
        vertical_direction: f64,
        horizontal_direction: f64,
        delta: f64,
        tile_map: &Option<TileMap>,
    ) {
        self.scope = Scope::new();
        self.scope.push((
            String::from("vertical_direction"),
            Box::new(vertical_direction),
        ));
        self.scope.push((
            String::from("horizontal_direction"),
            Box::new(horizontal_direction),
        ));
        self.scope.push((String::from("delta"), Box::new(delta)));
        if let Some(ref tile_map) = *tile_map {
            self.scope
                .push((String::from("map"), Box::new(tile_map.clone())));
        }
    }

    pub fn execute_script(&mut self, entity: &Entity) -> Entity {
        self.scope.push((entity.id(), Box::new(entity.clone())));

        let mut animation_step = 0.0;

        if let Some(ref sprite) = *entity.sprite().borrow() {
            animation_step = sprite.animation_step().get();
        }

        self.scope.push((
            String::from("animation_step"),
            Box::new(animation_step as f64),
        ));
        if let Ok(result) = self.inner_engine
            .eval_with_scope::<Entity>(&mut self.scope, &entity.script().borrow())
        {
            return result;
        }

        entity.clone()
    }
}
