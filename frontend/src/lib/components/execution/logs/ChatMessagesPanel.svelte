<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { MessageSquare, Send, Bot, User, Trash2, Loader2 } from 'lucide-svelte';

	let message = $state('');
	let isWaiting = $state(false);
	let chatHistory = $state<Array<{ role: 'user' | 'assistant', content: string }>>([
		{ role: 'assistant', content: "Hello! I'm the Nexus AI assistant. How can I help you build your workflow today?" }
	]);

	async function sendMessage() {
		if (!message.trim() || isWaiting) return;
		const userMsg = message.trim();
		chatHistory = [...chatHistory, { role: 'user', content: userMsg }];
		message = '';
		isWaiting = true;

		// Trigger workflow if a Chat Trigger node exists
		const hasChatTrigger = nexus.nodes.some(n => n.data.kind === 'chat-trigger');
		
		if (hasChatTrigger) {
			try {
				const results = await nexus.triggerWorkflowWithInput({ message: userMsg });
				
				if (results && results.nodeResults) {
					// Get the last node's output as the response
					const nodeIds = Object.keys(results.nodeResults);
					const lastNodeId = nodeIds[nodeIds.length - 1];
					const lastResult = results.nodeResults[lastNodeId];
					
					let aiResponse = "";
					if (lastResult.status === 'success') {
						const output = lastResult.output;
						aiResponse = output?.text || output?.message || output?.body || JSON.stringify(output);
					} else {
						aiResponse = `Sorry, I encountered an error: ${lastResult.message}`;
					}

					chatHistory = [...chatHistory, { 
						role: 'assistant', 
						content: aiResponse 
					}];
				}
			} catch (e) {
				console.error('Failed to trigger workflow from chat:', e);
				chatHistory = [...chatHistory, { role: 'assistant', content: "Error: Could not connect to the execution engine." }];
			} finally {
				isWaiting = false;
			}
		} else {
			// Fallback: Normal AI assistant behavior
			setTimeout(() => {
				chatHistory = [...chatHistory, { 
					role: 'assistant', 
					content: `I've received your request about "${userMsg}". (Tip: Add a 'Chat Trigger' node to your canvas to connect this chat to your workflow!)` 
				}];
				isWaiting = false;
			}, 1000);
		}
	}
</script>

<div class="flex flex-col h-full bg-card shadow-xl border-l animate-in slide-in-from-right duration-300">
	<header class="flex h-12 items-center justify-between px-4 border-b shrink-0 bg-muted/10">
		<div class="flex items-center gap-2">
			<MessageSquare class="h-4 w-4 text-primary" />
			<h3 class="text-sm font-bold tracking-tight">AI Chat</h3>
		</div>
		<Button variant="ghost" size="icon" class="h-8 w-8 text-muted-foreground hover:text-destructive" onclick={() => chatHistory = []}>
			<Trash2 class="h-4 w-4" />
		</Button>
	</header>

	<div class="flex-1 overflow-hidden">
		<ScrollArea.Root class="h-full">
			<div class="p-4 space-y-4">
				{#each chatHistory as msg}
					<div class="flex flex-col gap-1.5 {msg.role === 'user' ? 'items-end' : 'items-start'}">
						<div class="flex items-center gap-2 mb-1">
							{#if msg.role === 'assistant'}
								<div class="h-5 w-5 rounded-full bg-primary/10 flex items-center justify-center border border-primary/20 text-primary">
									<Bot class="h-3 w-3" />
								</div>
								<span class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground">Assistant</span>
							{:else}
								<span class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground">You</span>
								<div class="h-5 w-5 rounded-full bg-muted flex items-center justify-center border border-border text-muted-foreground">
									<User class="h-3 w-3" />
								</div>
							{/if}
						</div>
						<div class="max-w-[85%] px-3 py-2 rounded-2xl text-xs shadow-sm {msg.role === 'user' ? 'bg-primary text-primary-foreground rounded-tr-none' : 'bg-muted rounded-tl-none'}">
							{msg.content}
						</div>
					</div>
				{/each}
				
				{#if isWaiting}
					<div class="flex flex-col gap-1.5 items-start">
						<div class="flex items-center gap-2 mb-1">
							<div class="h-5 w-5 rounded-full bg-primary/10 flex items-center justify-center border border-primary/20 text-primary">
								<Loader2 class="h-3 w-3 animate-spin" />
							</div>
							<span class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground">Thinking...</span>
						</div>
					</div>
				{/if}
			</div>
		</ScrollArea.Root>
	</div>

	<footer class="p-4 border-t bg-muted/10">
		<form class="relative flex items-center gap-2" onsubmit={(e) => { e.preventDefault(); sendMessage(); }}>
			<Input 
				bind:value={message} 
				placeholder="Ask AI for help..." 
				disabled={isWaiting}
				class="pr-10 h-10 rounded-full bg-background border-border/50 shadow-inner" 
			/>
			<Button type="submit" size="icon" class="absolute right-1 h-8 w-8 rounded-full shadow-md" disabled={!message.trim() || isWaiting}>
				<Send class="h-3.5 w-3.5" />
			</Button>
		</form>
		<p class="text-[9px] text-center text-muted-foreground mt-2 opacity-50 uppercase font-bold tracking-tighter">Powered by Nexus Core Engine</p>
	</footer>
</div>
