export function getUrlParams(): Dict<string> {
    const url = new URL(window.location.toString());

    const params = {};
    url.searchParams.forEach((value, key) => {
        params[key] = value;
    });
    return params;
}
