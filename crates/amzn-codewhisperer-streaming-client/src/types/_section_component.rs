// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SectionComponent {
    /// Structure representing a simple text component with sensitive content, which can include
    /// Markdown formatting.
    pub text: ::std::option::Option<crate::types::Text>,
    /// Structure representing an alert with a type and content.
    pub alert: ::std::option::Option<crate::types::Alert>,
    /// Structure representing a resource item
    pub resource: ::std::option::Option<crate::types::Resource>,
    /// Structure representing a list of Items
    pub resource_list: ::std::option::Option<crate::types::ResourceList>,
}
impl SectionComponent {
    /// Structure representing a simple text component with sensitive content, which can include
    /// Markdown formatting.
    pub fn text(&self) -> ::std::option::Option<&crate::types::Text> {
        self.text.as_ref()
    }

    /// Structure representing an alert with a type and content.
    pub fn alert(&self) -> ::std::option::Option<&crate::types::Alert> {
        self.alert.as_ref()
    }

    /// Structure representing a resource item
    pub fn resource(&self) -> ::std::option::Option<&crate::types::Resource> {
        self.resource.as_ref()
    }

    /// Structure representing a list of Items
    pub fn resource_list(&self) -> ::std::option::Option<&crate::types::ResourceList> {
        self.resource_list.as_ref()
    }
}
impl SectionComponent {
    /// Creates a new builder-style object to manufacture
    /// [`SectionComponent`](crate::types::SectionComponent).
    pub fn builder() -> crate::types::builders::SectionComponentBuilder {
        crate::types::builders::SectionComponentBuilder::default()
    }
}

/// A builder for [`SectionComponent`](crate::types::SectionComponent).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SectionComponentBuilder {
    pub(crate) text: ::std::option::Option<crate::types::Text>,
    pub(crate) alert: ::std::option::Option<crate::types::Alert>,
    pub(crate) resource: ::std::option::Option<crate::types::Resource>,
    pub(crate) resource_list: ::std::option::Option<crate::types::ResourceList>,
}
impl SectionComponentBuilder {
    /// Structure representing a simple text component with sensitive content, which can include
    /// Markdown formatting.
    pub fn text(mut self, input: crate::types::Text) -> Self {
        self.text = ::std::option::Option::Some(input);
        self
    }

    /// Structure representing a simple text component with sensitive content, which can include
    /// Markdown formatting.
    pub fn set_text(mut self, input: ::std::option::Option<crate::types::Text>) -> Self {
        self.text = input;
        self
    }

    /// Structure representing a simple text component with sensitive content, which can include
    /// Markdown formatting.
    pub fn get_text(&self) -> &::std::option::Option<crate::types::Text> {
        &self.text
    }

    /// Structure representing an alert with a type and content.
    pub fn alert(mut self, input: crate::types::Alert) -> Self {
        self.alert = ::std::option::Option::Some(input);
        self
    }

    /// Structure representing an alert with a type and content.
    pub fn set_alert(mut self, input: ::std::option::Option<crate::types::Alert>) -> Self {
        self.alert = input;
        self
    }

    /// Structure representing an alert with a type and content.
    pub fn get_alert(&self) -> &::std::option::Option<crate::types::Alert> {
        &self.alert
    }

    /// Structure representing a resource item
    pub fn resource(mut self, input: crate::types::Resource) -> Self {
        self.resource = ::std::option::Option::Some(input);
        self
    }

    /// Structure representing a resource item
    pub fn set_resource(mut self, input: ::std::option::Option<crate::types::Resource>) -> Self {
        self.resource = input;
        self
    }

    /// Structure representing a resource item
    pub fn get_resource(&self) -> &::std::option::Option<crate::types::Resource> {
        &self.resource
    }

    /// Structure representing a list of Items
    pub fn resource_list(mut self, input: crate::types::ResourceList) -> Self {
        self.resource_list = ::std::option::Option::Some(input);
        self
    }

    /// Structure representing a list of Items
    pub fn set_resource_list(mut self, input: ::std::option::Option<crate::types::ResourceList>) -> Self {
        self.resource_list = input;
        self
    }

    /// Structure representing a list of Items
    pub fn get_resource_list(&self) -> &::std::option::Option<crate::types::ResourceList> {
        &self.resource_list
    }

    /// Consumes the builder and constructs a [`SectionComponent`](crate::types::SectionComponent).
    pub fn build(self) -> crate::types::SectionComponent {
        crate::types::SectionComponent {
            text: self.text,
            alert: self.alert,
            resource: self.resource,
            resource_list: self.resource_list,
        }
    }
}