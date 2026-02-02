/**
 * Workflow Canvas Store - Svelte 5 Runes
 * @version 2.0.0 - Migrated to Svelte 5 Runes (February 2026)
 */

import type { WorkflowNode, WorkflowEdge, WorkflowCanvas } from '$lib/types/workflow';

// Initial state
const initialState: WorkflowCanvas = {
	nodes: [],
	edges: [],
	zoom: 1,
	pan: { x: 0, y: 0 }
};

// Svelte 5 reactive state
let canvasState = $state<WorkflowCanvas>({ ...initialState });

// Derived values
const selectedNodeValue = $derived(canvasState.selectedNode);
const selectedEdgeValue = $derived(canvasState.selectedEdge);
const canvasNodesValue = $derived(canvasState.nodes);
const canvasEdgesValue = $derived(canvasState.edges);

// Store with actions
export const workflowCanvas = {
	// Getters
	get state() {
		return canvasState;
	},
	get nodes() {
		return canvasNodesValue;
	},
	get edges() {
		return canvasEdgesValue;
	},
	get zoom() {
		return canvasState.zoom;
	},
	get pan() {
		return canvasState.pan;
	},
	get selectedNode() {
		return selectedNodeValue;
	},
	get selectedEdge() {
		return selectedEdgeValue;
	},

	// Node operations
	addNode(node: WorkflowNode) {
		canvasState = {
			...canvasState,
			nodes: [...canvasState.nodes, node]
		};
	},

	updateNode(nodeId: number, updates: Partial<WorkflowNode>) {
		canvasState = {
			...canvasState,
			nodes: canvasState.nodes.map((node) => (node.id === nodeId ? { ...node, ...updates } : node))
		};
	},

	deleteNode(nodeId: number) {
		const newState: WorkflowCanvas = {
			...canvasState,
			nodes: canvasState.nodes.filter((node) => node.id !== nodeId),
			edges: canvasState.edges.filter(
				(edge) => edge.from_node_id !== nodeId && edge.to_node_id !== nodeId
			)
		};
		if (canvasState.selectedNode?.id !== nodeId && canvasState.selectedNode) {
			newState.selectedNode = canvasState.selectedNode;
		}
		canvasState = newState;
	},

	moveNode(nodeId: number, position: { x: number; y: number }) {
		canvasState = {
			...canvasState,
			nodes: canvasState.nodes.map((node) =>
				node.id === nodeId ? { ...node, position_x: position.x, position_y: position.y } : node
			)
		};
	},

	// Edge operations
	addEdge(edge: WorkflowEdge) {
		canvasState = {
			...canvasState,
			edges: [...canvasState.edges, edge]
		};
	},

	updateEdge(edgeId: number, updates: Partial<WorkflowEdge>) {
		canvasState = {
			...canvasState,
			edges: canvasState.edges.map((edge) => (edge.id === edgeId ? { ...edge, ...updates } : edge))
		};
	},

	deleteEdge(edgeId: number) {
		const newState: WorkflowCanvas = {
			...canvasState,
			edges: canvasState.edges.filter((edge) => edge.id !== edgeId)
		};
		if (canvasState.selectedEdge?.id !== edgeId && canvasState.selectedEdge) {
			newState.selectedEdge = canvasState.selectedEdge;
		}
		canvasState = newState;
	},

	// Selection
	selectNode(node?: WorkflowNode) {
		const newState: WorkflowCanvas = { ...canvasState };
		if (node) {
			newState.selectedNode = node;
		} else {
			delete newState.selectedNode;
		}
		delete newState.selectedEdge;
		canvasState = newState;
	},

	selectEdge(edge?: WorkflowEdge) {
		const newState: WorkflowCanvas = { ...canvasState };
		if (edge) {
			newState.selectedEdge = edge;
		} else {
			delete newState.selectedEdge;
		}
		delete newState.selectedNode;
		canvasState = newState;
	},

	clearSelection() {
		const newState: WorkflowCanvas = { ...canvasState };
		delete newState.selectedNode;
		delete newState.selectedEdge;
		canvasState = newState;
	},

	// Canvas operations
	setZoom(zoom: number) {
		canvasState = {
			...canvasState,
			zoom: Math.max(0.1, Math.min(2, zoom))
		};
	},

	setPan(pan: { x: number; y: number }) {
		canvasState = {
			...canvasState,
			pan
		};
	},

	// Load workflow
	loadWorkflow(nodes: WorkflowNode[], edges: WorkflowEdge[]) {
		canvasState = {
			nodes,
			edges,
			zoom: 1,
			pan: { x: 0, y: 0 }
		};
	},

	// Reset
	reset() {
		canvasState = {
			nodes: [],
			edges: [],
			zoom: 1,
			pan: { x: 0, y: 0 }
		};
	}
};

// Legacy exports for backward compatibility
export const selectedNode = {
	get value() {
		return selectedNodeValue;
	}
};

export const selectedEdge = {
	get value() {
		return selectedEdgeValue;
	}
};

export const canvasNodes = {
	get value() {
		return canvasNodesValue;
	}
};

export const canvasEdges = {
	get value() {
		return canvasEdgesValue;
	}
};
