<script lang="ts">
	import { Tabs as TabsPrimitive } from "bits-ui";
	import { clsx, type ClassValue } from "clsx";
	import { twMerge } from "tailwind-merge";
	
	function cn(...inputs: ClassValue[]) {
		return twMerge(clsx(inputs));
	}

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: TabsPrimitive.ListProps = $props();
</script>

<TabsPrimitive.List
	bind:ref
	class={cn(
		"inline-flex h-9 items-center justify-center rounded-lg bg-muted p-1 text-muted-foreground",
		className
	)}
	{...restProps}
/>