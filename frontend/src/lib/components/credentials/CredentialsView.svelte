<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import AddCredentialModal from './AddCredentialModal.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Card from '$lib/components/ui/card';
	import { Plus, Trash2, Key, Calendar } from 'lucide-svelte';

	let showAddModal = $state(false);
</script>

<div class="flex-1 p-8 overflow-y-auto bg-background">
	<div class="flex items-center justify-between mb-8">
		<div>
			<h2 class="text-3xl font-bold tracking-tight">Credentials</h2>
			<p class="text-muted-foreground">Manage your API keys and secrets securely.</p>
		</div>
		<Button onclick={() => showAddModal = true}>
			<Plus class="mr-2 h-4 w-4" /> Add Credential
		</Button>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
		{#if nexus.isFetchingCredentials}
			<div class="col-span-full flex justify-center py-12">
				<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
			</div>
		{:else if nexus.credentials.length === 0}
			<Card.Root class="col-span-full py-12">
				<Card.Content class="flex flex-col items-center justify-center text-center">
					<div class="h-12 w-12 rounded-full bg-muted flex items-center justify-center mb-4">
						<Key class="h-6 w-6 text-muted-foreground opacity-50" />
					</div>
					<h3 class="text-lg font-semibold">No credentials yet</h3>
					<p class="text-sm text-muted-foreground max-w-xs mx-auto">Add your first credential to start using AI and third-party nodes.</p>
				</Card.Content>
			</Card.Root>
		{:else}
			{#each nexus.credentials as cred}
				<Card.Root class="group transition-all hover:shadow-md border-border/50">
					<Card.Header class="pb-3 flex flex-row items-center justify-between space-y-0">
						<Badge variant="secondary" class="uppercase text-[10px] font-bold tracking-wider">
							{cred.provider}
						</Badge>
						<Button variant="ghost" size="icon" class="h-8 w-8 text-destructive opacity-0 group-hover:opacity-100 transition-opacity" 
							onclick={() => nexus.deleteCredential(cred.id)}>
							<Trash2 class="h-4 w-4" />
						</Button>
					</Card.Header>
					<Card.Content>
						<h3 class="text-lg font-bold truncate mb-4">{cred.name}</h3>
						<div class="flex items-center gap-2 text-xs text-muted-foreground">
							<Calendar class="h-3 w-3" />
							Added {new Date(cred.created_at).toLocaleDateString()}
						</div>
					</Card.Content>
				</Card.Root>
			{/each}
		{/if}
	</div>
</div>

{#if showAddModal}
	<AddCredentialModal onClose={() => showAddModal = false} />
{/if}