<script lang="ts">
	import { user } from '$lib/stores/user';
</script>

{#if $user !== null}
	<slot />
{:else}
	<div class="container center">
		<h1>Login Required</h1>
		<p>You must be logged in to view this page.</p>
	</div>
{/if}
