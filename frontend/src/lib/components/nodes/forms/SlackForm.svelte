<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
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

	const resources = [
		{ value: 'message', label: 'Message' },
		{ value: 'channel', label: 'Channel' },
		{ value: 'user', label: 'User' }
	];

	const operations: Record<string, { value: string, label: string }[]> = {
		message: [
			{ value: 'post', label: 'Send' },
			{ value: 'sendAndWait', label: 'Send and Wait' },
			{ value: 'postEphemeral', label: 'Send Ephemeral' },
			{ value: 'update', label: 'Update' },
			{ value: 'delete', label: 'Delete' },
			{ value: 'search', label: 'Search' }
		],
		channel: [
			{ value: 'create', label: 'Create' },
			{ value: 'getAll', label: 'Get Many' }
		],
		user: [
			{ value: 'info', label: 'Get' },
			{ value: 'getAll', label: 'Get Many' }
		]
	};
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
		<Label for="resource">Resource</Label>
		<select id="resource" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.resource ?? 'message'} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('resource', e.currentTarget.value)}>
			{#each resources as r}
				<option value={r.value}>{r.label}</option>
			{/each}
		</select>
	</div>

	<div class="grid gap-2">
		<Label for="operation">Operation</Label>
		<select id="operation" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.operation ?? 'post'} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('operation', e.currentTarget.value)}>
			{#each operations[config.resource ?? 'message'] || [] as o}
				<option value={o.value}>{o.label}</option>
			{/each}
		</select>
	</div>

	<Separator />

	{#if config.resource === 'message'}
		{#if config.operation === 'post' || config.operation === 'postEphemeral' || config.operation === 'sendAndWait'}
			<div class="grid gap-2">
				<Label for="channel">Channel</Label>
				<Input id="channel" type="text" value={config.channel ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('channel', e.currentTarget.value)} placeholder="e.g. #general or C12345678" />
			</div>

			{#if config.operation === 'sendAndWait'}
				<div class="grid grid-cols-2 gap-4">
					<div class="grid gap-2">
						<Label for="approve">Approve Label</Label>
						<Input id="approve" type="text" value={config.approveLabel ?? 'Approve'} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('approveLabel', e.currentTarget.value)} />
					</div>
					<div class="grid gap-2">
						<Label for="reject">Reject Label</Label>
						<Input id="reject" type="text" value={config.rejectLabel ?? 'Reject'} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('rejectLabel', e.currentTarget.value)} />
					</div>
				</div>
			{/if}

			{#if config.operation === 'postEphemeral'}
				<div class="grid gap-2">
					<Label for="user">User</Label>
					<Input id="user" type="text" value={config.user ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('user', e.currentTarget.value)} placeholder="User ID" />
				</div>
			{/if}

			<div class="grid gap-2">
				<Label for="text">Message Text</Label>
				<Textarea id="text" value={config.text ?? ''} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('text', e.currentTarget.value)} placeholder="Enter message text..." rows={4} />
			</div>
		{:else if config.operation === 'update' || config.operation === 'delete'}
			<div class="grid gap-2">
				<Label for="channelId">Channel ID</Label>
				<Input id="channelId" type="text" value={config.channelId ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('channelId', e.currentTarget.value)} placeholder="C12345678" />
			</div>
			<div class="grid gap-2">
				<Label for="ts">Message Timestamp (ts)</Label>
				<Input id="ts" type="text" value={config.ts ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('ts', e.currentTarget.value)} placeholder="1234567890.123456" />
			</div>
			{#if config.operation === 'update'}
				<div class="grid gap-2">
					<Label for="text">New Text</Label>
					<Textarea id="text" value={config.text ?? ''} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('text', e.currentTarget.value)} placeholder="Enter new text..." rows={4} />
				</div>
			{/if}
		{:else if config.operation === 'search'}
			<div class="grid gap-2">
				<Label for="query">Search Query</Label>
				<Input id="query" type="text" value={config.query ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('query', e.currentTarget.value)} placeholder="Search for something..." />
			</div>
		{/if}
	{:else if config.resource === 'channel'}
		{#if config.operation === 'create'}
			<div class="grid gap-2">
				<Label for="name">Channel Name</Label>
				<Input id="name" type="text" value={config.name ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('name', e.currentTarget.value)} placeholder="new-channel" />
			</div>
			<div class="flex items-center justify-between rounded-lg border p-3 shadow-sm bg-muted/20">
				<div class="space-y-0.5">
					<Label for="isPrivate" class="text-xs font-bold uppercase tracking-wider">Is Private</Label>
				</div>
				<Switch id="isPrivate" checked={config.isPrivate ?? false} onCheckedChange={(v: boolean) => update('isPrivate', v)} />
			</div>
		{/if}
	{:else if config.resource === 'user'}
		{#if config.operation === 'info'}
			<div class="grid gap-2">
				<Label for="user">User ID</Label>
				<Input id="user" type="text" value={config.user ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('user', e.currentTarget.value)} placeholder="U12345678" />
			</div>
		{/if}
	{/if}
</div>