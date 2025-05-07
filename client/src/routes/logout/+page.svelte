<script lang="ts">
	import ResponsivePad from '$lib/components/utilities/ResponsivePad.svelte';
	import { goto } from '$app/navigation';
	import { user } from '$lib/stores/user';

	function logout() {
		user.logout();
		// redirect to Games Page
		goto('/');
	}
</script>

<ResponsivePad />

<div class="container center">
	<h2>Logout?</h2>
	<p>
		<button class="button flat rounded padded bg-red" on:click={logout}>Yes, Logout</button>
	</p>
</div>

<style lang="less">
	@import '../../css/colors.less';
	button {
		margin-top: 1em;
		font-size: 1em;
		color: @light1;
	}
</style>
