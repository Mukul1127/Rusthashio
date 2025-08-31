use windows::core::{IInspectable, Result, h};
use winui3::{
    Activatable,
    Microsoft::UI::Xaml::{
        Controls::{Page, TextBlock},
        Navigation::{NavigatingCancelEventArgs, NavigationEventArgs},
    },
    XamlPage, XamlPageOverrides,
};

pub struct VerifyPage;

impl Activatable for VerifyPage {
    fn activate() -> Result<IInspectable> {
        let page = XamlPage::compose(VerifyPage)?;

        Ok(page.into())
    }
}

impl XamlPageOverrides for VerifyPage {
    fn OnNavigatedFrom(&self, _base: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        Ok(())
    }

    fn OnNavigatedTo(&self, page: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        let textblock = TextBlock::new()?;
        textblock.SetText(h!("Hi from Verify Page!"))?;

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
