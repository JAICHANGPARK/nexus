<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { Separator } from '$lib/components/ui/separator';

	let { node } = $props<{ node: Node }>();
	
	function update(key: string, value: any) {
		const currentConfig = (node.data.config as Record<string, any>) || {};
		nexus.nodes = nexus.nodes.map(n => n.id === node.id ? {
			...n,
			data: { ...n.data, config: { ...currentConfig, [key]: value } }
		} : n);
		if (nexus.selectedNode?.id === node.id) {
			nexus.selectedNode = {
				...nexus.selectedNode,
				data: { ...nexus.selectedNode.data, config: { ...currentConfig, [key]: value } }
			};
		}
	}

	let config = $derived((node.data.config as Record<string, any>) || {});

	const operations = [
		{ value: 'getAll', label: 'Get Many' },
		{ value: 'get', label: 'Get' },
		{ value: 'create', label: 'Create' },
		{ value: 'update', label: 'Update' },
		{ value: 'delete', label: 'Delete' }
	];

	$effect(() => {
		if (nexus.dataTables.length === 0 && !nexus.isFetchingDataTables) {
			nexus.fetchDataTables();
		}
	});
</script>

<div class="space-y-4 pb-10">
	<div class="grid gap-2">
		<Label for="table">Data Table</Label>
		<select id="table" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.tableId ?? ''} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('tableId', e.currentTarget.value)}>
			<option value="">Select a table...</option>
			{#each nexus.dataTables as table}
				<option value={table.id}>{table.name}</option>
			{/each}
		</select>
	</div>

	<div class="grid gap-2">
		<Label for="operation">Operation</Label>
		<select id="operation" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.operation ?? 'getAll'} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('operation', e.currentTarget.value)}>
			{#each operations as o}
				<option value={o.value}>{o.label}</option>
			{/each}
		</select>
	</div>

	<Separator />

	{#if config.operation === 'get' || config.operation === 'update' || config.operation === 'delete'}
		<div class="grid gap-2">
			<Label for="rowId">Row ID</Label>
			<Input id="rowId" type="text" value={config.rowId ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('rowId', e.currentTarget.value)} placeholder="{{ $input.id }}" />
		</div>
	{/if}

	{#if config.operation === 'create' || config.operation === 'update'}
		<div class="p-3 rounded-md bg-muted/30 border border-dashed">
			<p class="text-[10px] text-muted-foreground leading-relaxed">
				<strong>Tip:</strong> The node will automatically map incoming data fields to the table columns based on the selected schema.
			</p>
		</div>
	{/if}
</div>