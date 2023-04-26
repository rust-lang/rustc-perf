export async function postRequest<T>(path: string, body: any): Promise<T> {
    const response = await fetch(path, {
        method: "POST",
        body: JSON.stringify(body),
    });
    return await response.json();
}

export async function getRequest<T>(path: string): Promise<T> {
    const response = await fetch(path, {});
    return await response.json();
}
