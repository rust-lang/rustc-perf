export function generateUrlParams(params: Dict<string>): URLSearchParams {
    const originalUrl = window.location.toString();
    const url = new URL(originalUrl);
    for (const [key, value] of Object.entries(params)) {
        if (value !== null && value !== undefined && value !== "") {
            url.searchParams.set(key, value);
        }
    }
    return url.searchParams;
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
