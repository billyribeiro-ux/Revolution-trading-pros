import { writable, derived } from 'svelte/store';
import type { WorkflowNode, WorkflowEdge, WorkflowCanvas } from '$lib/types/workflow';

function createWorkflowStore() {
	const initialState: WorkflowCanvas = {
		nodes: [],
		edges: [],
		zoom: 1,
		pan: { x: 0, y: 0 }
	};
	const { subscribe, set, update } = writable<WorkflowCanvas>(initialState);

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
			update((state) => {
				const newState: WorkflowCanvas = {
					...state,
					nodes: state.nodes.filter((node) => node.id !== nodeId),
					edges: state.edges.filter(
						(edge) => edge.from_node_id !== nodeId && edge.to_node_id !== nodeId
					)
				};
				if (state.selectedNode?.id !== nodeId && state.selectedNode) {
					newState.selectedNode = state.selectedNode;
				}
				return newState;
			});
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
			update((state) => {
				const newState: WorkflowCanvas = {
					...state,
					edges: state.edges.filter((edge) => edge.id !== edgeId)
				};
				if (state.selectedEdge?.id !== edgeId && state.selectedEdge) {
					newState.selectedEdge = state.selectedEdge;
				}
				return newState;
			});
		},

		// Selection
		selectNode: (node?: WorkflowNode) => {
			update((state) => {
				const newState: WorkflowCanvas = { ...state };
				if (node) {
					newState.selectedNode = node;
				} else {
					delete newState.selectedNode;
				}
				delete newState.selectedEdge;
				return newState;
			});
		},

		selectEdge: (edge?: WorkflowEdge) => {
			update((state) => {
				const newState: WorkflowCanvas = { ...state };
				if (edge) {
					newState.selectedEdge = edge;
				} else {
					delete newState.selectedEdge;
				}
				delete newState.selectedNode;
				return newState;
			});
		},

		clearSelection: () => {
			update((state) => {
				const newState: WorkflowCanvas = { ...state };
				delete newState.selectedNode;
				delete newState.selectedEdge;
				return newState;
			});
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
				zoom: 1,
				pan: { x: 0, y: 0 }
			});
		},

		// Reset
		reset: () => {
			set({
				nodes: [],
				edges: [],
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
