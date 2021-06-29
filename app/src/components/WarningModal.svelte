<script lang="ts">
	import { Button, Modal, ModalBody, ModalFooter, ModalHeader } from 'sveltestrap/src';

	export let proceed;

	export let open = false;
	const close = () => (open = false);
	const toggle = () => (open = !open);

	export let header = 'Are you sure?';
</script>

<div>
	<Modal isOpen={open} {toggle}>
		<ModalHeader>{header}</ModalHeader>
		<ModalBody><slot /></ModalBody>
		<ModalFooter>
			<Button color="danger" on:click={close}>Cancel</Button>
			<Button
				color="primary"
				on:click={() => {
					proceed();
					close();
				}}>Ok</Button
			>
		</ModalFooter>
	</Modal>
</div>
