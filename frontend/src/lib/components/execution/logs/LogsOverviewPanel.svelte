<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { Trash2, X, ListFilter, Activity } from 'lucide-svelte';
	import LogsExecutionSummary from './LogsExecutionSummary.svelte';
	import LogsOverviewRows from './LogsOverviewRows.svelte';

	let { onClose } = $props<{ onClose: () => void }>();

	const execution = $derived(nexus.selectedExecution);
	const isEmpty = $derived(!execution || !execution.results || execution.results.length === 0);

	// Calculate total tokens from results if available
	const consumedTokens = $derived(() => {
		if (isEmpty) return { total: 0, prompt: 0, completion: 0 };
		return execution.results.reduce((acc: any, res: any) => {
			const usage = res.output?.usage || res.output?.token_usage;
			if (usage) {
				acc.total += usage.total_tokens || 0;
				acc.prompt += usage.prompt_tokens || 0;
				acc.completion += usage.completion_tokens || 0;
			}
			return acc;
		}, { total: 0, prompt: 0, completion: 0 });
	});

	const timeTook = $derived(() => {
		if (!execution || !execution.start_time) return 0;
		const start = new Date(execution.start_time).getTime();
		const end = execution.end_time ? new Date(execution.end_time).getTime() : Date.now();
		return end - start;
	});
</script>

<div class="flex flex-col h-full bg-card shadow-xl border-l animate-in slide-in-from-right duration-300">
	<!-- Header -->
	<header class="flex h-12 items-center justify-between px-4 border-b shrink-0 bg-muted/10">
		<div class="flex items-center gap-2">
			<Activity class="h-4 w-4 text-primary" />
			<h3 class="text-sm font-bold tracking-tight">Execution Logs</h3>
		</div>
		<div class="flex items-center gap-1">
			<Button variant="ghost" size="icon" class="h-8 w-8 text-muted-foreground hover:text-destructive" 
				onclick={() => { nexus.selectedExecution = null; nexus.fetchExecutions(); }}
				title="Clear Execution Data"
			>
				<Trash2 class="h-4 w-4" />
			</Button>
			<Separator orientation="vertical" class="h-4 mx-1" />
			<Button variant="ghost" size="icon" class="h-8 w-8" onclick={onClose}>
				<X class="h-4 w-4" />
			</Button>
		</div>
	</header>

	<div class="flex-1 overflow-hidden relative">
		{#if isEmpty}
			<div class="flex flex-col items-center justify-center h-full text-muted-foreground p-8 text-center">
				<div class="h-12 w-12 rounded-full bg-muted flex items-center justify-center mb-4 opacity-50">
					<ListFilter class="h-6 w-6" />
				</div>
				<h4 class="text-sm font-semibold text-foreground">No execution data</h4>
				<p class="text-xs mt-1">Run the workflow to see detailed execution logs here.</p>
			</div>
		{:else}
			<ScrollArea.Root class="h-full">
				<div class="p-4 pb-20">
					<LogsExecutionSummary 
						status={execution.status}
						startTime={new Date(execution.start_time).getTime()}
						duration={timeTook()}
						consumedTokens={consumedTokens()}
					/>

					<div class="mb-3 flex items-center justify-between">
						<h4 class="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">Node Execution Trace</h4>
						<span class="text-[10px] text-muted-foreground bg-muted px-1.5 py-0.5 rounded">{execution.results.length} nodes</span>
					</div>

					<LogsOverviewRows results={execution.results} />
				</div>
			</ScrollArea.Root>

			<!-- Floating View Switcher (n8n Style) -->
			<div class="absolute bottom-4 left-1/2 -translate-x-1/2 flex bg-background/80 backdrop-blur border rounded-full p-1 shadow-lg ring-1 ring-black/5">
				<Button variant="ghost" size="sm" class="h-7 px-4 rounded-full text-[10px] font-bold uppercase bg-primary text-primary-foreground shadow-sm">Overview</Button>
				<Button variant="ghost" size="sm" class="h-7 px-4 rounded-full text-[10px] font-bold uppercase hover:bg-muted">Details</Button>
			</div>
		{/if}
	</div>
</div>

<style>
	/* n8n 스타일의 세련된 스크롤바 커스텀 */
	:global(.scroll-area-viewport) {
		scrollbar-width: thin;
	}
</style>
