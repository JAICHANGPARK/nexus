<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import {
		triggerNodes,
		logicNodes,
		transformNodes,
		aiNodes,
		endNode,
		type NodeKind
	} from '$lib/flow/nodes/definitions';

	let { x, y, onAddNode } = $props<{ x: number, y: number, onAddNode: (kind: NodeKind) => void }>();

	const nodeCategories = [
		{ name: 'Triggers', nodes: triggerNodes, color: '#ff6d5a' },
		{ name: 'Logic', nodes: logicNodes, color: '#ffd166' },
		{ name: 'Transform', nodes: transformNodes, color: '#7b68ee' },
		{ name: 'AI', nodes: aiNodes, color: '#3b82f6' },
		{ name: 'End', nodes: endNode, color: '#6d7882' }
	];
</script>

<div class="context-menu" style="left: {x}px; top: {y}px;" role="menu" tabindex="-1" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.key === 'Escape' && (nexus.contextMenu = null)}>
	<div class="context-search">
		<input type="text" placeholder="Add node..." bind:value={nexus.nodeSearch} class="search-input" />
	</div>
	<div class="context-nodes">
		{#each nodeCategories as category}
			<div class="context-category">
				<div class="context-category-name">{category.name}</div>
				{#each category.nodes as node}
					<button class="context-node-item" role="menuitem" onclick={() => onAddNode(node.kind)}>
						<div class="node-icon-small" style="background: {category.color}">{node.icon}</div>
						<span class="node-name">{node.label}</span>
					</button>
				{/each}
			</div>
		{/each}
	</div>
</div>

<style>
	.context-menu {
		position: fixed;
		z-index: 1000;
		background: white;
		border-radius: 8px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
		border: 1px solid var(--border-color);
		width: 220px;
		max-height: 360px;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		outline: none;
	}

	.context-search { padding: 8px; border-bottom: 1px solid var(--border-color); }
	.search-input { width: 100%; padding: 6px 10px; border-radius: 4px; border: 1px solid var(--border-color); font-size: 12px; }

	.context-nodes { flex: 1; overflow-y: auto; padding: 4px 0; }
	.context-category-name { padding: 6px 12px; font-size: 10px; font-weight: 700; color: var(--text-secondary); text-transform: uppercase; }

	.context-node-item {
		width: 100%; display: flex; align-items: center; gap: 10px; padding: 6px 12px; border: none; background: transparent; cursor: pointer;
	}
	.context-node-item:hover { background: var(--bg-app); }
	.node-icon-small { width: 20px; height: 20px; border-radius: 4px; color: white; font-size: 10px; display: flex; align-items: center; justify-content: center; }
	.node-name { font-size: 12px; color: var(--text-primary); }
</style>