<script lang="ts">
	import ResponsivePad from '$lib/components/utilities/ResponsivePad.svelte';
</script>

<ResponsivePad />

<h1 class="container center">Automata Games</h1>

<p class="center">Laugh - Solve - Learn</p>

<div class="container options center">
	<div class="option">
		<input type="text" placeholder="Game Code" />
		<button class="button">Join Game</button>
	</div>
	<div class="option">
		<h2>Host</h2>
		<button class="button">Create Lobby</button>
	</div>
	<div class="option">
		<h2>Play Solo</h2>
		<button class="button">Choose Game</button>
	</div>
</div>

<style lang="less">
	@import '../css/colors.less';
	@import '../css/vars.less';

	.options {
		display: flex;
		flex-flow: column;
		margin: auto;
	}

	button,
	input {
		width: 100%;
		text-align: center;
		border: none;
		border-radius: 6px;
		margin-top: 8px;
		margin: none;
		box-sizing: border-box;
		padding: 8px;
		font-size: 18pt;
	}

	input {
		font-variant: small-caps;
	}

	button {
		color: @dark0;
		background-color: @green-bright;
		text-decoration: none;
		cursor: pointer;
	}

	/* Mobile */
	@media (max-width: @mobile) {
		.options {
			width: 60%;
		}

		.option {
			margin-top: 25px;
		}
	}

	/* Desktop + Tablet */
	@media (min-width: @tablet-min) {
		.options {
			width: 250px;
		}
		.option {
			margin-top: 30px;
		}
	}
</style>
