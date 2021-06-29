import { API_URL } from "$lib/constants";

/**
 * @type {import('@sveltejs/kit').RequestHandler}
 */
export async function put({ params, body }) {
    const { uuid } = params;

    try {
        let res = await fetch(`${API_URL}/recipes/${uuid}`, { method: "PUT", body: body });
        if (res.ok) {
            let json = await res.json();
            return {
                status: 200,
                body: json
            };
        }

        let error = await res.text();
        return {
            status: 500,
            body: `Could not update recipe: ${error}`
        };
    } catch (error) {
        return {
            status: 500,
            body: error
        };
    }
}

/**
 * @type {import('@sveltejs/kit').RequestHandler}
 */
export async function del({ params }) {
    const { uuid } = params;

    try {
        let res = await fetch(`${API_URL}/recipes/${uuid}`, { method: "DELETE" });
        if (res.ok) {
            let json = await res.json();
            return {
                status: 200,
                body: json
            };
        }

        let error = await res.text();
        return {
            status: 500,
            body: `Could not delete recipe: ${error}`
        };
    } catch (error) {
        return {
            status: 500,
            body: error
        }
    }
}