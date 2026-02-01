import { browser } from '$app/environment';
import type { Node, Edge } from '@xyflow/svelte';
import type { Credential, McpServer, ExecutionResult, ActiveView, SidebarTab, ViewMode } from './types';
import type { NodeKind } from '$lib/flow/nodes/definitions';

const API_BASE = import.meta.env.VITE_API_BASE ?? 'http://localhost:3001';

export class NexusState {
	// View State
	activeView = $state<ActiveView>('canvas');
	activeTab = $state<SidebarTab>('nodes');
	workflowName = $state('Untitled Workflow');
	leftSidebarOpen = $state(true);
	rightSidebarOpen = $state(true);
	isMaximized = $state(false);
	panOnDrag = $state(true);
	showMinimap = $state(true);
	showChatPanel = $state(false);

	// Flow State
	nodes = $state<Node[]>([]);
	edges = $state<Edge[]>([]);
	selectedNodeIds = $state<Set<string>>(new Set());
	selectedEdgeIds = $state<Set<string>>(new Set());
	selectedNode = $state<Node | null>(null);

	// Data
	credentials = $state<Credential[]>([]);
	isFetchingCredentials = $state(false);
	mcpServers = $state<McpServer[]>([]);
	mcpTools = $state<Record<string, any[]>>({});
	isCheckingStatus = $state<Record<string, boolean>>({});
	executions = $state<any[]>([]);
	selectedExecution = $state<any | null>(null);

	// Execution State
	isExecuting = $state(false);
	isSaving = $state(false);
	executingNodeId = $state<string | null>(null);
	isNodeExecuting = $state<Record<string, boolean>>({});
	executionResults = $state<ExecutionResult>({ nodeResults: {}, summary: null });
	nodeViewModes = $state<Record<string, ViewMode>>({});

	// Search & Context
	nodeSearch = $state('');
	contextMenu = $state<{ x: number; y: number; clientX: number; clientY: number } | null>(null);

	private saveTimeout: any = null;

	constructor() {
		if (browser) {
			this.loadFromStorage();
			this.fetchCredentials();
			this.fetchMcpServers();
			
			// Initialize auto-save outside of root effect for better control
			this.setupAutoSave();
		}
	}

	async fetchMcpServers() {
		try {
			const response = await fetch(`${API_BASE}/api/mcp/servers`);
			if (response.ok) {
				const servers = await response.json();
				this.mcpServers = servers.map((s: any) => ({
					...s,
					url: s.endpoint // Map backend endpoint to frontend url
				}));
			}
		} catch (e) {
			console.error('Failed to fetch MCP servers:', e);
		}
	}

	async checkMcpServerStatus(id: string) {
		this.isCheckingStatus[id] = true;
		try {
			const response = await fetch(`${API_BASE}/api/mcp/servers/${id}/status`);
			if (response.ok) {
				const result = await response.json();
				this.mcpServers = this.mcpServers.map(s => 
					s.id === id ? { ...s, status: result.status as any } : s
				);
				if (result.tools) {
					this.mcpTools[id] = result.tools;
				}
			}
		} catch (e) {
			console.error('Failed to check MCP status:', e);
		} finally {
			this.isCheckingStatus[id] = false;
		}
	}

	async fetchMcpTools(id: string) {
		try {
			const response = await fetch(`${API_BASE}/api/mcp/servers/${id}/tools`);
			if (response.ok) {
				const result = await response.json();
				this.mcpTools[id] = result.tools || [];
				if (result.status) {
					this.mcpServers = this.mcpServers.map(s => 
						s.id === id ? { ...s, status: result.status as any } : s
					);
				}
			}
		} catch (e) {
			console.error('Failed to fetch MCP tools:', e);
		}
	}

	private setupAutoSave() {
		// Use a reactive tracker to watch changes
		$effect.root(() => {
			$effect(() => {
				// Access reactive properties
				const track = {
					n: this.nodes,
					e: this.edges,
					w: this.workflowName,
					m: this.mcpServers
				};
				
				if (this.saveTimeout) clearTimeout(this.saveTimeout);
				this.saveTimeout = setTimeout(() => {
					this.saveToStorage();
				}, 1500);
			});
		});
	}

	loadFromStorage() {
		const savedNodes = localStorage.getItem('nexus_nodes');
		const savedEdges = localStorage.getItem('nexus_edges');
		const savedName = localStorage.getItem('nexus_name');
		const savedMcp = localStorage.getItem('nexus_mcp_servers');

		try {
			if (savedNodes) this.nodes = JSON.parse(savedNodes);
			if (savedMcp) this.mcpServers = JSON.parse(savedMcp);
			if (savedEdges) this.edges = JSON.parse(savedEdges);
			if (savedName) this.workflowName = savedName;
		} catch (e) {
			console.error('Failed to load storage:', e);
		}

		if (this.nodes.length === 0) {
			this.nodes = [{
				id: crypto.randomUUID(),
				type: 'workflowNode',
				position: { x: 100, y: 100 },
				data: { kind: 'trigger-start' as NodeKind, label: 'Start' }
			}];
		}
	}

	saveToStorage() {
		if (!browser) return;
		try {
			localStorage.setItem('nexus_nodes', JSON.stringify(this.nodes));
			localStorage.setItem('nexus_edges', JSON.stringify(this.edges));
			localStorage.setItem('nexus_name', this.workflowName);
			localStorage.setItem('nexus_mcp_servers', JSON.stringify(this.mcpServers));
		} catch (e) {
			console.error('Save failed:', e);
		}
	}

	async saveWorkflow() {
		this.isSaving = true;
		this.saveToStorage();
		// Artificial delay for UI feedback
		await new Promise(resolve => setTimeout(resolve, 500));
		this.isSaving = false;
	}

	// Actions
	async fetchCredentials() {
		this.isFetchingCredentials = true;
		try {
			const response = await fetch(`${API_BASE}/api/credentials`);
			if (!response.ok) throw new Error('Fetch failed');
			this.credentials = await response.json();
		} catch (e) {
			console.error('Fetch credentials error:', e);
		} finally {
			this.isFetchingCredentials = false;
		}
	}

	async executeWorkflow() {
		return await this.triggerWorkflowWithInput(null);
	}

	private isValidUuid(id: string): boolean {
		return /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(id);
	}

	async triggerWorkflowWithInput(initialInput: any) {
		console.log('Triggering workflow with input:', initialInput);
		this.isExecuting = true;
		this.executionResults = { nodeResults: {}, summary: null };
		this.executingNodeId = null;

		// 1. Reset all nodes visual state before starting
		this.nodes = this.nodes.map(n => ({
			...n,
			data: { ...n.data, executionStatus: undefined, itemCount: undefined }
		}));

		try {
			// Find the specific trigger node if initialInput is present
			const chatTriggerNode = initialInput ? this.nodes.find(n => n.data.kind === 'chat-trigger') : null;

			const idMap = new Map<string, string>();
			const normalizeId = (oldId: string) => {
				if (this.isValidUuid(oldId)) return oldId;
				if (idMap.has(oldId)) return idMap.get(oldId)!;
				const newId = crypto.randomUUID();
				idMap.set(oldId, newId);
				return newId;
			};

			const payloadNodes = this.nodes.map(n => ({
				id: normalizeId(n.id),
				kind: n.data.kind,
				label: n.data.label,
				position: n.position,
				config: n.data.kind === 'chat-trigger' && initialInput 
					? { ...n.data.config, initialInput } 
					: (n.data.config || {})
			}));

			const payloadEdges = this.edges.map(e => ({
				id: normalizeId(e.id),
				from: normalizeId(e.source),
				to: normalizeId(e.target)
			}));

			const triggerNodeId = chatTriggerNode ? normalizeId(chatTriggerNode.id) : null;

			const response = await fetch(`${API_BASE}/api/workflows/execute`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					workflow_id: 'current',
					nodes: payloadNodes,
					edges: payloadEdges,
					trigger_node_id: triggerNodeId
				})
			});

			if (!response.ok) {
				const errorText = await response.text();
				throw new Error(`Execution server error: ${response.status} ${errorText}`);
			}
			
			const result = await response.json();
			if (result.results) {
				const reverseIdMap = new Map<string, string>();
				idMap.forEach((newId, oldId) => reverseIdMap.set(newId, oldId));

				// 2. Simulate sequential execution for visual feedback
				for (const r of result.results) {
					const originalId = reverseIdMap.get(r.node_id) || r.node_id;
					
					// Set as "Running"
					this.executingNodeId = originalId;
					this.nodes = this.nodes.map(n => n.id === originalId ? {
						...n, data: { ...n.data, executionStatus: 'running' as any }
					} : n);

					// Small delay to show "Running" state
					await new Promise(resolve => setTimeout(resolve, 400));

					// Set final result
					let count = 0;
					if (r.output) {
						count = Array.isArray(r.output) ? r.output.length : (typeof r.output === 'object' ? 1 : 0);
					}

					this.nodes = this.nodes.map(n => n.id === originalId ? {
						...n, data: { ...n.data, executionStatus: r.success ? 'success' : 'error', itemCount: count }
					} : n);

					// Stop simulation if this node failed (it reflects backend stop)
					if (!r.success) break;
				}

				this.executingNodeId = null;

				const nodeResults: Record<string, any> = {};
				result.results.forEach((r: any) => {
					const originalId = reverseIdMap.get(r.node_id) || r.node_id;
					nodeResults[originalId] = {
						status: r.success ? 'success' : 'error',
						message: r.error,
						output: r.output
					};
				});

				this.executionResults = {
					nodeResults,
					summary: result.success ? {
						total: result.results.length,
						success: result.results.filter((r: any) => r.success).length,
						errors: result.results.filter((r: any) => !r.success).length,
						duration: 0
					} : null
				};
				await this.fetchExecutions();
				return this.executionResults;
			}
			return null;
		} catch (e) {
			console.error('Workflow trigger failed:', e);
			this.executingNodeId = null;
			return null;
		} finally {
			this.isExecuting = false;
		}
	}

	async executeSingleNode(nodeId: string) {
		const node = this.nodes.find(n => n.id === nodeId);
		if (!node) return;

		this.isNodeExecuting[nodeId] = true;
		
		const normalizedNode = {
			...node,
			id: this.isValidUuid(node.id) ? node.id : crypto.randomUUID(),
			kind: node.data.kind,
			label: node.data.label,
			config: node.data.config || {}
		};

		try {
			const response = await fetch(`${API_BASE}/api/nodes/execute`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					node: normalizedNode
				})
			});

			if (!response.ok) throw new Error(`Status ${response.status}`);

			const result = await response.json();
			let count = 0;
			if (result.output) {
				count = Array.isArray(result.output) ? result.output.length : (typeof result.output === 'object' ? 1 : 0);
			}

			this.nodes = this.nodes.map(n => n.id === nodeId ? {
				...n,
				data: { ...n.data, executionStatus: result.success ? 'success' : 'error', itemCount: count }
			} : n);

			this.executionResults.nodeResults[nodeId] = {
				status: result.success ? 'success' : 'error',
				message: result.error,
				count,
				output: result.output
			};
			await this.fetchExecutions();
		} catch (e) {
			console.error('Single node execution failed:', e);
		} finally {
			this.isNodeExecuting[nodeId] = false;
		}
	}

	async deleteCredential(id: string) {
		try {
			await fetch(`${API_BASE}/api/credentials/${id}`, { method: 'DELETE' });
			await this.fetchCredentials();
		} catch (e) {
			console.error('Delete failed:', e);
		}
	}

	async fetchExecutions() {
		try {
			const response = await fetch(`${API_BASE}/api/executions`);
			this.executions = await response.json();
		} catch (e) {
			console.error('Fetch executions failed:', e);
		}
	}

	selectExecution(execution: any) {
		this.selectedExecution = execution;
		this.nodeViewModes = {};
		if (execution?.results) {
			execution.results.forEach((r: any) => {
				this.nodeViewModes[r.node_id] = 'json';
			});
		}
	}

	async addMcpServer(name: string, url: string, headers: Record<string, string> = {}) {
		if (!name || !url) return;
		
		try {
			const response = await fetch(`${API_BASE}/api/mcp/servers`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					name,
					transport: 'streamable-http',
					endpoint: url,
					headers,
					args: [],
					env: [],
					auto_start: true
				})
			});

			if (!response.ok) throw new Error('Failed to register MCP server on backend');
			
			const newServer = await response.json();
			this.mcpServers = [...this.mcpServers, {
				...newServer,
				url: newServer.endpoint
			}];
		} catch (e) {
			console.error('MCP Registration failed:', e);
		}
	}

	removeMcpServer(id: string) {
		this.mcpServers = this.mcpServers.filter(s => s.id !== id);
	}

	toggleLeftSidebar() {
		this.leftSidebarOpen = !this.leftSidebarOpen;
	}

	toggleRightSidebar() {
		this.rightSidebarOpen = !this.rightSidebarOpen;
	}
}

export const nexus = new NexusState();