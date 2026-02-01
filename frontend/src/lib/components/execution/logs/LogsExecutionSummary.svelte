<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { Clock, Zap, Coins } from 'lucide-svelte';

	let { status, startTime, duration, consumedTokens } = $props<{
		status: string;
		startTime: number;
		duration: number;
		consumedTokens?: { total: number; prompt: number; completion: number };
	}>();

	function formatDuration(ms: number) {
		if (ms < 1000) return `${ms}ms`;
		return `${(ms / 1000).toFixed(2)}s`;
	}
</script>

<div class="p-4 bg-muted/30 rounded-lg border border-border/50 mb-4">
	<div class="flex items-center justify-between mb-3">
		<div class="flex items-center gap-2">
			<div class="h-2 w-2 rounded-full {status === 'success' ? 'bg-green-500' : 'bg-red-500'}"></div>
			<span class="text-sm font-bold uppercase tracking-tight">{status}</span>
		</div>
		<Badge variant="outline" class="font-mono text-[10px]">
			ID: {Math.random().toString(36).substring(7)}
		</Badge>
	</div>

	<div class="grid grid-cols-2 gap-4 text-xs">
		<div class="flex items-center gap-2 text-muted-foreground">
			<Clock class="h-3.5 w-3.5" />
			<span>{new Date(startTime).toLocaleTimeString()}</span>
		</div>
		<div class="flex items-center gap-2 text-muted-foreground">
			<Zap class="h-3.5 w-3.5" />
			<span>{formatDuration(duration)}</span>
		</div>
		{#if consumedTokens && consumedTokens.total > 0}
			<div class="col-span-2 flex items-center gap-2 text-muted-foreground border-t pt-2 mt-1">
				<Coins class="h-3.5 w-3.5 text-amber-500" />
				<span class="font-medium text-foreground">{consumedTokens.total} tokens</span>
				<span class="text-[10px]">(P: {consumedTokens.prompt} / C: {consumedTokens.completion})</span>
			</div>
		{/if}
	</div>
</div>
