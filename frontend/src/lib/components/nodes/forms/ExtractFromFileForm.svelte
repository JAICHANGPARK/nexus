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
	let operation = $derived(config.operation || 'csv');
</script>

<div class="space-y-4">
	<div class="grid gap-2">
		<Label>Operation</Label>
		<select 
			class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
			value={operation}
			onchange={(e) => update('operation', e.currentTarget.value)}
		>
			<option value="csv">Extract from CSV</option>
			<option value="fromJson">Extract from JSON</option>
			<option value="text">Extract from Text File</option>
			<option value="binaryToPropery">Move File to Base64 String</option>
		</select>
	</div>

	<div class="grid gap-2">
		<Label for="binaryPropertyName">Input Binary Field</Label>
		<Input id="binaryPropertyName" value={config.binaryPropertyName ?? 'data'} oninput={(e: any) => update('binaryPropertyName', e.target.value)} placeholder="e.g. data" />
	</div>

	<div class="grid gap-2">
		<Label for="destinationKey">Destination Output Field</Label>
		<Input id="destinationKey" value={config.destinationKey ?? 'data'} oninput={(e: any) => update('destinationKey', e.target.value)} placeholder="e.g. data" />
	</div>
</div>
