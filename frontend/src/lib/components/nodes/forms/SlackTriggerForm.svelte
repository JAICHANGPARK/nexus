<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { Switch } from '$lib/components/ui/switch';
	import { Separator } from '$lib/components/ui/separator';

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

	const triggerOptions = [
		{ value: 'message', label: 'New Message' },
		{ value: 'app_mention', label: 'App Mention' },
		{ value: 'reaction_added', label: 'Reaction Added' },
		{ value: 'team_join', label: 'New User Joined' },
		{ value: 'channel_created', label: 'New Channel Created' },
		{ value: 'file_shared', label: 'File Shared' }
	];
</script>

<div class="space-y-4 pb-10">
	<div class="grid gap-2">
		<Label for="credential">Credential</Label>
		<select id="credential" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.credentialId ?? ''} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('credentialId', e.currentTarget.value)}>
			<option value="">(Environment Variable: SLACK_TOKEN)</option>
			{#each nexus.credentials.filter(c => c.provider === 'slack') as cred}
				<option value={cred.id}>{cred.name}</option>
			{/each}
		</select>
	</div>

	<div class="grid gap-2">
		<Label for="trigger">Trigger On</Label>
		<select id="trigger" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.trigger ?? 'message'} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('trigger', e.currentTarget.value)}>
			{#each triggerOptions as opt}
				<option value={opt.value}>{opt.label}</option>
			{/each}
		</select>
	</div>

	<div class="flex items-center justify-between rounded-lg border p-3 shadow-sm bg-muted/20">
		<div class="space-y-0.5">
			<Label for="watchWorkspace" class="text-xs font-bold uppercase tracking-wider">Watch Whole Workspace</Label>
		</div>
		<Switch id="watchWorkspace" checked={config.watchWorkspace ?? false} onCheckedChange={(v: boolean) => update('watchWorkspace', v)} />
	</div>

	{#if !config.watchWorkspace}
		<div class="grid gap-2">
			<Label for="channelId">Channel ID to Watch</Label>
			<Input id="channelId" type="text" value={config.channelId ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('channelId', e.currentTarget.value)} placeholder="C12345678" />
		</div>
	{/if}

	<Separator />

	<div class="flex items-center justify-between rounded-lg border p-3 shadow-sm bg-muted/20">
		<div class="space-y-0.5">
			<Label for="resolveIds" class="text-xs font-bold uppercase tracking-wider">Resolve IDs (Names)</Label>
		</div>
		<Switch id="resolveIds" checked={config.resolveIds ?? false} onCheckedChange={(v: boolean) => update('resolveIds', v)} />
	</div>
</div>