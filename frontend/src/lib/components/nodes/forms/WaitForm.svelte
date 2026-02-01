<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';

	let { node } = $props<{ node: any }>();

	function updateConfig(key: string, value: any) {
		nexus.nodes = nexus.nodes.map((n) => n.id === node.id ? {
			...n, data: { ...n.data, config: { ...(n.data.config as any || {}), [key]: value } }
		} : n);
		if (nexus.selectedNode?.id === node.id) {
			nexus.selectedNode = {
				...nexus.selectedNode,
				data: { ...nexus.selectedNode.data, config: { ...(nexus.selectedNode.data.config as any || {}), [key]: value } }
			};
		}
	}

	const units = [
		{ value: 'seconds', label: 'Seconds' },
		{ value: 'minutes', label: 'Minutes' },
		{ value: 'hours', label: 'Hours' },
	];

	let selectedUnitValue = $derived(node.data.config?.unit || 'seconds');
</script>

<div class="space-y-6">
	<div class="space-y-2">
		<Label for="amount">Wait Amount</Label>
		<Input
			id="amount"
			type="number"
			value={node.data.config?.amount ?? 1}
			oninput={(e: Event & { currentTarget: HTMLInputElement }) => updateConfig('amount', parseFloat(e.currentTarget.value))}
			min="0"
		/>
	</div>

	<div class="space-y-2">
		<Label for="unit">Wait Unit</Label>
		<select id="unit" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={selectedUnitValue} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => updateConfig('unit', e.currentTarget.value)}>
			{#each units as unit}
				<option value={unit.value}>{unit.label}</option>
			{/each}
		</select>
	</div>
</div>
