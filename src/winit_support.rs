use crate::events::{Event, EventType, EventDispatcher};
use crate::window::{Window, WindowProps};

impl Event {
    fn from_winit_event(winit_event: winit::Event) -> Event {
        use winit::WindowEvent as WinitWindowEventType;
        use winit::Event::DeviceEvent as WinitDeviceEvent;
        use winit::Event::WindowEvent as WinitWindowEvent;

        let event_type = match winit_event {
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
        };

        Event::new(event_type)
    }

    // fn to_winit_event(event: events::Event, winit_window: winit::Window) -> winit::Event {
    //     use winit::dpi::{LogicalSize, PhysicalSize, LogicalPosition, PhysicalPosition};
    //     use winit::Event::WindowEvent as WinitWindowEvent;

    //     match event.event_type {
    //         events::EventType::WindowClose => WinitWindowEvent { event: winit::WindowEvent::CloseRequested, window_id: winit_window.id() },
    //         events::EventType::WindowResize { width, height } => WinitWindowEvent { event: winit::WindowEvent::Resized(LogicalSize::from_physical(PhysicalSize::new(width, height), winit_window.get_hidpi_factor())), window_id: winit_window.id() },
    //         events::EventType::WindowFocus => WinitWindowEvent { event: winit::WindowEvent::Focused(true), window_id: winit_window.id() },
    //         events::EventType::WindowLostFocus => WinitWindowEvent { event: winit::WindowEvent::Focused(false), window_id: winit_window.id() },
    //         events::EventType::WindowMoved { window_x, window_y } => WinitWindowEvent { event: winit::WindowEvent::Moved(LogicalPosition::from_physical(PhysicalPosition::new(window_x, window_y), winit_window.get_hidpi_factor())), window_id: winit_window.id() },

    //         events::EventType::MouseMoved {mouse_x, mouse_y } => WinitWindowEvent { event: winit::WindowEvent::CursorMoved { position: LogicalPosition::from_physical(PhysicalPosition::new(mouse_x, mouse_y), device_id: winit_window.get_hidpi_factor())), window_id: winit_window.id(), .. },
    //     }
    // }
}

pub struct WinitWindow {
    winit_events_loop: winit::EventsLoop,
    winit_window: winit::Window,
}

impl WinitWindow {
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

        Self {
            winit_events_loop,
            winit_window,
        }
    }

    pub fn get_winit_window(&self) -> &winit::Window {
        &self.winit_window
    }
}

impl Window for WinitWindow {
    fn on_update(&mut self, event_dispatcher: &mut EventDispatcher) {
        self.winit_events_loop.poll_events(|winit_event: winit::Event| {
            event_dispatcher.dispatch(Event::from_winit_event(winit_event));
        });
    }
}