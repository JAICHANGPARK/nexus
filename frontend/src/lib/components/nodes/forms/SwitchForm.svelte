<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { Plus, Trash2, Settings2, ChevronDown, ChevronRight } from 'lucide-svelte';

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
	let rules = $derived(config.rules?.values || []);
	let expandedRules = $state<Record<number, boolean>>({});

	function addRule() {
		const newRules = [...rules, {
			conditions: {
				conditions: [{ leftValue: '', rightValue: '', operator: { type: 'string', operation: 'equals' } }],
				combinator: 'and'
			},
			outputKey: `Output ${rules.length}`
		}];
		update('rules', { values: newRules });
	}

	function removeRule(index: number) {
		const newRules = rules.filter((_: any, i: number) => i !== index);
		update('rules', { values: newRules });
	}

	function updateRule(index: number, key: string, value: any) {
		const newRules = rules.map((r: any, i: number) => {
			if (i === index) {
				if (key === 'outputKey') return { ...r, outputKey: value };
				if (key === 'conditions') return { ...r, conditions: value };
			}
			return r;
		});
		update('rules', { values: newRules });
	}

	function toggleExpand(i: number) {
		expandedRules[i] = !expandedRules[i];
	}
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<Label class="text-xs font-bold uppercase tracking-widest text-muted-foreground">Routing Rules</Label>
		<Button variant="outline" size="icon" class="h-6 w-6" onclick={addRule}>
			<Plus class="h-3 w-3" />
		</Button>
	</div>

	<div class="space-y-3">
		{#each rules as rule, i}
			<div class="rounded-lg border bg-card shadow-sm overflow-hidden">
				<div class="flex items-center gap-2 p-2 bg-muted/20">
					<button class="p-1 hover:bg-muted rounded" onclick={() => toggleExpand(i)}>
						{#if expandedRules[i]}<ChevronDown class="h-3 w-3" />{:else}<ChevronRight class="h-3 w-3" />{/if}
					</button>
					<Input value={rule.outputKey} oninput={(e: any) => updateRule(i, 'outputKey', e.target.value)} class="h-7 text-[10px] font-bold uppercase py-0 px-2 border-none bg-transparent focus-visible:ring-0" />
					<Button variant="ghost" size="icon" class="h-6 w-6 text-destructive ml-auto" onclick={() => removeRule(i)}>
						<Trash2 class="h-3 w-3" />
					</Button>
				</div>

				{#if expandedRules[i]}
					<div class="p-3 space-y-4">
						{#each rule.conditions.conditions as cond, ci}
							<div class="space-y-2 border-l-2 border-primary/20 pl-3 pt-1">
								<Input value={cond.leftValue} oninput={(e: any) => {
									const newConds = [...rule.conditions.conditions];
									newConds[ci] = { ...cond, leftValue: e.target.value };
									updateRule(i, 'conditions', { ...rule.conditions, conditions: newConds });
								}} placeholder="Value 1" class="h-7 text-xs" />
								
								<div class="grid grid-cols-2 gap-2">
									<select class="h-7 rounded-md border border-input bg-background px-2 text-[10px]" value={cond.operator.operation} onchange={(e) => {
										const newConds = [...rule.conditions.conditions];
										newConds[ci] = { ...cond, operator: { ...cond.operator, operation: e.currentTarget.value } };
										updateRule(i, 'conditions', { ...rule.conditions, conditions: newConds });
									}}>
										<option value="equals">Equals</option>
										<option value="contains">Contains</option>
										<option value="startsWith">Starts With</option>
									</select>
									<Input value={cond.rightValue} oninput={(e: any) => {
										const newConds = [...rule.conditions.conditions];
										newConds[ci] = { ...cond, rightValue: e.target.value };
										updateRule(i, 'conditions', { ...rule.conditions, conditions: newConds });
									}} placeholder="Value 2" class="h-7 text-xs" />
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</div>

	<div class="pt-4 border-t space-y-4">
		<div class="flex items-center gap-2">
			<Settings2 class="h-4 w-4 text-muted-foreground" />
			<span class="text-xs font-bold uppercase">Options</span>
		</div>
		<div class="flex items-center justify-between">
			<Label class="text-xs">Fallback Output</Label>
			<select class="h-8 rounded-md border border-input bg-background px-2 text-xs" value={config.options?.fallbackOutput ?? 'none'} onchange={(e) => update('options', { ...config.options, fallbackOutput: e.currentTarget.value })}>
				<option value="none">Ignore</option>
				<option value="fallback">Extra Output</option>
			</select>
		</div>
	</div>
</div>
