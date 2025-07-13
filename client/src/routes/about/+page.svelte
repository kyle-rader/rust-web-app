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
		RustWebApp is an early-stage template repository designed to accelerate the development of
		modern web applications. It combines the performance and safety of Rust on the backend with
		the reactivity and developer experience of SvelteJS on the frontend.
	</p>

	<p>
		This template implements an islands architecture, allowing you to build interactive web
		applications with server-side rendering and client-side hydration where needed. It includes
		authentication, database integration, API development, and a complete development
		environment with Docker support.
	</p>

	<p>
		While still in early development, this template provides a solid foundation for building
		SaaS applications, content management systems, or any other web service. It's designed to
		get you started quickly while maintaining the flexibility to scale as your needs grow.
	</p>

	<p>
		<strong>Note:</strong> This is a work in progress. You may encounter rough edges or missing features,
		but the core architecture is stable and ready for development.
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
