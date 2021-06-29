import { API_URL } from '$lib/constants';
import { get, writable, Writable, Readable } from 'svelte/store';

export interface IRecipe {
    uuid: string
    title: string
    description: string
    ingredients: string[]
    method: string[]
    image: string
    author: string
    servings: string
}

export type State = {
    recipes: IRecipe[],
    error: string
}

export default class RecipeStore {
    state: Writable<State>;

    constructor() {
        this.state = writable({ recipes: [], error: '' });
    }

    addRecipe = (recipe) => {
        this.state.update((oldState) => {
            return {
                recipes: [
                    ...oldState.recipes,
                    recipe],
                error: ''
            };
        });
    }

    updateRecipe = (recipe) => {
        this.state.update((oldState) => {
            let index = oldState.recipes.findIndex(r => r.uuid == recipe.uuid);
            if (index > -1) {
                oldState.recipes.splice(index, 1, recipe)
                return {
                    recipes: oldState.recipes,
                    error: ''
                }
            }
        });
    }

    deleteRecipe = (recipe) => {
        this.state.update((oldState) => {
            let index = oldState.recipes.findIndex(r => r.uuid == recipe.uuid)
            if (index > -1) {
                oldState.recipes.splice(index, 1);
                return { recipes: oldState.recipes, error: '' };
            }
        });
    }

    error = (error) => {
        this.state.update((oldState) => {
            return { recipes: oldState.recipes, error: error };
        });
    }

}