<script lang="ts">
	import { nexus } from '$lib/nexus.svelte';
	import type { Node } from '@xyflow/svelte';
	import { Label } from '$lib/components/ui/label';
	import { Info, Code2, Variable } from 'lucide-svelte';

	let { node } = $props<{ node: Node }>();
	
	function update(key: string, value: any) {
		const currentConfig = (node.data.config as Record<string, any>) || {};
		nexus.nodes = nexus.nodes.map(n => n.id === node.id ? {
			...n,
			data: { ...n.data, config: { ...currentConfig, [key]: value } }
		} : n);
		if (nexus.selectedNode?.id === node.id) {
			nexus.selectedNode = {
				...nexus.selectedNode,
				data: { ...nexus.selectedNode.data, config: { ...currentConfig, [key]: value } }
			};
		}
	}

	let config = $derived((node.data.config as Record<string, any>) || {});

	const languages = [
		{ value: 'javascript', label: 'JavaScript (QuickJS)' },
		{ value: 'python', label: 'Python 3' }
	];

	let selectedLanguage = $derived(config.language || 'javascript');
	let codeLabel = $derived(selectedLanguage === 'python' ? 'Python Code' : 'JavaScript Code');
	let inputVarName = $derived(selectedLanguage === 'python' ? 'data' : '$input');
	let defaultCode = $derived(selectedLanguage === 'python' 
		? 'def main(input_data):\n    # Use data to access incoming data\n    return data' 
		: 'const main = () => {\n    // Use $input to access incoming data\n    return $input;\n};\n\nreturn main();'
	);
</script>

<div class="space-y-6">
	<!-- Language Selection -->
	<div class="grid gap-2">
		<div class="flex items-center gap-2">
			<Code2 class="h-4 w-4 text-muted-foreground" />
			<Label for="language">Programming Language</Label>
		</div>
		<select id="language" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" 
			value={selectedLanguage} 
			onchange={(e: Event & { currentTarget: HTMLSelectElement }) => {
				update('language', e.currentTarget.value);
				if (!config.code) {
					const newDefault = e.currentTarget.value === 'python' ? 'return data' : 'return $input;';
					update('code', newDefault);
				}
			}}>
			{#each languages as lang}
				<option value={lang.value}>{lang.label}</option>
			{/each}
		</select>
	</div>

	<!-- Code Editor Section -->
	<div class="grid gap-2">
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2">
				<div class="h-2 w-2 rounded-full {selectedLanguage === 'python' ? 'bg-blue-400' : 'bg-yellow-400'}"></div>
				<Label for="code">{codeLabel}</Label>
			</div>
			<div class="flex items-center gap-1 text-[10px] text-muted-foreground bg-muted px-1.5 py-0.5 rounded font-mono">
				{inputVarName} variable available
			</div>
		</div>
		
		<div class="relative group">
			<textarea
				id="code"
				class="flex min-h-[400px] w-full rounded-md border border-input bg-slate-950 px-4 py-3 text-[11px] font-mono text-slate-200 ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 resize-y leading-relaxed shadow-inner"
				value={config.code || defaultCode}
				oninput={(e: Event & { currentTarget: HTMLTextAreaElement }) => update('code', e.currentTarget.value)}
				spellcheck="false"
			></textarea>
			<div class="absolute bottom-2 right-3 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none">
				<span class="text-[9px] text-slate-500 uppercase font-bold tracking-tighter">{selectedLanguage}</span>
			</div>
		</div>
		
		<div class="rounded-md border bg-muted/30 p-2.5">
			<div class="flex gap-2 text-[10px] text-muted-foreground">
				<Info class="h-3 w-3 shrink-0 mt-0.5" />
				<p>
					The script must return a JSON-serializable object. 
					In <strong>JavaScript</strong>, the last expression is returned. 
					In <strong>Python</strong>, the <code>main()</code> function's return value is used.
				</p>
			</div>
		</div>
	</div>

	<!-- Variables Section (Dify Style Placeholder) -->
	<div class="grid gap-3 pt-2 border-t">
		<div class="flex items-center gap-2">
			<Variable class="h-4 w-4 text-muted-foreground" />
			<Label>Input Variables</Label>
		</div>
		<div class="text-[11px] p-3 border border-dashed rounded-lg bg-muted/10 text-center text-muted-foreground italic">
			Variables are automatically mapped from connected nodes via <code>$input</code>.
		</div>
	</div>
</div>
