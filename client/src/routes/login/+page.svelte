<script lang="ts">
	import LoginForm from './login_form.svelte';
	import { post } from '$lib/requests';
	import type { User } from '$lib/types/user';
	import { user } from '$lib/stores/user';
	import { goto } from '$app/navigation';

	async function login(event: any) {
		let { email, password } = event.detail;
		console.log('Logging in', email, password);

		const res = await post('/api/login', { email, password });
		if (res.ok) {
			const user_details: User = await res.json();
			console.log('Logged in!', user_details);
			user.login(user_details);
			goto('/games');
		} else {
			console.log('Login failed');
		}
	}
</script>

<h1 class="container center">Login</h1>

<LoginForm on:login={login} />
