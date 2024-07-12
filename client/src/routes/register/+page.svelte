<script>
	let displayName = '';
	let email = '';
	let password = '';
	let confirmPassword = '';

	function register() {
		alert(
			`TODO:\nRegistering\nHandle: ${displayName}\nEmail: ${email}\nPassword: ${password}\nConfirm Password: ${confirmPassword}`
		);
	}
</script>

<h1 class="container center">Register</h1>

<div class="container center max-w">
	<form on:submit|preventDefault={register}>
		<div class="form-item">
			<label for="displayName">Display Name</label>
			<input type="text" id="displayName" required bind:value={displayName} />
			<small>👀 Other players will see this.</small>
		</div>

		<div class="form-item">
			<label for="email">Email</label>
			<input type="email" id="email" required bind:value={email} />
			<small>🔒 Other players will not see this.</small>
		</div>

		<div class="form-item">
			<label for="password">Password</label>
			<input type="password" id="password" required bind:value={password} />
			<small>🔐 At least 12 characters long.</small>
		</div>

		<div class="form-item">
			<label for="confirmPassword">Confirm Password</label>
			<input type="password" id="confirmPassword" required bind:value={confirmPassword} />
			<small>🔐 The same characters again.</small>
		</div>
		<div class="form-item">
			<button class="button" type="submit">Register</button>
		</div>
	</form>

	<p class="center">
		<small>Already have an account? <a href="/login">Log in</a></small>
	</p>

	<div class="container center">
		<h3>Why register?</h3>
		<p>You will need an account to play solo games or host multiplayer games.</p>
	</div>
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
		background-color: var(--green);
	}

	.max-w {
		max-width: 380px;
		margin: auto;
	}
</style>
