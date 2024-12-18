// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Structure to represent get code transformation plan response.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTransformationPlanOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub transformation_plan: crate::types::TransformationPlan,
    _request_id: Option<String>,
}
impl GetTransformationPlanOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn transformation_plan(&self) -> &crate::types::TransformationPlan {
        &self.transformation_plan
    }
}
impl ::aws_types::request_id::RequestId for GetTransformationPlanOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetTransformationPlanOutput {
    /// Creates a new builder-style object to manufacture
    /// [`GetTransformationPlanOutput`](crate::operation::get_transformation_plan::GetTransformationPlanOutput).
    pub fn builder() -> crate::operation::get_transformation_plan::builders::GetTransformationPlanOutputBuilder {
        crate::operation::get_transformation_plan::builders::GetTransformationPlanOutputBuilder::default()
    }
}

/// A builder for
/// [`GetTransformationPlanOutput`](crate::operation::get_transformation_plan::GetTransformationPlanOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetTransformationPlanOutputBuilder {
    pub(crate) transformation_plan: ::std::option::Option<crate::types::TransformationPlan>,
    _request_id: Option<String>,
}
impl GetTransformationPlanOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn transformation_plan(mut self, input: crate::types::TransformationPlan) -> Self {
        self.transformation_plan = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_transformation_plan(mut self, input: ::std::option::Option<crate::types::TransformationPlan>) -> Self {
        self.transformation_plan = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_transformation_plan(&self) -> &::std::option::Option<crate::types::TransformationPlan> {
        &self.transformation_plan
    }

    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }

    /// Consumes the builder and constructs a
    /// [`GetTransformationPlanOutput`](crate::operation::get_transformation_plan::GetTransformationPlanOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`transformation_plan`](crate::operation::get_transformation_plan::builders::GetTransformationPlanOutputBuilder::transformation_plan)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_transformation_plan::GetTransformationPlanOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_transformation_plan::GetTransformationPlanOutput {
            transformation_plan: self.transformation_plan.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "transformation_plan",
                    "transformation_plan was not specified but it is required when building GetTransformationPlanOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}
