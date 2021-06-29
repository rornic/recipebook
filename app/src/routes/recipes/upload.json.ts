import { API_URL } from "$lib/constants";

export async function post({ params, body }) {
    const data = body;
    try {
        const res = await fetch(`${API_URL}/upload`,
            {
                method: 'POST',
                body: JSON.stringify(data)
            });

        if (res.ok) {
            const json = await res.json();
            return {
                status: 200,
                body: json
            };
        }
        const error = await res.text();
        return {
            status: 500,
            body: `Could not upload image: ${error}`
        };
    } catch (error) {
        return {
            status: 500,
            body: error
        }
    }
}