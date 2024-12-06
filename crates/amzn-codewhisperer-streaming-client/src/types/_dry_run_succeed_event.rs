// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Streaming Response Event when DryRun is succeessful
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DryRunSucceedEvent {}
impl DryRunSucceedEvent {
    /// Creates a new builder-style object to manufacture
    /// [`DryRunSucceedEvent`](crate::types::DryRunSucceedEvent).
    pub fn builder() -> crate::types::builders::DryRunSucceedEventBuilder {
        crate::types::builders::DryRunSucceedEventBuilder::default()
    }
}

/// A builder for [`DryRunSucceedEvent`](crate::types::DryRunSucceedEvent).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DryRunSucceedEventBuilder {}
impl DryRunSucceedEventBuilder {
    /// Consumes the builder and constructs a
    /// [`DryRunSucceedEvent`](crate::types::DryRunSucceedEvent).
    pub fn build(self) -> crate::types::DryRunSucceedEvent {
        crate::types::DryRunSucceedEvent {}
    }
}