<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { ChevronRight, ChevronDown, CheckCircle2, XCircle } from 'lucide-svelte';
	import { nexus } from '$lib/nexus.svelte';

	let { results } = $props<{ results: any[] }>();
	let expandedNodes = $state<Set<string>>(new Set());

	function toggleExpand(id: string) {
		if (expandedNodes.has(id)) expandedNodes.delete(id);
		else expandedNodes.add(id);
		expandedNodes = new Set(expandedNodes);
	}
</script>

<div class="flex flex-col border rounded-lg overflow-hidden bg-background">
	{#each results as result}
		<div class="group border-b last:border-0">
			<button 
				class="w-full flex items-center justify-between p-3 hover:bg-muted/50 transition-colors text-left"
				onclick={() => toggleExpand(result.node_id)}
			>
				<div class="flex items-center gap-3 overflow-hidden">
					{#if result.success}
						<CheckCircle2 class="h-4 w-4 text-green-500 shrink-0" />
					{:else}
						<XCircle class="h-4 w-4 text-red-500 shrink-0" />
					{/if}
					<span class="text-xs font-bold truncate">{result.node_name}</span>
				</div>
				<div class="flex items-center gap-2">
					<span class="text-[10px] text-muted-foreground font-mono">{result.execution_time_ms}ms</span>
					{#if expandedNodes.has(result.node_id)}
						<ChevronDown class="h-3.5 w-3.5 opacity-50" />
					{:else}
						<ChevronRight class="h-3.5 w-3.5 opacity-50" />
					{/if}
				</div>
			</button>

			{#if expandedNodes.has(result.node_id)}
				<div class="p-3 bg-muted/20 border-t text-[10px]">
					{#if result.error}
						<div class="p-2 rounded bg-destructive/10 text-destructive mb-2 font-medium">
							{result.error}
						</div>
					{/if}
					{#if result.output}
						<div class="space-y-1">
							<div class="flex items-center justify-between opacity-70 mb-1 uppercase font-bold tracking-widest text-[9px]">
								Output Data
								<button class="hover:text-primary underline" onclick={() => console.log('Copy JSON')}>Copy</button>
							</div>
							<pre class="bg-slate-950 text-slate-200 p-3 rounded overflow-x-auto leading-relaxed max-h-40">{JSON.stringify(result.output, null, 2)}</pre>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{/each}
</div>
