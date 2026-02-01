<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import {
		SvelteFlow,
		Background,
		BackgroundVariant,
		MiniMap,
		addEdge,
		useSvelteFlow,
		type Connection,
		type Edge,
		type Node
	} from '@xyflow/svelte';
	import WorkflowNode from '$lib/flow/nodes/WorkflowNode.svelte';
	import NoteNode from '$lib/flow/nodes/NoteNode.svelte';
	import WorkflowControl from '$lib/flow/WorkflowControl.svelte';
	import NodeContextMenu from './NodeContextMenu.svelte';
	import { nodeLibrary, type NodeKind } from '$lib/flow/nodes/definitions';
	import { onMount } from 'svelte';

	const nodeTypes = {
		workflowNode: WorkflowNode,
		note: NoteNode
	};

	const { screenToFlowPosition, getViewport } = useSvelteFlow();

	function onConnect(connection: Connection) {
		const exists = nexus.edges.some(
			(e) => e.source === connection.source && e.target === connection.target &&
				e.sourceHandle === connection.sourceHandle && e.targetHandle === connection.targetHandle
		);
		if (!exists) {
			const newEdge = { ...connection, id: crypto.randomUUID() };
			nexus.edges = addEdge(newEdge, nexus.edges as any);
		}
	}

	function handleAddNode(kind: NodeKind, clientPos?: { x: number; y: number }) {
		let position;
		if (clientPos) {
			position = screenToFlowPosition(clientPos);
		} else {
			const viewport = getViewport();
			position = {
				x: (window.innerWidth / 2 - viewport.x) / viewport.zoom,
				y: (window.innerHeight / 2 - viewport.y) / viewport.zoom
			};
		}

		const newNode: Node = {
			id: crypto.randomUUID(),
			type: kind === 'note' ? 'note' : 'workflowNode',
			position,
			data: { kind, label: getNodeLabel(kind) }
		};
		nexus.nodes = [...nexus.nodes, newNode];
		nexus.contextMenu = null;
	}

	function getNodeLabel(kind: NodeKind): string {
		return nodeLibrary.find((n) => n.kind === kind)?.label || kind;
	}

	function onPaneContextMenu({ event }: { event: MouseEvent }) {
		event.preventDefault();
		nexus.contextMenu = {
			x: event.clientX,
			y: event.clientY,
			clientX: event.clientX,
			clientY: event.clientY
		};
	}

	function onDelete({ nodes, edges }: { nodes: Node[]; edges: Edge[] }) {
		const nodeIds = new Set(nodes.map(n => n.id));
		const edgeIds = new Set(edges.map(e => e.id));
		
		nexus.nodes = nexus.nodes.filter(n => !nodeIds.has(n.id));
		nexus.edges = nexus.edges.filter(e => !edgeIds.has(e.id) && !nodeIds.has(e.source) && !nodeIds.has(e.target));
		
		if (nexus.selectedNode && nodeIds.has(nexus.selectedNode.id)) {
			nexus.selectedNode = null;
		}
	}

	onMount(() => {
		// Sidebar 등 외부에서 호출하는 노드 추가 이벤트 핸들러
		const handler = (e: any) => handleAddNode(e.detail.kind);
		window.addEventListener('addNode', handler);
		return () => window.removeEventListener('addNode', handler);
	});
</script>

<div class="canvas-container" role="presentation" oncontextmenu={(e) => e.preventDefault()}>
	<SvelteFlow
		bind:nodes={nexus.nodes}
		bind:edges={nexus.edges}
		{nodeTypes}
		panOnDrag={nexus.panOnDrag}
		selectionOnDrag={!nexus.panOnDrag}
		onconnect={onConnect}
		onnodeclick={({ node }) => { nexus.selectedNode = node; nexus.selectedExecution = null; }}
		onpaneclick={() => { nexus.selectedNode = null; nexus.selectedExecution = null; nexus.contextMenu = null; }}
		onpanecontextmenu={onPaneContextMenu}
		ondelete={onDelete}
		fitView
	>
		<Background variant={BackgroundVariant.Dots} />
		{#if nexus.showMinimap}
			<MiniMap 
				width={160} 
				height={100} 
				position="bottom-right"
				style="background-color: white; border: 1px solid var(--border-color); border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.1);"
			/>
		{/if}
	</SvelteFlow>

	<WorkflowControl
		onAddNode={(event) => onPaneContextMenu({ event })}
		onAddMemo={() => handleAddNode('note')}
		onToggleMaximize={() => nexus.isMaximized = !nexus.isMaximized}
		isMaximized={nexus.isMaximized}
		panOnDrag={nexus.panOnDrag}
		onTogglePanMode={(mode) => (nexus.panOnDrag = mode)}
		onAutoLayout={() => console.log('Auto layout')}
	/>

	{#if nexus.contextMenu}
		<NodeContextMenu 
			x={nexus.contextMenu.clientX} 
			y={nexus.contextMenu.clientY} 
			onAddNode={(kind) => handleAddNode(kind, { x: nexus.contextMenu!.clientX, y: nexus.contextMenu!.clientY })} 
		/>
	{/if}
</div>

<style>
	.canvas-container {
		flex: 1;
		position: relative;
		overflow: hidden;
		background: var(--bg-app);
	}
	:global(.flow-canvas) { background: var(--bg-app) !important; }
</style>
