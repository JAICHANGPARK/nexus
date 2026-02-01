<script lang="ts">
	import { Checkbox as CheckboxPrimitive } from "bits-ui";
	import CheckIcon from "@lucide/svelte/icons/check";
	import { clsx, type ClassValue } from "clsx";
	import { twMerge } from "tailwind-merge";
	
	function cn(...inputs: ClassValue[]) {
		return twMerge(clsx(inputs));
	}

	let {
		ref = $bindable(null),
		class: className,
		checked = $bindable(false),
		...restProps
	}: CheckboxPrimitive.RootProps = $props();
</script>

<CheckboxPrimitive.Root
	bind:ref
	bind:checked
	class={cn(
		"peer border-primary data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive size-4 shrink-0 rounded-sm border shadow-xs outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 transition-colors",
		className
	)}
	{...restProps}
>
	<CheckboxPrimitive.Indicator
		class="flex items-center justify-center text-current"
	>
		<CheckIcon class="size-3.5" />
	</CheckboxPrimitive.Indicator>
</CheckboxPrimitive.Root>