import { createErrorInstance } from "@aws/amazon-q-developer-clshared/errors";

export const GenericRequestError = createErrorInstance("GenericRequestError");
export const CredentialsError = createErrorInstance("CredentialsError");
