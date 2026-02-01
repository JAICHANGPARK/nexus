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
		{ value: 'file', label: 'File' },
		{ value: 'reaction', label: 'Reaction' },
		{ value: 'star', label: 'Star' },
		{ value: 'user', label: 'User' },
		{ value: 'userGroup', label: 'User Group' },
		{ value: 'userProfile', label: 'User Profile' }
	];

	const operations: Record<string, { value: string, label: string }[]> = {
		message: [
			{ value: 'post', label: 'Send' },
			{ value: 'sendAndWait', label: 'Send and Wait' },
			{ value: 'postEphemeral', label: 'Send Ephemeral' },
			{ value: 'update', label: 'Update' },
			{ value: 'delete', label: 'Delete' },
			{ value: 'getPermalink', label: 'Get Permalink' },
			{ value: 'search', label: 'Search' }
		],
		channel: [
			{ value: 'create', label: 'Create' },
			{ value: 'get', label: 'Get' },
			{ value: 'getAll', label: 'Get Many' },
			{ value: 'history', label: 'History' },
			{ value: 'invite', label: 'Invite' },
			{ value: 'join', label: 'Join' },
			{ value: 'kick', label: 'Kick' },
			{ value: 'leave', label: 'Leave' },
			{ value: 'member', label: 'Members' },
			{ value: 'rename', label: 'Rename' },
			{ value: 'replies', label: 'Replies' },
			{ value: 'setPurpose', label: 'Set Purpose' },
			{ value: 'setTopic', label: 'Set Topic' },
			{ value: 'archive', label: 'Archive' },
			{ value: 'unarchive', label: 'Unarchive' },
			{ value: 'close', label: 'Close' }
		],
		file: [
			{ value: 'upload', label: 'Upload' },
			{ value: 'get', label: 'Get' },
			{ value: 'getAll', label: 'Get Many' }
		],
		reaction: [
			{ value: 'add', label: 'Add' },
			{ value: 'get', label: 'Get' },
			{ value: 'remove', label: 'Remove' }
		],
		star: [
			{ value: 'add', label: 'Add' },
			{ value: 'delete', label: 'Delete' },
			{ value: 'getAll', label: 'Get Many' }
		],
		user: [
			{ value: 'info', label: 'Get' },
			{ value: 'getAll', label: 'Get Many' }
		],
		userGroup: [
			{ value: 'create', label: 'Create' },
			{ value: 'update', label: 'Update' },
			{ value: 'getAll', label: 'Get Many' },
			{ value: 'enable', label: 'Enable' },
			{ value: 'disable', label: 'Disable' },
			{ value: 'getUsers', label: 'Get Users' },
			{ value: 'updateUsers', label: 'Update Users' }
		],
		userProfile: [
			{ value: 'get', label: 'Get' },
			{ value: 'update', label: 'Update' }
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
		{:else if config.operation === 'update' || config.operation === 'delete' || config.operation === 'getPermalink'}
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
		{:else if ['get', 'history', 'invite', 'join', 'kick', 'leave', 'member', 'rename', 'replies', 'setPurpose', 'setTopic', 'archive', 'unarchive', 'close'].includes(config.operation)}
			<div class="grid gap-2">
				<Label for="channelId">Channel ID</Label>
				<Input id="channelId" type="text" value={config.channelId ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('channelId', e.currentTarget.value)} placeholder="C12345678" />
			</div>
			{#if config.operation === 'invite'}
				<div class="grid gap-2">
					<Label for="userIds">User IDs (comma separated)</Label>
					<Input id="userIds" type="text" value={config.userIds ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('userIds', e.currentTarget.value)} placeholder="U123,U456" />
				</div>
			{:else if config.operation === 'kick'}
				<div class="grid gap-2">
					<Label for="userId">User ID</Label>
					<Input id="userId" type="text" value={config.userId ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('userId', e.currentTarget.value)} placeholder="U12345678" />
				</div>
			{:else if config.operation === 'rename'}
				<div class="grid gap-2">
					<Label for="name">New Name</Label>
					<Input id="name" type="text" value={config.name ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('name', e.currentTarget.value)} placeholder="new-name" />
				</div>
			{:else if config.operation === 'replies'}
				<div class="grid gap-2">
					<Label for="ts">Thread Timestamp (ts)</Label>
					<Input id="ts" type="text" value={config.ts ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('ts', e.currentTarget.value)} placeholder="1234567890.123456" />
				</div>
			{:else if config.operation === 'setPurpose'}
				<div class="grid gap-2">
					<Label for="purpose">Purpose</Label>
					<Input id="purpose" type="text" value={config.purpose ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('purpose', e.currentTarget.value)} placeholder="Channel purpose" />
				</div>
			{:else if config.operation === 'setTopic'}
				<div class="grid gap-2">
					<Label for="topic">Topic</Label>
					<Input id="topic" type="text" value={config.topic ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('topic', e.currentTarget.value)} placeholder="Channel topic" />
				</div>
			{/if}
		{/if}
	{:else if config.resource === 'file'}
		{#if config.operation === 'upload'}
			<div class="grid gap-2">
				<Label for="channels">Channel IDs (comma separated)</Label>
				<Input id="channels" type="text" value={config.channels ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('channels', e.currentTarget.value)} placeholder="C123,C456" />
			</div>
			<div class="grid gap-2">
				<Label for="fileContent">File Content (Text)</Label>
				<Textarea id="fileContent" value={config.fileContent ?? ''} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('fileContent', e.currentTarget.value)} placeholder="File content..." rows={4} />
			</div>
			<div class="grid gap-2">
				<Label for="title">Title</Label>
				<Input id="title" type="text" value={config.title ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('title', e.currentTarget.value)} placeholder="My File" />
			</div>
		{:else if config.operation === 'get'}
			<div class="grid gap-2">
				<Label for="fileId">File ID</Label>
				<Input id="fileId" type="text" value={config.fileId ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('fileId', e.currentTarget.value)} placeholder="F12345678" />
			</div>
		{/if}
	{:else if config.resource === 'reaction'}
		<div class="grid gap-2">
			<Label for="channelId">Channel ID</Label>
			<Input id="channelId" type="text" value={config.channelId ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('channelId', e.currentTarget.value)} placeholder="C12345678" />
		</div>
		<div class="grid gap-2">
			<Label for="ts">Message Timestamp (ts)</Label>
			<Input id="ts" type="text" value={config.ts ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('ts', e.currentTarget.value)} placeholder="1234567890.123456" />
		</div>
		{#if config.operation === 'add' || config.operation === 'remove'}
			<div class="grid gap-2">
				<Label for="name">Reaction Name (Emoji)</Label>
				<Input id="name" type="text" value={config.name ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('name', e.currentTarget.value)} placeholder="thumbsup" />
			</div>
		{/if}
	{:else if config.resource === 'star'}
		{#if config.operation === 'add' || config.operation === 'delete'}
			<div class="grid gap-2">
				<Label for="channelId">Channel ID</Label>
				<Input id="channelId" type="text" value={config.channelId ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('channelId', e.currentTarget.value)} placeholder="C12345678" />
			</div>
			<div class="grid gap-2">
				<Label for="ts">Message Timestamp (ts)</Label>
				<Input id="ts" type="text" value={config.ts ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('ts', e.currentTarget.value)} placeholder="1234567890.123456" />
			</div>
			<div class="grid gap-2">
				<Label for="fileId">File ID</Label>
				<Input id="fileId" type="text" value={config.fileId ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('fileId', e.currentTarget.value)} placeholder="F12345678" />
			</div>
		{/if}
	{:else if config.resource === 'user'}
		{#if config.operation === 'info'}
			<div class="grid gap-2">
				<Label for="user">User ID</Label>
				<Input id="user" type="text" value={config.user ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('user', e.currentTarget.value)} placeholder="U12345678" />
			</div>
		{/if}
	{:else if config.resource === 'userGroup'}
		{#if ['enable', 'disable', 'getUsers', 'updateUsers', 'update'].includes(config.operation)}
			<div class="grid gap-2">
				<Label for="userGroupId">User Group ID</Label>
				<Input id="userGroupId" type="text" value={config.userGroupId ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('userGroupId', e.currentTarget.value)} placeholder="S12345678" />
			</div>
		{/if}
		{#if config.operation === 'create' || config.operation === 'update'}
			<div class="grid gap-2">
				<Label for="name">Name</Label>
				<Input id="name" type="text" value={config.name ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('name', e.currentTarget.value)} placeholder="My Group" />
			</div>
			<div class="grid gap-2">
				<Label for="handle">Handle</Label>
				<Input id="handle" type="text" value={config.handle ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('handle', e.currentTarget.value)} placeholder="my-group" />
			</div>
			<div class="grid gap-2">
				<Label for="description">Description</Label>
				<Input id="description" type="text" value={config.description ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('description', e.currentTarget.value)} placeholder="Description..." />
			</div>
		{/if}
		{#if config.operation === 'updateUsers'}
			<div class="grid gap-2">
				<Label for="users">User IDs (comma separated)</Label>
				<Input id="users" type="text" value={config.users ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('users', e.currentTarget.value)} placeholder="U123,U456" />
			</div>
		{/if}
	{:else if config.resource === 'userProfile'}
		<div class="grid gap-2">
			<Label for="user">User ID</Label>
			<Input id="user" type="text" value={config.user ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('user', e.currentTarget.value)} placeholder="U12345678" />
		</div>
		{#if config.operation === 'update'}
			<div class="grid gap-2">
				<Label for="firstName">First Name</Label>
				<Input id="firstName" type="text" value={config.firstName ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('firstName', e.currentTarget.value)} />
			</div>
			<div class="grid gap-2">
				<Label for="lastName">Last Name</Label>
				<Input id="lastName" type="text" value={config.lastName ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('lastName', e.currentTarget.value)} />
			</div>
		{/if}
	{/if}
</div>