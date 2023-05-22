export function createUrlParams(params: Dict<string>): URLSearchParams {
    return createUrlWithParams(params).searchParams;
}

export function createUrlWithParams(params: Dict<any>): URL {
    const originalUrl = window.location.toString();
    const url = new URL(originalUrl);
    for (const [key, value] of Object.entries(params)) {
        if (value !== null && value !== undefined) {
            const stringified = value.toString();
            if (stringified !== "") {
                url.searchParams.set(key, stringified);
            }
        }
    }
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
