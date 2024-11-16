<script lang="ts">
</script>

<h1 class="container center">Automata Games</h1>

<div class="options center align-middle">
	<div class="game-option">
		<h2>Join</h2>
		<input type="text" placeholder="Game Code" />
		<button class="button">Join Game</button>
	</div>
	<div class="game-option">
		<h2>Host</h2>
		<button class="button">Create Lobby</button>
	</div>
	<div class="game-option">
		<h2>Play</h2>
		<button class="button">Choose Game</button>
	</div>
</div>

<style>
	.options {
		display: flex;
		justify-content: center;
	}

	.game-option {
		margin: 20px;
		padding: 20px;
		width: 200px;
		justify-content: center;
	}

	input {
		text-align: center;
		width: 100%;
		margin: 12px 0 0 0;
		padding: 8px 0px;
		border: none;
		border-radius: 6px;
		font-size: 20px;
		font-variant: small-caps;
	}

	button {
		border: none;
		color: var(--dark0);
		background-color: var(--green);
		width: 100%;
		margin: 8px 0px;
		padding: 12px 0px;
		border-radius: 6px;
		text-align: center;
		text-decoration: none;
		font-size: 18px;
		cursor: pointer;
	}
</style>
