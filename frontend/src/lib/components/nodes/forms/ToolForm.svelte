<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Checkbox } from '$lib/components/ui/checkbox/index.js';
	import { Badge } from '$lib/components/ui/badge';
	import { Loader2, Wrench, AlertCircle } from 'lucide-svelte';
	import { onMount } from 'svelte';

	let { node } = $props<{ node: Node }>();
	
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let availableTools = $state<any[]>([]);

	const API_BASE = import.meta.env.VITE_API_BASE ?? 'http://localhost:3001';

	let config = $derived((node.data.config as Record<string, any>) || {});
	let selectedServerId = $derived(config.serverId || '');
	let enabledTools = $derived(config.selectedTools || []);

	async function fetchTools(serverId: string) {
		if (!serverId) return;
		isLoading = true;
		error = null;
		try {
			const response = await fetch(`${API_BASE}/api/mcp/servers/${serverId}/tools`);
			if (!response.ok) throw new Error('Failed to fetch tools');
			const data = await response.json();
			availableTools = data.tools || [];
		} catch (e) {
			console.error(e);
			error = "Could not connect to MCP server.";
		} finally {
			isLoading = false;
		}
	}

	function update(key: string, value: any) {
		const currentConfig = (node.data.config as Record<string, any>) || {};
		nexus.nodes = nexus.nodes.map(n => n.id === node.id ? {
			...n,
			data: { ...n.data, config: { ...currentConfig, [key]: value } }
		} : n);
		if (nexus.selectedNode) {
			nexus.selectedNode = nexus.nodes.find(n => n.id === node.id) || null;
		}
	}

	function toggleTool(toolName: string) {
		let current = [...enabledTools];
		if (current.includes(toolName)) {
			current = current.filter(t => t !== toolName);
		} else {
			current.push(toolName);
		}
		update('selectedTools', current);
	}

	// Fetch tools when server selection changes
	$effect(() => {
		if (selectedServerId) {
			fetchTools(selectedServerId);
		}
	});
</script>

<div class="space-y-6">
	<div class="grid gap-2">
		<Label for="mcp-server">MCP Server</Label>
		<select 
			id="mcp-server" 
			class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
			value={selectedServerId}
			onchange={(e) => update('serverId', e.currentTarget.value)}
		>
			<option value="">Select a server...</option>
			{#each nexus.mcpServers as server}
				<option value={server.id}>{server.name}</option>
			{/each}
		</select>
	</div>

	<div class="space-y-3">
		<div class="flex items-center justify-between">
			<Label class="text-xs font-bold uppercase tracking-widest text-muted-foreground">Available Tools</Label>
			{#if enabledTools.length > 0}
				<Badge variant="secondary" class="text-[9px] h-4">{enabledTools.length} active</Badge>
			{/if}
		</div>

		{#if !selectedServerId}
			<div class="flex flex-col items-center justify-center py-8 rounded-lg border border-dashed text-muted-foreground bg-muted/5">
				<Wrench class="h-6 w-6 mb-2 opacity-20" />
				<p class="text-[10px]">Please select an MCP server first</p>
			</div>
		{:else if isLoading}
			<div class="flex items-center justify-center py-8">
				<Loader2 class="h-5 w-5 animate-spin text-primary opacity-50" />
			</div>
		{:else if error}
			<div class="p-3 rounded-md bg-destructive/10 border border-destructive/20 text-destructive text-[10px] flex gap-2">
				<AlertCircle class="h-3.5 w-3.5 shrink-0" />
				<span>{error}</span>
			</div>
		{:else if availableTools.length === 0}
			<p class="text-center py-4 text-xs text-muted-foreground italic">No tools found on this server.</p>
		{:else}
			<div class="grid gap-2 max-h-[300px] overflow-y-auto pr-1">
				{#each availableTools as tool}
					<div class="flex items-start gap-3 p-2 rounded-md border bg-background hover:bg-muted/30 transition-colors">
						<Checkbox 
							id={tool.name} 
							checked={enabledTools.includes(tool.name)} 
							onCheckedChange={() => toggleTool(tool.name)} 
							class="mt-0.5"
						/>
						<div class="grid gap-1 cursor-pointer" onclick={() => toggleTool(tool.name)} role="presentation">
							<Label for={tool.name} class="text-xs font-bold leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
								{tool.name}
							</Label>
							<p class="text-[10px] text-muted-foreground leading-normal line-clamp-2">
								{tool.description || 'No description available.'}
							</p>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>
