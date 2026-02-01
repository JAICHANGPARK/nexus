<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Info, Plus, Library } from 'lucide-svelte';
</script>

<div class="flex-1 flex flex-col overflow-hidden bg-muted/5">
	<div class="flex h-12 items-center justify-between border-b px-6 shrink-0 bg-background">
		<div class="flex items-center gap-2">
			<Library class="h-4 w-4 text-primary" />
			<h2 class="text-sm font-bold tracking-tight">Knowledge Base</h2>
		</div>
		<Button size="sm" class="gap-1.5 h-8">
			<Plus class="h-3.5 w-3.5" />
			Create Knowledge Base
		</Button>
	</div>

	<div class="flex-1 overflow-auto p-8">
		<div class="max-w-4xl mx-auto flex flex-col items-center justify-center py-20 text-center space-y-4">
			<div class="h-20 w-20 rounded-full bg-muted flex items-center justify-center mb-2">
				<Library class="h-10 w-10 text-muted-foreground/40" />
			</div>
			<h3 class="text-lg font-bold">No Knowledge Bases yet</h3>
			<p class="text-sm text-muted-foreground max-w-sm">
				Create a knowledge base to index your documents and provide context to your AI agents using RAG (Retrieval-Augmented Generation).
			</p>
			<Button variant="outline">
				Learn more about Knowledge Bases
			</Button>
		</div>
	</div>
</div>