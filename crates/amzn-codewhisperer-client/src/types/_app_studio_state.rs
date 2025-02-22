// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Description of a user's context when they are calling Q Chat from AppStudio
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AppStudioState {
    /// The namespace of the context. Examples: 'ui.Button', 'ui.Table.DataSource',
    /// 'ui.Table.RowActions.Button', 'logic.invokeAWS', 'logic.JavaScript'
    pub namespace: ::std::string::String,
    /// The name of the property. Examples: 'visibility', 'disability', 'value', 'code'
    pub property_name: ::std::string::String,
    /// The value of the property.
    pub property_value: ::std::option::Option<::std::string::String>,
    /// Context about how the property is used
    pub property_context: ::std::string::String,
}
impl AppStudioState {
    /// The namespace of the context. Examples: 'ui.Button', 'ui.Table.DataSource',
    /// 'ui.Table.RowActions.Button', 'logic.invokeAWS', 'logic.JavaScript'
    pub fn namespace(&self) -> &str {
        use std::ops::Deref;
        self.namespace.deref()
    }

    /// The name of the property. Examples: 'visibility', 'disability', 'value', 'code'
    pub fn property_name(&self) -> &str {
        use std::ops::Deref;
        self.property_name.deref()
    }

    /// The value of the property.
    pub fn property_value(&self) -> ::std::option::Option<&str> {
        self.property_value.as_deref()
    }

    /// Context about how the property is used
    pub fn property_context(&self) -> &str {
        use std::ops::Deref;
        self.property_context.deref()
    }
}
impl ::std::fmt::Debug for AppStudioState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AppStudioState");
        formatter.field("namespace", &"*** Sensitive Data Redacted ***");
        formatter.field("property_name", &"*** Sensitive Data Redacted ***");
        formatter.field("property_value", &"*** Sensitive Data Redacted ***");
        formatter.field("property_context", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl AppStudioState {
    /// Creates a new builder-style object to manufacture
    /// [`AppStudioState`](crate::types::AppStudioState).
    pub fn builder() -> crate::types::builders::AppStudioStateBuilder {
        crate::types::builders::AppStudioStateBuilder::default()
    }
}

/// A builder for [`AppStudioState`](crate::types::AppStudioState).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct AppStudioStateBuilder {
    pub(crate) namespace: ::std::option::Option<::std::string::String>,
    pub(crate) property_name: ::std::option::Option<::std::string::String>,
    pub(crate) property_value: ::std::option::Option<::std::string::String>,
    pub(crate) property_context: ::std::option::Option<::std::string::String>,
}
impl AppStudioStateBuilder {
    /// The namespace of the context. Examples: 'ui.Button', 'ui.Table.DataSource',
    /// 'ui.Table.RowActions.Button', 'logic.invokeAWS', 'logic.JavaScript' This field is
    /// required.
    pub fn namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.namespace = ::std::option::Option::Some(input.into());
        self
    }

    /// The namespace of the context. Examples: 'ui.Button', 'ui.Table.DataSource',
    /// 'ui.Table.RowActions.Button', 'logic.invokeAWS', 'logic.JavaScript'
    pub fn set_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.namespace = input;
        self
    }

    /// The namespace of the context. Examples: 'ui.Button', 'ui.Table.DataSource',
    /// 'ui.Table.RowActions.Button', 'logic.invokeAWS', 'logic.JavaScript'
    pub fn get_namespace(&self) -> &::std::option::Option<::std::string::String> {
        &self.namespace
    }

    /// The name of the property. Examples: 'visibility', 'disability', 'value', 'code'
    /// This field is required.
    pub fn property_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.property_name = ::std::option::Option::Some(input.into());
        self
    }

    /// The name of the property. Examples: 'visibility', 'disability', 'value', 'code'
    pub fn set_property_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.property_name = input;
        self
    }

    /// The name of the property. Examples: 'visibility', 'disability', 'value', 'code'
    pub fn get_property_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.property_name
    }

    /// The value of the property.
    pub fn property_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.property_value = ::std::option::Option::Some(input.into());
        self
    }

    /// The value of the property.
    pub fn set_property_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.property_value = input;
        self
    }

    /// The value of the property.
    pub fn get_property_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.property_value
    }

    /// Context about how the property is used
    /// This field is required.
    pub fn property_context(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.property_context = ::std::option::Option::Some(input.into());
        self
    }

    /// Context about how the property is used
    pub fn set_property_context(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.property_context = input;
        self
    }

    /// Context about how the property is used
    pub fn get_property_context(&self) -> &::std::option::Option<::std::string::String> {
        &self.property_context
    }

    /// Consumes the builder and constructs a [`AppStudioState`](crate::types::AppStudioState).
    /// This method will fail if any of the following fields are not set:
    /// - [`namespace`](crate::types::builders::AppStudioStateBuilder::namespace)
    /// - [`property_name`](crate::types::builders::AppStudioStateBuilder::property_name)
    /// - [`property_context`](crate::types::builders::AppStudioStateBuilder::property_context)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::AppStudioState, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::AppStudioState {
            namespace: self.namespace.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "namespace",
                    "namespace was not specified but it is required when building AppStudioState",
                )
            })?,
            property_name: self.property_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "property_name",
                    "property_name was not specified but it is required when building AppStudioState",
                )
            })?,
            property_value: self.property_value,
            property_context: self.property_context.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "property_context",
                    "property_context was not specified but it is required when building AppStudioState",
                )
            })?,
        })
    }
}
impl ::std::fmt::Debug for AppStudioStateBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AppStudioStateBuilder");
        formatter.field("namespace", &"*** Sensitive Data Redacted ***");
        formatter.field("property_name", &"*** Sensitive Data Redacted ***");
        formatter.field("property_value", &"*** Sensitive Data Redacted ***");
        formatter.field("property_context", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
