use crux_core::{render::Render, App};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Event {
    Increment,
    Decrement,
    Reset,}

#[derive(Default)]
pub struct Model {
    inner: Bag,
    count: isize,
}

pub struct Bag {
    count: isize,
}

impl Default for Bag {
    fn default() -> Bag {
        Bag {
            count: -1,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ViewModel {
    pub count: String,
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
#[effect(app = "Planar")]
pub struct Capabilities {
    render: Render<Event>,
}

#[derive(Default)]
pub struct Planar;

impl App for Planar {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match event {
            Event::Increment => model.count += 2,
            Event::Decrement => model.count -= 3,
            Event::Reset => model.count = 0,
        };

        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            count: format!("Count is: {}", model.inner.count),
        }
    }
}
