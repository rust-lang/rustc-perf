export async function postRequest<T>(path: string, body: any): Promise<T> {
    const response = await fetch(path, {
        method: "POST",
        body: JSON.stringify(body),
    });
    return await response.json();
}

export async function getRequest<T>(path: string, params: Dict<string> = {}): Promise<T> {
    let url = path;

    if (Object.keys(params).length > 0) {
        const urlParams = new URLSearchParams();
        for (const [key, value] of Object.entries(params)) {
            urlParams.set(key, value);
        }
        url = `${path}?${urlParams}`;
    }

    const response = await fetch(url, {});
    return await response.json();
}
