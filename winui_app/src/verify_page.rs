use windows::core::{IInspectable, Result};
use winui3::{
    Activatable,
    Microsoft::UI::Xaml::{
        Controls::Page,
        Navigation::{NavigatingCancelEventArgs, NavigationEventArgs},
    },
    XamlPage, XamlPageOverrides,
};

pub struct VerifyPage;

impl Activatable for VerifyPage {
    fn activate() -> Result<IInspectable> {
        let page = XamlPage::compose(VerifyPage)
            .inspect_err(|err| log::error!("Failed to create a XamlPage: {:?}", err))?;

        Ok(page.into())
    }
}

impl XamlPageOverrides for VerifyPage {
    fn OnNavigatedFrom(&self, _base: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        log::debug!("OnNavigatedFrom");
        Ok(())
    }

    fn OnNavigatedTo(&self, _page: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        log::debug!("OnNavigatedTo");
        Ok(())
    }

    fn OnNavigatingFrom(
        &self,
        _base: &Page,
        _args: Option<&NavigatingCancelEventArgs>,
    ) -> Result<()> {
        log::debug!("OnNavigatingFrom");
        Ok(())
    }
}

impl Drop for VerifyPage {
    fn drop(&mut self) {
        log::debug!("VerifyPage::drop");
    }
}
