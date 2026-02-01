<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import { getTableKeys, getSchema } from '$lib/utils';
	import type { NodeKind } from '$lib/flow/nodes/definitions';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Separator } from '$lib/components/ui/separator';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { X, Play, Loader2, Info, Box } from 'lucide-svelte';
	import OpenAIForm from '../nodes/forms/OpenAIForm.svelte';
	import HttpRequestForm from '../nodes/forms/HttpRequestForm.svelte';
	import OpenRouterForm from '../nodes/forms/OpenRouterForm.svelte';
	import ChatTriggerForm from '../nodes/forms/ChatTriggerForm.svelte';
	import AgentForm from '../nodes/forms/AgentForm.svelte';
	import CodeForm from '../nodes/forms/CodeForm.svelte';
	import ToolForm from '../nodes/forms/ToolForm.svelte';
	import WaitForm from '../nodes/forms/WaitForm.svelte';
	
	// Logs and Chat
	import LogsOverviewPanel from '../execution/logs/LogsOverviewPanel.svelte';
	import ChatMessagesPanel from '../execution/logs/ChatMessagesPanel.svelte';

	function getNodeColor(kind: NodeKind): string {
		if (kind?.startsWith('trigger-') && kind !== 'trigger-end') return 'bg-red-500';
		if (['if', 'switch', 'merge'].includes(kind)) return 'bg-yellow-500';
		if (['ai-agent', 'llm'].includes(kind)) return 'bg-blue-500';
		if (kind === 'note') return 'bg-yellow-300';
		return 'bg-purple-500';
	}

	function updateNodeData(key: string, value: any) {
		if (!nexus.selectedNode) return;
		nexus.nodes = nexus.nodes.map((n) => n.id === nexus.selectedNode!.id ? {
			...n, data: { ...n.data, config: { ...(n.data.config as any || {}), [key]: value } }
		} : n);
		if (nexus.selectedNode) {
			nexus.selectedNode = nexus.nodes.find(n => n.id === nexus.selectedNode!.id) || null;
		}
	}
</script>

<aside class="flex w-80 flex-col border-l bg-card shrink-0 h-full overflow-hidden relative">
	{#if nexus.showChatPanel}
		<ChatMessagesPanel />
	{:else if nexus.selectedExecution}
		<LogsOverviewPanel onClose={() => nexus.selectedExecution = null} />
	{:else if nexus.selectedNode}
		<!-- Node Configuration -->
		<div class="flex h-12 items-center justify-between border-b px-4 shrink-0 bg-muted/10">
			<div class="flex items-center gap-2 overflow-hidden">
				<div class="h-3 w-3 rounded-full shrink-0 {getNodeColor(nexus.selectedNode.data.kind as NodeKind)}"></div>
				<span class="text-sm font-bold truncate">{nexus.selectedNode.data.label}</span>
			</div>
			<div class="flex items-center gap-1">
				<Button variant="ghost" size="icon" class="h-8 w-8" onclick={() => nexus.executeSingleNode(nexus.selectedNode!.id)} disabled={nexus.isNodeExecuting[nexus.selectedNode.id]}>
					{#if nexus.isNodeExecuting[nexus.selectedNode.id]} 
						<Loader2 class="h-4 w-4 animate-spin" /> 
					{:else} 
						<Play class="h-4 w-4 fill-current" /> 
					{/if}
				</Button>
				<Button variant="ghost" size="icon" class="h-8 w-8" onclick={() => (nexus.selectedNode = null)}>
					<X class="h-4 w-4" />
				</Button>
			</div>
		</div>
		
		<ScrollArea.Root class="flex-1">
			<div class="p-4 pb-10">
				<Tabs.Root value="config" class="w-full">
					<Tabs.List class="grid w-full grid-cols-2 h-8 mb-6">
						<Tabs.Trigger value="config" class="text-xs">Parameters</Tabs.Trigger>
						<Tabs.Trigger value="output" class="text-xs">Output</Tabs.Trigger>
					</Tabs.List>
					
					<Tabs.Content value="config" class="m-0 border-none space-y-6">
						{#if nexus.selectedNode.data.kind === 'openai'}
							<OpenAIForm node={nexus.selectedNode} />
						{:else if nexus.selectedNode.data.kind === 'http-request'}
							<HttpRequestForm node={nexus.selectedNode} />
						{:else if nexus.selectedNode.data.kind === 'openrouter'}
							<OpenRouterForm node={nexus.selectedNode} />
						{:else if nexus.selectedNode.data.kind === 'chat-trigger'}
							<ChatTriggerForm node={nexus.selectedNode} />
						{:else if nexus.selectedNode.data.kind === 'ai-agent'}
							<AgentForm node={nexus.selectedNode} />
						{:else if nexus.selectedNode.data.kind === 'code'}
							<CodeForm node={nexus.selectedNode} />
						{:else if nexus.selectedNode.data.kind === 'tool'}
							<ToolForm node={nexus.selectedNode} />
						{:else if nexus.selectedNode.data.kind === 'wait'}
							<WaitForm node={nexus.selectedNode} />
						{:else}
							<div class="rounded-lg border bg-muted/20 p-4 space-y-4">
								<div class="space-y-1">
									<div class="text-[10px] font-bold uppercase text-muted-foreground tracking-widest">Node ID</div>
									<div class="font-mono text-[10px] break-all bg-background border p-2 rounded-md">{nexus.selectedNode.id}</div>
								</div>
								<Separator />
								<div class="space-y-1">
									<div class="text-[10px] font-bold uppercase text-muted-foreground tracking-widest">Kind</div>
									<Badge variant="outline" class="text-[10px] uppercase font-bold tracking-tighter">{nexus.selectedNode.data.kind}</Badge>
								</div>
							</div>
						{/if}
					</Tabs.Content>

					<Tabs.Content value="output" class="m-0 border-none">
						{#if nexus.executionResults.nodeResults[nexus.selectedNode.id]}
							{@const result = nexus.executionResults.nodeResults[nexus.selectedNode.id]}
							<div class="space-y-4">
								<div class="flex items-center justify-between px-1">
									<span class="text-[10px] font-bold uppercase text-muted-foreground tracking-widest">Latest Data</span>
									<Badge variant={result.status === 'success' ? 'default' : 'destructive'} class="h-4 text-[8px] uppercase">
										{result.status}
									</Badge>
								</div>
								{#if result.output}
									<pre class="bg-slate-950 text-slate-200 p-4 rounded-lg text-[10px] overflow-x-auto shadow-inner leading-relaxed border border-slate-800">{JSON.stringify(result.output, null, 2)}</pre>
								{:else if result.message}
									<div class="p-4 rounded-lg bg-destructive/10 border border-destructive/20 text-destructive text-xs">
										{result.message}
									</div>
								{/if}
							</div>
						{:else}
							<div class="flex flex-col items-center justify-center py-12 text-center text-muted-foreground">
								<Box class="h-8 w-8 mb-2 opacity-20" />
								<p class="text-xs">No output available.<br/>Run the workflow or node.</p>
							</div>
						{/if}
					</Tabs.Content>
				</Tabs.Root>
			</div>
		</ScrollArea.Root>
	{:else}
		<!-- Empty State -->
		<div class="flex flex-col items-center justify-center h-full text-muted-foreground p-8 text-center bg-muted/5">
			<div class="h-16 w-16 rounded-full bg-muted flex items-center justify-center mb-4 border shadow-inner">
				<Box class="h-8 w-8 opacity-20" />
			</div>
			<h3 class="text-sm font-bold text-foreground mb-1 tracking-tight">No Selection</h3>
			<p class="text-[11px] leading-relaxed max-w-[180px]">Select a node on the canvas or an execution from the history to view details.</p>
		</div>
	{/if}
</aside>