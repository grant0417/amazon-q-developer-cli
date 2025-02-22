// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsAccountConnection {
    #[allow(missing_docs)] // documentation missing in model
    pub status: ::std::option::Option<crate::types::AccountConnectionStatus>,
    #[allow(missing_docs)] // documentation missing in model
    pub created_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// Represents the AWS account ID of the customer
    pub account_id: ::std::option::Option<::std::string::String>,
    /// Resource associated to a connector, eg: IamRole
    pub resource: ::std::option::Option<crate::types::ConnectorResource>,
}
impl AwsAccountConnection {
    #[allow(missing_docs)] // documentation missing in model
    pub fn status(&self) -> ::std::option::Option<&crate::types::AccountConnectionStatus> {
        self.status.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn created_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_date.as_ref()
    }

    /// Represents the AWS account ID of the customer
    pub fn account_id(&self) -> ::std::option::Option<&str> {
        self.account_id.as_deref()
    }

    /// Resource associated to a connector, eg: IamRole
    pub fn resource(&self) -> ::std::option::Option<&crate::types::ConnectorResource> {
        self.resource.as_ref()
    }
}
impl AwsAccountConnection {
    /// Creates a new builder-style object to manufacture
    /// [`AwsAccountConnection`](crate::types::AwsAccountConnection).
    pub fn builder() -> crate::types::builders::AwsAccountConnectionBuilder {
        crate::types::builders::AwsAccountConnectionBuilder::default()
    }
}

/// A builder for [`AwsAccountConnection`](crate::types::AwsAccountConnection).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AwsAccountConnectionBuilder {
    pub(crate) status: ::std::option::Option<crate::types::AccountConnectionStatus>,
    pub(crate) created_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
    pub(crate) resource: ::std::option::Option<crate::types::ConnectorResource>,
}
impl AwsAccountConnectionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn status(mut self, input: crate::types::AccountConnectionStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::AccountConnectionStatus>) -> Self {
        self.status = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_status(&self) -> &::std::option::Option<crate::types::AccountConnectionStatus> {
        &self.status
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn created_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_date = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_created_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_date = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_created_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_date
    }

    /// Represents the AWS account ID of the customer
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }

    /// Represents the AWS account ID of the customer
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }

    /// Represents the AWS account ID of the customer
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.account_id
    }

    /// Resource associated to a connector, eg: IamRole
    pub fn resource(mut self, input: crate::types::ConnectorResource) -> Self {
        self.resource = ::std::option::Option::Some(input);
        self
    }

    /// Resource associated to a connector, eg: IamRole
    pub fn set_resource(mut self, input: ::std::option::Option<crate::types::ConnectorResource>) -> Self {
        self.resource = input;
        self
    }

    /// Resource associated to a connector, eg: IamRole
    pub fn get_resource(&self) -> &::std::option::Option<crate::types::ConnectorResource> {
        &self.resource
    }

    /// Consumes the builder and constructs a
    /// [`AwsAccountConnection`](crate::types::AwsAccountConnection).
    pub fn build(self) -> crate::types::AwsAccountConnection {
        crate::types::AwsAccountConnection {
            status: self.status,
            created_date: self.created_date,
            account_id: self.account_id,
            resource: self.resource,
        }
    }
}
