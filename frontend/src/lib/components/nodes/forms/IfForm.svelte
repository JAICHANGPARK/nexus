<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { Plus, Trash2, Settings2 } from 'lucide-svelte';

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
	let conditions = $derived(config.conditions?.conditions || []);
	let combinator = $derived(config.conditions?.combinator || 'and');

	function addCondition() {
		const newConditions = [...conditions, {
			id: Math.random().toString(36).substr(2, 9),
			leftValue: '',
			rightValue: '',
			operator: { type: 'string', operation: 'equals' }
		}];
		update('conditions', { ...config.conditions, conditions: newConditions, combinator });
	}

	function removeCondition(index: number) {
		const newConditions = conditions.filter((_: any, i: number) => i !== index);
		update('conditions', { ...config.conditions, conditions: newConditions, combinator });
	}

	function updateCondition(index: number, key: string, value: any) {
		const newConditions = conditions.map((c: any, i: number) => {
			if (i === index) {
				if (key.includes('.')) {
					const [parent, child] = key.split('.');
					return { ...c, [parent]: { ...c[parent], [child]: value } };
				}
				return { ...c, [key]: value };
			}
			return c;
		});
		update('conditions', { ...config.conditions, conditions: newConditions, combinator });
	}
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<Label>Conditions</Label>
		<select 
			class="h-7 rounded-md border border-input bg-transparent px-2 text-[10px] font-bold uppercase"
			value={combinator}
			onchange={(e) => update('conditions', { ...config.conditions, conditions, combinator: e.currentTarget.value })}
		>
			<option value="and">ALL (AND)</option>
			<option value="or">ANY (OR)</option>
		</select>
	</div>

	<div class="space-y-4">
		{#each conditions as cond, i}
			<div class="p-3 rounded-lg border bg-muted/30 space-y-3 relative group">
				<div class="grid gap-1.5">
					<Label class="text-[10px] text-muted-foreground uppercase font-bold">Value 1</Label>
					<Input value={cond.leftValue} oninput={(e: any) => updateCondition(i, 'leftValue', e.target.value)} placeholder="{{ $input.field }}" class="h-8 text-xs" />
				</div>

				<div class="grid grid-cols-2 gap-2">
					<div class="grid gap-1.5">
						<Label class="text-[10px] text-muted-foreground uppercase font-bold">Type</Label>
						<select 
							class="h-8 rounded-md border border-input bg-background px-2 text-xs"
							value={cond.operator.type}
							onchange={(e) => updateCondition(i, 'operator.type', e.currentTarget.value)}
						>
							<option value="string">String</option>
							<option value="number">Number</option>
							<option value="boolean">Boolean</option>
						</select>
					</div>
					<div class="grid gap-1.5">
						<Label class="text-[10px] text-muted-foreground uppercase font-bold">Operation</Label>
						<select 
							class="h-8 rounded-md border border-input bg-background px-2 text-xs"
							value={cond.operator.operation}
							onchange={(e) => updateCondition(i, 'operator.operation', e.currentTarget.value)}
						>
							{#if cond.operator.type === 'string'}
								<option value="equals">Equals</option>
								<option value="notEquals">Not Equals</option>
								<option value="contains">Contains</option>
								<option value="notContains">Not Contains</option>
								<option value="startsWith">Starts With</option>
								<option value="endsWith">Ends With</option>
							{:else if cond.operator.type === 'number'}
								<option value="equals">Equal</option>
								<option value="notEquals">Not Equal</option>
								<option value="gt">Greater Than</option>
								<option value="gte">Greater or Equal</option>
								<option value="lt">Less Than</option>
								<option value="lte">Less or Equal</option>
							{:else}
								<option value="true">Is True</option>
								<option value="false">Is False</option>
							{/if}
						</select>
					</div>
				</div>

				{#if cond.operator.operation !== 'true' && cond.operator.operation !== 'false'}
					<div class="grid gap-1.5">
						<Label class="text-[10px] text-muted-foreground uppercase font-bold">Value 2</Label>
						<Input value={cond.rightValue} oninput={(e: any) => updateCondition(i, 'rightValue', e.target.value)} placeholder="Value to compare" class="h-8 text-xs" />
					</div>
				{/if}

				<Button variant="ghost" size="icon" class="absolute -top-2 -right-2 h-6 w-6 rounded-full bg-background border shadow-sm opacity-0 group-hover:opacity-100 transition-opacity" onclick={() => removeCondition(i)}>
					<Trash2 class="h-3 w-3 text-destructive" />
				</Button>
			</div>
		{/each}

		<Button variant="outline" size="sm" class="w-full h-8 border-dashed" onclick={addCondition}>
			<Plus class="h-3 w-3 mr-2" /> Add Condition
		</Button>
	</div>

	<div class="pt-4 border-t space-y-4">
		<div class="flex items-center gap-2">
			<Settings2 class="h-4 w-4 text-muted-foreground" />
			<span class="text-xs font-bold uppercase">Options</span>
		</div>
		<div class="flex items-center justify-between">
			<Label class="text-xs">Ignore Case</Label>
			<input type="checkbox" checked={config.options?.ignoreCase ?? true} onchange={(e) => update('options', { ...config.options, ignoreCase: e.currentTarget.checked })} />
		</div>
	</div>
</div>
