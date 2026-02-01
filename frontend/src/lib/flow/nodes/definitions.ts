// n8n-style Basic Nodes + AI Nodes + Chat
export type NodeKind =
	// Triggers (no inputs)
	| 'trigger-start'
	| 'trigger-schedule'
	| 'trigger-webhook'
	| 'chat-trigger'
	// Logic/Flow Control
	| 'if'
	| 'switch'
	| 'merge'
	// Transform
	| 'code'
	| 'http-request'
	| 'set'
	| 'filter'
	| 'wait'
	| 'rss-feed-read'
	| 'slack'
	// AI / LLM
	| 'ai-agent'
	| 'llm'
	| 'openai'
	| 'openrouter'
	| 'tool'
	| 'rss-read-tool'
	// End
	| 'trigger-end';

export type NodePort = {
	id: string;
	label: string;
	type?: 'data' | 'tool';
};

export type NodeDefinition = {
	kind: NodeKind;
	label: string;
	detail: string;
	tone: 'emerald' | 'cobalt' | 'amber' | 'slate';
	icon: string;
	width: number;
	minHeight: number;
	inputs: NodePort[];
	outputs: NodePort[];
};

export const NODE_DEFAULT_WIDTH = 220;
export const NODE_DEFAULT_HEIGHT = 84;
export const NODE_HEADER_HEIGHT = 56;
export const PORT_ROW_HEIGHT = 24;
export const PORT_SECTION_TOP = 8;
export const PORT_SECTION_BOTTOM = 12;

// ========== TRIGGER NODES (No inputs) ==========
export const triggerNodes: NodeDefinition[] = [
	{
		kind: 'trigger-start',
		label: 'Manual Trigger',
		detail: 'Starts workflow on button click',
		tone: 'emerald',
		icon: 'MT',
		width: 200,
		minHeight: 80,
		inputs: [],
		outputs: [{ id: 'out', label: '' }]
	},
	{
		kind: 'trigger-schedule',
		label: 'Schedule Trigger',
		detail: 'Runs on a schedule',
		tone: 'emerald',
		icon: 'ST',
		width: 200,
		minHeight: 80,
		inputs: [],
		outputs: [{ id: 'out', label: '' }]
	},
	{
		kind: 'trigger-webhook',
		label: 'Webhook',
		detail: 'Triggers via HTTP request',
		tone: 'emerald',
		icon: 'WH',
		width: 200,
		minHeight: 80,
		inputs: [],
		outputs: [{ id: 'out', label: '' }]
	},
	{
		kind: 'chat-trigger',
		label: 'Chat Trigger',
		detail: 'Starts workflow from chat interface',
		tone: 'emerald',
		icon: 'CH',
		width: 220,
		minHeight: 80,
		inputs: [],
		outputs: [{ id: 'out', label: '' }]
	},
	{
		kind: 'rss-feed-trigger',
		label: 'RSS Feed Trigger',
		detail: 'Triggers when an RSS feed is updated',
		tone: 'emerald',
		icon: 'RS',
		width: 220,
		minHeight: 80,
		inputs: [],
		outputs: [{ id: 'out', label: '' }]
	}
];

// ========== LOGIC / FLOW CONTROL ==========
export const logicNodes: NodeDefinition[] = [
	{
		kind: 'if',
		label: 'IF',
		detail: 'Split based on condition',
		tone: 'amber',
		icon: 'IF',
		width: 200,
		minHeight: 100,
		inputs: [{ id: 'in', label: '' }],
		outputs: [
			{ id: 'true', label: 'true' },
			{ id: 'false', label: 'false' }
		]
	},
	{
		kind: 'switch',
		label: 'Switch',
		detail: 'Route to multiple outputs',
		tone: 'amber',
		icon: 'SW',
		width: 200,
		minHeight: 120,
		inputs: [{ id: 'in', label: '' }],
		outputs: [
			{ id: '0', label: '0' },
			{ id: '1', label: '1' },
			{ id: '2', label: '2' }
		]
	},
	{
		kind: 'merge',
		label: 'Merge',
		detail: 'Merge multiple inputs',
		tone: 'amber',
		icon: 'MG',
		width: 200,
		minHeight: 100,
		inputs: [
			{ id: 'in1', label: '' },
			{ id: 'in2', label: '' }
		],
		outputs: [{ id: 'out', label: '' }]
	}
];

// ========== TRANSFORM NODES ==========
export const transformNodes: NodeDefinition[] = [
	{
		kind: 'code',
		label: 'Code',
		detail: 'Run JavaScript logic',
		tone: 'cobalt',
		icon: 'JS',
		width: 220,
		minHeight: 80,
		inputs: [{ id: 'in', label: '' }],
		outputs: [{ id: 'out', label: '' }]
	},
	{
		kind: 'http-request',
		label: 'HTTP Request',
		detail: 'Make HTTP calls',
		tone: 'cobalt',
		icon: 'HT',
		width: 200,
		minHeight: 80,
		inputs: [{ id: 'in', label: '' }],
		outputs: [
			{ id: 'out', label: '' },
			{ id: 'error', label: 'error' }
		]
	},
	{
		kind: 'set',
		label: 'Set',
		detail: 'Set field values',
		tone: 'cobalt',
		icon: 'SV',
		width: 200,
		minHeight: 80,
		inputs: [{ id: 'in', label: '' }],
		outputs: [{ id: 'out', label: '' }]
	},
	{
		kind: 'filter',
		label: 'Filter',
		detail: 'Filter items',
		tone: 'cobalt',
		icon: 'FI',
		width: 200,
		minHeight: 80,
		inputs: [{ id: 'in', label: '' }],
		outputs: [{ id: 'out', label: '' }]
	},
	{
		kind: 'wait',
		label: 'Wait',
		detail: 'Wait for time',
		tone: 'cobalt',
		icon: 'WT',
		width: 200,
		minHeight: 80,
		inputs: [{ id: 'in', label: '' }],
		outputs: [{ id: 'out', label: '' }]
	},
	{
		kind: 'rss-feed-read',
		label: 'RSS Read',
		detail: 'Read RSS feed',
		tone: 'cobalt',
		icon: 'RR',
		width: 200,
		minHeight: 80,
		inputs: [{ id: 'in', label: '' }],
		outputs: [{ id: 'out', label: '' }]
	},
	{
		kind: 'slack',
		label: 'Slack',
		detail: 'Send Slack message',
		tone: 'cobalt',
		icon: 'SL',
		width: 200,
		minHeight: 80,
		inputs: [{ id: 'in', label: '' }],
		outputs: [{ id: 'out', label: '' }]
	}
];

// ========== AI / LLM NODES ==========
export const aiNodes: NodeDefinition[] = [
	{
		kind: 'ai-agent',
		label: 'AI Agent',
		detail: 'Autonomous AI agent with tools',
		tone: 'cobalt',
		icon: 'AI',
		width: 220,
		minHeight: 100,
		inputs: [
			{ id: 'in', label: '' },
			{ id: 'tools', label: 'Tools', type: 'tool' }
		],
		outputs: [{ id: 'out', label: '' }]
	},
	{
		kind: 'llm',
		label: 'LLM',
		detail: 'Language model via OpenRouter',
		tone: 'cobalt',
		icon: 'LL',
		width: 220,
		minHeight: 80,
		inputs: [{ id: 'in', label: '' }],
		outputs: [
			{ id: 'out', label: '' },
			{ id: 'error', label: 'error' }
		]
	},
	{
		kind: 'openai',
		label: 'OpenAI',
		detail: 'GPT-4o, GPT-4 Turbo, DALL-E, etc.',
		tone: 'cobalt',
		icon: 'OA',
		width: 220,
		minHeight: 80,
		inputs: [{ id: 'in', label: '' }],
		outputs: [
			{ id: 'out', label: '' },
			{ id: 'error', label: 'error' }
		]
	},
	{
		kind: 'openrouter',
		label: 'OpenRouter',
		detail: 'Access 100s of models via unified API',
		tone: 'cobalt',
		icon: 'OR',
		width: 220,
		minHeight: 80,
		inputs: [{ id: 'in', label: '' }],
		outputs: [
			{ id: 'out', label: '' },
			{ id: 'error', label: 'error' }
		]
	}
];

// ========== TOOL NODES ==========
export const toolNodes: NodeDefinition[] = [
	{
		kind: 'tool',
		label: 'MCP Tool',
		detail: 'Connect to MCP server tool',
		tone: 'amber',
		icon: 'TL',
		width: 180,
		minHeight: 60,
		inputs: [],
		outputs: [{ id: 'tool', label: '', type: 'tool' }]
	},
	{
		kind: 'rss-read-tool',
		label: 'RSS Read Tool',
		detail: 'RSS Feed tool for AI Agents',
		tone: 'amber',
		icon: 'RT',
		width: 180,
		minHeight: 60,
		inputs: [],
		outputs: [{ id: 'tool', label: '', type: 'tool' }]
	}
];

// ========== END NODE ==========
export const endNode: NodeDefinition[] = [
	{
		kind: 'trigger-end',
		label: 'End',
		detail: 'End of workflow',
		tone: 'slate',
		icon: 'EN',
		width: 200,
		minHeight: 80,
		inputs: [{ id: 'in', label: '' }],
		outputs: []
	}
];

// ========== FULL LIBRARY ==========
export const nodeLibrary: NodeDefinition[] = [
	...triggerNodes,
	...logicNodes,
	...transformNodes,
	...aiNodes,
	...toolNodes,
	...endNode
];

const nodeDefinitionMap = new Map<NodeKind, NodeDefinition>(
	nodeLibrary.map((item) => [item.kind, item])
);

export const getNodeDefinition = (kind: NodeKind) => nodeDefinitionMap.get(kind);

export const getNodePorts = (kind: NodeKind, side: 'input' | 'output') => {
	const definition = getNodeDefinition(kind);
	if (!definition) {
		return side === 'input' ? [{ id: 'in', label: '' }] : [{ id: 'out', label: '' }];
	}
	return side === 'input' ? definition.inputs : definition.outputs;
};

export const getNodeDimensions = (kind: NodeKind) => {
	const definition = getNodeDefinition(kind);
	const width = definition?.width ?? NODE_DEFAULT_WIDTH;
	
	// Only count data ports for height
	const dataInputs = definition?.inputs.filter(p => p.type !== 'tool') ?? [];
	const dataOutputs = definition?.outputs.filter(p => p.type !== 'tool') ?? [];
	
	const portRows = Math.max(dataInputs.length, dataOutputs.length);
	const portsHeight =
		portRows > 1 ? portRows * PORT_ROW_HEIGHT + PORT_SECTION_TOP + PORT_SECTION_BOTTOM : 0;
	const minHeight = definition?.minHeight ?? NODE_DEFAULT_HEIGHT;
	const height = Math.max(minHeight, NODE_HEADER_HEIGHT + portsHeight);
	return { width, height };
};

export const getPortOffset = (
	ports: NodePort[],
	portId: string | null | undefined,
	nodeHeight: number
) => {
	if (ports.length <= 1) {
		return nodeHeight / 2;
	}
	const index = ports.findIndex((port) => port.id === portId);
	const safeIndex = index === -1 ? 0 : index;
	return NODE_HEADER_HEIGHT + PORT_SECTION_TOP + PORT_ROW_HEIGHT * safeIndex + PORT_ROW_HEIGHT / 2;
};
