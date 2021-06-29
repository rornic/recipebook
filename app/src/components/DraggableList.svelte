<script lang="ts">
	import { Icon } from 'sveltestrap/src';
	export let items: any[];

	let dragging = -1;

	let dragPos = { x: 0, y: 0 };
	const startDrag = (evt, i) => {
		evt.preventDefault();
		dragging = i;
	};

	const stopDrag = () => {
		dragging = -1;
	};

	const move = (evt) => {
		dragPos.x = evt.clientX;
		dragPos.y = evt.clientY;
	};

	const reorder = (evt, i) => {
		let rect = evt.target.getBoundingClientRect();
		if (dragging >= 0) {
			let reorderedItems = items;
			reorderedItems.splice(i, 0, reorderedItems.splice(dragging, 1)[0]);
			items = reorderedItems;
			dragging = i;
		}
	};

	import { quintOut } from 'svelte/easing';
	import { crossfade } from 'svelte/transition';
	import { flip } from 'svelte/animate';

	const [send, receive] = crossfade({
		duration: (d) => Math.sqrt(d * 200),

		fallback(node, params) {
			const style = getComputedStyle(node);
			const transform = style.transform === 'none' ? '' : style.transform;

			return {
				duration: 200,
				easing: quintOut,
				css: (t) => `
					transform: ${transform} scale(${t});
					opacity: ${t}
				`
			};
		}
	});
</script>

<svelte:window on:mouseup={stopDrag} on:mousemove={(e) => move(e)} />

<ol>
	{#each items as item, i (item)}
		<li animate:flip on:mouseenter={(e) => reorder(e, i)}>
			<div
				in:receive={{ key: i }}
				out:send={{ key: i }}
				style={dragging == i ? 'opacity:0;' : ''}
				class="d-flex flex-row"
			>
				<Icon onmousedown={(e) => startDrag(e, i)} class="mx-2 drag-icon" name="list" />
				<div class="flex-grow-1">
					<slot {i} />
				</div>
			</div>
		</li>
	{/each}
	{#if dragging > -1}
		<div
			on:mouseup={(e) => stopDrag()}
			style="pointer-events:none; position:absolute; top:{dragPos.y}px;"
			class="d-flex flex-row dragging"
		>
			<Icon class="mx-2 drag-icon" name="list" />
			<div class="flex-grow-1">
				<slot i={dragging} />
			</div>
		</div>
	{/if}
</ol>

<style>
	li :global(.drag-icon) {
		opacity: 0;
		transition: 0.25s;
		cursor: grab;
	}

	li:hover :global(.drag-icon) {
		opacity: 1;
	}

	ol,
	ul {
		padding-left: 0;
		list-style: none;
	}

	.dragging {
		transform: rotate(10deg);
	}
</style>
