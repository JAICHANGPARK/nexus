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
	let action = $derived(config.action || 'format');
</script>

<div class="space-y-4">
	<div class="grid gap-2">
		<Label>Action</Label>
		<select 
			class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
			value={action}
			onchange={(e) => update('action', e.currentTarget.value)}
		>
			<option value="format">Format a Date</option>
			<option value="calculate">Calculate a Date</option>
			<option value="extractDate">Extract Part of a Date</option>
		</select>
	</div>

	<div class="grid gap-2">
		<Label for="value">Date Value</Label>
		<Input id="value" type="text" value={config.value ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('value', e.currentTarget.value)} placeholder={"{{ $input.date }} or ISO string"} />
	</div>

	{#if action === 'format'}
		<div class="grid gap-2">
			<Label for="toFormat">To Format</Label>
			<Input id="toFormat" type="text" value={config.toFormat ?? 'YYYY-MM-DD'} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('toFormat', e.currentTarget.value)} placeholder="YYYY-MM-DD or HH:mm:ss" />
		</div>
	{/if}

	{#if action === 'calculate'}
		<div class="grid gap-2">
			<Label>Operation</Label>
			<select 
				class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
				value={config.operation ?? 'add'}
				onchange={(e) => update('operation', e.currentTarget.value)}
			>
				<option value="add">Add</option>
				<option value="subtract">Subtract</option>
			</select>
		</div>
		<div class="grid grid-cols-2 gap-4">
			<div class="grid gap-2">
				<Label for="duration">Duration</Label>
				<Input id="duration" type="number" value={config.duration ?? 0} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('duration', parseInt(e.currentTarget.value))} />
			</div>
			<div class="grid gap-2">
				<Label>Unit</Label>
				<select 
					class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
					value={config.timeUnit ?? 'days'}
					onchange={(e) => update('timeUnit', e.currentTarget.value)}
				>
					<option value="seconds">Seconds</option>
					<option value="minutes">Minutes</option>
					<option value="hours">Hours</option>
					<option value="days">Days</option>
					<option value="weeks">Weeks</option>
					<option value="months">Months</option>
					<option value="years">Years</option>
				</select>
			</div>
		</div>
	{/if}

	{#if action === 'extractDate'}
		<div class="grid gap-2">
			<Label>Part to Extract</Label>
			<select 
				class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
				value={config.part ?? 'month'}
				onchange={(e) => update('part', e.currentTarget.value)}
			>
				<option value="year">Year</option>
				<option value="month">Month</option>
				<option value="day">Day</option>
				<option value="hour">Hour</option>
				<option value="minute">Minute</option>
				<option value="second">Second</option>
			</select>
		</div>
	{/if}

	<div class="grid gap-2">
		<Label for="outputField">Output Property Name</Label>
		<Input id="outputField" type="text" value={config.dataPropertyName ?? 'data'} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('dataPropertyName', e.currentTarget.value)} placeholder="data" />
	</div>
</div>
