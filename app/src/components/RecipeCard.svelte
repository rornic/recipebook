<script lang="ts">
	import { Card, CardBody, CardImg } from 'sveltestrap/src';

	let clazz = '';
	export { clazz as class };

	export let style;

	export let recipe;
</script>

<a href="/recipes/{recipe.uuid}">
	<Card {...$$restProps} class={`recipe-card hover-grow shadow h-100 ${clazz}`} {style}>
		<CardImg class="recipe-card-img" src={recipe.image} />
		<CardBody>
			<h4>{recipe.title}</h4>
		</CardBody>
	</Card>
</a>

<style>
	a {
		text-decoration: none;
	}

	:global(.recipe-card) {
		cursor: pointer;
	}

	:global(.recipe-card-img) {
		object-fit: cover;
		min-height: 80%;
	}
</style>
