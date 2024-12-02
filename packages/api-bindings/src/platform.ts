import { GetPlatformInfoResponse } from "@aws/amazon-q-developer-clproto/fig";
import { sendGetPlatformInfoRequest } from "./requests.js";
import {
  AppBundleType,
  DesktopEnvironment,
  DisplayServerProtocol,
  Os,
} from "@aws/amazon-q-developer-clproto/fig";

export { AppBundleType, DesktopEnvironment, DisplayServerProtocol, Os };

export function getPlatformInfo(): Promise<GetPlatformInfoResponse> {
  return sendGetPlatformInfoRequest({});
}
