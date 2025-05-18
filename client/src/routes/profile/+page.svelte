<script lang="ts">
	import ResponsivePad from '$lib/components/utilities/ResponsivePad.svelte';
	import RequiresAuth from '$lib/components/RequiresAuth.svelte';
	import { onMount } from 'svelte';

	let email = '';
	let originalEmail = '';
	let loading = false;
	let error: string | null = null;
	let success: string | null = null;
	let editing = false;

	onMount(async () => {
		loading = true;
		error = null;
		success = null;
		try {
			const res = await fetch('/api/account/me');
			if (!res.ok) throw new Error('Failed to fetch profile');
			const data = await res.json();
			email = data.email;
			originalEmail = data.email;
		} catch (e) {
			error = (e as any).message || 'Unknown error';
		} finally {
			loading = false;
		}
	});

	async function saveEmail() {
		loading = true;
		error = null;
		success = null;
		try {
			const res = await fetch('/api/account/me', {
				method: 'PATCH',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email })
			});
			if (!res.ok) {
				const err = await res.json();
				throw new Error(err.error || 'Failed to update email');
			}
			const data = await res.json();
			success = 'Email updated!';
			originalEmail = data.email;
			editing = false;
		} catch (e) {
			error = (e as any).message || 'Unknown error';
		} finally {
			loading = false;
		}
	}
</script>

<RequiresAuth>
	<ResponsivePad />
	<div class="container center" style="max-width: 400px; margin-top: 2rem;">
		<h1>Profile</h1>

		{#if loading}
			<p>Loading...</p>
		{:else}
			<form on:submit|preventDefault={saveEmail} style="margin-top: 2rem;">
				<label for="email">Email</label>
				<div style="display: flex; gap: 0.5rem; align-items: center;">
					<input
						id="email"
						type="email"
						bind:value={email}
						disabled={!editing}
						required
						style="flex:1;"
					/>
					{#if !editing}
						<button
							type="button"
							on:click={() => {
								editing = true;
							}}
						>
							Edit
						</button>
					{:else}
						<button type="submit" disabled={email === originalEmail || loading}>
							Save
						</button>
						<button
							type="button"
							on:click={() => {
								email = originalEmail;
								editing = false;
							}}
						>
							Cancel
						</button>
					{/if}
				</div>
			</form>
			{#if error}
				<p style="color: red;">{error}</p>
			{/if}
			{#if success}
				<p style="color: green;">{success}</p>
			{/if}
		{/if}
	</div>
</RequiresAuth>
