use ::library_foundation_reintroduction::window_uniform::WindowUniformEvent;
use ::library_foundation_reintroduction::window_uniform::WindowUniformEventWindow;
use ::library_foundation_reintroduction::window_uniform::WindowUniformWindow;
use ::library_foundation_reintroduction::window_uniform::WindowUniformKeyState;
use ::library_foundation_reintroduction::window_uniform::WindowUniformPhysicalKey;
use ::library_foundation_reintroduction::window_uniform::WindowUniformKeyCode;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuideOwn;
use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;
use crate::application_v1_1_c2::self_::ApplicationPartMain;
use crate::application_v1_1_c2::self_::Application;


pub struct ApplicationContinuation {}

impl ApplicationContinuation {
    pub fn continue_loop_window_event(application: Application)
    -> Result<(WindowUniformWindow, ApplicationPartMain), ErrorFoundationApplicationGuide>
    {
        type WE = WindowUniformEvent<()>;
        type WEW = WindowUniformEventWindow;
        type KS = WindowUniformKeyState;
        type PK = WindowUniformPhysicalKey;
        type KC = WindowUniformKeyCode;
        let (wp_application, mut mp_application) = application.as_raw();
        let (window, window_event_loop) = wp_application.as_raw();
        let mut render_error_o = None;
        window_event_loop.run(|event, window_target| {
            match event {
                WE::AboutToWait => window.request_redraw(),
                WE::WindowEvent { event: WEW::RedrawRequested, .. }
                if !window_target.exiting() && !mp_application.is_window_minimized() =>
                    match mp_application.render(&window) {
                        Ok(_) => (),
                        Err(e) => {
                            render_error_o = Some(e);
                            window_target.exit();
                        },
                    },
                WE::WindowEvent { event: WEW::CloseRequested, .. } => window_target.exit(),
                WE::WindowEvent { event: WEW::Resized(size), .. } =>
                    match (size.width, size.height) {
                        (0, 0) => { mp_application.set_be_window_minimized(true); },
                        _ => {
                            mp_application.set_be_window_minimized(false);
                            mp_application.set_flag_signal_window_resized(true);
                        },
                    },
                WE::WindowEvent { event: WEW::KeyboardInput { event: keyboard_input_event, .. }, .. } =>
                    match (keyboard_input_event.state, keyboard_input_event.physical_key) {
                        (KS::Pressed, PK::Code(KC::Escape)) => window_target.exit(),
                        (KS::Pressed, PK::Code(KC::Space)) => mp_application.set_scene_name_next(),
                        _ => (),
                    },
                _ => (),
            }
        })
        .or(Err(ErrorFoundationApplicationGuideOwn::WindowEventLoopRunAbort))?;
        match render_error_o {
            None => Ok((window, mp_application)),
            Some(e) => Err(e),
        }
    }
}