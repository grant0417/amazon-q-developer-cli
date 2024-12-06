// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct SendEventInput {
    #[allow(missing_docs)] // documentation missing in model
    pub client_token: ::std::option::Option<::std::string::String>,
    /// Currently supported providers for receiving events.
    pub provider_id: ::std::option::Option<crate::types::SupportedProviderId>,
    #[allow(missing_docs)] // documentation missing in model
    pub event_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub event_version: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub event: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl SendEventInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }

    /// Currently supported providers for receiving events.
    pub fn provider_id(&self) -> ::std::option::Option<&crate::types::SupportedProviderId> {
        self.provider_id.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn event_id(&self) -> ::std::option::Option<&str> {
        self.event_id.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn event_version(&self) -> ::std::option::Option<&str> {
        self.event_version.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn event(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
        self.event.as_ref()
    }
}
impl ::std::fmt::Debug for SendEventInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SendEventInput");
        formatter.field("client_token", &self.client_token);
        formatter.field("provider_id", &self.provider_id);
        formatter.field("event_id", &self.event_id);
        formatter.field("event_version", &self.event_version);
        formatter.field("event", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl SendEventInput {
    /// Creates a new builder-style object to manufacture
    /// [`SendEventInput`](crate::operation::send_event::SendEventInput).
    pub fn builder() -> crate::operation::send_event::builders::SendEventInputBuilder {
        crate::operation::send_event::builders::SendEventInputBuilder::default()
    }
}

/// A builder for [`SendEventInput`](crate::operation::send_event::SendEventInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct SendEventInputBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) provider_id: ::std::option::Option<crate::types::SupportedProviderId>,
    pub(crate) event_id: ::std::option::Option<::std::string::String>,
    pub(crate) event_version: ::std::option::Option<::std::string::String>,
    pub(crate) event: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl SendEventInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }

    /// Currently supported providers for receiving events.
    /// This field is required.
    pub fn provider_id(mut self, input: crate::types::SupportedProviderId) -> Self {
        self.provider_id = ::std::option::Option::Some(input);
        self
    }

    /// Currently supported providers for receiving events.
    pub fn set_provider_id(mut self, input: ::std::option::Option<crate::types::SupportedProviderId>) -> Self {
        self.provider_id = input;
        self
    }

    /// Currently supported providers for receiving events.
    pub fn get_provider_id(&self) -> &::std::option::Option<crate::types::SupportedProviderId> {
        &self.provider_id
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn event_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_event_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_event_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.event_id
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn event_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_version = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_event_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_version = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_event_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.event_version
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn event(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.event = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_event(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.event = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_event(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.event
    }

    /// Consumes the builder and constructs a
    /// [`SendEventInput`](crate::operation::send_event::SendEventInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::send_event::SendEventInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::send_event::SendEventInput {
            client_token: self.client_token,
            provider_id: self.provider_id,
            event_id: self.event_id,
            event_version: self.event_version,
            event: self.event,
        })
    }
}
impl ::std::fmt::Debug for SendEventInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("SendEventInputBuilder");
        formatter.field("client_token", &self.client_token);
        formatter.field("provider_id", &self.provider_id);
        formatter.field("event_id", &self.event_id);
        formatter.field("event_version", &self.event_version);
        formatter.field("event", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}