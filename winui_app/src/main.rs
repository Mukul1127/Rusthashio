#![windows_subsystem = "console"]

mod app;
mod main_window;
mod pages;
mod utils;

use windows::{
    Win32::{
        Storage::Packaging::Appx::{
            AddPackageDependency, AddPackageDependencyOptions_None,
            CreatePackageDependencyOptions_None, PACKAGE_VERSION, PACKAGE_VERSION_0,
            PACKAGEDEPENDENCY_CONTEXT, PackageDependencyLifetimeKind_Process,
            PackageDependencyProcessorArchitectures_None, RemovePackageDependency,
            TryCreatePackageDependency,
        },
        System::WinRT::RO_INIT_SINGLETHREADED,
    },
    core::{HSTRING, IInspectable, Ref, Result, h},
};
use winui3::Microsoft::UI::Xaml::{
    Application, ApplicationInitializationCallback, ApplicationInitializationCallbackParams,
    DispatcherShutdownMode, UnhandledExceptionEventArgs, UnhandledExceptionEventHandler,
};

use app::App;

// Windows App SDK v1.7
const WINDOWSAPPSDK_RUNTIME_VERSION_UINT64: u64 = 0x1B5801B3009A0000;
const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME: &HSTRING =
    h!("Microsoft.WindowsAppRuntime.1.7_8wekyb3d8bbwe");

fn main() -> Result<()> {
    simple_logger::init_with_env().expect("failed to initialize the logger");

    // Initialize Apartment
    unsafe {
        windows::Win32::System::WinRT::RoInitialize(RO_INIT_SINGLETHREADED)?;
    }

    // Initalize Windows App SDK
    struct DependencyGuard(PACKAGEDEPENDENCY_CONTEXT);
    impl Drop for DependencyGuard {
        fn drop(&mut self) {
            unsafe {
                _ = RemovePackageDependency(self.0);
            };
        }
    }

    let _dependency_guard = unsafe {
        let package_dependency_id = TryCreatePackageDependency(
            windows::Win32::Security::PSID::default(),
            WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME,
            PACKAGE_VERSION {
                Anonymous: PACKAGE_VERSION_0 {
                    Version: WINDOWSAPPSDK_RUNTIME_VERSION_UINT64,
                },
            },
            PackageDependencyProcessorArchitectures_None,
            PackageDependencyLifetimeKind_Process,
            None,
            CreatePackageDependencyOptions_None,
        )?;

        let mut dependency_context = PACKAGEDEPENDENCY_CONTEXT::default();
        AddPackageDependency(
            package_dependency_id,
            0,
            AddPackageDependencyOptions_None,
            &mut dependency_context,
            None,
        )?;

        DependencyGuard(dependency_context)
    };

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

    let sender = sender.as_ref();
    let args = args.as_ref();

    let sender_name = sender
        .map(|s| s.GetRuntimeClassName())
        .transpose()?
        .map_or_else(|| String::new(), |name| name.to_string_lossy());

    if let Some(args) = args {
        log::error!(
            target: &sender_name,
            "Unhandled exception: {} - {}",
            args.Exception()?,
            args.Message()?
        );
    } else {
        log::error!(target: &sender_name, "Unhandled exception occurred");
    }

    Ok(())
}
