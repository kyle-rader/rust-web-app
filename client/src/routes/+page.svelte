<script lang="ts">
	import ResponsivePad from '$lib/components/utilities/ResponsivePad.svelte';
</script>

<ResponsivePad />

<h1 class="container center">RustWebApp</h1>

<p class="center">Build - Deploy - Scale</p>

<div class="container options center">
	<div class="option">
		<h2>Quick Start</h2>
		<button class="button">Get Started</button>
	</div>
	<div class="option">
		<h2>Documentation</h2>
		<button class="button">View Docs</button>
	</div>
	<div class="option">
		<h2>Examples</h2>
		<button class="button">Browse Examples</button>
	</div>
</div>

<style lang="less">
	@import '../css/colors.less';
	@import '../css/vars.less';

	p {
		margin-top: 15px;
	}

	.options {
		display: flex;
		flex-flow: column;
		width: 290px;
		margin: auto;
	}

	.option {
		border: 2px solid @green-bright;
		box-shadow: @green-bright -2px -2px 0px 2px;
		padding: 15px;
	}

	button,
	input {
		width: 100%;
		text-align: center;
		border: none;
		border-radius: 6px;
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
		margin-top: 12px;
	}

	/* Mobile */
	@media (max-width: @mobile) {
		.option {
			margin-top: 25px;
		}
	}

	/* Desktop + Tablet */
	@media (min-width: @tablet-min) {
		.option {
			margin-top: 30px;
		}
	}
</style>
