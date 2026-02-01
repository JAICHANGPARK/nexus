<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import { useSvelteFlow, MiniMap } from '@xyflow/svelte';
	import { browser } from '$app/environment';
	import { PanelLeftClose, PanelLeftOpen, PanelRightClose, PanelRightOpen } from 'lucide-svelte';

	let { 
		onAddNode, 
		onAddMemo,
		onToggleMaximize, 
		isMaximized,
		panOnDrag,
		onTogglePanMode,
		onAutoLayout
	} = $props<{
		onAddNode: (e: MouseEvent) => void;
		onAddMemo: () => void;
		onToggleMaximize: () => void;
		isMaximized: boolean;
		panOnDrag: boolean;
		onTogglePanMode: (mode: boolean) => void;
		onAutoLayout: () => void;
	}>();

	const flow = browser ? useSvelteFlow() : null;
	let zoomLevel = $state(100);

	// Update zoom level percentage
	$effect(() => {
		if (flow) {
			// Svelte Flow doesn't have a direct zoom listener in this version easily, 
			// but we can approximate or use their internal store if needed.
		}
	});

	function handleFitView() { if (flow) flow.fitView(); }
	function handleZoomIn() { if (flow) flow.zoomIn(); }
	function handleZoomOut() { if (flow) flow.zoomOut(); }

</script>

<div class="workflow-footer">
	<div class="footer-container">
		<!-- Left Group: Sidebar Toggle & Undo/Redo -->
		<div class="group left">
			<div class="action-bar">
				<button class="icon-btn" onclick={() => nexus.toggleLeftSidebar()} title="Toggle Sidebar">
					{#if nexus.leftSidebarOpen}
						<PanelLeftClose class="h-4 w-4" />
					{:else}
						<PanelLeftOpen class="h-4 w-4" />
					{/if}
				</button>
				<div class="v-divider"></div>
				<button class="icon-btn disabled" title="Undo (Ctrl+Z)" aria-label="Undo">
					<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M5.82843 6.99955L8.36396 9.53509L6.94975 10.9493L2 5.99955L6.94975 1.0498L8.36396 2.46402L5.82843 4.99955H13C17.4183 4.99955 21 8.58127 21 12.9996C21 17.4178 17.4183 20.9996 13 20.9996H4V18.9996H13C16.3137 18.9996 19 16.3133 19 12.9996C19 9.68584 16.3137 6.99955 13 6.99955H5.82843Z"></path></svg>
				</button>
				<button class="icon-btn disabled" title="Redo (Ctrl+Y)" aria-label="Redo">
					<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M16 7H11C7.68629 7 5 9.68629 5 13C5 16.3137 7.68629 19 11 19H20V21H11C6.58172 21 3 17.4183 3 13C3 8.58172 6.58172 5 11 5H16V1L22 6L16 11V7Z"></path></svg>
				</button>
				<div class="v-divider"></div>
				<button class="icon-btn" onclick={handleFitView} title="Reset View" aria-label="Fit View">
					<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M12 2C17.5228 2 22 6.47715 22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2 17.5228 2 12H4C4 16.4183 7.58172 20 12 20C16.4183 20 20 16.4183 20 12C20 7.58172 16.4183 4 12 4C9.25022 4 6.82447 5.38734 5.38451 7.50024L8 7.5V9.5H2V3.5H4L3.99989 5.99918C5.82434 3.57075 8.72873 2 12 2ZM13 7L12.9998 11.585L16.2426 14.8284L14.8284 16.2426L10.9998 12.413L11 7H13Z"></path></svg>
				</button>
			</div>

			<button class="badge-btn">
				변수 검사
			</button>
		</div>

		<!-- Right Group: Zoom & Sidebar Toggle -->
		<div class="group right">
			{#if nexus.showMinimap}
				<div class="minimap-container">
					<MiniMap 
						width={120} 
						height={80} 
						position="bottom-right" 
						style="position: relative; bottom: 0; right: 0; margin: 0; border-radius: 8px; border: 0.5px solid #e5e5e7;"
					/>
				</div>
			{/if}
			
			<div class="zoom-bar">
				<button class="icon-btn" onclick={handleZoomOut} title="Zoom Out" aria-label="Zoom Out">
					<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748ZM7 10H15V12H7V10Z"></path></svg>
				</button>
				<button class="zoom-text" onclick={() => nexus.showMinimap = !nexus.showMinimap} aria-label="Toggle Minimap">
					{zoomLevel}%
				</button>
				<button class="icon-btn" onclick={handleZoomIn} title="Zoom In" aria-label="Zoom In">
					<svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748ZM7 10H15V12H7V10Z"></path></svg>
				</button>
			</div>

			<div class="action-bar">
				<button class="icon-btn" onclick={() => nexus.toggleRightSidebar()} title="Toggle Properties">
					{#if nexus.rightSidebarOpen}
						<PanelRightClose class="h-4 w-4" />
					{:else}
						<PanelRightOpen class="h-4 w-4" />
					{/if}
				</button>
			</div>
		</div>
	</div>
</div>

<!-- Original Control Buttons (Floating separately or integrated) -->
<div class="floating-controls">
	<button class="control-btn main-add" onclick={onAddNode} title="Add Block" aria-label="Add Node">
		<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>
	</button>
	<button class="control-btn memo-add" onclick={onAddMemo} title="Add Note" aria-label="Add Memo">
		<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15.5 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V8.5L15.5 3z"/><path d="M15 3v6h6"/><path d="M9 18h6"/><path d="M9 14h6"/></svg>
	</button>
	<div class="vertical-bar">
		<button class="control-btn" class:active={!panOnDrag} onclick={() => onTogglePanMode(false)} title="Select Mode" aria-label="Select Mode">
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m3 3 7.07 16.97 2.51-7.39 7.39-2.51L3 3z"/><path d="m13 13 6 6"/></svg>
		</button>
		<button class="control-btn" class:active={panOnDrag} onclick={() => onTogglePanMode(true)} title="Hand Mode" aria-label="Hand Mode">
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 11V6a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0"/><path d="M14 10V4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0"/><path d="M10 10.5V3a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0"/><path d="M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15"/><path d="M10 18h4"/></svg>
		</button>
		<div class="h-divider"></div>
		<button class="control-btn" onclick={onAutoLayout} title="Auto Layout" aria-label="Auto Layout">
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/></svg>
		</button>
		<button class="control-btn" class:active={isMaximized} onclick={onToggleMaximize} title="Maximize" aria-label="Maximize">
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 3h6v6"/><path d="M9 21H3v-6"/><path d="M21 3l-7 7"/><path d="M3 21l7-7"/></svg>
		</button>
	</div>
</div>

<style>
	.workflow-footer {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 0 16px 12px;
		pointer-events: none;
		z-index: 100;
	}

	.footer-container {
		display: flex;
		justify-content: space-between;
		align-items: flex-end;
		width: 100%;
	}

	.group {
		display: flex;
		align-items: center;
		gap: 8px;
		pointer-events: auto;
	}

	.action-bar, .zoom-bar {
		display: flex;
		align-items: center;
		background: rgba(255, 255, 255, 0.9);
		backdrop-filter: blur(4px);
		border: 0.5px solid #e5e5e7;
		border-radius: 8px;
		padding: 2px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
	}

	.icon-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;
		background: transparent;
		color: #86868b;
		cursor: pointer;
		border-radius: 6px;
		transition: all 0.2s;
	}

	.icon-btn:hover:not(.disabled) {
		background: #f5f5f7;
		color: #1d1d1f;
	}

	.icon-btn.disabled {
		color: #d2d2d7;
		cursor: not-allowed;
	}

	.v-divider {
		width: 1px;
		height: 14px;
		background: #e5e5e7;
		margin: 0 4px;
	}

	.badge-btn {
		height: 24px;
		padding: 0 10px;
		background: white;
		border: 0.5px solid #e5e5e7;
		border-radius: 6px;
		font-size: 11px;
		font-weight: 600;
		color: #86868b;
		cursor: pointer;
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
	}

	.badge-btn:hover {
		background: #f5f5f7;
	}

	.zoom-text {
		font-size: 12px;
		font-weight: 500;
		color: #86868b;
		padding: 0 8px;
		min-width: 45px;
		text-align: center;
		cursor: pointer;
		background: transparent;
		border: none;
	}

	.minimap-container {
		position: absolute;
		bottom: 48px;
		right: 0;
		background: white;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
	}

	/* Floating Vertical Controls */
	.floating-controls {
		position: absolute;
		left: 20px;
		top: 50%;
		transform: translateY(-50%);
		display: flex;
		flex-direction: column;
		gap: 12px;
		pointer-events: auto;
		z-index: 100;
	}

	.vertical-bar {
		display: flex;
		flex-direction: column;
		background: white;
		border: 0.5px solid #e5e5e7;
		border-radius: 8px;
		padding: 2px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
	}

	.control-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;
		background: transparent;
		color: #86868b;
		cursor: pointer;
		border-radius: 6px;
	}

	.control-btn:hover {
		background: #f5f5f7;
		color: #1d1d1f;
	}

	.control-btn.active {
		background: #f0f7ff;
		color: #0071e3;
	}

	.main-add {
		background: #0071e3;
		color: white;
		border-radius: 8px;
		box-shadow: 0 4px 12px rgba(0, 113, 227, 0.3);
	}

	.main-add:hover {
		background: #0077ed;
		color: white;
		transform: scale(1.05);
	}

	.memo-add {
		background: #ffd54f;
		color: #5d4037;
		border-radius: 8px;
		box-shadow: 0 4px 12px rgba(255, 213, 79, 0.3);
	}

	.memo-add:hover {
		background: #ffca28;
		transform: scale(1.05);
	}

	.h-divider {
		height: 1px;
		width: 14px;
		background: #e5e5e7;
		margin: 4px auto;
	}

	@media (prefers-color-scheme: dark) {
		.action-bar, .zoom-bar, .vertical-bar, .badge-btn, .minimap-container {
			background: #1c1c1e;
			border-color: #3a3a3c;
		}
		.icon-btn:hover:not(.disabled), .badge-btn:hover, .control-btn:hover {
			background: #2c2c2e;
			color: white;
		}
	}
</style>
