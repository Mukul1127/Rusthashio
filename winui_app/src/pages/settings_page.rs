use windows::core::{IInspectable, Result, h};
use winui3::{
    Activatable,
    Microsoft::UI::Xaml::{
        Controls::{Page, TextBlock},
        Navigation::{NavigatingCancelEventArgs, NavigationEventArgs},
    },
    XamlPage, XamlPageOverrides,
};

pub struct SettingsPage;

impl Activatable for SettingsPage {
    fn activate() -> Result<IInspectable> {
        let page = XamlPage::compose(SettingsPage)?;

        Ok(page.into())
    }
}

impl XamlPageOverrides for SettingsPage {
    fn OnNavigatedFrom(&self, _base: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        Ok(())
    }

    fn OnNavigatedTo(&self, page: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        let textblock = TextBlock::new()?;
        textblock.SetText(h!("Hi from Settings Page!"))?;

        page.SetContent(&textblock)?;

        Ok(())
    }

    fn OnNavigatingFrom(
        &self,
        _base: &Page,
        _args: Option<&NavigatingCancelEventArgs>,
    ) -> Result<()> {
        Ok(())
    }
}
