use uniforms::State;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::{self, Window},
};

#[derive(Default)]
pub struct App<'window> {
    window: Option<Window>,
    state: Option<State<'window>>,
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let state = State::new(&self.window.as_ref().unwrap()).unwrap();
        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(state) = self.state.as_mut() {
                    state.resize(size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = self.state.as_mut() {
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            state.resize(self.window.as_ref().unwrap().inner_size());
                        }
                        Err(e) => {
                            eprintln!("{:?}", e);
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Space),
                        ..
                    },
                ..
            } => {
                dbg!("Space");
                // 切换渲染管线
            }
            _ => {}
        }
    }
}

impl<'window> App<'window> {
    pub fn run() {
        let event_loop = EventLoop::new().unwrap();
        let mut app = App::default();
        let _ = event_loop.run_app(&mut app);
    }
}
