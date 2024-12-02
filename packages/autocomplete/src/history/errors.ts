import { createErrorInstance } from "@aws/amazon-q-developer-clshared/errors";

export const MissingSpecError = createErrorInstance("MissingSpecError");
export const HistoryReadingError = createErrorInstance("HistoryReadingError");
export const SuggestionNotFoundError = createErrorInstance(
  "SuggestionNotFoundError",
);
