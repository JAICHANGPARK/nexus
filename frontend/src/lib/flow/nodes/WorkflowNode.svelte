<script lang="ts">
	import { Handle, Position, useSvelteFlow } from '@xyflow/svelte';
	import type { NodeKind } from './definitions';
	import { getNodeDefinition, getNodePorts } from './definitions';
	import { Loader2 } from 'lucide-svelte';

	type WorkflowNodeData = {
		kind: NodeKind;
		label: string;
		config?: Record<string, unknown>;
		executionStatus?: 'success' | 'error' | 'running';
		itemCount?: number;
	};

	const { data, id, selected }: { data: WorkflowNodeData; id: string; selected: boolean } = $props();
	const { getNode } = useSvelteFlow();
	const definition = $derived(getNodeDefinition(data.kind));
	const inputPorts = $derived(getNodePorts(data.kind, 'input'));
	const outputPorts = $derived(getNodePorts(data.kind, 'output'));
	const executionStatus = $derived(data.executionStatus);
	const itemCount = $derived(data.itemCount);

	function getNodeColor(kind: NodeKind): string {
		// n8n-style colors
		if (kind.startsWith('trigger-') && kind !== 'trigger-end') {
			return '#ff6d5a'; // Orange for triggers
		}
		if (kind === 'if' || kind === 'switch' || kind === 'merge') {
			return '#ffd166'; // Yellow for logic
		}
		if (kind === 'code' || kind === 'http-request' || kind === 'set' || kind === 'filter') {
			return '#7b68ee'; // Purple for transform
		}
		if (kind === 'ai-agent' || kind === 'llm' || kind === 'openai' || kind === 'openrouter') {
			return '#3b82f6'; // Blue for AI
		}
		return '#6d7882'; // Gray default
	}

	function getExecutionColor(): string | null {
		if (executionStatus === 'success') return '#22c55e';
		if (executionStatus === 'error') return '#ef4444';
		if (executionStatus === 'running') return '#3b82f6';
		return null;
	}

	function isTriggerNode(kind: NodeKind): boolean {
		return kind.startsWith('trigger-') && kind !== 'trigger-end';
	}
</script>

<div
	class="node-container"
	class:selected
	class:execution-success={executionStatus === 'success'}
	class:execution-error={executionStatus === 'error'}
	class:execution-running={executionStatus === 'running'}
	style="--node-color: {getNodeColor(data.kind)}; --execution-color: {getExecutionColor() ||
		getNodeColor(data.kind)}"
>
	<!-- Input Handles -->
	{#if !isTriggerNode(data.kind)}
		{#each inputPorts as port, i}
			{#if port.type === 'tool'}
				<div class="tool-handle-wrapper bottom">
					<Handle type="target" position={Position.Bottom} id={port.id} class="node-handle tool-triangle" />
					<span class="handle-label bottom">{port.label}</span>
				</div>
			{:else}
				<div class="input-handle-wrapper" style="top: {16 + i * 24}px;">
					<Handle type="target" position={Position.Left} id={port.id} class="node-handle input" />
					{#if inputPorts.length > 1}
						<span class="handle-label left">{port.label}</span>
					{/if}
				</div>
			{/if}
		{/each}
	{/if}

	<!-- Node Content -->
	<div class="node-box">
		<div class="node-header">
			<div class="node-icon">
				{#if executionStatus === 'running'}
					<Loader2 class="h-4 w-4 animate-spin text-white" />
				{:else}
					{definition?.icon?.slice(0, 2) || data.label.slice(0, 2)}
				{/if}
			</div>
			<div class="node-title">{data.label}</div>
		</div>
		{#if definition?.detail}
			<div class="node-subtitle">{definition.detail}</div>
		{/if}
		{#if executionStatus && executionStatus !== 'running'}
			<div
				class="execution-indicator"
				class:success={executionStatus === 'success'}
				class:error={executionStatus === 'error'}
			>
				{executionStatus === 'success' ? '✓' : '✗'}
			</div>
		{/if}
	</div>

	<!-- Output Handles -->
	{#each outputPorts as port, i}
		{#if port.type === 'tool'}
			<div class="tool-handle-wrapper top">
				<Handle type="source" position={Position.Top} id={port.id} class="node-handle tool-triangle" />
				<span class="handle-label top">{port.label}</span>
			</div>
		{:else}
			<div class="output-handle-wrapper" style="top: {16 + i * 24}px;">
				<Handle type="source" position={Position.Right} id={port.id} class="node-handle output" />
				{#if itemCount !== undefined && i === 0}
					<div class="item-count">{itemCount} {itemCount === 1 ? 'item' : 'items'}</div>
				{/if}
				{#if outputPorts.length > 1}
					<span class="handle-label right">{port.label}</span>
				{/if}
			</div>
		{/if}
	{/each}
</div>

<style>
	.node-container {
		position: relative;
		display: flex;
		align-items: center;
		--node-color: #6d7882;
	}

	.node-box {
		width: 220px;
		background: white;
		border: 2px solid var(--node-color);
		border-radius: 8px;
		padding: 12px 16px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		position: relative;
		z-index: 1;
		transition: all 0.3s ease;
	}

	.node-header {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.node-icon {
		width: 28px;
		height: 28px;
		border-radius: 6px;
		background: var(--node-color);
		color: white;
		font-size: 10px;
		font-weight: 700;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		transition: background 0.3s ease;
	}

	.node-title {
		font-size: 14px;
		font-weight: 600;
		color: #1d1d1f;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.node-subtitle {
		font-size: 11px;
		color: #86868b;
		margin-top: 4px;
		margin-left: 38px;
	}

	.execution-indicator {
		position: absolute;
		top: -8px;
		right: -8px;
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background: white;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 12px;
		font-weight: bold;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
		border: 2px solid currentColor;
	}

	.execution-indicator.success {
		color: #22c55e;
		border-color: #22c55e;
	}

	.execution-indicator.error {
		color: #ef4444;
		border-color: #ef4444;
	}

	/* Handle Wrappers */
	.input-handle-wrapper,
	.output-handle-wrapper {
		position: absolute;
		display: flex;
		align-items: center;
		z-index: 2;
		height: 0;
	}

	.input-handle-wrapper {
		left: 0;
		flex-direction: row;
	}

	.output-handle-wrapper {
		right: 0;
		flex-direction: row-reverse;
	}

	.tool-handle-wrapper {
		position: absolute;
		display: flex;
		flex-direction: column;
		align-items: center;
		z-index: 2;
		width: 100%;
		left: 0;
	}

	.tool-handle-wrapper.bottom {
		bottom: 0;
	}

	.tool-handle-wrapper.top {
		top: 0;
	}

	.item-count {
		margin-right: 12px;
		background: #f0f0f0;
		color: #555;
		padding: 2px 6px;
		border-radius: 10px;
		font-size: 9px;
		font-weight: 600;
		white-space: nowrap;
		pointer-events: none;
		border: 1px solid #ddd;
	}

	/* Handle Labels */
	.handle-label {
		font-size: 10px;
		color: #86868b;
		white-space: nowrap;
		background: white;
		padding: 2px 4px;
		border-radius: 4px;
		border: 1px solid #e5e5e7;
	}

	.handle-label.left {
		margin-left: 8px;
	}

	.handle-label.right {
		margin-right: 8px;
	}

	.handle-label.bottom {
		margin-top: 14px;
	}

	.handle-label.top {
		margin-top: -24px;
	}

	/* Svelte Flow Handle Styling - Scoped */
	:global(.node-handle) {
		width: 12px !important;
		height: 12px !important;
		background: white !important;
		border: 2px solid var(--node-color) !important;
		border-radius: 50% !important;
		position: absolute !important;
		top: 50% !important;
		transform: translateY(-50%) !important;
		transition: all 0.2s ease !important;
		z-index: 5;
	}

	:global(.node-handle.tool-triangle) {
		width: 14px !important;
		height: 12px !important;
		border-radius: 0 !important;
		background: #ffd166 !important;
		border: none !important;
		clip-path: polygon(50% 0%, 0% 100%, 100% 100%);
		left: 50% !important;
		transform: translateX(-50%) !important;
	}

	:global(.tool-handle-wrapper.bottom .node-handle.tool-triangle) {
		bottom: -6px !important;
		top: auto !important;
	}

	:global(.tool-handle-wrapper.top .node-handle.tool-triangle) {
		top: -6px !important;
		clip-path: polygon(50% 100%, 0% 0%, 100% 0%);
	}

	:global(.node-handle.input) {
		left: -6px !important;
	}

	:global(.node-handle.output) {
		right: -6px !important;
	}

	:global(.node-handle:hover) {
		background: var(--node-color) !important;
		transform: translateY(-50%) scale(1.3) !important;
		cursor: crosshair;
	}

	:global(.node-handle.connecting) {
		background: var(--node-color) !important;
		transform: scale(1.2) !important;
	}

	/* Selected state */
	.node-container.selected .node-box {
		border-color: #0071e3 !important;
		box-shadow: 0 0 0 1px #0071e3, 0 8px 24px rgba(0, 0, 0, 0.15) !important;
		border-width: 2px !important;
	}

	:global(.svelte-flow__node.selected .node-box) {
		border-color: #0071e3 !important;
		box-shadow: 0 0 0 1px #0071e3, 0 8px 24px rgba(0, 0, 0, 0.15) !important;
		border-width: 2px !important;
	}

	/* Execution status styles */
	.node-container.execution-success .node-box {
		border-color: #22c55e;
		box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.2);
	}

	.node-container.execution-error .node-box {
		border-color: #ef4444;
		box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.2);
	}

	.node-container.execution-running .node-box {
		border-color: #3b82f6;
		box-shadow: 0 0 0 4px rgba(59, 130, 246, 0.3);
		animation: pulse 1.5s infinite;
	}

	@keyframes pulse {
		0% { transform: scale(1); }
		50% { transform: scale(1.02); }
		100% { transform: scale(1); }
	}

	.node-container.execution-success .node-icon {
		background: #22c55e;
	}

	.node-container.execution-error .node-icon {
		background: #ef4444;
	}

	.node-container.execution-running .node-icon {
		background: #3b82f6;
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.node-box {
			background: #1c1c1e;
			border-color: var(--node-color);
		}

		.node-title {
			color: #ffffff;
		}

		.node-subtitle {
			color: #8e8e93;
		}

		.handle-label {
			background: #1c1c1e;
			color: #8e8e93;
			border-color: #3a3a3c;
		}

		:global(.node-handle) {
			background: #1c1c1e !important;
		}
	}
</style>