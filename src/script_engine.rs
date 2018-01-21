use rhai::{Engine, RegisterFn, Scope};

use sprite::Sprite;
use entity::Entity;
use scene::Scene;

pub struct ScriptEngine {
    inner_engine: Engine,
    scope: Scope,
}

impl ScriptEngine {
    pub fn new() -> Self {
        let mut inner_engine = Engine::new();
        let scope = Scope::new();

        inner_engine.register_type::<Sprite>();
        inner_engine.register_type::<Entity>();
        inner_engine.register_type::<Scene>();

        ScriptEngine {
            inner_engine,
            scope,
        }
    }

    pub fn update(&mut self) {
        let result = self.inner_engine
            .eval_with_scope::<i32>(&mut self.scope, "4 + 4");
        if let Ok(result) = result {
            println!("{}", result);
        }
    }
}
