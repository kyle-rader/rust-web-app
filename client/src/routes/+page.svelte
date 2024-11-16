<script lang="ts">
</script>

<h1 class="container center">Automata Games</h1>

<div class="container center">
	<div class="game-option">
		<h2>Join</h2>
		<input type="text" placeholder="Game Code" />
	</div>
	<div class="game-option">
		<h2>Host</h2>
		<button>Create Lobby</button>
	</div>
	<div class="game-option">
		<h2>Play</h2>
		<button>Choose Game</button>
	</div>
</div>

<style>
	.game-option {
		display: inline-block;
		margin: 20px;
		padding: 20px;
		border: 1px solid var(--green);
		border-radius: 10px;
		width: 200px;
		max-height: 175px;
	}

	input {
		display: block;
		text-align: center;
		width: 150px;
		margin: 10px auto;
		padding: 8px;
		font-size: 20px;
		font-variant: small-caps;
	}
	button {
		background-color: var(--green);
		border: none;
		color: white;
		padding: 15px 32px;
		text-align: center;
		text-decoration: none;
		display: inline-block;
		font-size: 16px;
		margin: 10px auto;
		cursor: pointer;
	}
</style>
