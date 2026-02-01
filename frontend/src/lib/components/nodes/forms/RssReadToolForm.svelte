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
		<Label for="toolName">Tool Name</Label>
		<Input id="toolName" type="text" value={config.toolName ?? 'rss_reader'} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('toolName', e.currentTarget.value)} placeholder="rss_reader" />
		<p class="text-[10px] text-muted-foreground">The name the AI will use to call this tool.</p>
	</div>

	<div class="grid gap-2">
		<Label for="url">Feed URL</Label>
		<Input id="url" type="text" value={config.url ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('url', e.currentTarget.value)} placeholder="https://example.com/rss" />
	</div>

	<div class="grid gap-2">
		<Label for="description">Description</Label>
		<Textarea id="description" value={config.description ?? 'Reads entries from an RSS feed.'} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('description', e.currentTarget.value)} placeholder="Help the AI understand when to use this tool..." rows={3} />
	</div>
</div>
