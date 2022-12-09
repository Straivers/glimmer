use geometry::{Extent, Point};
use shell::{
    ButtonState, MouseButton, VirtualKeyCode, Window, WindowControl, WindowDesc, WindowHandler,
};

fn main() {
    let f = "hi";

    let main_window = WindowDesc {
        title: "Sandbox",
        size: Extent::new(1280, 720),
        min_size: None,
        max_size: None,
        position: None,
        resizable: true,
        visible: true,
        transparent: false,
        always_on_top: false,
        handler: &mut |window| {
            println!("{}", f);
            AppWindow::new(window)
        },
    };

    shell::run([main_window]);
}

struct AppWindow {
    window: Window,
    click_count: u64,
}

impl AppWindow {
    pub fn new(window: Window) -> Self {
        Self {
            window,
            click_count: 0,
        }
    }
}

impl WindowHandler for AppWindow {
    fn on_destroy(&mut self) {
        // no-op
    }

    fn on_close_request(&mut self, _control: &mut dyn WindowControl<Self>) -> bool {
        // always close the window opon request
        true
    }

    fn on_mouse_button(
        &mut self,
        control: &mut dyn WindowControl<Self>,
        button: MouseButton,
        state: ButtonState,
        _at: Point<i32>,
    ) {
        match button {
            MouseButton::Left => {
                if ButtonState::Released == state {
                    self.click_count += 1;
                    self.window
                        .set_title(&format!("Sandbox-Child-{}", self.click_count));
                }
            }
            MouseButton::Middle => {}
            MouseButton::Right => {
                if ButtonState::Released == state {
                    control.spawn(WindowDesc {
                        title: "Sandbox-Child",
                        size: Extent::new(1280, 720),
                        min_size: None,
                        max_size: None,
                        position: None,
                        resizable: true,
                        visible: true,
                        transparent: false,
                        always_on_top: false,
                        handler: &mut AppWindow::new,
                    });
                }
            }
            MouseButton::Other(_) => {}
        }
    }

    fn on_cursor_move(&mut self, _control: &mut dyn WindowControl<Self>, _at: Point<i32>) {
        // no-op
    }

    fn on_key(
        &mut self,
        control: &mut dyn WindowControl<Self>,
        key: VirtualKeyCode,
        state: ButtonState,
    ) {
        match key {
            VirtualKeyCode::Escape => {
                if ButtonState::Pressed == state {
                    self.window.destroy();
                }
            }
            VirtualKeyCode::N => {
                if ButtonState::Released == state {
                    control.spawn(WindowDesc {
                        title: "Sandbox-Child",
                        size: Extent::new(1280, 720),
                        min_size: None,
                        max_size: None,
                        position: None,
                        resizable: true,
                        visible: true,
                        transparent: false,
                        always_on_top: false,
                        handler: &mut AppWindow::new,
                    });
                }
            }
            _ => {}
        }
    }

    fn on_resize(&mut self, _control: &mut dyn WindowControl<Self>, _inner_size: Extent<u32>) {
        // no-op
    }

    fn on_rescale(
        &mut self,
        _control: &mut dyn WindowControl<Self>,
        _scale_factor: f64,
        _new_inner_size: Extent<u32>,
    ) {
        // no-op
    }

    fn on_redraw(&mut self, _control: &mut dyn WindowControl<Self>) {
        // no-op
    }
}
