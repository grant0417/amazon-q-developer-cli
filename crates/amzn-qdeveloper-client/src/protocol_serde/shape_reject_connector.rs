// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_reject_connector_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::reject_connector::RejectConnectorOutput,
    crate::operation::reject_connector::RejectConnectorError,
> {
    #[allow(unused_mut)]
    let mut generic_builder =
        crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
            .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::reject_connector::RejectConnectorError::unhandled(
                generic,
            ));
        },
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => {
            crate::operation::reject_connector::RejectConnectorError::ResourceNotFoundError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundErrorBuilder::default();
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_not_found_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?
                };
                tmp
            })
        },
        "InternalServerException" => crate::operation::reject_connector::RejectConnectorError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?
            };
            tmp
        }),
        "AccessDeniedException" => crate::operation::reject_connector::RejectConnectorError::AccessDeniedError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedErrorBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?
            };
            tmp
        }),
        "ConflictException" => crate::operation::reject_connector::RejectConnectorError::ConflictError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictErrorBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::conflict_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::reject_connector::RejectConnectorError::ValidationError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationErrorBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::reject_connector::RejectConnectorError::ThrottlingError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingErrorBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::reject_connector::RejectConnectorError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_reject_connector_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::reject_connector::RejectConnectorOutput,
    crate::operation::reject_connector::RejectConnectorError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::reject_connector::builders::RejectConnectorOutputBuilder::default();
        output = crate::protocol_serde::shape_reject_connector::de_reject_connector(_response_body, output)
            .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::reject_connector_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::reject_connector::RejectConnectorError::unhandled)?
    })
}

pub fn ser_reject_connector_input(
    input: &crate::operation::reject_connector::RejectConnectorInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_reject_connector_input::ser_reject_connector_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_reject_connector(
    value: &[u8],
    mut builder: crate::operation::reject_connector::builders::RejectConnectorOutputBuilder,
) -> Result<
    crate::operation::reject_connector::builders::RejectConnectorOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "connectorId" => {
                    builder = builder.set_connector_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                },
                "connectorName" => {
                    builder = builder.set_connector_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                },
                "connectorType" => {
                    builder = builder.set_connector_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                },
                "accountConnection" => {
                    builder = builder.set_account_connection(
                        crate::protocol_serde::shape_account_connection::de_account_connection(tokens)?,
                    );
                },
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                    format!("expected object key or end object, found: {:?}", other),
                ));
            },
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}