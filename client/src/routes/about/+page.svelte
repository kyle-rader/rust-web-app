<script lang="ts">
	import TitlePad from '$lib/components/utilities/ResponsivePad.svelte';
	// fetch the api status from /api/status
	async function fetchStatus() {
		const res = await fetch('/api/status');
		const data = await res.json();
		return data;
	}

	let status = '';
	fetchStatus()
		.then((data) => {
			status = JSON.stringify(data, null, 2);
		})
		.catch((err) => {
			status = 'Error fetching API status.';
		});
</script>

<TitlePad />
<h1 class="container center">About RustWebApp</h1>
<div class="container">
	<div class="container">
		<h2>API Status</h2>
		{#if status === ''}
			<p>Loading...</p>
		{:else}
			<code>
				<pre>{status}</pre>
			</code>
		{/if}
	</div>

	<p>
		RustWebApp is a game development company that focuses on creating games that are fun and
		engaging. We believe that games should be accessible to everyone, which is why we strive to
		create games that are easy to learn and play. Our goal is to create games that bring people
		together and provide a sense of community.
	</p>
</div>

<style>
	p {
		margin-top: 10px;
		font-weight: 300;
		line-height: 1.3em;
	}
	.container {
		margin: 10px auto;
		max-width: 800px;
	}
</style>
