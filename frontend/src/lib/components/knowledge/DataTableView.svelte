<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Plus, Database, Search, MoreHorizontal, Settings2, Trash2, Loader2, X } from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';
	import * as Table from '$lib/components/ui/table';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import DataTableDetailsView from './DataTableDetailsView.svelte';

	let searchQuery = $state('');
	let isCreateModalOpen = $state(false);
	let newTableName = $state('');
	let newTableDesc = $state('');

	let filteredTables = $derived(
		nexus.dataTables.filter(t => 
			t.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
			(t.description?.toLowerCase().includes(searchQuery.toLowerCase()))
		)
	);

	function formatDate(dateStr: string) {
		const date = new Date(dateStr);
		return date.toLocaleString();
	}

	async function handleCreate() {
		if (!newTableName) return;
		await nexus.createDataTable(newTableName, newTableDesc, { columns: [] });
		isCreateModalOpen = false;
		newTableName = '';
		newTableDesc = '';
	}

	function selectDataTable(table: any) {
		nexus.selectedDataTable = table;
		nexus.fetchDataTableRows(table.id);
	}
</script>

{#if nexus.selectedDataTable}
	<DataTableDetailsView table={nexus.selectedDataTable} />
{:else}
	<div class="flex-1 flex flex-col overflow-hidden bg-muted/5">
		<div class="flex h-12 items-center justify-between border-b px-6 shrink-0 bg-background">
			<div class="flex items-center gap-2">
				<Database class="h-4 w-4 text-primary" />
				<h2 class="text-sm font-bold tracking-tight">Data Tables</h2>
			</div>
			<Button size="sm" class="gap-1.5 h-8" onclick={() => isCreateModalOpen = true}>
				<Plus class="h-3.5 w-3.5" />
				New Data Table
			</Button>
		</div>

		<div class="flex-1 overflow-auto p-6">
			<div class="max-w-6xl mx-auto space-y-6">
				<div class="flex items-center justify-between gap-4">
					<div class="relative w-72">
						<Search class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
						<Input placeholder="Search tables..." bind:value={searchQuery} class="pl-8 h-9" />
					</div>
					{#if nexus.isFetchingDataTables}
						<Loader2 class="h-4 w-4 animate-spin text-muted-foreground" />
					{/if}
				</div>

				<div class="rounded-md border bg-background shadow-sm">
					<Table.Root>
						<Table.Header>
							<Table.Row class="hover:bg-transparent">
								<Table.Head>Name</Table.Head>
								<Table.Head>Description</Table.Head>
								<Table.Head>Updated</Table.Head>
								<Table.Head class="w-12"></Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each filteredTables as table}
								<Table.Row class="cursor-pointer group" onclick={() => selectDataTable(table)}>
									<Table.Cell class="font-medium group-hover:text-primary transition-colors">{table.name}</Table.Cell>
									<Table.Cell class="text-muted-foreground">{table.description || '-'}</Table.Cell>
									<Table.Cell class="text-muted-foreground">{formatDate(table.updated_at)}</Table.Cell>
									<Table.Cell>
										<DropdownMenu.Root>
											<DropdownMenu.Trigger>
												{#snippet child({ props })}
													<Button {...props} variant="ghost" size="icon" class="h-8 w-8" onclick={(e) => e.stopPropagation()}>
														<MoreHorizontal class="h-4 w-4" />
													</Button>
												{/snippet}
											</DropdownMenu.Trigger>
											<DropdownMenu.Content align="end">
												<DropdownMenu.Item class="gap-2"><Settings2 class="h-3.5 w-3.5" /> Edit Schema</DropdownMenu.Item>
												<DropdownMenu.Separator />
												<DropdownMenu.Item class="text-destructive gap-2" onclick={(e) => { e.stopPropagation(); nexus.deleteDataTable(table.id); }}>
													<Trash2 class="h-3.5 w-3.5" /> Delete Table
												</DropdownMenu.Item>
											</DropdownMenu.Content>
										</DropdownMenu.Root>
									</Table.Cell>
								</Table.Row>
							{:else}
								<Table.Row>
									<Table.Cell colspan={4} class="h-32 text-center text-muted-foreground italic">
										{searchQuery ? 'No tables match your search.' : 'No data tables created yet.'}
									</Table.Cell>
								</Table.Row>
							{/each}
						</Table.Body>
					</Table.Root>
				</div>
			</div>
		</div>
	</div>
{/if}

<Dialog.Root bind:open={isCreateModalOpen}>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Create Data Table</Dialog.Title>
			<Dialog.Description>
				Add a new internal table to store structured data.
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="grid gap-2">
				<Label for="name">Name</Label>
				<Input id="name" bind:value={newTableName} placeholder="e.g. leads_data" />
			</div>
			<div class="grid gap-2">
				<Label for="description">Description</Label>
				<Textarea id="description" bind:value={newTableDesc} placeholder="What is this table for?" />
			</div>
		</div>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => isCreateModalOpen = false}>Cancel</Button>
			<Button onclick={handleCreate} disabled={!newTableName}>Create Table</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>