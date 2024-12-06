// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_test_generation::_get_test_generation_input::GetTestGenerationInputBuilder;
pub use crate::operation::get_test_generation::_get_test_generation_output::GetTestGenerationOutputBuilder;

impl crate::operation::get_test_generation::builders::GetTestGenerationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_test_generation::GetTestGenerationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_test_generation::GetTestGenerationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_test_generation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetTestGeneration`.
///
/// API to get test generation job.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetTestGenerationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_test_generation::builders::GetTestGenerationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_test_generation::GetTestGenerationOutput,
        crate::operation::get_test_generation::GetTestGenerationError,
    > for GetTestGenerationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_test_generation::GetTestGenerationOutput,
            crate::operation::get_test_generation::GetTestGenerationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetTestGenerationFluentBuilder {
    /// Creates a new `GetTestGenerationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }

    /// Access the GetTestGeneration as a reference.
    pub fn as_input(&self) -> &crate::operation::get_test_generation::builders::GetTestGenerationInputBuilder {
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
        crate::operation::get_test_generation::GetTestGenerationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_test_generation::GetTestGenerationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_test_generation::GetTestGeneration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_test_generation::GetTestGeneration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_test_generation::GetTestGenerationOutput,
        crate::operation::get_test_generation::GetTestGenerationError,
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

    /// Test generation job group name
    pub fn test_generation_job_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.test_generation_job_group_name(input.into());
        self
    }

    /// Test generation job group name
    pub fn set_test_generation_job_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_test_generation_job_group_name(input);
        self
    }

    /// Test generation job group name
    pub fn get_test_generation_job_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_test_generation_job_group_name()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn test_generation_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.test_generation_job_id(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_test_generation_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_test_generation_job_id(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_test_generation_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_test_generation_job_id()
    }
}