<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';

	let { node } = $props<{ node: Node }>();
	
	function update(key: string, value: any) {
		const currentConfig = (node.data.config as Record<string, any>) || {};
		nexus.nodes = nexus.nodes.map(n => n.id === node.id ? {
			...n,
			data: { ...n.data, config: { ...currentConfig, [key]: value } }
		} : n);
		// Sync selectedNode to reflect changes in UI
		if (nexus.selectedNode?.id === node.id) {
			nexus.selectedNode = {
				...nexus.selectedNode,
				data: { ...nexus.selectedNode.data, config: { ...currentConfig, [key]: value } }
			};
		}
	}

	let config = $derived((node.data.config as Record<string, any>) || {});
</script>

<div class="space-y-4">
	<div class="grid gap-2">
		<Label for="credential">Credential</Label>
		<select id="credential" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.credentialId ?? ''} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('credentialId', e.currentTarget.value)}>
			<option value="">(Environment Variable)</option>
			{#each nexus.credentials.filter(c => c.provider === 'openrouter') as cred}
				<option value={cred.id}>{cred.name}</option>
			{/each}
		</select>
	</div>

	<div class="grid gap-2">
		<Label for="model">Model</Label>
		<div class="flex gap-2">
			<input 
				id="model" 
				type="text" 
				class="flex h-9 flex-1 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
				value={config.model ?? ''} 
				oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('model', e.currentTarget.value)}
				placeholder="e.g. anthropic/claude-3.5-sonnet"
			/>
			<select class="flex h-9 w-[120px] rounded-md border border-input bg-transparent px-2 py-1 text-[10px] shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
				onchange={(e: Event & { currentTarget: HTMLSelectElement }) => {
					if (e.currentTarget.value) update('model', e.currentTarget.value);
				}}>
				<option value="">Presets</option>
				<option value="openai/gpt-4o">GPT-4o</option>
				<option value="anthropic/claude-3.5-sonnet">Claude 3.5 Sonnet</option>
				<option value="google/gemini-pro-1.5">Gemini Pro 1.5</option>
				<option value="meta-llama/llama-3-70b-instruct">Llama 3 70B</option>
			</select>
		</div>
	</div>

	<div class="grid gap-2">
		<Label for="prompt">Prompt</Label>
		<Textarea id="prompt" value={config.prompt ?? ''} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('prompt', e.currentTarget.value)} rows={5} />
	</div>

	<div class="grid gap-3">
		<div class="flex items-center justify-between">
			<Label for="temperature" class="text-xs">Temperature</Label>
			<span class="text-[10px] font-mono font-bold bg-muted px-1.5 py-0.5 rounded text-muted-foreground">{config.temperature ?? 0.7}</span>
		</div>
		<input id="temperature" type="range" min="0" max="2" step="0.1" class="w-full accent-primary h-1.5 bg-muted rounded-lg appearance-none cursor-pointer" 
			value={config.temperature ?? 0.7} 
			oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('temperature', parseFloat(e.currentTarget.value))} />
	</div>

	<div class="grid gap-2">
		<Label for="max-tokens">Max Tokens</Label>
		<Input id="max-tokens" type="number" value={config.maxTokens ?? 1000} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('maxTokens', parseInt(e.currentTarget.value))} />
	</div>

	<div class="grid gap-2">
		<Label for="response-format">Response Format</Label>
		<select id="response-format" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.responseFormat ?? 'text'} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('responseFormat', e.currentTarget.value)}>
			<option value="text">Text</option>
			<option value="json_object">JSON Object</option>
		</select>
	</div>
</div>
