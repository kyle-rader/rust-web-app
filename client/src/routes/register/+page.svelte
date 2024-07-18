<script lang="ts">
	import RegisterForm from './register_form.svelte';
	import { post } from '$lib/requests.js';

	const MinPasswordLength = 12;
	const RegisterUri = '/api/user/register';

	let error: String | null = null;

	function register(event: any) {
		console.log('Got user:', event.detail.user);
	}
</script>

<RegisterForm minPasswordLength={MinPasswordLength} on:submit={register} />

<div class="container">
	<p class="center">
		<small>Already have an account? <a href="/login">Log in</a></small>
	</p>

	<div class="container center">
		<h3>Why register?</h3>
		<p>You will need an account to play solo games or host multiplayer games.</p>
	</div>
</div>
