use std::cell::RefCell;

use windows::core::{HSTRING, Result};
use winui3::{
    Microsoft::UI::Xaml::{
        Application, Controls::XamlControlsResources, LaunchActivatedEventArgs, Markup::IXamlType,
    },
    XamlApp, XamlAppOverrides,
};

use crate::main_window::MainWindow;
use crate::pages::hash_page::HashPage;
use crate::pages::settings_page::SettingsPage;
use crate::pages::verify_page::VerifyPage;

pub(crate) struct App {
    window: RefCell<Option<MainWindow>>,
}

impl App {
    pub(crate) fn create() -> Result<Application> {
        let app = App {
            window: RefCell::new(None),
        };
        XamlApp::compose(app)
    }
}

impl XamlAppOverrides for App {
    fn OnLaunched(&self, base: &Application, _: Option<&LaunchActivatedEventArgs>) -> Result<()> {
        log::debug!("App::OnLaunched");

        let resources = base.Resources()?;
        let merged_dictionaries = resources.MergedDictionaries()?;
        let xaml_controls_resources = XamlControlsResources::new()?;
        merged_dictionaries.Append(&xaml_controls_resources)?;

        let window = MainWindow::new()?;
        window.InitializeComponent()?;
        window.Activate()?;

        self.window.borrow_mut().replace(window);
        Ok(())
    }

    fn TryResolveXamlType(&self, full_name: &HSTRING) -> Result<IXamlType> {
        match full_name.to_string().as_str() {
            "HashPage" => winui3::XamlCustomType::<HashPage>::new(full_name),
            "VerifyPage" => winui3::XamlCustomType::<VerifyPage>::new(full_name),
            "SettingsPage" => winui3::XamlCustomType::<SettingsPage>::new(full_name),
            _ => Err(windows::core::Error::empty()),
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        log::debug!("App::drop");
    }
}
