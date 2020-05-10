//! Main menu state

use amethyst::prelude::*;
use amethyst::winit::{ElementState, Event, MouseButton, WindowEvent};
use log::*;

use crate::states::Quickplay;

pub struct MainMenu;

impl SimpleState for MainMenu {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        info!("Entered main menu");
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(Event::WindowEvent {
            event:
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                },
            ..
        }) = event
        {
            Trans::Switch(Box::new(Quickplay))
        } else {
            Trans::None
        }
    }
}
