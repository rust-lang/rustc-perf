import {decode} from "msgpack-lite";

export async function postJson<T>(path: string, body: any): Promise<T> {
  const response = await fetch(path, {
    method: "POST",
    body: JSON.stringify(body),
  });
  return await response.json();
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

  const response = await fetch(url, {});
  return await response.json();
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
