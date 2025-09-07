<script lang="ts">
	import type { HTMLButtonAttributes } from 'svelte/elements';

	interface Props extends HTMLButtonAttributes {
		preset?:
			| 'filled'
			| 'tonal'
			| 'outlined'
			| 'glass'
			| 'elevated'
			| 'ghost'
			| 'ghost-icon'
			| 'gradient';
		color?: 'primary' | 'secondary' | 'tertiary' | 'success' | 'warning' | 'error' | 'surface';
		size?: 'sm' | 'base' | 'lg';
	}

	let {
		preset = 'filled',
		color = 'primary',
		size = 'base',
		class: className = '',
		children,
		...restProps
	}: Props = $props();

	const sizeClasses = {
		sm: 'btn-sm',
		base: 'btn-base',
		lg: 'btn-lg'
	};

	// Explicit class mappings to ensure Tailwind includes them
	const presetClasses = {
		filled: {
			primary: 'preset-filled-primary-500',
			secondary: 'preset-filled-secondary-500',
			tertiary: 'preset-filled-tertiary-500',
			success: 'preset-filled-success-500',
			warning: 'preset-filled-warning-500',
			error: 'preset-filled-error-500',
			surface: 'preset-filled-surface-500'
		},
		outlined: {
			primary: 'preset-outlined-primary-500',
			secondary: 'preset-outlined-secondary-500',
			tertiary: 'preset-outlined-tertiary-500',
			success: 'preset-outlined-success-500',
			warning: 'preset-outlined-warning-500',
			error: 'preset-outlined-error-500',
			surface: 'preset-outlined-surface-500'
		},
		tonal: {
			primary: 'preset-tonal-primary-500',
			secondary: 'preset-tonal-secondary-500',
			tertiary: 'preset-tonal-tertiary-500',
			success: 'preset-tonal-success-500',
			warning: 'preset-tonal-warning-500',
			error: 'preset-tonal-error-500',
			surface: 'preset-tonal-surface-500'
		},
		glass: {
			primary: 'preset-glass-primary-500',
			secondary: 'preset-glass-secondary-500',
			tertiary: 'preset-glass-tertiary-500',
			success: 'preset-glass-success-500',
			warning: 'preset-glass-warning-500',
			error: 'preset-glass-error-500',
			surface: 'preset-glass-surface-500'
		},
		elevated: {
			primary: 'preset-elevated-primary-500',
			secondary: 'preset-elevated-secondary-500',
			tertiary: 'preset-elevated-tertiary-500',
			success: 'preset-elevated-success-500',
			warning: 'preset-elevated-warning-500',
			error: 'preset-elevated-error-500',
			surface: 'preset-elevated-surface-500'
		},
		ghost: {
			primary: 'preset-ghost-primary-500',
			secondary: 'preset-ghost-secondary-500',
			tertiary: 'preset-ghost-tertiary-500',
			success: 'preset-ghost-success-500',
			warning: 'preset-ghost-warning-500',
			error: 'preset-ghost-error-500',
			surface: 'preset-ghost-surface-500'
		},
		'ghost-icon': {
			primary: 'preset-ghost-icon-primary-500',
			secondary: 'preset-ghost-icon-secondary-500',
			tertiary: 'preset-ghost-icon-tertiary-500',
			success: 'preset-ghost-icon-success-500',
			warning: 'preset-ghost-icon-warning-500',
			error: 'preset-ghost-icon-error-500',
			surface: 'preset-ghost-icon-surface-500'
		},
		gradient: {
			primary: 'preset-gradient-primary-500',
			secondary: 'preset-gradient-secondary-500',
			tertiary: 'preset-gradient-tertiary-500',
			success: 'preset-gradient-success-500',
			warning: 'preset-gradient-warning-500',
			error: 'preset-gradient-error-500',
			surface: 'preset-gradient-surface-500'
		}
	};

	const classes = $derived(`btn ${presetClasses[preset][color]} ${sizeClasses[size]} ${className}`);
</script>

<button class={classes} {...restProps}>
	{@render children?.()}
</button>
