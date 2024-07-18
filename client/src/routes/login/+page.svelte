<script lang="ts">
	import LoginForm from './login_form.svelte';
	import { post } from '$lib/requests';

	async function login(event: any) {
		let { email, password } = event.detail;
		console.log('Logging in', email, password);

		const res = await post('/api/login', { email, password });
		if (res.ok) {
			const { token } = await res.json();
			console.log('Logged in!', token);
		} else {
			console.log('Login failed');
		}
	}
</script>

<h1 class="container center">Login</h1>

<LoginForm on:login={login} />
