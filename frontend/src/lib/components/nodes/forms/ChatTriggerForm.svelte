<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Switch } from '$lib/components/ui/switch';

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
		<Label for="mode">Mode</Label>
		<select id="mode" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.mode ?? 'hostedChat'} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('mode', e.currentTarget.value)}>
			<option value="hostedChat">Hosted Chat</option>
			<option value="webhook">Webhook</option>
		</select>
	</div>

	<div class="flex items-center justify-between rounded-lg border p-3 shadow-sm bg-muted/20">
		<div class="space-y-0.5">
			<Label for="public-access" class="text-xs font-bold uppercase tracking-wider">Public Access</Label>
			<div class="text-[10px] text-muted-foreground">Allow anyone to access this chat</div>
		</div>
		<Switch id="public-access" checked={config.public ?? false} onCheckedChange={(v: boolean) => update('public', v)} />
	</div>

	<div class="grid gap-2">
		<Label for="initial-messages">Initial Messages</Label>
		<Textarea id="initial-messages" value={config.initialMessages ?? ''} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('initialMessages', e.currentTarget.value)} placeholder="Enter starting messages..." rows={4} />
	</div>
</div>
