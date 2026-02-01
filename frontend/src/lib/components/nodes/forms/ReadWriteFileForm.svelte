<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';

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
	let operation = $derived(config.operation || 'read');
</script>

<div class="space-y-4">
	<div class="grid gap-2">
		<Label>Operation</Label>
		<select 
			class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
			value={operation}
			onchange={(e) => update('operation', e.currentTarget.value)}
		>
			<option value="read">Read File(s) From Disk</option>
			<option value="write">Write File to Disk</option>
		</select>
	</div>

	{#if operation === 'read'}
		<div class="grid gap-2">
			<Label for="fileSelector">File(s) Selector</Label>
			<Input id="fileSelector" value={config.fileSelector ?? ''} oninput={(e: any) => update('fileSelector', e.target.value)} placeholder="e.g. /data/*.csv" />
			<p class="text-[10px] text-muted-foreground">Supports patterns like **/*.png</p>
		</div>
	{:else}
		<div class="grid gap-2">
			<Label for="fileName">File Path and Name</Label>
			<Input id="fileName" value={config.fileName ?? ''} oninput={(e: any) => update('fileName', e.target.value)} placeholder="e.g. /data/output.csv" />
		</div>
		<div class="grid gap-2">
			<Label for="dataPropertyName">Input Binary Field</Label>
			<Input id="dataPropertyName" value={config.dataPropertyName ?? 'data'} oninput={(e: any) => update('dataPropertyName', e.target.value)} placeholder="e.g. data" />
		</div>
		<div class="flex items-center justify-between">
			<Label class="text-xs">Append</Label>
			<input type="checkbox" checked={config.append ?? false} onchange={(e) => update('append', e.currentTarget.checked)} />
		</div>
	{/if}
</div>
