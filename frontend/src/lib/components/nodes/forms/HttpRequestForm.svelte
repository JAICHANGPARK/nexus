<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';
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

<div class="space-y-4 pb-4">
	<div class="grid gap-2">
		<Label for="method">Method</Label>
		<select id="method" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.method ?? 'GET'} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('method', e.currentTarget.value)}>
			{#each ['GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'HEAD', 'OPTIONS'] as m}
				<option value={m}>{m}</option>
			{/each}
		</select>
	</div>

	<div class="grid gap-2">
		<Label for="url">URL</Label>
		<Input id="url" type="text" value={config.url ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('url', e.currentTarget.value)} placeholder="https://api.example.com" />
	</div>

	<div class="flex items-center justify-between rounded-lg border p-3 shadow-sm bg-muted/20">
		<div class="space-y-0.5">
			<Label for="full-response" class="text-xs font-bold uppercase tracking-wider">Full Response</Label>
			<div class="text-[10px] text-muted-foreground">Include headers and status code</div>
		</div>
		<Switch id="full-response" checked={config.fullResponse ?? false} onCheckedChange={(v: boolean) => update('fullResponse', v)} />
	</div>

	<div class="grid gap-2">
		<Label for="authentication">Authentication</Label>
		<select id="authentication" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={config.authentication ?? 'none'} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('authentication', e.currentTarget.value)}>
			<option value="none">None</option>
			<option value="basicAuth">Basic Auth</option>
			<option value="headerAuth">Header Auth</option>
		</select>
	</div>

	{#if config.authentication === 'basicAuth'}
		<div class="grid gap-3 rounded-lg border p-3 bg-muted/10">
			<div class="grid gap-1.5">
				<Label for="user">User</Label>
				<Input id="user" type="text" value={config.user ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('user', e.currentTarget.value)} />
			</div>
			<div class="grid gap-1.5">
				<Label for="password">Password</Label>
				<Input id="password" type="password" value={config.password ?? ''} oninput={(e: Event & { currentTarget: HTMLInputElement }) => update('password', e.currentTarget.value)} />
			</div>
		</div>
	{/if}

	<div class="flex items-center justify-between rounded-lg border p-3 shadow-sm bg-muted/20">
		<div class="space-y-0.5">
			<Label for="send-query" class="text-xs font-bold uppercase tracking-wider">Send Query Parameters</Label>
		</div>
		<Switch id="send-query" checked={config.sendQuery ?? false} onCheckedChange={(v: boolean) => update('sendQuery', v)} />
	</div>

	{#if config.sendQuery}
		<div class="grid gap-2 pl-2">
			<Label for="json-query">Query Parameters (JSON)</Label>
			<Textarea id="json-query" value={config.jsonQuery ?? ''} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('jsonQuery', e.currentTarget.value)} placeholder={`{"name": "value"}`} rows={3} />
		</div>
	{/if}

	<div class="flex items-center justify-between rounded-lg border p-3 shadow-sm bg-muted/20">
		<div class="space-y-0.5">
			<Label for="send-body" class="text-xs font-bold uppercase tracking-wider">Send Body</Label>
		</div>
		<Switch id="send-body" checked={config.sendBody ?? false} onCheckedChange={(v: boolean) => update('sendBody', v)} />
	</div>

	{#if config.sendBody}
		<div class="grid gap-3 rounded-lg border p-3 bg-muted/10">
			<div class="grid gap-1.5">
				<Label for="content-type">Body Content Type</Label>
				<select id="content-type" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
					value={config.contentType ?? 'json'} 
					onchange={(e: Event & { currentTarget: HTMLSelectElement }) => update('contentType', e.currentTarget.value)}>
					<option value="json">JSON</option>
					<option value="form-urlencoded">Form Urlencoded</option>
					<option value="raw">Raw</option>
				</select>
			</div>
			<div class="grid gap-1.5">
				<Label for="body">Body</Label>
				<Textarea id="body" value={config.body ?? ''} oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('body', e.currentTarget.value)} placeholder={`{"key": "value"}`} rows={5} />
			</div>
		</div>
	{/if}
</div>