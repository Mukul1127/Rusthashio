#![windows_subsystem = "console"]

mod app;
mod main_window;
mod pages;
mod utils;

use windows::core::{IInspectable, Ref, Result};
use winui3::{
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback, ApplicationInitializationCallbackParams,
        DispatcherShutdownMode, UnhandledExceptionEventArgs, UnhandledExceptionEventHandler,
    },
    bootstrap::{PackageDependency as WinUIDependency, WindowsAppSDKVersion},
};

use app::App;

fn main() -> Result<()> {
    simple_logger::init_with_env().expect("failed to initialize the logger");

    winui3::init_apartment(winui3::ApartmentType::SingleThreaded)?;

    // Disposed otherwise without assignment
    let _winui_dependency = WinUIDependency::initialize_version(WindowsAppSDKVersion::V1_7)?;

    Application::Start(&ApplicationInitializationCallback::new(app_start))?;

    Ok(())
}

fn app_start(_: Ref<'_, ApplicationInitializationCallbackParams>) -> Result<()> {
    log::debug!("Application::Start");

    let app = App::create()?;
    app.SetDispatcherShutdownMode(DispatcherShutdownMode::OnLastWindowClose)?;
    app.UnhandledException(Some(&UnhandledExceptionEventHandler::new(
        unhandled_exception_handler,
    )))?;

    Ok(())
}

fn unhandled_exception_handler(
    sender: Ref<'_, IInspectable>,
    args: Ref<'_, UnhandledExceptionEventArgs>,
) -> Result<()> {
    log::debug!("unhandled_exception_handler");

    let sender = sender
        .as_ref()
        .map(|s| s.GetRuntimeClassName())
        .transpose()?
        .map_or_else(|| String::new(), |name| name.to_string_lossy());

    match args.as_ref() {
        Some(args) => log::error!(
            target: sender.as_ref(),
            "Unhandled exception: {} - {}",
            args.Exception()?,
            args.Message()?
        ),
        None => log::error!(target: sender.as_ref(), "Unhandled exception occurred"),
    }

    Ok(())
}
