<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';

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
</script>

<div class="space-y-4">
	<div class="grid gap-2">
		<Label for="credential">Credential</Label>
		<select id="credential" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50" 
			value={config.credentialId ?? ''} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('credentialId', e.currentTarget.value)}>
			<option value="">(Environment Variable)</option>
			{#each nexus.credentials.filter(c => c.provider === 'openai') as cred}
				<option value={cred.id}>{cred.name}</option>
			{/each}
		</select>
	</div>

	<div class="grid gap-2">
		<Label for="resource">Resource</Label>
		<select id="resource" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.resource ?? 'chat'} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('resource', e.currentTarget.value)}>
			<option value="chat">Chat</option>
			<option value="image">Image</option>
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
				placeholder="e.g. gpt-4o"
			/>
			<select class="flex h-9 w-[120px] rounded-md border border-input bg-transparent px-2 py-1 text-[10px] shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
				onchange={(e: Event & { currentTarget: HTMLSelectElement }) => {
					if (e.currentTarget.value) update('model', e.currentTarget.value);
				}}>
				<option value="">Presets</option>
				{#if config.resource === 'image'}
					<option value="dall-e-3">DALL-E 3</option>
					<option value="dall-e-2">DALL-E 2</option>
				{:else}
					<option value="gpt-4o">GPT-4o</option>
					<option value="gpt-4-turbo">GPT-4 Turbo</option>
					<option value="gpt-3.5-turbo">GPT-3.5 Turbo</option>
				{/if}
			</select>
		</div>
	</div>

	<div class="grid gap-2">
		<Label for="prompt">Prompt</Label>
		<Textarea id="prompt" value={config.prompt ?? ''} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('prompt', e.currentTarget.value)} placeholder="Enter your prompt here..." rows={6} />
	</div>

	{#if config.resource === 'chat'}
		<div class="grid gap-2">
			<Label for="systemMessage">System Message</Label>
			<Textarea id="systemMessage" value={config.systemMessage ?? ''} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('systemMessage', e.currentTarget.value)} placeholder="Enter system message..." rows={3} />
		</div>
	{/if}
</div>
