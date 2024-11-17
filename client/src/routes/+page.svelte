<script lang="ts">
</script>

<h1 class="container center">Automata Games</h1>

<div class="container options center">
	<div class="option">
		<h2>Join</h2>
		<input type="text" placeholder="Game Code" />
		<button class="button">Join Game</button>
	</div>
	<div class="option">
		<h2>Host</h2>
		<button class="button">Create Lobby</button>
	</div>
	<div class="option">
		<h2>Play</h2>
		<button class="button">Choose Game</button>
	</div>
</div>

<style>
	/* Mobile */
	@media (max-width: 600px) {
		.options {
			width: 95%;
			margin: auto;
		}

		.option {
			display: block;
			margin-top: 6px;
		}
	}

	/* Tablet */
	@media (min-width: 601px) and (max-width: 1024px) {
		.options {
			width: 50%;
		}
		.option {
			display: inline-block;
		}
	}

	/* Desktop */
	@media (min-width: 1025px) {
		.options {
			width: 30%;
		}
		.option {
			display: inline-block;
		}
	}

	button,
	input {
		width: 100%;
		text-align: center;
		border: none;
		border-radius: 6px;
	}

	input {
		/* font-variant: small-caps; */
	}

	button {
		color: var(--dark0);
		background-color: var(--green);
		text-decoration: none;
		cursor: pointer;
	}
</style>
