use windows::core::{IInspectable, Result, h};
use winui3::{
    Activatable,
    Microsoft::UI::Xaml::{
        Controls::{Page, TextBlock},
        Navigation::{NavigatingCancelEventArgs, NavigationEventArgs},
    },
    XamlPage, XamlPageOverrides,
};

pub struct HashPage;

impl Activatable for HashPage {
    fn activate() -> Result<IInspectable> {
        let page = XamlPage::compose(HashPage)?;

        Ok(page.into())
    }
}

impl XamlPageOverrides for HashPage {
    fn OnNavigatedFrom(&self, _base: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        Ok(())
    }

    fn OnNavigatedTo(&self, page: &Page, _args: Option<&NavigationEventArgs>) -> Result<()> {
        let textblock = TextBlock::new()?;
        textblock.SetText(h!("Hi from Hash Page!"))?;

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
