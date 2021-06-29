import { API_URL } from "$lib/constants";

export async function get({ params }) {
    try {
        const res = await fetch(`${API_URL}/recipes`);
        if (res.ok) {
            const recipes = await res.json();
            return {
                status: 200,
                body: recipes
            };
        }

        let error = await res.text();
        return {
            status: 500,
            body: `Could not get all recipes: ${error}`
        }
    } catch (error) {
        return {
            status: 500,
            body: error
        }
    }
}

export async function post({ params }) {
    try {
        const res = await fetch(`${API_URL}/recipes`, { method: "POST" });
        if (res.ok) {
            const recipe = await res.json();
            return {
                status: 200,
                body: recipe
            };
        }

        let error = await res.text();
        return {
            status: 500,
            body: `Could not create new recipe: ${error}`
        }
    } catch (error) {
        return {
            status: 500,
            body: error
        };
    }
}

const mocked = () => {
    return {
        body: [
            {
                uuid: "8ce64745-5435-4a43-99b1-d2035c4f5de9",
                title: "thai green curry",
                description: "A delicious Thai green curry.",
                ingredients: ["400g coconut milk", "2 tbsp Thai green curry paste", "100g sugar snap peas", "200g chestnut mushrooms, sliced", "300g chicken thighs, chopped into small pieces", "2 kaffir lime leaves", "handful of thai basil", "basmati rice", "1 lime, halved"],
                method: [
                    "Fry the curry paste in some oil for a few minutes.",
                    "Add the chicken to the pan with the curry paste and cook until seared.",
                    "Add the coconut milk and stir to combine.",
                    "Start cooking the basmati rice while the chicken cooks.",
                    "Once the rice is nearly done, add the kaffir lime leaves and squeeze the juice of half a lime into the pan.",
                    "Serve the curry with rice and garnish with a wedge of lime and a sprinkle of thai basil."],
                image: "https://thelemonapron.com/wp-content/uploads/2018/02/Thai-Green-Curry-6-e1518632894974.jpg",
                author: "awd",
                servings: 4
            },
            {
                uuid: "f4c6eb33-2587-4ef2-932e-62e516e02c47",
                title: "biscoff cheesecake",
                image: "https://i.pinimg.com/originals/69/f6/0d/69f60d79c2a88009d192c143093c0ed6.jpg",
                method: [""],
                ingredients: [""],
                description: ""
            },
            {
                uuid: "edd3f0ae-e1e1-44af-a85c-09b83a2aaa3b",
                title: "crispy hoisin duck pancakes",
                image: "https://food-images.files.bbci.co.uk/food/recipes/duckpancakeswithhois_67779_16x9.jpg",
                method: [""],
                ingredients: [""],
                description: ""
            },
            {
                uuid: "161229cf-2bda-4a6d-8477-ece57f2499c9",
                title: "neapolitan pizza",
                image: "https://eightydishes.com/wp-content/uploads/2020/03/Neapolitan-Pizza-Thumbnail.jpg",
                method: [""],
                ingredients: [""],
                description: ""
            }
        ]
    }
}