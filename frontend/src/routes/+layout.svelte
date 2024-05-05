<script lang="ts">
	// Import global styles
	import '../css/app.css';
</script>

<div class="app">
	<nav>
		<a href="/">Home</a>
		<a href="/about">About</a>

		<a href="/login">Login</a>
		<a href="/register">Register</a>
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
		color: var(--blue);
		background-color: var(--dark1);
		padding: 0.5rem 1rem;
		width: 100%;
		text-align: center;
		border-style: groove;
		border-color: var(--dark2);
		border-width: 2px;
		transition: 0.3s;
	}

	a:hover {
		color: var(--blue-bright);
		background-color: var(--dark2);
		transition: 0.3s;
	}
</style>
