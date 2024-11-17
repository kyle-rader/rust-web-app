<script lang="ts">
	import ResponsivePad from '$lib/components/utilities/ResponsivePad.svelte';
	import RequiresAuth from '$lib/components/RequiresAuth.svelte';
</script>

<RequiresAuth>
	<ResponsivePad />
	<div class="container center">
		<h1>Profile</h1>
	</div>
</RequiresAuth>
