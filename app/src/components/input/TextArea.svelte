<script lang="ts">
	import { onMount } from 'svelte';
	import Input from 'sveltestrap/src/Input.svelte';

	let className;
	export { className as class };

	export let value: string;
	export let change: () => void;
	export let placeholder = '';

	let textarea;
	const resize = () => {
		textarea.style.height = 'auto';
		textarea.style.height = textarea.style.lineHeight / 2 + textarea.scrollHeight + 5 + 'px';
	};

	onMount(() => {
		resize();
	});
</script>

<Input
	bind:inner={textarea}
	rows="1"
	{placeholder}
	on:change={(event) => {
		resize();
		change();
	}}
	on:input={resize}
	on:focus={resize}
	on:blur={resize}
	type="textarea"
	class="input textarea p-0 bg-transparent border-0 {className}"
	bind:value
/>

<style>
	:global(.textarea) {
		resize: none;
		width: 100%;
		max-height: 100%;
		overflow: hidden;
		box-sizing: border-box;
	}
</style>
