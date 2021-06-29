<script context="module" lang="ts">
	export async function load({ page, session, fetch, context }) {
		return {
			status: 302,
			redirect: '/recipes'
		};
	}
</script>
