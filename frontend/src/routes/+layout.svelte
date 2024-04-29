<script lang="ts">
	// Import global styles
	import '../css/app.css';
</script>

<nav>
	<a href="/">Home</a>
	<a href="/about">About</a>

	<a href="/login">Login</a>
	<a href="/register">Register</a>
</nav>

<slot />

<style>
	nav {
		display: flex;
		justify-content: space-around;
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
