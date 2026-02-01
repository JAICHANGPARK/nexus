<script lang="ts">
	import { useSvelteFlow } from '@xyflow/svelte';

	type NoteNodeData = {
		text?: string;
		color?: string;
	};

	const { data, id, selected }: { data: NoteNodeData; id: string; selected: boolean } = $props();
	const { updateNodeData } = useSvelteFlow();

	function handleInput(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		updateNodeData(id, { text: target.value });
	}
</script>

<div class="note-container" class:selected style="--note-bg: {data.color || '#fff9c4'}">
	<textarea
		class="note-textarea"
		value={data.text || ''}
		oninput={handleInput}
		placeholder="메모를 입력하세요..."
	></textarea>
</div>

<style>
	.note-container {
		width: 200px;
		height: 150px;
		background: var(--note-bg);
		border-radius: 4px;
		padding: 12px;
		box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
		border: 1px solid rgba(0, 0, 0, 0.05);
		display: flex;
		flex-direction: column;
	}

	.note-textarea {
		flex: 1;
		width: 100%;
		height: 100%;
		background: transparent;
		border: none;
		resize: none;
		font-family: inherit;
		font-size: 14px;
		color: #333;
		outline: none;
		line-height: 1.5;
	}

	.note-textarea::placeholder {
		color: rgba(0, 0, 0, 0.2);
	}

	/* Selection state */
	.note-container.selected {
		border-color: #fbc02d !important;
		box-shadow: 0 0 0 2px rgba(255, 213, 79, 0.5), 0 8px 16px rgba(0, 0, 0, 0.1) !important;
		border-width: 2px !important;
	}

	:global(.svelte-flow__node.selected .note-container) {
		border-color: #fbc02d !important;
		box-shadow: 0 0 0 2px rgba(255, 213, 79, 0.5), 0 8px 16px rgba(0, 0, 0, 0.1) !important;
		border-width: 2px !important;
	}
</style>
