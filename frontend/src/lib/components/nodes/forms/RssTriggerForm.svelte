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
</script>

<div class="space-y-4">
	<div class="grid gap-2">
		<Label for="feedUrl">Feed URL</Label>
		<Input id="feedUrl" type="text" value={config.feedUrl ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('feedUrl', e.currentTarget.value)} placeholder="https://example.com/rss" />
	</div>
	
	<div class="grid gap-2">
		<Label for="pollInterval">Poll Interval (minutes)</Label>
		<Input id="pollInterval" type="number" value={config.pollInterval ?? 15} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('pollInterval', parseInt(e.currentTarget.value))} placeholder="15" />
	</div>
</div>
