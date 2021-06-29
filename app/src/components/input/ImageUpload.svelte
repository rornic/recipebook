<script lang="ts">
	import { Button, Icon } from 'sveltestrap/src';
	let input;
</script>

<input
	bind:this={input}
	on:change
	type="file"
	accept="image/png,image/jpeg"
	capture="environment"
	style="display:none"
/>
<div class="container">
	<img class="image img-fluid" alt="" {...$$restProps} />
	<div class="middle">
		<Button color="primary" on:click={() => input.click()}>
			<Icon name="upload" />
			Upload
		</Button>
	</div>
</div>

<style>
	.container {
		position: relative;
	}

	.image {
		opacity: 1;
		display: block;
		width: 100%;
		height: auto;
		transition: 0.5s ease;
		backface-visibility: hidden;
		object-fit: cover;
	}

	.container:hover .image {
		opacity: 0.3;
	}

	.container:hover .middle {
		opacity: 1;
	}

	.middle {
		transition: 0.5s ease;
		opacity: 0;
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		-ms-transform: translate(-50%, -50%);
		text-align: center;
	}
</style>
