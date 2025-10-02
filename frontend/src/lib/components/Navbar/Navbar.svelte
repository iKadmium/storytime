<script lang="ts">
	import { page } from '$app/stores';

	// Navigation items
	const navItems = [
		{ href: '/', label: 'Home' },
		{ href: '/characters', label: 'Characters' },
		{ href: '/prompts', label: 'Prompts' },
		{ href: '/jobs', label: 'Jobs' }
	];

	// Mobile menu state
	let mobileMenuOpen = false;

	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
	}

	function closeMobileMenu() {
		mobileMenuOpen = false;
	}
</script>

<nav class="navbar bg-surface-50-900-token border-surface-300-600-token border-b">
	<div class="navbar-section hidden md:flex">
		<!-- Brand/Logo -->
		<a href="/" class="btn-ghost btn text-lg font-bold text-primary-500"> 📚 Storytime </a>
	</div>

	<!-- Desktop Navigation -->
	<div class="navbar-section hidden md:flex">
		<!-- Navigation Links -->
		<div class="flex space-x-2">
			{#each navItems as item, index (index)}
				<a
					href={item.href}
					class="btn-ghost btn"
					class:variant-filled-primary={$page.url.pathname === item.href}
					class:hover:variant-soft-primary={$page.url.pathname !== item.href}
				>
					{item.label}
				</a>
			{/each}
		</div>
	</div>

	<!-- Mobile Hamburger Menu Button -->
	<div class="navbar-section md:hidden">
		<button class="variant-ghost-surface btn btn-sm" on:click={toggleMobileMenu} aria-label="Toggle mobile menu">
			<!-- Hamburger Icon -->
			<div class="hamburger" class:active={mobileMenuOpen}>
				<span class="hamburger-line"></span>
				<span class="hamburger-line"></span>
				<span class="hamburger-line"></span>
			</div>
		</button>
	</div>
</nav>

<!-- Mobile Menu Tray -->
{#if mobileMenuOpen}
	<div class="mobile-menu-overlay md:hidden" on:click={closeMobileMenu} on:keydown role="button" tabindex="0">
		<div class="mobile-menu-tray" on:click|stopPropagation on:keydown role="button" tabindex="0">
			<div class="border-surface-300-600-token border-b p-4">
				<a href="/" class="btn-ghost btn text-lg font-bold text-primary-500"> 📚 Storytime </a>
			</div>
			<div class="space-y-2 p-4">
				{#each navItems as item, index (index)}
					<a
						href={item.href}
						class="btn-ghost btn block w-full justify-start"
						class:variant-filled-primary={$page.url.pathname === item.href}
						class:hover:variant-soft-primary={$page.url.pathname !== item.href}
						on:click={closeMobileMenu}
					>
						{item.label}
					</a>
				{/each}
			</div>
		</div>
	</div>
{/if}

<style>
	/* Hamburger Icon Styles */
	.hamburger {
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		width: 24px;
		height: 18px;
		cursor: pointer;
	}

	.hamburger-line {
		display: block;
		height: 2px;
		width: 100%;
		background-color: currentColor;
		border-radius: 1px;
		transition: all 0.3s ease-in-out;
		transform-origin: center;
	}

	/* Active hamburger animation (turns into X) */
	.hamburger.active .hamburger-line:nth-child(1) {
		transform: translateY(8px) rotate(45deg);
	}

	.hamburger.active .hamburger-line:nth-child(2) {
		opacity: 0;
	}

	.hamburger.active .hamburger-line:nth-child(3) {
		transform: translateY(-8px) rotate(-45deg);
	}

	/* Mobile Menu Overlay */
	.mobile-menu-overlay {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background-color: rgba(0, 0, 0, 0.5);
		z-index: 50;
		display: flex;
		justify-content: flex-end;
	}

	/* Mobile Menu Tray */
	.mobile-menu-tray {
		width: 280px;
		height: 100vh;
		background-color: var(--color-surface-900);
		border-left-color: var(--color-surface-600);
		box-shadow: -4px 0 8px rgba(0, 0, 0, 0.1);
		animation: slideIn 0.3s ease-out;
	}

	@keyframes slideIn {
		from {
			transform: translateX(100%);
		}
		to {
			transform: translateX(0);
		}
	}

	/* Prevent body scroll when menu is open */
	:global(body:has(.mobile-menu-overlay)) {
		overflow: hidden;
	}
</style>
