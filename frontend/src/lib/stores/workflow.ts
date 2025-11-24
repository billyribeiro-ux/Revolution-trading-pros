import { writable, derived } from 'svelte/store';
import type { Workflow, WorkflowNode, WorkflowEdge, WorkflowCanvas } from '$lib/types/workflow';

function createWorkflowStore() {
	const { subscribe, set, update } = writable<WorkflowCanvas>({
		nodes: [],
		edges: [],
		selectedNode: undefined,
		selectedEdge: undefined,
		zoom: 1,
		pan: { x: 0, y: 0 }
	});

	return {
		subscribe,

		// Node operations
		addNode: (node: WorkflowNode) => {
			update((state) => ({
				...state,
				nodes: [...state.nodes, node]
			}));
		},

		updateNode: (nodeId: number, updates: Partial<WorkflowNode>) => {
			update((state) => ({
				...state,
				nodes: state.nodes.map((node) => (node.id === nodeId ? { ...node, ...updates } : node))
			}));
		},

		deleteNode: (nodeId: number) => {
			update((state) => ({
				...state,
				nodes: state.nodes.filter((node) => node.id !== nodeId),
				edges: state.edges.filter(
					(edge) => edge.from_node_id !== nodeId && edge.to_node_id !== nodeId
				),
				selectedNode: state.selectedNode?.id === nodeId ? undefined : state.selectedNode
			}));
		},

		moveNode: (nodeId: number, position: { x: number; y: number }) => {
			update((state) => ({
				...state,
				nodes: state.nodes.map((node) =>
					node.id === nodeId ? { ...node, position_x: position.x, position_y: position.y } : node
				)
			}));
		},

		// Edge operations
		addEdge: (edge: WorkflowEdge) => {
			update((state) => ({
				...state,
				edges: [...state.edges, edge]
			}));
		},

		updateEdge: (edgeId: number, updates: Partial<WorkflowEdge>) => {
			update((state) => ({
				...state,
				edges: state.edges.map((edge) => (edge.id === edgeId ? { ...edge, ...updates } : edge))
			}));
		},

		deleteEdge: (edgeId: number) => {
			update((state) => ({
				...state,
				edges: state.edges.filter((edge) => edge.id !== edgeId),
				selectedEdge: state.selectedEdge?.id === edgeId ? undefined : state.selectedEdge
			}));
		},

		// Selection
		selectNode: (node?: WorkflowNode) => {
			update((state) => ({
				...state,
				selectedNode: node,
				selectedEdge: undefined
			}));
		},

		selectEdge: (edge?: WorkflowEdge) => {
			update((state) => ({
				...state,
				selectedEdge: edge,
				selectedNode: undefined
			}));
		},

		clearSelection: () => {
			update((state) => ({
				...state,
				selectedNode: undefined,
				selectedEdge: undefined
			}));
		},

		// Canvas operations
		setZoom: (zoom: number) => {
			update((state) => ({
				...state,
				zoom: Math.max(0.1, Math.min(2, zoom))
			}));
		},

		setPan: (pan: { x: number; y: number }) => {
			update((state) => ({
				...state,
				pan
			}));
		},

		// Load workflow
		loadWorkflow: (nodes: WorkflowNode[], edges: WorkflowEdge[]) => {
			set({
				nodes,
				edges,
				selectedNode: undefined,
				selectedEdge: undefined,
				zoom: 1,
				pan: { x: 0, y: 0 }
			});
		},

		// Reset
		reset: () => {
			set({
				nodes: [],
				edges: [],
				selectedNode: undefined,
				selectedEdge: undefined,
				zoom: 1,
				pan: { x: 0, y: 0 }
			});
		}
	};
}

export const workflowCanvas = createWorkflowStore();

// Derived stores
export const selectedNode = derived(workflowCanvas, ($canvas) => $canvas.selectedNode);

export const selectedEdge = derived(workflowCanvas, ($canvas) => $canvas.selectedEdge);

export const canvasNodes = derived(workflowCanvas, ($canvas) => $canvas.nodes);

export const canvasEdges = derived(workflowCanvas, ($canvas) => $canvas.edges);
