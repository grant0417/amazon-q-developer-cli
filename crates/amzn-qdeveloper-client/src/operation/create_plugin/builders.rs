// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_plugin::_create_plugin_input::CreatePluginInputBuilder;
pub use crate::operation::create_plugin::_create_plugin_output::CreatePluginOutputBuilder;

impl crate::operation::create_plugin::builders::CreatePluginInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_plugin::CreatePluginOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_plugin::CreatePluginError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_plugin();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreatePlugin`.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePluginFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_plugin::builders::CreatePluginInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_plugin::CreatePluginOutput,
        crate::operation::create_plugin::CreatePluginError,
    > for CreatePluginFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_plugin::CreatePluginOutput,
            crate::operation::create_plugin::CreatePluginError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreatePluginFluentBuilder {
    /// Creates a new `CreatePluginFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }

    /// Access the CreatePlugin as a reference.
    pub fn as_input(&self) -> &crate::operation::create_plugin::builders::CreatePluginInputBuilder {
        &self.inner
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_plugin::CreatePluginOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_plugin::CreatePluginError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_plugin::CreatePlugin::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_plugin::CreatePlugin::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_plugin::CreatePluginOutput,
        crate::operation::create_plugin::CreatePluginError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }

    pub(crate) fn config_override(
        mut self,
        config_override: impl ::std::convert::Into<crate::config::Builder>,
    ) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(
        &mut self,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> &mut Self {
        self.config_override = config_override;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn plugin_provider(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.plugin_provider(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_plugin_provider(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_plugin_provider(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_plugin_provider(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_plugin_provider()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn plugin_credential(mut self, input: crate::types::PluginCredential) -> Self {
        self.inner = self.inner.plugin_credential(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_plugin_credential(mut self, input: ::std::option::Option<crate::types::PluginCredential>) -> Self {
        self.inner = self.inner.set_plugin_credential(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_plugin_credential(&self) -> &::std::option::Option<crate::types::PluginCredential> {
        self.inner.get_plugin_credential()
    }

    /// Adds a key-value pair to `pluginProperties`.
    ///
    /// To override the contents of this collection use
    /// [`set_plugin_properties`](Self::set_plugin_properties).
    #[allow(missing_docs)] // documentation missing in model
    pub fn plugin_properties(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.plugin_properties(k.into(), v.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_plugin_properties(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_plugin_properties(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_plugin_properties(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_plugin_properties()
    }

    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    #[allow(missing_docs)] // documentation missing in model
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
