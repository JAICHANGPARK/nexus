<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Loader2 } from 'lucide-svelte';

	let { onClose } = $props<{ onClose: () => void }>();

	let provider = $state('openai');
	let name = $state('');
	let apiKey = $state('');
	let isSubmitting = $state(false);

	const API_BASE = import.meta.env.VITE_API_BASE ?? 'http://localhost:3001';

	async function create() {
		if (!name || !apiKey) return;
		isSubmitting = true;
		try {
			await fetch(`${API_BASE}/api/credentials`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ provider, name, data: { api_key: apiKey } })
			});
			await nexus.fetchCredentials();
			onClose();
		} catch (e) {
			console.error(e);
		} finally {
			isSubmitting = false;
		}
	}
</script>

<Dialog.Root open={true} onOpenChange={(open) => !open && onClose()}>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Add New Credential</Dialog.Title>
			<Dialog.Description>
				Enter your provider details below. Your keys are stored securely.
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="grid gap-2">
				<Label for="cred-provider">Provider</Label>
				<select id="cred-provider" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
					bind:value={provider}>
					<option value="openai">OpenAI</option>
					<option value="openrouter">OpenRouter</option>
					<option value="anthropic">Anthropic</option>
					<option value="google">Google Gemini</option>
				</select>
			</div>
			<div class="grid gap-2">
				<Label for="cred-name">Name</Label>
				<Input id="cred-name" placeholder="e.g. My OpenAI Key" bind:value={name} />
			</div>
			<div class="grid gap-2">
				<Label for="cred-key">API Key</Label>
				<Input id="cred-key" type="password" placeholder="sk-..." bind:value={apiKey} />
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={onClose}>Cancel</Button>
			<Button onclick={create} disabled={isSubmitting}>
				{#if isSubmitting}
					<Loader2 class="mr-2 h-4 w-4 animate-spin" />
					Saving...
				{:else}
					Create Credential
				{/if}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>