<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	export let minPasswordLength = 12;
	let user = {
		display_name: '',
		email: '',
		password: ''
	};

	let confirm_password: String = '';

	$: passwordsNotEmpty = user.password.length > 0 && confirm_password.length > 0;
	$: passwordsMatch = user.password === confirm_password;
	$: passwordLen = Math.min(user.password.length, confirm_password.length);
	$: passwordsLongEnough = passwordLen >= minPasswordLength;
	$: needsMoreCharacters = Math.max(0, minPasswordLength - passwordLen);

	$: ready = passwordsNotEmpty && passwordsMatch && passwordsLongEnough;

	function submit() {
		dispatch('submit', {
			user
		});
	}
</script>

<div class="container center max-w">
	<form on:submit={submit}>
		<div class="form-item">
			<label for="displayName">Display Name</label>
			<input type="text" id="displayName" required bind:value={user.display_name} />
			<small>👀 Other players will see this.</small>
		</div>

		<div class="form-item">
			<label for="email">Email</label>
			<input type="email" id="email" required bind:value={user.email} />
			<small>🔒 Other players will not see this.</small>
		</div>

		<div class="form-item">
			<label for="password">Password</label>
			<input type="password" id="password" required bind:value={user.password} />
			<small> 🔐 At least 12 characters long. </small>
		</div>

		<div class="form-item">
			<label for="confirmPassword"> Confirm Password</label>

			<input type="password" id="confirmPassword" required bind:value={confirm_password} />
			{#if passwordsNotEmpty}
				<small>
					{#if passwordsMatch}
						✅ Passwords match!
					{:else}
						❌ Passwords do not match!
					{/if}
				</small>
				<small>
					{#if passwordsLongEnough}
						✅ Password is long enough!
					{:else}
						❌ Password needs {needsMoreCharacters} more characters.
					{/if}
				</small>
			{/if}
		</div>
		<div class="form-item">
			<button type="submit" class="button" class:ready disabled={!ready}> Register </button>
		</div>
	</form>
</div>

<style>
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
		background-color: var(--light2);
	}
	.button.ready {
		background-color: var(--green);
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
