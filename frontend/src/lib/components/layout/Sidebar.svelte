<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { Search, Plus, Trash2, Box, Cpu, Wrench, X } from 'lucide-svelte';
	import {
		triggerNodes,
		logicNodes,
		transformNodes,
		aiNodes,
		endNode,
		type NodeKind
	} from '$lib/flow/nodes/definitions';

	const nodeCategories = [
		{ name: 'Triggers', nodes: triggerNodes, color: 'bg-red-500', icon: Box },
		{ name: 'Logic', nodes: logicNodes, color: 'bg-yellow-500', icon: Cpu },
		{ name: 'Transform', nodes: transformNodes, color: 'bg-purple-500', icon: Wrench },
		{ name: 'AI', nodes: aiNodes, color: 'bg-blue-500', icon: Cpu },
		{ name: 'End', nodes: endNode, color: 'bg-slate-500', icon: Box }
	];

	let filteredCategories = $derived(
		nodeCategories
			.map((cat) => ({
				...cat,
				nodes: cat.nodes.filter(
					(n) =>
						n.label.toLowerCase().includes(nexus.nodeSearch.toLowerCase()) ||
						n.detail.toLowerCase().includes(nexus.nodeSearch.toLowerCase())
				)
			}))
			.filter((cat) => cat.nodes.length > 0)
	);

	let newMcpName = $state('');
	let newMcpUrl = $state('');
	let newMcpHeaders = $state<Array<{ key: string, value: string }>>([{ key: '', value: '' }]);

	function addHeaderField() {
		newMcpHeaders = [...newMcpHeaders, { key: '', value: '' }];
	}

	function removeHeaderField(index: number) {
		newMcpHeaders = newMcpHeaders.filter((_, i) => i !== index);
		if (newMcpHeaders.length === 0) newMcpHeaders = [{ key: '', value: '' }];
	}

	function handleRegisterServer() {
		const headers: Record<string, string> = {};
		newMcpHeaders.forEach(h => {
			if (h.key.trim()) headers[h.key.trim()] = h.value;
		});
		
		nexus.addMcpServer(newMcpName, newMcpUrl, headers);
		newMcpName = '';
		newMcpUrl = '';
		newMcpHeaders = [{ key: '', value: '' }];
	}

	function addNode(kind: NodeKind) {
		window.dispatchEvent(new CustomEvent('addNode', { detail: { kind } }));
	}
</script>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 4px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: var(--border-color);
		border-radius: 10px;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb:hover {
		background: #888;
	}
</style>

<aside class="flex w-64 flex-col border-r bg-card shrink-0 h-full overflow-hidden">
	<Tabs.Root value={nexus.activeTab} onValueChange={(v) => nexus.activeTab = v as any} class="flex flex-col h-full">
		<div class="p-2 border-b">
			<Tabs.List class="grid w-full grid-cols-2 h-8">
				<Tabs.Trigger value="nodes" class="text-[10px] uppercase font-bold">Nodes</Tabs.Trigger>
				<Tabs.Trigger value="executions" class="text-[10px] uppercase font-bold" onclick={() => nexus.fetchExecutions()}>History</Tabs.Trigger>
			</Tabs.List>
		</div>

		<div class="flex-1 overflow-hidden">
			<Tabs.Content value="nodes" class="h-full flex flex-col m-0 border-none">
				<div class="p-3 shrink-0">
					<div class="relative">
						<Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
						<Input 
							type="search" 
							placeholder="Search nodes..." 
							bind:value={nexus.nodeSearch}
							class="pl-8 h-9" 
						/>
					</div>
				</div>
				<div class="flex-1 overflow-y-auto px-2 pb-4 custom-scrollbar">
					{#each filteredCategories as category}
						<div class="mb-4">
							<h4 class="mb-2 px-2 text-[10px] font-bold uppercase tracking-wider text-muted-foreground">
								{category.name}
							</h4>
							<div class="space-y-1">
								{#each category.nodes as node}
									<button 
										class="flex w-full items-center gap-3 rounded-md px-2 py-1.5 text-left text-sm hover:bg-accent hover:text-accent-foreground transition-colors group"
										onclick={() => addNode(node.kind)}
									>
										<div class="flex h-7 w-7 items-center justify-center rounded border {category.color} text-white shadow-sm font-bold text-xs">
											{node.icon}
										</div>
										<div class="flex-1 overflow-hidden">
											<div class="font-medium leading-none mb-1">{node.label}</div>
											<div class="text-[10px] text-muted-foreground truncate">{node.detail}</div>
										</div>
									</button>
								{/each}
							</div>
						</div>
					{/each}
				</div>
			</Tabs.Content>

			<Tabs.Content value="executions" class="h-full m-0 border-none">
				<div class="h-full overflow-y-auto custom-scrollbar">
					{#if nexus.executions.length === 0}
						<div class="flex flex-col items-center justify-center h-40 text-muted-foreground p-4 text-center">
							<div class="text-2xl mb-2">📜</div>
							<p class="text-xs">No executions yet.</p>
						</div>
					{:else}
						<div class="flex flex-col">
							{#each nexus.executions as execution}
								<button 
									class="flex flex-col gap-1 border-b p-3 text-left transition-colors hover:bg-muted {nexus.selectedExecution?.id === execution.id ? 'bg-muted' : ''}"
									onclick={() => nexus.selectExecution(execution)}
								>
									<div class="flex items-center justify-between gap-2">
										<span class="text-xs font-bold truncate">{execution.workflow_name}</span>
										<Badge variant={execution.status === 'success' ? 'default' : 'destructive'} class="h-4 px-1 text-[8px] uppercase">
											{execution.status}
										</Badge>
									</div>
									<span class="text-[10px] text-muted-foreground">
										{new Date(execution.start_time).toLocaleString()}
									</span>
								</button>
							{/each}
						</div>
					{/if}
				</div>
			</Tabs.Content>

			<Tabs.Content value="tools" class="h-full m-0 border-none">
				<ScrollArea.Root class="h-full">
					<div class="p-4 space-y-6">
						<div>
							<h3 class="text-sm font-bold mb-4 flex items-center gap-2">
								<Wrench class="h-4 w-4" /> Register MCP Server
							</h3>
							<div class="grid gap-4">
								<div class="grid gap-1.5">
									<Label for="new-mcp-name" class="text-[10px] font-bold uppercase text-muted-foreground">Server Name</Label>
									<Input id="new-mcp-name" placeholder="e.g. Context7" bind:value={newMcpName} class="h-8" />
								</div>
								<div class="grid gap-1.5">
									<Label for="new-mcp-url" class="text-[10px] font-bold uppercase text-muted-foreground">URL</Label>
									<Input id="new-mcp-url" placeholder="https://..." bind:value={newMcpUrl} class="h-8" />
								</div>
								
								<div class="grid gap-2">
									<div class="flex items-center justify-between">
										<Label class="text-[10px] font-bold uppercase text-muted-foreground">Headers</Label>
										<Button variant="ghost" size="icon" class="h-5 w-5" onclick={addHeaderField}>
											<Plus class="h-3 w-3" />
										</Button>
									</div>
									<div class="space-y-2">
										{#each newMcpHeaders as header, i}
											<div class="flex items-center gap-1">
												<Input placeholder="Key" bind:value={header.key} class="h-7 text-[10px] px-2 flex-1" />
												<Input placeholder="Value" bind:value={header.value} class="h-7 text-[10px] px-2 flex-1" />
												{#if newMcpHeaders.length > 1}
													<Button variant="ghost" size="icon" class="h-7 w-7 text-muted-foreground" onclick={() => removeHeaderField(i)}>
														<X class="h-3 w-3" />
													</Button>
												{/if}
											</div>
										{/each}
									</div>
								</div>

								<Button size="sm" class="w-full h-8" onclick={handleRegisterServer}>
									<Plus class="h-3 w-3 mr-1" /> Register Server
								</Button>
							</div>
						</div>

						<Separator />

						<div>
							<h4 class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground mb-3">Registered Servers</h4>
							<div class="space-y-2 font-medium">
								{#each nexus.mcpServers as server}
									<div class="flex flex-col gap-1 rounded-lg border p-3 bg-background shadow-sm group">
										<div class="flex items-center justify-between">
											<div class="text-xs font-bold truncate">{server.name}</div>
											<Button variant="ghost" size="icon" class="h-6 w-6 text-destructive opacity-0 group-hover:opacity-100" onclick={() => nexus.removeMcpServer(server.id)}>
												<Trash2 class="h-3 w-3" />
											</Button>
										</div>
										<div class="text-[9px] text-muted-foreground truncate">{server.url}</div>
										{#if server.headers && Object.keys(server.headers).length > 0}
											<div class="mt-2 flex flex-wrap gap-1">
												{#each Object.keys(server.headers) as key}
													<Badge variant="outline" class="text-[8px] h-4 py-0 px-1 font-mono">{key}</Badge>
												{/each}
											</div>
										{/if}
									</div>
								{:else}
									<p class="text-[10px] text-center text-muted-foreground py-4 italic">No servers registered</p>
								{/each}
							</div>
						</div>
					</div>
				</ScrollArea.Root>
			</Tabs.Content>
		</div>
	</Tabs.Root>
</aside>