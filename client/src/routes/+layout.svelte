<script lang="ts">
	import '../css/app.less';
	import { user } from '$lib/stores/user';
</script>

<div class="app">
	<nav>
		<a href="/" class="bg-dark2 button">Home</a>

		{#if $user}
			<a href="/stats" class="bg-dark2 button">Stats</a>
			<a href="/profile" class="bg-dark2 button">Profile</a>
			<a href="/logout" class="bg-dark2 button">Logout</a>
		{:else}
			<a href="/about" class="bg-dark2 button">About</a>
			<a href="/register" class="bg-dark2 button">Register</a>
			<a href="/login" class="bg-dark2 button">Login</a>
		{/if}
	</nav>

	<!-- Slot for the page content -->
	<div class="body">
		<slot />
	</div>

	<footer>
		<p>© 2024 Automata Games</p>
	</footer>
</div>

<style lang="less">
	@import '../css/colors.less';
	.app {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}

	nav {
		display: flex;
		justify-content: space-around;
	}

	footer {
		text-align: center;
		padding: 20px;
		margin-top: auto;
	}

	a {
		color: @light0;
		padding: 12px 1rem;
		width: 100%;
		text-align: center;
		transition: 0.3s;
		font-size: 14pt;
	}
</style>
