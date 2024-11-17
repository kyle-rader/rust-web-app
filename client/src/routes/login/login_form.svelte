<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	let email = '';
	let password = '';
	$: ready = email.length > 0 && password.length > 0;

	function login() {
		dispatch('login', { email, password });
	}
</script>

<div class="container center max-w">
	<form on:submit|preventDefault={login}>
		<div class="form-item">
			<label for="email">Email</label>
			<input type="email" id="email" bind:value={email} required />
		</div>
		<div class="form-item">
			<label for="password">Password</label>
			<input type="password" id="password" bind:value={password} required />
		</div>

		<button class="button" class:ready type="submit">Login</button>
	</form>
</div>

<style lang="less">
	@import '../../css/colors.less';
	form {
		display: flex;
		flex-direction: column;
		width: 100%;
		margin: 0 auto;
	}

	.form-item {
		display: flex;
		flex-direction: column;
		margin-bottom: 1.2rem;
	}

	label {
		text-align: left;
		font-size: 1.2rem;
		margin-bottom: 0.5rem;
	}

	input {
		padding: 0.5rem;
		font-size: 1.1rem;
		border-radius: 0.5rem;
		border: none;
	}

	small {
		margin-top: 0.5rem;
		text-align: left;
		font-size: 0.9rem;
	}

	.button {
		padding: 0.5rem;
		font-size: 1.2rem;
		border-radius: 0.5rem;
		border: none;
		background-color: @dark3;
	}
	.button.ready {
		background-color: @green;
	}
	.button.ready::before {
		content: '🚀';
		margin-right: 0.5rem;
	}
	.button.ready::after {
		content: '🎉';
		margin-left: 0.5rem;
	}

	.max-w {
		max-width: 380px;
		margin: auto;
	}
</style>
