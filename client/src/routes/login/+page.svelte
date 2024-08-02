<script lang="ts">
	import LoginForm from './login_form.svelte';
	import { post } from '$lib/requests';
	import type { User } from '$lib/types/user';
	import { user } from '$lib/stores/user';
	import { goto } from '$app/navigation';

	let error: string | null = null;

	async function login(event: any) {
		let { email, password } = event.detail;

		const res = await post('/api/login', { email, password });
		if (res.ok) {
			error = null;
			const user_details: User = await res.json();
			user.login(user_details);
			goto('/');
		} else {
			error = (await res.json()).msg;
		}
	}
</script>

<h1 class="container center">Login</h1>

<LoginForm on:login={login} />

{#if error}
	<div class="container center">
		<h2 class="red">{error}</h2>
	</div>
{/if}
