use crate::utils::{hstring_reference, view_item_to_type};

use std::ops::Deref;
use windows::{
    Foundation::TypedEventHandler,
    core::{IInspectable, Interface, Ref, Result, h},
};
use winui3::{
    Microsoft::UI::Xaml::{
        Controls::*,
        Data::{Binding, BindingMode},
        GridLengthHelper, GridUnitType,
        Media::MicaBackdrop,
        Navigation::{NavigatedEventHandler, NavigationEventArgs},
        PropertyPath, Window,
    },
    xaml_typename,
};

pub(crate) struct MainWindow {
    window: Window,
}

impl MainWindow {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            window: Window::new()?,
        })
    }

    #[allow(non_snake_case)]
    pub(crate) fn InitializeComponent(&self) -> Result<()> {
        self.SetExtendsContentIntoTitleBar(true)?;
        self.SetSystemBackdrop(&MicaBackdrop::new()?)?;

        let grid = Grid::new()?;
        let row_definitions = grid.RowDefinitions()?;
        let grid_children = grid.Children()?;

        let row0 = RowDefinition::new()?;
        row0.SetHeight(GridLengthHelper::Auto()?)?;
        row_definitions.Append(&row0)?;

        let row1 = RowDefinition::new()?;
        row1.SetHeight(GridLengthHelper::FromValueAndType(
            1f64,
            GridUnitType::Star,
        )?)?;
        row_definitions.Append(&row1)?;

        let title_icon = FontIconSource::new()?;
        title_icon.SetGlyph(h!("\u{F28B}"))?;

        let titlebar = TitleBar::new()?;
        titlebar.SetIsBackButtonVisible(true)?;
        titlebar.SetIsPaneToggleButtonVisible(true)?;
        titlebar.SetIsBackButtonEnabled(false)?;
        titlebar.SetIconSource(&title_icon)?;
        titlebar.SetTitle(h!("Rusthashio"))?;
        Grid::SetRow(&titlebar, 0)?;
        grid_children.Append(&titlebar)?;

        let nav_view = NavigationView::new()?;
        nav_view.SetIsBackButtonVisible(NavigationViewBackButtonVisible::Collapsed)?;
        nav_view.SetIsPaneToggleButtonVisible(false)?;
        Grid::SetRow(&nav_view, 1)?;
        grid_children.Append(&nav_view)?;

        let hash_icon = FontIcon::new()?;
        hash_icon.SetGlyph(h!("\u{E8A6}"))?;

        let hash_item = NavigationViewItem::new()?;
        hash_item.SetContent(&hstring_reference(h!("Hash"))?)?;
        hash_item.SetTag(&hstring_reference(h!("HashPage"))?)?;
        hash_item.SetIcon(&hash_icon)?;
        nav_view.MenuItems()?.Append(&hash_item)?;

        let verify_icon = FontIcon::new()?;
        verify_icon.SetGlyph(h!("\u{E9D5}"))?;

        let verify_item = NavigationViewItem::new()?;
        verify_item.SetContent(&hstring_reference(h!("Verify"))?)?;
        verify_item.SetTag(&hstring_reference(h!("VerifyPage"))?)?;
        verify_item.SetIcon(&verify_icon)?;
        nav_view.MenuItems()?.Append(&verify_item)?;

        let frame = Frame::new()?;
        nav_view.SetContent(&frame)?;

        let binding = Binding::new()?;
        binding.SetSource(&frame.cast::<IInspectable>()?)?;
        binding.SetPath(&PropertyPath::CreateInstance(h!("CanGoBack"))?)?;
        binding.SetMode(BindingMode::OneWay)?;

        titlebar.SetBinding(&TitleBar::IsBackButtonEnabledProperty()?, &binding)?;

        {
            let frame_clone = frame.clone();
            let nav_view_clone = nav_view.clone();

            frame.Navigated(&NavigatedEventHandler::new(
                move |_sender: Ref<'_, IInspectable>,
                      _args: Ref<'_, NavigationEventArgs>|
                      -> Result<()> {
                    // Set tab
                    if let Ok(source_page_type) = frame_clone.SourcePageType() {
                        if source_page_type.Name == "SettingsPage" {
                            nav_view_clone.SetSelectedItem(&nav_view_clone.SettingsItem()?)?;
                            nav_view_clone.SetHeader(&hstring_reference(h!("Settings"))?)?;
                        } else {
                            // Match menu item by tag (FullName equivalent)
                            let menu_items = nav_view_clone.MenuItems()?;
                            for item in menu_items.into_iter() {
                                let nv_item: NavigationViewItem = item.cast()?;
                                let page_type = view_item_to_type(&nv_item)?;
                                if page_type == frame_clone.SourcePageType()? {
                                    nav_view_clone.SetSelectedItem(&item)?;
                                    if let Ok(content) = nv_item.Content() {
                                        nav_view_clone.SetHeader(&content)?;
                                    }
                                    break;
                                }
                            }
                        }
                    }

                    Ok(())
                },
            ))?;
        }

        {
            let frame_clone = frame.clone();

            titlebar.BackRequested(&TypedEventHandler::new(
                move |_sender: Ref<'_, TitleBar>, _args: Ref<'_, IInspectable>| -> Result<()> {
                    frame_clone.GoBack()?;
                    Ok(())
                },
            ))?;
        }

        {
            let nav_view_clone = nav_view.clone();

            titlebar.PaneToggleRequested(&TypedEventHandler::new(
                move |_sender: Ref<'_, TitleBar>, _args: Ref<'_, IInspectable>| -> Result<()> {
                    nav_view_clone.SetIsPaneOpen(!nav_view_clone.IsPaneOpen()?)?;
                    Ok(())
                },
            ))?;
        }

        {
            let frame_clone = frame.clone();

            nav_view.SelectionChanged(&TypedEventHandler::new(
                move |_sender: Ref<'_, NavigationView>,
                      args: Ref<'_, NavigationViewSelectionChangedEventArgs>|
                      -> Result<()> {
                    let args = args.as_ref().expect("args should not be None");
                    if args.IsSettingsSelected()? {
                        // Navigate to SettingsPage
                        let page_type = xaml_typename("SettingsPage");
                        frame_clone.Navigate2(&page_type)?;
                    } else if let Ok(selected_item) = args.SelectedItemContainer() {
                        // Navigate to tag
                        let page_type = view_item_to_type(&selected_item)?;
                        frame_clone.Navigate2(&page_type)?;
                    }
                    Ok(())
                },
            ))?;
        }

        nav_view.SetSelectedItem(&hash_item)?;

        self.SetContent(&grid)?;

        Ok(())
    }
}

impl Deref for MainWindow {
    type Target = Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
