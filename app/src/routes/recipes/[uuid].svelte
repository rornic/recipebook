<script lang="ts">
	import type RecipeStore from '../../store/recipes';
	import type { IRecipe } from '../../store/recipes';
	import Recipe from '../../components/Recipe.svelte';
	import { page } from '$app/stores';

	import { getContext } from 'svelte';
	import { goto } from '$app/navigation';

	const recipeStore: RecipeStore = getContext<RecipeStore>('recipeStore');

	let recipe: IRecipe;
	const unsubscribe = recipeStore.state.subscribe((state) => {
		recipe = state.recipes.find((r) => r.uuid == $page.params.uuid);
	});

	let onUpdate = async () => {
		let res = await fetch(`${recipe.uuid}.json`, { method: 'PUT', body: JSON.stringify(recipe) });
		if (res.ok) {
			console.debug(`Successfully updated recipe ${recipe.uuid}`);
			let updatedRecipe = await res.json();
			recipeStore.updateRecipe(updatedRecipe);
		} else {
			let message = await res.json();
			recipeStore.error(`Failed to update recipe ${recipe.uuid}: ${message}`);
		}
	};

	let onDelete = async () => {
		let res = await fetch(`${recipe.uuid}.json`, { method: 'DELETE' });
		if (res.ok) {
			console.debug(`Successfully deleted recipe ${recipe.uuid}`);
			recipeStore.deleteRecipe(recipe);
			goto('/');
		} else {
			let message = await res.json();
			recipeStore.error(`Failed to delete recipe ${recipe.uuid}: ${message}`);
		}
	};

	let onUploadImage = (event) => {
		let file = event.target.files[0];
		let reader = new FileReader();
		reader.onload = async (evt) => {
			try {
				// First, get presigned upload URL
				const data = {
					file_name: file.name
				};

				let urlRes = await fetch('upload.json', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify(data)
				});
				let urlJson = await urlRes.json();

				// Next, upload file to URL
				let uploadRes = await fetch(urlJson.uploadUrl, {
					method: 'PUT',
					headers: {
						'X-Amz-Acl': 'public-read',
						'Content-Type': file.type
					},
					body: reader.result
				});
				recipe.image = urlJson.url;
				onUpdate();
			} catch (e) {
				console.debug(e);
			}
		};
		reader.readAsArrayBuffer(file);
	};
</script>

<Recipe {onUploadImage} {onDelete} {onUpdate} {recipe} />
