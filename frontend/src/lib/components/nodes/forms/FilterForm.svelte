<script lang="ts">
	import IfForm from './IfForm.svelte';
	import type { Node } from '@xyflow/svelte';

	let { node } = $props<{ node: Node }>();
</script>

<IfForm {node} />
