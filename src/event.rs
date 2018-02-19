use std::cell::{Cell, RefCell};

use orbtk::Rect;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum EventCondition {
    Enter
}

impl Default for EventCondition {
    fn default() -> Self {
        EventCondition::Enter
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum EventAction {
    None,
    SwitchScene(String)
}

impl Default for EventAction {
    fn default() -> Self {
        EventAction::None
    }
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename = "Event")]
pub struct EventConfig {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub entity: String,
    pub condition: EventCondition,
    pub action: EventAction,
}

#[derive(Clone)]
pub struct Event {
    rect: Cell<Rect>,
    entity: RefCell<String>,
    condition: RefCell<EventCondition>,
    action: RefCell<EventAction>
}

impl Event {
    pub fn from_config(config: &EventConfig) -> Self {
        Event {
            rect: Cell::new(Rect::new(config.x, config.y, config.width, config.height)),
            entity: RefCell::new(config.entity.clone()),
            condition: RefCell::new(config.condition.clone()),
            action: RefCell::new(config.action.clone()),
        }
    }
    pub fn rect(&self) -> &Cell<Rect> {
        &self.rect
    }

    pub fn entity(&self) -> &RefCell<String> {
        &self.entity
    }

    pub fn condition(&self) -> &RefCell<EventCondition> {
        &self.condition
    }

    pub fn action(&self) -> &RefCell<EventAction> {
        &self.action
    }
}