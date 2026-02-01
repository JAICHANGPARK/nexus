import type { Node, Edge } from '@xyflow/svelte';
import type { NodeKind } from '$lib/flow/nodes/definitions';

export interface Credential {
	id: string;
	name: string;
	provider: string;
	data: Record<string, string>;
	created_at: string;
}

export interface McpServer {
	id: string;
	name: string;
	url: string;
	headers?: Record<string, string>;
	transport: 'streamable-http';
	status: 'connected' | 'error' | 'none' | 'disconnected';
}

export interface ExecutionResult {
	nodeResults: Record<
		string,
		{ status: 'success' | 'error'; message?: string; count?: number; output?: any }
	>;
	summary: { total: number; success: number; errors: number; duration: number } | null;
	fullResults?: any;
}

export type ViewMode = 'json' | 'table' | 'schema';
export type ActiveView = 'canvas' | 'credentials' | 'tools' | 'knowledge-base' | 'data-table';
export type SidebarTab = 'nodes' | 'executions';
