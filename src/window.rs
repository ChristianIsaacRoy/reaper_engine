extern crate winit;

use winit::Event as WinitEvent;
use winit::Window as WinitWindow;

use crate::event;

pub struct WindowProps {
    title: String,
    width: f64,
    height: f64,
}

impl WindowProps {
    pub fn defaults() -> Self {
        WindowProps {
            title: String::from("Reaper Engine"),
            width: 1280.0,
            height: 720.0,
        }
    }
}

pub struct Window {
    winit_events_loop: winit::EventsLoop,
    winit_window: WinitWindow,
}

impl Window {
    pub fn new(props: &WindowProps) -> Self {
        let winit_events_loop = winit::EventsLoop::new();
        let dpi = winit_events_loop.get_primary_monitor().get_hidpi_factor();
        let winit_window = winit::WindowBuilder::new()
            .with_dimensions(winit::dpi::LogicalSize::from_physical(
                winit::dpi::PhysicalSize::new(props.width as _, props.height as _),
                dpi,
            ))
            .with_title(&props.title)
            .build(&winit_events_loop)
            .unwrap();

        info!(
            "Creating a window {} ({} {})",
            &props.title, props.width, props.height
        );

        Window {
            winit_events_loop,
            winit_window,
        }
    }

    pub fn on_update(&mut self, event_dispatcher: &mut event::Dispatcher) {
        use event::{Event, EventType};
        use winit::WindowEvent as WinitWindowEventType;
        use WinitEvent::DeviceEvent as WinitDeviceEvent;
        use WinitEvent::WindowEvent as WinitWindowEvent;

        let on_winit_event = |winit_event: WinitEvent| {
            let event = Event::new(match winit_event {
                WinitWindowEvent {
                    event: window_event_type,
                    ..
                } => match window_event_type {
                    WinitWindowEventType::CloseRequested => EventType::WindowClose,
                    WinitWindowEventType::Resized(size) => EventType::WindowResize {
                        width: size.width,
                        height: size.height,
                    },
                    WinitWindowEventType::Focused(focused) => {
                        if focused {
                            EventType::WindowFocus
                        } else {
                            EventType::WindowLostFocus
                        }
                    }
                    WinitWindowEventType::Moved(position) => EventType::WindowMoved {
                        window_x: position.x,
                        window_y: position.y,
                    },
                    WinitWindowEventType::CursorMoved { position, .. } => EventType::MouseMoved {
                        mouse_x: position.x,
                        mouse_y: position.y,
                    },
                    _ => EventType::None,
                },
                WinitDeviceEvent { event, .. } => match event {
                    _ => EventType::None,
                },
                _ => EventType::None,
            });
            event_dispatcher.dispatch(event);
        };

        self.winit_events_loop.poll_events(on_winit_event);
    }
}
