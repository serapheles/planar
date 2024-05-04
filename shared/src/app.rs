use crux_core::{render::Render, App};
use crux_kv::{KeyValue, KeyValueOutput};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Event {
    Increment,
    Decrement,
    Reset,
    Initialize,
    Read,
    Write,
    Set(KeyValueOutput),
}

#[derive(Default)]
pub struct Model {
    value: i32,
    count: String,
}

#[derive(Serialize, Deserialize)]
pub struct ViewModel {
    pub result: String,
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
#[effect(app = "Planar")]
pub struct Capabilities {
    render: Render<Event>,
    key_value: KeyValue<Event>,
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
            Event::Increment => model.count += "+",
            Event::Decrement => model.count += "-",
            Event::Reset => model.count = "0".parse().unwrap(),
            Event::Initialize => {model.count = "C".parse().unwrap()}

            Event::Write => {
                caps.key_value
                    .write("test", 42i32.to_ne_bytes().to_vec(), Event::Set);
            }
            Event::Set(KeyValueOutput::Write(_success)) => {
                caps.render.render()
            }
            Event::Read => caps.key_value.read("test", Event::Set),
            Event::Set(KeyValueOutput::Read(value)) => {
                if let Some(value) = value {
                    // TODO: should KeyValueOutput::Read be generic over the value type?
                    let (int_bytes, _rest) = value.split_at(std::mem::size_of::<i32>());
                    model.value = i32::from_ne_bytes(int_bytes.try_into().unwrap());
                }
                caps.render.render()
            }

        };

        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            result: format!("Count is: {}", model.count),
        }
    }
}
