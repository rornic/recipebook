<script lang="ts">
	import { Button, Col, Container, Icon, Input, Label, Row } from 'sveltestrap/src';
	import TextArea from '../components/input/TextArea.svelte';
	import ImageUpload from '../components/input/ImageUpload.svelte';
	import type { IRecipe } from '../store/recipes';
	import WarningModal from './WarningModal.svelte';
	import DraggableList from './DraggableList.svelte';

	export let recipe: IRecipe;
	export let onUpdate;
	export let onDelete;
	export let onUploadImage;

	const addStep = () => {
		recipe.method = [...recipe.method, ''];
		update();
	};

	const addIngredient = () => {
		recipe.ingredients = [...recipe.ingredients, ''];
		update();
	};

	let servingsInput: HTMLInputElement;
	const update = () => {
		let valid = true;
		if (!servingsInput.value.match('^[0-9]+$')) {
			valid = false;
			servingsInput.classList.add('is-invalid');
		} else {
			servingsInput.classList.remove('is-invalid');
		}
		valid && onUpdate();
	};

	let deleteWarningOpen = false;
</script>

{#if recipe}
	<Container fluid>
		<Row class="bg-header text-white p-4 justify-content-center">
			<Col md="auto" class="p-2 justify-content-center">
				<ImageUpload
					on:change={onUploadImage}
					alt={recipe.title}
					src={recipe.image}
					style="width:300px;max-height:300px"
				/>
			</Col>
			<Col md="4" class="p-2 d-flex flex-column">
				<h2 class="m-0">
					<Input
						on:change={update}
						placeholder="title"
						class="input p-0 bg-transparent border-0"
						type="text"
						bind:value={recipe.title}
					/>
				</h2>
				<small class="text-muted">By {recipe.author}</small>
				<p class="mt-4">
					<TextArea change={update} placeholder="description" bind:value={recipe.description} />
				</p>
				<div class="flex-grow-1" />
				<div>
					<Button color="danger" on:click={() => (deleteWarningOpen = true)}>
						<Icon name="trash" />
						Delete
					</Button>
					<WarningModal proceed={onDelete} bind:open={deleteWarningOpen}
						>If you delete this recipe, it will be gone forever.</WarningModal
					>
				</div>
			</Col>
		</Row>
		<Row class="mt-4 p-4 justify-content-center">
			<Col md="auto">
				<h6>
					Serves <Input
						bind:inner={servingsInput}
						class="input bg-transparent border-0"
						style="display:inline;width:10ch;"
						min="1"
						max="32"
						on:change={update}
						type="number"
						id="servings"
						bind:value={recipe.servings}
					/>
					<div id="servingsFeedback" class="invalid-feedback">Please input a number.</div>
				</h6>
				<ul>
					{#each recipe.ingredients as _, i}
						<li>
							<p class="m-0">
								<TextArea
									change={update}
									placeholder="ingredient"
									bind:value={recipe.ingredients[i]}
								/>
							</p>
							<hr class="mt-0 w-75" />
						</li>
					{/each}
					<Button class="btn-circle" color="primary" on:click={addIngredient}>
						<Icon name="plus-circle-fill" />
						Add
					</Button>
				</ul>
			</Col>
			<Col md="1" />
			<Col md="4">
				<div class="ml-6">
					<DraggableList onReorder={update} let:i bind:items={recipe.method}>
						<h6>Step {i + 1}</h6>
						<p style="padding-left:0em;">
							<TextArea change={update} placeholder="step" bind:value={recipe.method[i]} />
						</p>
					</DraggableList>
					<Button color="primary" on:click={addStep}>
						<Icon name="plus-circle-fill" />
						Add
					</Button>
				</div>
			</Col>
		</Row>
	</Container>
{/if}

<style>
	:global(.method-header:hover .btn-delete-step) {
		opacity: 1;
	}
	:global(.method-header .btn-delete-step) {
		opacity: 0;
		transition: 0.2s;
	}

	ol,
	ul {
		padding-left: 0;
		list-style: none;
	}

	:global(.bg-header) {
		background-color: #e4e8ec;
	}
</style>
