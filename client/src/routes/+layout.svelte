<script lang="ts">
	// Import global styles
	import '../css/app.css';
</script>

<div class="app">
	<nav>
		<a href="/" class="bg-green">Home</a>
		<a href="/about" class="bg-blue">About</a>
		<a href="/login" class="bg-orange">Login</a>
		<a href="/register" class="bg-aqua">Register</a>
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
		color: var(--dark1);
	}

	@media (hover: hover) {
		a::before {
			content: '- ';
		}
		a::after {
			content: ' -';
		}

		a::before,
		a::after {
			opacity: 0;
			transition: 0.3s;
		}

		a:hover::before,
		a:hover::after {
			opacity: 100;
		}

		a:hover {
			background-color: var(--light2);
			color: var(--dark2);
		}
	}
</style>
