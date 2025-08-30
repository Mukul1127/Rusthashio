use windows::{
    Foundation::{IPropertyValue, IReference, PropertyValue},
    core::{HSTRING, Interface, Result},
};
use winui3::{
    Microsoft::UI::Xaml::Controls::NavigationViewItem,
    Windows::UI::Xaml::Interop::{TypeKind, TypeName},
};

pub(crate) fn view_item_to_type(view_item: &NavigationViewItem) -> Result<TypeName> {
    let tag = view_item.Tag()?;
    let prop_value = tag.cast::<IPropertyValue>()?;
    let tag_hstr = prop_value.GetString()?;
    let page_type = TypeName {
        Name: tag_hstr,
        Kind: TypeKind::Custom,
    };
    Ok(page_type)
}

pub(crate) fn hstring_reference(text: &HSTRING) -> Result<IReference<HSTRING>> {
    PropertyValue::CreateString(text)?.cast()
}
