mod lib;
mod config;
mod window;
#[macro_use] mod termination;
mod application;
mod initialization;
mod composition;
mod data;

use crate::termination::TerminationProcessMain;
use crate::application::main::Application;
use crate::initialization::Initialization;
use crate::initialization::window::InitializationWindow;
use crate::composition::execution::CompositionExecution;
use crate::config::vulkan::VULKAN_VALIDATION_LAYER;
use crate::config::vulkan::VULKAN_DEVICE_PHYSICAL_EXTENSION_S;
use crate::data::d3_model_resource::DataD3ModelResource;
use crate::data::d3_model_resource::DataD3ModelResourceTutorialFormatObj;


fn main() -> TerminationProcessMain {
    let mut initialization = Initialization::create();
    if let Err(termination) = initialization.logger_console.initialize() {
        return termination;
    }
    let window =
        match InitializationWindow::create_main_event_loop_default() {
            Err(termination) => return termination,
            Ok(uniform_window_application) => uniform_window_application,
        };
    let create_application_result = unsafe {
        Application::create(
            &window.entity_main,
            Some(&VULKAN_VALIDATION_LAYER),
            &VULKAN_DEVICE_PHYSICAL_EXTENSION_S,
            DataD3ModelResource::TutorialFormatObj(DataD3ModelResourceTutorialFormatObj::VikingRoom))
    };
    let application =
        match create_application_result {
            Err(termination) => return termination,
            Ok(application) => application,
        };
    let _result = CompositionExecution::bind_window_event_loop_default(window, application);
    TerminationProcessMain::Ok
}
