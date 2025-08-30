use windows::core::{IInspectable, Result};
use winui3::{
    Activatable,
    Microsoft::UI::Xaml::{
        Controls::Page,
        Navigation::{NavigatingCancelEventArgs, NavigationEventArgs},
    },
    XamlPage, XamlPageOverrides,
};

pub struct HashPage;

impl Activatable for HashPage {
    fn activate() -> Result<IInspectable> {
        let page = XamlPage::compose(HashPage)
            .inspect_err(|err| log::error!("Failed to create a XamlPage: {:?}", err))?;

        Ok(page.into())
    }
}

impl XamlPageOverrides for HashPage {
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

impl Drop for HashPage {
    fn drop(&mut self) {
        log::debug!("HashPage::drop");
    }
}
