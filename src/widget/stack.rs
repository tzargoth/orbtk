use crate::{
    enums::ParentType,
    layout_object::StretchLayoutObject,
    widget::{Template, Widget},
};

/// The `Stack` represents a layout widget that is used to stack its children on the z-axis.
///
/// # Others
///
/// * `ParentType`- Mutli.
/// * `StretchLayoutObject` - Used to layout the widget.
pub struct Stack;

impl Widget for Stack {
    fn create() -> Template {
        Template::default()
            .as_parent_type(ParentType::Multi)
            .with_layout_object(StretchLayoutObject::default())
            .with_debug_name("Stack")
    }
}
