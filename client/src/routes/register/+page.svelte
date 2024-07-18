<script lang="ts">
	import RegisterForm from './register_form.svelte';
	import { post } from '$lib/requests';
	import type { User } from '$lib/types/user';
	import type { Error } from '$lib/types/error';

	let success: User | null = null;
	let error: Error | null = null;

	async function register(event: any) {
		let res = await post('/api/user/register', event.detail.user);
		console.log(res);
		if (res.ok) {
			error = null;
			success = await res.json();
			console.log(success);
		} else {
			error = await res.json();
			console.log(error);
		}
	}
</script>

<h1 class="container center">Register</h1>

{#if success}
	<p class="container center">Successfully registered as {success.display_name}!</p>
	<p class="container center">
		Please check your email to verify your account and <a href="/login">log in</a>
	</p>
{:else}
	<RegisterForm on:submit={register} />
{/if}

{#if error}
	<p class="container center">❌ Error: {error.msg}</p>
{/if}

<style>
</style>
