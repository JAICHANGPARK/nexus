<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Button } from '$lib/components/ui/button';
	import { Plus, Trash2 } from 'lucide-svelte';

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
	let operation = $derived(config.operation || 'select');
</script>

<div class="space-y-4">
	<div class="grid gap-2">
		<Label>Credential</Label>
		<select 
			class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
			value={config.credentialId || ''}
			onchange={(e) => update('credentialId', e.currentTarget.value)}
		>
			<option value="">Select a credential...</option>
			{#each nexus.credentials.filter(c => c.provider === 'postgres') as cred}
				<option value={cred.id}>{cred.name}</option>
			{/each}
		</select>
	</div>

	<div class="grid gap-2">
		<Label>Operation</Label>
		<select 
			class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
			value={operation}
			onchange={(e) => update('operation', e.currentTarget.value)}
		>
			<option value="select">Select</option>
			<option value="insert">Insert</option>
			<option value="update">Update</option>
			<option value="upsert">Upsert</option>
			<option value="delete">Delete</option>
			<option value="executeQuery">Execute Query</option>
		</select>
	</div>

	{#if operation !== 'executeQuery'}
		<div class="grid grid-cols-2 gap-4">
			<div class="grid gap-2">
				<Label for="schema">Schema</Label>
				<Input id="schema" value={config.schema ?? 'public'} oninput={(e: any) => update('schema', e.target.value)} />
			</div>
			<div class="grid gap-2">
				<Label for="table">Table</Label>
				<Input id="table" value={config.table ?? ''} oninput={(e: any) => update('table', e.target.value)} placeholder="table_name" />
			</div>
		</div>
	{/if}

	{#if operation === 'executeQuery'}
		<div class="grid gap-2">
			<Label for="query">Query</Label>
			<Textarea id="query" value={config.query ?? ''} oninput={(e: any) => update('query', e.target.value)} placeholder="SELECT * FROM users WHERE id = $1" rows={5} />
			<p class="text-[10px] text-muted-foreground">Use $1, $2 for parameters from Query Parameters option.</p>
		</div>
	{/if}

	{#if operation === 'select' || operation === 'delete'}
		<div class="grid gap-2">
			<Label for="where">Where Clause (JSON or String)</Label>
			<Textarea id="where" value={config.where ?? ''} oninput={(e: any) => update('where', e.target.value)} placeholder={'id = 1 or {"status": "active"}'} rows={3} />
		</div>
	{/if}

	{#if operation === 'insert' || operation === 'update' || operation === 'upsert'}
		<div class="grid gap-2">
			<Label for="columns">Columns to Set (Comma separated)</Label>
			<Input id="columns" value={config.columns ?? ''} oninput={(e: any) => update('columns', e.target.value)} placeholder="email, name, age" />
		</div>
		{#if operation === 'update' || operation === 'upsert'}
			<div class="grid gap-2">
				<Label for="updateKey">Update Key (Column to match on)</Label>
				<Input id="updateKey" value={config.updateKey ?? 'id'} oninput={(e: any) => update('updateKey', e.target.value)} />
			</div>
		{/if}
	{/if}

	{#if operation === 'select'}
		<div class="grid grid-cols-2 gap-4">
			<div class="grid gap-2">
				<Label for="limit">Limit</Label>
				<Input id="limit" type="number" value={config.limit ?? 50} oninput={(e: any) => update('limit', parseInt(e.target.value))} />
			</div>
			<div class="grid gap-2">
				<Label for="sort">Sort (e.g. id DESC)</Label>
				<Input id="sort" value={config.sort ?? ''} oninput={(e: any) => update('sort', e.target.value)} />
			</div>
		</div>
	{/if}
</div>
