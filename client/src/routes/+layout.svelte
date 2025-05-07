<script lang="ts">
	import '../css/app.less';
	import { user } from '$lib/stores/user';
</script>

<div class="app">
	<nav>
		<a href="/" class="bg-dark2 button"><i class="fas fa-home blue-bright"></i>Home</a>
		{#if $user}
			<a href="/stats" class="bg-dark2 button"
				><i class="far fa-chart-line orange-bright"></i>Stats</a
			>
			<a href="/profile" class="bg-dark2 button"
				><i class="fas fa-user green-bright"></i>Profile</a
			>
			<a href="/logout" class="bg-dark2 button"
				><i class="fad fa-portal-enter aqua"></i>Logout</a
			>
		{:else}
			<a href="/about" class="bg-dark2 button"
				><i class="far fa-info-circle orange-bright"></i>About</a
			>
			<a href="/register" class="bg-dark2 button"
				><i class="fad fa-user-plus green-bright"></i>Register</a
			>
			<a href="/login" class="bg-dark2 button"
				><i class="fad fa-portal-enter aqua"></i>Login</a
			>
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
	@import '../css/vars.less';

	.app {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}

	nav {
		display: flex;
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

	a {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
	}

	@media (max-width: @mobile) {
		a {
			flex-direction: column;
		}
		a > svg {
			width: 20px;
		}
	}
	@media (min-width: @tablet-min) {
		a > svg {
			width: 25px;
		}

		a:hover > svg.blue-bright {
			color: @blue;
		}
		a:hover > svg.green-bright {
			color: @green;
		}
		a:hover > svg.orange-bright {
			color: @orange;
		}
		a:hover > svg.aqua {
			color: @aqua;
		}
	}
</style>
