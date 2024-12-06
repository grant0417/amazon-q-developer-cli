// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ByUserAnalytics {
    #[allow(missing_docs)] // documentation missing in model
    pub s3_uri: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub toggle: crate::types::OptInFeatureToggle,
}
impl ByUserAnalytics {
    #[allow(missing_docs)] // documentation missing in model
    pub fn s3_uri(&self) -> ::std::option::Option<&str> {
        self.s3_uri.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn toggle(&self) -> &crate::types::OptInFeatureToggle {
        &self.toggle
    }
}
impl ByUserAnalytics {
    /// Creates a new builder-style object to manufacture
    /// [`ByUserAnalytics`](crate::types::ByUserAnalytics).
    pub fn builder() -> crate::types::builders::ByUserAnalyticsBuilder {
        crate::types::builders::ByUserAnalyticsBuilder::default()
    }
}

/// A builder for [`ByUserAnalytics`](crate::types::ByUserAnalytics).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ByUserAnalyticsBuilder {
    pub(crate) s3_uri: ::std::option::Option<::std::string::String>,
    pub(crate) toggle: ::std::option::Option<crate::types::OptInFeatureToggle>,
}
impl ByUserAnalyticsBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn s3_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_uri = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_s3_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_uri = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_s3_uri(&self) -> &::std::option::Option<::std::string::String> {
        &self.s3_uri
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn toggle(mut self, input: crate::types::OptInFeatureToggle) -> Self {
        self.toggle = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_toggle(mut self, input: ::std::option::Option<crate::types::OptInFeatureToggle>) -> Self {
        self.toggle = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_toggle(&self) -> &::std::option::Option<crate::types::OptInFeatureToggle> {
        &self.toggle
    }

    /// Consumes the builder and constructs a [`ByUserAnalytics`](crate::types::ByUserAnalytics).
    /// This method will fail if any of the following fields are not set:
    /// - [`toggle`](crate::types::builders::ByUserAnalyticsBuilder::toggle)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::ByUserAnalytics, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ByUserAnalytics {
            s3_uri: self.s3_uri,
            toggle: self.toggle.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "toggle",
                    "toggle was not specified but it is required when building ByUserAnalytics",
                )
            })?,
        })
    }
}