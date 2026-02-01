<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import { Button } from '$lib/components/ui/button';
	import { ArrowLeft, Plus, Search, MoreHorizontal, Settings2, Trash2, Loader2, Save, ChevronDown } from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';
	import * as Table from '$lib/components/ui/table';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Badge } from '$lib/components/ui/badge';

	let { table } = $props<{ table: any }>();
	let searchQuery = $state('');

	function backToList() {
		nexus.selectedDataTable = null;
		nexus.selectedDataTableRows = [];
	}

	async function addRow() {
		const emptyRow = {};
		// In a real grid, we'd open an editor or add a blank editable row
		// For now, let's just add an empty record to the backend
		await nexus.addDataTableRow(table.id, emptyRow);
	}

	async function addColumn() {
		// Mock schema update
		const newColumnName = prompt('Enter column name:');
		if (!newColumnName) return;
		
		const currentSchema = table.schema || { columns: [] };
		const newSchema = {
			...currentSchema,
			columns: [...(currentSchema.columns || []), { name: newColumnName, type: 'string' }]
		};
		await nexus.updateDataTableSchema(table.id, newSchema);
	}

	let columns = $derived(table.schema?.columns || []);
	let filteredRows = $derived(
		nexus.selectedDataTableRows.filter(row => 
			JSON.stringify(row.data).toLowerCase().includes(searchQuery.toLowerCase())
		)
	);
</script>

<div class="flex-1 flex flex-col overflow-hidden bg-background">
	<!-- Toolbar -->
	<div class="flex h-12 items-center justify-between border-b px-4 shrink-0">
		<div class="flex items-center gap-3">
			<Button variant="ghost" size="icon" class="h-8 w-8" onclick={backToList}>
				<ArrowLeft class="h-4 w-4" />
			</Button>
			<div class="flex items-center gap-2">
				<span class="text-xs text-muted-foreground">Data Tables</span>
				<span class="text-xs text-muted-foreground">/</span>
				<span class="text-sm font-bold">{table.name}</span>
			</div>
			{#if nexus.isFetchingDataTableRows}
				<Loader2 class="h-3 w-3 animate-spin text-muted-foreground" />
			{/if}
		</div>

		<div class="flex items-center gap-2">
			<div class="relative w-48">
				<Search class="absolute left-2 top-2 h-3.5 w-3.5 text-muted-foreground" />
				<Input placeholder="Search rows..." bind:value={searchQuery} class="pl-7 h-8 text-xs" />
			</div>
			<Button size="sm" variant="outline" class="h-8 text-xs gap-1.5" onclick={addColumn}>
				<Plus class="h-3 w-3" /> Add Column
			</Button>
			<Button size="sm" class="h-8 text-xs gap-1.5" onclick={addRow}>
				<Plus class="h-3 w-3" /> Add Row
			</Button>
		</div>
	</div>

	<!-- Grid -->
	<div class="flex-1 overflow-auto">
		<Table.Root>
			<Table.Header class="bg-muted/30 sticky top-0 z-10 shadow-sm">
				<Table.Row class="hover:bg-transparent">
					<Table.Head class="w-12 text-[10px] uppercase font-bold text-center">#</Table.Head>
					{#each columns as col}
						<Table.Head class="text-[10px] uppercase font-bold border-l min-w-[150px]">
							<div class="flex items-center justify-between group">
								{col.name}
								<Button variant="ghost" size="icon" class="h-4 w-4 opacity-0 group-hover:opacity-100 transition-opacity">
									<ChevronDown class="h-3 w-3" />
								</Button>
							</div>
						</Table.Head>
					{/each}
					<Table.Head class="w-12 border-l"></Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each filteredRows as row, i}
					<Table.Row class="group">
						<Table.Cell class="text-center text-[10px] text-muted-foreground font-mono">{i + 1}</Table.Cell>
						{#each columns as col}
							<Table.Cell class="border-l text-xs">
								<input 
									type="text" 
									class="w-full bg-transparent border-none focus:outline-none focus:ring-1 focus:ring-primary/30 rounded px-1"
									value={row.data[col.name] || ''}
									onchange={(e) => {
										const newData = { ...row.data, [col.name]: e.currentTarget.value };
										nexus.updateDataTableRow(table.id, row.id, newData);
									}}
								/>
							</Table.Cell>
						{/each}
						<Table.Cell class="border-l text-center">
							<Button variant="ghost" size="icon" class="h-6 w-6 opacity-0 group-hover:opacity-100 transition-opacity text-destructive" onclick={() => nexus.deleteDataTableRow(table.id, row.id)}>
								<Trash2 class="h-3.5 w-3.5" />
							</Button>
						</Table.Cell>
					</Table.Row>
				{:else}
					<Table.Row>
						<Table.Cell colspan={columns.length + 2} class="h-64 text-center">
							<div class="flex flex-col items-center justify-center space-y-2 opacity-40">
								<Plus class="h-8 w-8 mb-2" />
								<p class="text-sm font-medium">No rows found</p>
								<p class="text-xs">Click "Add Row" to start adding data to this table.</p>
							</div>
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
</div>

<style>
	:global(thead) {
		background-color: hsl(var(--muted) / 0.5);
	}
</style>