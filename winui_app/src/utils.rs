use windows::{
    Foundation::IPropertyValue,
    core::{Interface, Result},
};
use winui3::{
    Microsoft::UI::Xaml::Controls::NavigationViewItem,
    Windows::UI::Xaml::Interop::{TypeKind, TypeName},
};

pub(crate) fn view_item_to_type(view_item: &NavigationViewItem) -> Result<TypeName> {
    let tag = view_item.Tag()?;
    let prop_value = tag.cast::<IPropertyValue>()?;
    let tag_hstr = prop_value.GetString()?;
    Ok(TypeName {
        Name: tag_hstr,
        Kind: TypeKind::Custom,
    })
}

#[macro_export]
macro_rules! string_to_iinspectable {
    ($text:expr) => {
        windows::Foundation::PropertyValue::CreateString(windows::core::h!($text))
    };
}
