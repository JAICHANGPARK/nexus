<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import { SvelteFlowProvider } from '@xyflow/svelte';
	import Header from '$lib/components/layout/Header.svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import FlowCanvas from '$lib/components/flow/FlowCanvas.svelte';
	import RightPanel from '$lib/components/layout/RightPanel.svelte';
	import CredentialsView from '$lib/components/credentials/CredentialsView.svelte';
	import ToolsView from '$lib/components/tools/ToolsView.svelte';
	import DataTableView from '$lib/components/knowledge/DataTableView.svelte';
	import KnowledgeBaseView from '$lib/components/knowledge/KnowledgeBaseView.svelte';
	import '@xyflow/svelte/dist/style.css';

	function onKeyDown(event: KeyboardEvent) {
		// Don't delete if user is typing in an input or textarea
		const activeElement = document.activeElement;
		const isTyping = activeElement instanceof HTMLInputElement || 
		                 activeElement instanceof HTMLTextAreaElement ||
		                 activeElement?.hasAttribute('contenteditable');
		
		if (isTyping) return;

		if (event.key === 'Delete' || event.key === 'Backspace') {
			// Filter out selected nodes
			const deletedNodeIds = new Set(
				nexus.nodes.filter(n => n.selected).map(n => n.id)
			);

			if (deletedNodeIds.size > 0) {
				nexus.nodes = nexus.nodes.filter(n => !deletedNodeIds.has(n.id));
				// Also remove edges connected to deleted nodes
				nexus.edges = nexus.edges.filter(
					e => !deletedNodeIds.has(e.source) && !deletedNodeIds.has(e.target)
				);
			}

			// Filter out selected edges
			nexus.edges = nexus.edges.filter(e => !e.selected);
			
			// Clear selected node reference if it was deleted
			if (nexus.selectedNode && deletedNodeIds.has(nexus.selectedNode.id)) {
				nexus.selectedNode = null;
			}
		}
	}
</script>

<svelte:window onkeydown={onKeyDown} />

<div class="layout">
	<Header />

	{#if nexus.activeView === 'canvas'}
		<div class="main">
			<aside class="sidebar-wrapper" class:collapsed={!nexus.leftSidebarOpen}>
				<Sidebar />
			</aside>
			
			<main class="content">
				<SvelteFlowProvider>
					<FlowCanvas />
				</SvelteFlowProvider>
			</main>

			<aside class="panel-wrapper" class:collapsed={!nexus.rightSidebarOpen || (!nexus.selectedNode && !nexus.selectedExecution)}>
				<RightPanel />
			</aside>
		</div>
	{:else if nexus.activeView === 'credentials'}
		<CredentialsView />
	{:else if nexus.activeView === 'tools'}
		<ToolsView />
	{:else if nexus.activeView === 'data-table'}
		<DataTableView />
	{:else if nexus.activeView === 'knowledge-base'}
		<KnowledgeBaseView />
	{/if}
</div>

<style>
	.layout {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: var(--bg-app);
		color: var(--text-primary);
		font-family: Inter, system-ui, sans-serif;
	}

	.main {
		display: flex;
		flex: 1;
		overflow: hidden;
	}

	.sidebar-wrapper {
		width: 256px; /* w-64 */
		transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
		overflow: hidden;
		border-right: 1px solid var(--border-color);
		flex-shrink: 0;
	}

	.sidebar-wrapper.collapsed {
		width: 0;
		border-right-width: 0;
	}

	.panel-wrapper {
		width: 320px; /* w-80 */
		transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
		overflow: hidden;
		border-left: 1px solid var(--border-color);
		flex-shrink: 0;
	}

	.panel-wrapper.collapsed {
		width: 0;
		border-left-width: 0;
	}

	.content {
		flex: 1;
		display: flex;
		flex-direction: column;
		position: relative;
	}
</style>