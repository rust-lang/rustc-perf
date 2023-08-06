/**
 * Creates a URL with the current window location and current parameters, and adds
 * the passed parameters to the URL.
 */
export function createUrlWithAppendedParams(params: Dict<any>): URL {
  const originalUrl = window.location.toString();
  const url = new URL(originalUrl);
  for (const [key, value] of Object.entries(params)) {
    if (value !== null && value !== undefined) {
      const stringified = value.toString();
      if (stringified !== "") {
        url.searchParams.set(key, stringified);
      } else {
        url.searchParams.delete(key);
      }
    }
  }
  return url;
}

/**
 * Creates a URL with the current window location and the passed parameters.
 */
export function createUrlFromParams(params: Dict<string>): URL {
  const url = new URL(window.location.toString());
  const searchParams = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    searchParams.set(key, value);
  }
  url.search = searchParams.toString();
  return url;
}

export function getUrlParams(): Dict<string> {
  const url = new URL(window.location.toString());

  const params = {};
  url.searchParams.forEach((value, key) => {
    params[key] = value;
  });
  return params;
}

export function navigateToUrlParams(params: URLSearchParams) {
  window.location.search = params.toString();
}

/**
 * Changes the current URL without navigating away from the page and without
 * creating a history entry.
 */
export function changeUrl(params: Dict<string>) {
  if (history.replaceState) {
    history.replaceState({}, null, createUrlFromParams(params).toString());
  }
}
