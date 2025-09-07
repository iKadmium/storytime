<script lang="ts">
	import { page } from '$app/stores';

	// Navigation items
	const navItems = [
		{ href: '/', label: 'Home' },
		{ href: '/characters', label: 'Characters' },
		{ href: '/prompts', label: 'Prompts' },
		{ href: '/jobs', label: 'Jobs' }
	];
</script>

<nav class="navbar bg-surface-50-900-token border-surface-300-600-token border-b">
	<div class="navbar-section">
		<!-- Brand/Logo -->
		<a href="/" class="btn btn-ghost text-primary-500 text-lg font-bold"> 📚 Storytime </a>
	</div>

	<div class="navbar-section">
		<!-- Navigation Links -->
		<div class="flex space-x-2">
			{#each navItems as item}
				<a
					href={item.href}
					class="btn btn-ghost"
					class:variant-filled-primary={$page.url.pathname === item.href}
					class:hover:variant-soft-primary={$page.url.pathname !== item.href}
				>
					{item.label}
				</a>
			{/each}
		</div>
	</div>
</nav>
