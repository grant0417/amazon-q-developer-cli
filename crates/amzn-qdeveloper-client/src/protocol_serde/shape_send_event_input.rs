// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_send_event_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::send_event::SendEventInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.provider_id {
        object.key("providerId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.event_id {
        object.key("eventId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.event_version {
        object.key("eventVersion").string(var_4.as_str());
    }
    if let Some(var_5) = &input.event {
        object
            .key("event")
            .string_unchecked(&::aws_smithy_types::base64::encode(var_5));
    }
    Ok(())
}
