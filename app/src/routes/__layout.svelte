<script context="module">
	export const load = async ({ page }) => ({
		props: {
			key: page.path
		}
	});
</script>

<script lang="ts">
	import RecipeStore from '../store/recipes';
	import { setContext } from 'svelte';
	import PageTransition from '../components/PageTransition.svelte';
	import { Col, Row, Toast, ToastHeader, ToastBody, Navbar, NavbarBrand } from 'sveltestrap/src';

	const recipeStore = new RecipeStore();
	setContext<RecipeStore>('recipeStore', recipeStore);
	const fetchRecipes = async () => {
		try {
			const url = '/recipes.json';
			const res = await fetch(url);
			if (res.ok) {
				const recipes = await res.json();
				recipeStore.state.set({ recipes: recipes, error: '' });
			} else {
				let error = await res.text();
				recipeStore.error(error);
			}
		} catch (error) {
			recipeStore.error(error);
		}
	};

	fetchRecipes().then(() => {
		console.debug('Fetched state from backend.');
	});

	export let key;

	const state = recipeStore.state;
</script>

<svelte:head>
	<title>Pocket Recipes</title>
	<link rel="preconnect" href="https://fonts.gstatic.com" />
	<link href="https://fonts.googleapis.com/css2?family=Arvo&display=swap" rel="stylesheet" />
	<link href="https://fonts.googleapis.com/css2?family=Open+Sans&display=swap" rel="stylesheet" />
	<link
		rel="stylesheet"
		href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.1/dist/css/bootstrap.min.css"
	/>
	<link
		rel="stylesheet"
		href="https://cdn.jsdelivr.net/npm/bootstrap-icons@1.4.1/font/bootstrap-icons.css"
	/>
</svelte:head>

<Navbar color="primary">
	<NavbarBrand class="text-white" href="/recipes"
		><img
			alt="Pocket recipes icon"
			class="mx-2"
			style="width:2em;height:auto;display:inline-block;"
			src="/icon.png"
		/>Pocket Recipes</NavbarBrand
	>
</Navbar>
<PageTransition refresh={key}>
	{#if $state.error}
		<div
			class="p-3 d-flex justify-content-center align-items-center"
			style="position:absolute; top:0; z-index: 1"
		>
			<Toast duration={2}>
				<ToastHeader class="bg-danger text-white">Error</ToastHeader>
				<ToastBody class="text-danger bg-white">{$state.error}</ToastBody>
			</Toast>
		</div>
	{/if}
	<slot />
</PageTransition>

<style>
	:global(h1, h2, h3, h4, h5, h6, .btn, .h1, .h2, .h3, .h4, .h5, .h6) {
		font-family: 'Arvo';
		color: #5e5b52;
	}

	:global(p) {
		font-family: 'Open Sans';
		color: #5e5b52;
	}

	:global(html, body) {
		height: 100vh;
	}

	:global(.hover-grow) {
		transition: 0.2s;
	}

	:global(.hover-grow:hover) {
		transform: scale(1.05);
	}

	:global(.hover-shrink) {
		transition: 0.2s;
	}

	:global(.hover-shrink:hover) {
		transform: scale(0.95);
	}

	:global(.input) {
		font-size: 100% !important;
		font-family: inherit !important;
		color: inherit !important;
		text-transform: inherit;
	}

	:global(.form-control:focus) {
		background-color: white !important;
	}
</style>
