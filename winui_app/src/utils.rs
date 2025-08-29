use windows::{
    Foundation::{IPropertyValue, IReference, PropertyValue},
    core::{HSTRING, IInspectable, Interface, Result},
};
use winui3::{
    Microsoft::UI::Xaml::Controls::{NavigationViewItem, NavigationViewItemBase},
    Windows::UI::Xaml::Interop::TypeName,
    xaml_typename,
};

pub trait HasTag {
    fn get_tag(&self) -> Result<IInspectable>;
}

impl HasTag for NavigationViewItem {
    fn get_tag(&self) -> Result<IInspectable> {
        self.Tag()
    }
}

impl HasTag for NavigationViewItemBase {
    fn get_tag(&self) -> Result<IInspectable> {
        self.Tag()
    }
}

pub(crate) fn view_item_to_type<T: HasTag>(view_item: &T) -> Result<TypeName> {
    let tag = view_item.get_tag()?;
    let prop_value = tag.cast::<IPropertyValue>()?;
    let tag_hstr = prop_value.GetString()?;
    let tag_string = tag_hstr.to_string();
    let page_type = xaml_typename(&tag_string);
    Ok(page_type)
}

pub(crate) fn hstring_reference(text: &HSTRING) -> Result<IReference<HSTRING>> {
    PropertyValue::CreateString(text)?.cast()
}
