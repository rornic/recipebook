<script lang="ts">
	import { goto } from '$app/navigation';
	import { API_URL } from '$lib/constants';
	import type { IRecipe } from 'src/store/recipes';

	import type RecipeStore from 'src/store/recipes';

	import { getContext } from 'svelte';

	import {
		Container,
		Row,
		Col,
		Toast,
		ToastBody,
		ToastHeader,
		Button,
		Icon,
		Spinner
	} from 'sveltestrap/src';

	import RecipeCard from '../../components/RecipeCard.svelte';

	const recipeStore: RecipeStore = getContext<RecipeStore>('recipeStore');
	const state = recipeStore.state;

	const addRecipe = async () => {
		loadingNewRecipe = true;
		let res = await fetch('recipes.json', { method: 'POST' });
		loadingNewRecipe = false;

		if (res.ok) {
			let recipe: IRecipe = await res.json();
			recipeStore.addRecipe(recipe);
			goto(`/recipes/${recipe.uuid}`);
		} else {
			let message = await res.text();
			recipeStore.error(message);
		}
	};

	let loadingNewRecipe = false;
</script>

<Container class="p-0">
	<Row class="p-4 m-0">
		{#each $state.recipes as recipe}
			<Col xs="auto" class="p-4">
				<RecipeCard style="max-width: 300px; max-height: 400px;" {recipe} />
			</Col>
		{/each}
		<Col
			xs="auto"
			class="p-4 m-4 d-flex align-items-center justify-content-center"
			style="width:300px;"
		>
			<Button color="primary" disabled={loadingNewRecipe} on:click={addRecipe}>
				{#if loadingNewRecipe}
					<Spinner size="sm" />
				{:else}
					<Icon name="plus-circle-fill" />
				{/if}
				Add
			</Button>
		</Col>
	</Row>
</Container>
