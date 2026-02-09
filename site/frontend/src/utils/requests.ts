import {decode} from "msgpack-lite";
import {isObject, isString} from "./getType";

export async function postJson<T>(path: string, body: any): Promise<T> {
  const response = await fetch(path, {
    method: "POST",
    body: JSON.stringify(body),
  });
  return await response.json();
}

export type JsonServerError = {error: string};
export type JsonResponse<T> = T | JsonServerError;

export function jsonResponseHasError<T>(
  response: JsonResponse<T>
): response is JsonServerError {
  return isObject(response) && "error" in response;
}

export async function getJson<T>(
  path: string,
  params: Dict<string> = {}
): Promise<T> {
  let url = path;

  if (Object.keys(params).length > 0) {
    const urlParams = new URLSearchParams();
    for (const [key, value] of Object.entries(params)) {
      if (value !== null) {
        urlParams.set(key, value);
      }
    }
    url = `${path}?${urlParams}`;
  }

  let response: Response;
  try {
    response = await fetch(url, {});
  } catch (error) {
    throw {error: `Network error: ${String(error)}`};
  }

  let json: unknown;
  try {
    json = await response.clone().json();
  } catch {
    const text = await response.text();
    if (!response.ok) {
      throw {
        error: text || `Request failed with status ${response.status}`,
      };
    }
    throw {error: text || "Invalid JSON response"};
  }

  // If the response is an error, throw it
  if (!response.ok) {
    if (jsonResponseHasError(json as JsonResponse<T>)) {
      throw json;
    }
    if (isString(json)) {
      throw {error: json};
    }
    if (isObject(json)) {
      throw {error: JSON.stringify(json)};
    }
    throw {error: String(json)};
  }
  return json as T;
}

export async function postMsgpack<T>(path: string, body: any): Promise<T> {
  const response = await fetch(path, {
    method: "POST",
    body: JSON.stringify(body),
    mode: "cors",
  });
  if (response.ok) {
    const buffer = await response.clone().arrayBuffer();
    return decode(new Uint8Array(buffer));
  } else {
    const text = await response.text();
    alert(text);
    throw new Error(`Invalid response from server: ${text}`);
  }
}
