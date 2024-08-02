<script lang="ts">
	// Import global styles
	import '../css/app.css';

	import { user } from '$lib/stores/user';
</script>

<div class="app">
	<nav>
		<a href="/" class="bg-purple button">Home</a>

		{#if $user !== null}
			<a href="/games" class="bg-blue button">Games</a>
			<a href="/profile" class="bg-orange button">Profile</a>
			<a href="/logout" class="bg-aqua button">Logout</a>
		{:else}
			<a href="/about" class="bg-blue button">About</a>
			<a href="/login" class="bg-orange button">Login</a>
			<a href="/register" class="bg-aqua button">Register</a>
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

<style>
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
		padding: 12px 1rem;
		width: 100%;
		text-align: center;
		transition: 0.3s;
	}
</style>
