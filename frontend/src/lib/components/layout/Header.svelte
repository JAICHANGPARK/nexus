<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import { downloadJSON } from '$lib/utils';
	import type { Node } from '@xyflow/svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Input } from '$lib/components/ui/input';
	import { Separator } from '$lib/components/ui/separator';
	import * as Tabs from '$lib/components/ui/tabs';
	import { ChevronDown, Save, Play, Download, Upload } from 'lucide-svelte';

	const API_BASE = import.meta.env.VITE_API_BASE ?? 'http://localhost:3001';

	function exportNexus() {
		const workflow = {
			name: nexus.workflowName,
			nodes: nexus.nodes.map((n) => ({
				id: n.id,
				kind: n.data.kind,
				label: n.data.label,
				position: n.position,
				config: n.data.config
			})),
			edges: nexus.edges.map((e) => ({
				id: e.id,
				source: e.source,
				target: e.target
			}))
		};
		downloadJSON(workflow, 'workflow-nexus.json');
	}

	async function exportN8n() {
		try {
			const response = await fetch(`${API_BASE}/api/workflows/current/export/n8n`);
			const workflow = await response.json();
			downloadJSON(workflow, 'workflow-n8n.json');
		} catch (e) {
			console.error('Export failed:', e);
		}
	}

	function handleImportNexus(e: Event) {
		const file = (e.target as HTMLInputElement).files?.[0];
		if (!file) return;
		const reader = new FileReader();
		reader.onload = (event) => {
			const data = JSON.parse(event.target?.result as string);
			if (data.name) nexus.workflowName = data.name;
			if (data.nodes) nexus.nodes = data.nodes.map((n: any) => ({ ...n, type: 'workflowNode' }));
			if (data.edges) nexus.edges = data.edges;
		};
		reader.readAsText(file);
	}

	async function onSave() {
		nexus.isSaving = true;
		nexus.saveToStorage();
		setTimeout(() => { nexus.isSaving = false; }, 500);
	}
</script>

<header class="flex h-12 items-center justify-between border-b bg-card px-4 shrink-0">
	<div class="flex items-center gap-4">
		<span class="text-lg font-bold tracking-tighter text-primary">Nexus</span>
		
		<Separator orientation="vertical" class="h-6" />
		
		<div class="flex items-center bg-muted/20 rounded-md p-1">
			<Button variant={nexus.activeView === 'canvas' ? 'secondary' : 'ghost'} size="sm" class="h-7 text-xs px-3" onclick={() => nexus.activeView = 'canvas'}>Studio</Button>
			<Button variant={nexus.activeView === 'credentials' ? 'secondary' : 'ghost'} size="sm" class="h-7 text-xs px-3" onclick={() => { nexus.activeView = 'credentials'; nexus.fetchCredentials(); }}>Credentials</Button>
			
			<DropdownMenu.Root>
				<DropdownMenu.Trigger>
					{#snippet child({ props })}
						<Button {...props} variant={['data-table', 'knowledge-base'].includes(nexus.activeView) ? 'secondary' : 'ghost'} size="sm" class="h-7 text-xs px-3 gap-1">
							지식 <ChevronDown class="h-3 w-3 opacity-50" />
						</Button>
					{/snippet}
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="start">
					<DropdownMenu.Item onclick={() => { nexus.activeView = 'data-table'; nexus.fetchDataTables(); }}>Data Table</DropdownMenu.Item>
					<DropdownMenu.Item onclick={() => nexus.activeView = 'knowledge-base'}>Knowledge Base</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>

			<Button variant={nexus.activeView === 'tools' ? 'secondary' : 'ghost'} size="sm" class="h-7 text-xs px-3" onclick={() => { nexus.activeView = 'tools'; nexus.fetchMcpServers(); }}>Tools</Button>
		</div>

		{#if nexus.activeView === 'canvas'}
			<Separator orientation="vertical" class="h-6" />
			<Input 
				bind:value={nexus.workflowName} 
				class="h-8 w-48 border-none bg-transparent font-semibold focus-visible:ring-1 focus-visible:bg-white" 
			/>
		{/if}
	</div>

	<div class="flex items-center gap-2">
		{#if nexus.activeView === 'canvas'}
			<DropdownMenu.Root>
				<DropdownMenu.Trigger>
					{#snippet child({ props })}
						<Button {...props} variant="outline" size="sm" class="gap-1 h-8">
							<Download class="h-3.5 w-3.5" />
							Export <ChevronDown class="h-3 w-3 opacity-50" />
						</Button>
					{/snippet}
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end">
					<DropdownMenu.Item onclick={exportNexus}>Nexus Format (.json)</DropdownMenu.Item>
					<DropdownMenu.Item onclick={exportN8n}>n8n Format (.json)</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>

			<DropdownMenu.Root>
				<DropdownMenu.Trigger>
					{#snippet child({ props })}
						<Button {...props} variant="outline" size="sm" class="gap-1 h-8">
							<Upload class="h-3.5 w-3.5" />
							Import <ChevronDown class="h-3 w-3 opacity-50" />
						</Button>
					{/snippet}
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end">
					<DropdownMenu.Item>
						{#snippet child({ props })}
							<label {...props} class="cursor-pointer w-full px-2 py-1.5 text-sm hover:bg-accent rounded-sm flex items-center">
								Nexus Format
								<input type="file" accept=".json" onchange={handleImportNexus} class="hidden" />
							</label>
						{/snippet}
					</DropdownMenu.Item>
					<DropdownMenu.Item disabled>n8n Format (Coming soon)</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>

			<Separator orientation="vertical" class="h-6 mx-1" />

			<Button variant="secondary" size="sm" class="h-8 gap-1" onclick={() => nexus.saveWorkflow()} disabled={nexus.isSaving}>
				<Save class="h-3.5 w-3.5" />
				{nexus.isSaving ? 'Saving...' : 'Save'}
			</Button>
			
			<Button size="sm" class="h-8 gap-1" onclick={() => nexus.executeWorkflow()} disabled={nexus.isExecuting}>
				{#if nexus.isExecuting}
					<div class="h-3 w-3 animate-spin rounded-full border-2 border-current border-t-transparent"></div>
					Running...
				{:else}
					<Play class="h-3.5 w-3.5 fill-current" />
					Run
				{/if}
			</Button>

			<Separator orientation="vertical" class="h-6 mx-1" />

			<Button variant={nexus.showChatPanel ? "default" : "outline"} size="sm" class="h-8 w-8 p-0 relative" onclick={() => nexus.showChatPanel = !nexus.showChatPanel}>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-bot"><path d="M12 8V4H8"/><rect width="16" height="12" x="4" y="8" rx="2"/><path d="M2 14h2"/><path d="M20 14h2"/><path d="M15 13v2"/><path d="M9 13v2"/></svg>
				{#if !nexus.showChatPanel}
					<span class="absolute -top-1 -right-1 flex h-3 w-3">
						<span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary opacity-75"></span>
						<span class="relative inline-flex rounded-full h-3 w-3 bg-primary"></span>
					</span>
				{/if}
			</Button>
		{/if}
	</div>
</header>