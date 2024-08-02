<script lang="ts">
	import { goto } from '$app/navigation';
	import { user } from '$lib/stores/user';

	function logout() {
		user.logout();
		// redirect to Games Page
		goto('/games');
	}
</script>

<div class="container center">
	<h2>Confirm Logout</h2>
	<p>
		<button class="button flat rounded padded bg-green" on:click={logout}>Really logout</button>
	</p>
</div>

<style>
	button {
		margin-top: 1em;
		font-size: 1em;
	}
</style>
