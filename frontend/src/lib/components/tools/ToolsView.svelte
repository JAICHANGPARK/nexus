<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Badge } from '$lib/components/ui/badge';
	import * as Card from '$lib/components/ui/card';
	import { Separator } from '$lib/components/ui/separator';
	import { Plus, Trash2, Wrench, X, Globe, Lock } from 'lucide-svelte';

	let newMcpName = $state('');
	let newMcpUrl = $state('');
	let newMcpHeaders = $state<Array<{ key: string, value: string }>>([{ key: '', value: '' }]);

	function addHeaderField() {
		newMcpHeaders = [...newMcpHeaders, { key: '', value: '' }];
	}

	function removeHeaderField(index: number) {
		newMcpHeaders = newMcpHeaders.filter((_, i) => i !== index);
		if (newMcpHeaders.length === 0) newMcpHeaders = [{ key: '', value: '' }];
	}

	function handleRegisterServer() {
		if (!newMcpName || !newMcpUrl) return;
		const headers: Record<string, string> = {};
		newMcpHeaders.forEach(h => {
			if (h.key.trim()) headers[h.key.trim()] = h.value;
		});
		
		nexus.addMcpServer(newMcpName, newMcpUrl, headers);
		newMcpName = '';
		newMcpUrl = '';
		newMcpHeaders = [{ key: '', value: '' }];
	}

	let expandedServerId = $state<string | null>(null);

	function toggleServerTools(id: string) {
		if (expandedServerId === id) {
			expandedServerId = null;
		} else {
			expandedServerId = id;
			if (!nexus.mcpTools[id]) {
				nexus.fetchMcpTools(id);
			}
		}
	}
</script>

<div class="flex-1 p-8 overflow-y-auto bg-background">
	<div class="max-w-5xl mx-auto space-y-10">
		<div class="flex items-center justify-between">
			<div>
				<h2 class="text-3xl font-bold tracking-tight">Tools & MCP Servers</h2>
				<p class="text-muted-foreground">Extend your agent capabilities with Model Context Protocol servers.</p>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-3 gap-10">
			<!-- Registration Form -->
			<div class="lg:col-span-1">
				<Card.Root>
					<Card.Header>
						<Card.Title class="text-lg">Register New Server</Card.Title>
						<Card.Description>Connect a new HTTP-based MCP server.</Card.Description>
					</Card.Header>
					<Card.Content class="space-y-4">
						<div class="space-y-2">
							<Label for="name">Server Name</Label>
							<Input id="name" placeholder="e.g. Google Search" bind:value={newMcpName} />
						</div>
						<div class="space-y-2">
							<Label for="url">Endpoint URL</Label>
							<Input id="url" placeholder="https://..." bind:value={newMcpUrl} />
						</div>
						
						<div class="space-y-3 pt-2">
							<div class="flex items-center justify-between">
								<Label class="text-xs font-bold uppercase text-muted-foreground">Custom Headers</Label>
								<Button variant="ghost" size="icon" class="h-6 w-6" onclick={addHeaderField}>
									<Plus class="h-4 w-4" />
								</Button>
							</div>
							<div class="space-y-2">
								{#each newMcpHeaders as header, i}
									<div class="flex items-center gap-2">
										<Input placeholder="Key" bind:value={header.key} class="h-8 text-xs px-2 flex-1" />
										<Input placeholder="Value" bind:value={header.value} class="h-8 text-xs px-2 flex-1" />
										<Button variant="ghost" size="icon" class="h-8 w-8 text-muted-foreground" onclick={() => removeHeaderField(i)}>
											<X class="h-4 w-4" />
										</Button>
									</div>
								{/each}
							</div>
						</div>
					</Card.Content>
					<Card.Footer>
						<Button class="w-full" onclick={handleRegisterServer} disabled={!newMcpName || !newMcpUrl}>
							<Plus class="mr-2 h-4 w-4" /> Register Server
						</Button>
					</Card.Footer>
				</Card.Root>
			</div>

			<!-- Server List -->
			<div class="lg:col-span-2 space-y-4">
				<h3 class="text-sm font-bold uppercase tracking-widest text-muted-foreground px-1">Registered Servers</h3>
				
				{#if nexus.isFetchingMcpServers}
					<div class="flex justify-center py-12">
						<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
					</div>
				{:else if nexus.mcpServers.length === 0}
					<div class="rounded-xl border border-dashed p-12 text-center">
						<div class="h-12 w-12 rounded-full bg-muted flex items-center justify-center mx-auto mb-4">
							<Wrench class="h-6 w-6 text-muted-foreground opacity-50" />
						</div>
						<h4 class="text-lg font-medium">No servers registered</h4>
						<p class="text-sm text-muted-foreground">Add an MCP server to enable tool-calling for your agents.</p>
					</div>
				{:else}
					<div class="space-y-4">
						{#each nexus.mcpServers as server}
							<div class="space-y-2">
								<Card.Root class="group transition-all hover:shadow-sm border-border/50">
									<Card.Header class="pb-2 flex flex-row items-center justify-between space-y-0">
										<div class="flex items-center gap-2">
											<div class="h-2.5 w-2.5 rounded-full {
												server.status === 'connected' ? 'bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.6)]' : 
												server.status === 'error' ? 'bg-destructive' : 'bg-slate-300'
											}"></div>
											<span class="text-[10px] font-medium uppercase tracking-tight text-muted-foreground">
												{server.status || 'disconnected'}
											</span>
											<Badge variant="outline" class="text-[9px] uppercase tracking-tighter">
												{server.transport}
											</Badge>
										</div>
										<div class="flex items-center gap-1">
											<Button 
												variant="ghost" 
												size="sm" 
												class="h-8 text-xs gap-1.5"
												onclick={() => nexus.checkMcpServerStatus(server.id)}
												disabled={nexus.isCheckingStatus[server.id]}
											>
												{#if nexus.isCheckingStatus[server.id]}
													<div class="h-3 w-3 border-2 border-primary border-t-transparent animate-spin rounded-full"></div>
												{:else}
													<Wrench class="h-3.5 w-3.5" />
												{/if}
												Verify
											</Button>
											<Button variant="ghost" size="icon" class="h-8 w-8 text-destructive opacity-0 group-hover:opacity-100 transition-opacity" 
												onclick={() => nexus.removeMcpServer(server.id)}>
												<Trash2 class="h-4 w-4" />
											</Button>
										</div>
									</Card.Header>
									<Card.Content>
										<div class="flex items-start justify-between">
											<div class="flex-1 min-w-0">
												<h4 class="text-md font-bold truncate mb-1">{server.name}</h4>
												<div class="flex items-center gap-1.5 text-xs text-muted-foreground truncate">
													<Globe class="h-3 w-3 shrink-0" />
													{server.url}
												</div>
											</div>
											<Button 
												variant="secondary" 
												size="sm" 
												class="ml-4 h-8 text-xs"
												onclick={() => toggleServerTools(server.id)}
											>
												{expandedServerId === server.id ? 'Hide Tools' : 'Show Tools'}
											</Button>
										</div>
										
										{#if server.headers && Object.keys(server.headers).length > 0}
											<div class="flex items-center gap-1.5 pt-3 border-t mt-4">
												<Lock class="h-3 w-3 text-muted-foreground opacity-50" />
												<div class="flex flex-wrap gap-1">
													{#each Object.keys(server.headers) as key}
														<Badge variant="secondary" class="text-[8px] h-4 py-0 px-1 font-mono">{key}</Badge>
													{/each}
												</div>
											</div>
										{/if}
									</Card.Content>
								</Card.Root>

								{#if expandedServerId === server.id}
									<div class="ml-4 pl-4 border-l-2 border-muted py-2 space-y-3">
										<h5 class="text-[10px] font-bold uppercase tracking-widest text-muted-foreground">Available Tools</h5>
										
										{#if !nexus.mcpTools[server.id]}
											<div class="flex items-center gap-2 text-xs text-muted-foreground py-2">
												<div class="h-3 w-3 border-2 border-muted-foreground border-t-transparent animate-spin rounded-full"></div>
												Loading tools...
											</div>
										{:else if nexus.mcpTools[server.id].length === 0}
											<div class="text-xs text-muted-foreground py-2 italic">
												No tools discovered on this server.
											</div>
										{:else}
											<div class="grid grid-cols-1 gap-2">
												{#each nexus.mcpTools[server.id] as tool}
													<div class="p-3 rounded-lg bg-muted/30 border border-border/50 flex flex-col gap-1 hover:bg-muted/50 transition-colors">
														<div class="flex items-center gap-2">
															<span class="font-mono text-xs font-bold text-primary">{tool.name}</span>
														</div>
														{#if tool.description}
															<p class="text-[11px] text-muted-foreground line-clamp-2">
																{tool.description}
															</p>
														{/if}
													</div>
												{/each}
											</div>
										{/if}
									</div>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>
