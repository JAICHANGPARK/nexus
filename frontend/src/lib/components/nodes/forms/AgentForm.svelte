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

	const providers = [
		{ value: 'openai', label: 'OpenAI' },
		{ value: 'openrouter', label: 'OpenRouter' },
		{ value: 'anthropic', label: 'Anthropic' }
	];

	const modelsByProvider: Record<string, {value: string, label: string}[]> = {
		openai: [
			{ value: 'gpt-4o', label: 'GPT-4o' },
			{ value: 'gpt-4-turbo', label: 'GPT-4 Turbo' },
			{ value: 'gpt-3.5-turbo', label: 'GPT-3.5 Turbo' }
		],
		openrouter: [
			{ value: 'openai/gpt-4o', label: 'GPT-4o (via OR)' },
			{ value: 'anthropic/claude-3.5-sonnet', label: 'Claude 3.5 Sonnet' },
			{ value: 'google/gemini-pro-1.5', label: 'Gemini Pro 1.5' },
			{ value: 'meta-llama/llama-3-70b-instruct', label: 'Llama 3 70B' }
		],
		anthropic: [
			{ value: 'claude-3-5-sonnet-20240620', label: 'Claude 3.5 Sonnet' },
			{ value: 'claude-3-opus-20240229', label: 'Claude 3 Opus' }
		]
	};

	let provider = $derived(config.provider ?? 'openai');
	let availableModels = $derived(modelsByProvider[provider] || []);
</script>

<div class="space-y-4">
	<div class="grid gap-2">
		<Label for="provider">LLM Provider</Label>
		<select id="provider" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.provider ?? 'openai'} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => {
				update('provider', e.currentTarget.value);
				// Reset model when provider changes
				const firstModel = modelsByProvider[e.currentTarget.value]?.[0]?.value;
				if (firstModel) update('model', firstModel);
			}}>
			{#each providers as p}
				<option value={p.value}>{p.label}</option>
			{/each}
		</select>
	</div>

	<div class="grid gap-2">
		<Label for="credential">Credential</Label>
		<select id="credential" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.credentialId ?? ''} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('credentialId', e.currentTarget.value)}>
			<option value="">(Environment Variable)</option>
			{#each nexus.credentials.filter(c => c.provider === provider) as cred}
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
				placeholder="e.g. gpt-4o"
			/>
			<select class="flex h-9 w-[120px] rounded-md border border-input bg-transparent px-2 py-1 text-[10px] shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
				onchange={(e: Event & { currentTarget: HTMLSelectElement }) => {
					if (e.currentTarget.value) update('model', e.currentTarget.value);
				}}>
				<option value="">Presets</option>
				{#each availableModels as m}
					<option value={m.value}>{m.label}</option>
				{/each}
			</select>
		</div>
		<p class="text-[10px] text-muted-foreground italic">Type a custom model ID or pick from presets.</p>
	</div>

	<div class="grid gap-2">
		<Label for="systemMessage">Agent System Prompt (Role)</Label>
		<Textarea id="systemMessage" value={config.systemMessage ?? ''} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('systemMessage', e.currentTarget.value)} placeholder="You are a helpful assistant that can use tools..." rows={3} />
	</div>

	<div class="grid gap-2">
		<Label for="prompt">User Prompt / Instructions</Label>
		<Textarea id="prompt" value={config.prompt ?? ''} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('prompt', e.currentTarget.value)} placeholder="What should the agent do?" rows={4} />
	</div>
</div>