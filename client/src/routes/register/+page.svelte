<script>
</script>

<h1 class="container center">Register</h1>

<div class="container center">
	<form>
		<div class="form-item">
			<label for="handle">Display Name</label>
			<input class="textbox" type="text" id="handle" name="handle" required />
			<small>👀 Other players will see this.</small>
		</div>

		<div class="form-item">
			<label for="email">Email</label>
			<input class="textbox" type="email" id="email" name="email" required />
			<small>🔒 Other players will not see this.</small>
		</div>

		<div class="form-item">
			<label for="password">Password</label>
			<input class="textbox" type="password" id="password" name="password" required />
		</div>

		<div class="form-item">
			<label for="confirmPassword">Confirm Password</label>
			<input
				class="textbox"
				type="password"
				id="confirmPassword"
				name="confirmPassword"
				required
			/>
		</div>
		<div class="form-item">
			<button class="button" type="submit">Register</button>
		</div>
	</form>
</div>

<h3 class="container center">Why register?</h3>
<p class="container center">
	You will need an account to host multiplayer games or to play solo games.
</p>

<style>
	form {
		display: flex;
		flex-direction: column;
		width: 100%;
		max-width: 360px;
		margin: 0 auto;
	}

	.form-item {
		display: flex;
		flex-direction: column;
		margin-bottom: 1.2rem;
	}

	.form-item label {
		text-align: left;
		font-size: 1.2rem;
		margin-bottom: 0.5rem;
	}

	.form-item input {
		padding: 0.5rem;
		font-size: 1.1rem;
		border-radius: 0.5rem;
		border: none;
	}

	.form-item small {
		margin-top: 0.5rem;
		text-align: left;
	}

	.button {
		padding: 0.5rem;
		font-size: 1.2rem;
		border-radius: 0.5rem;
		border: none;
		background-color: var(--green);
	}
</style>
