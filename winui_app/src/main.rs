#![windows_subsystem = "windows"]

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
        System::WinRT::{RO_INIT_SINGLETHREADED, RoInitialize},
    },
    core::{HSTRING, Ref, Result, h},
};
use winui3::Microsoft::UI::Xaml::{
    Application, ApplicationInitializationCallback, ApplicationInitializationCallbackParams,
};

use app::App;

// Windows App SDK v1.7
const WINDOWSAPPSDK_RUNTIME_VERSION_UINT64: u64 = 0x1B5801B3009A0000;
const WINDOWSAPPSDK_RUNTIME_PACKAGE_FRAMEWORK_PACKAGEFAMILYNAME: &HSTRING =
    h!("Microsoft.WindowsAppRuntime.1.7_8wekyb3d8bbwe");

fn main() -> Result<()> {
    // Initialize Apartment
    unsafe {
        RoInitialize(RO_INIT_SINGLETHREADED)?;
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

    Application::Start(&ApplicationInitializationCallback::new(
        |_: Ref<'_, ApplicationInitializationCallbackParams>| -> Result<()> {
            _ = App::new()?;
            Ok(())
        },
    ))?;

    Ok(())
}
