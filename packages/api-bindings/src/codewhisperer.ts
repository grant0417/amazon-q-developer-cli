import { CodewhispererCustomization as Customization } from "@aws/amazon-q-developer-clproto/fig";
import { sendCodewhispererListCustomizationRequest } from "./requests.js";

const listCustomizations = async () =>
  (await sendCodewhispererListCustomizationRequest({})).customizations;

export { listCustomizations, Customization };
