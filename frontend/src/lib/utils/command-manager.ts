/**
 * Command Manager - Undo/Redo System
 * ═══════════════════════════════════════════════════════════════════════════
 * Command pattern for reversible operations
 */

export interface Command {
	execute(): void;
	undo(): void;
	redo?(): void;
	description: string;
	timestamp: number;
}

export interface CommandManagerOptions {
	maxHistory?: number;
	onHistoryChange?: (canUndo: boolean, canRedo: boolean) => void;
}

export class CommandManager {
	private history: Command[] = [];
	private currentIndex = -1;
	private maxHistory: number;
	private onHistoryChange?: (canUndo: boolean, canRedo: boolean) => void;
	private isExecuting = false;

	constructor(options: CommandManagerOptions = {}) {
		this.maxHistory = options.maxHistory ?? 100;
		this.onHistoryChange = options.onHistoryChange;
	}

	execute(command: Command): void {
		if (this.isExecuting) return;
		this.isExecuting = true;

		try {
			this.history = this.history.slice(0, this.currentIndex + 1);
			command.timestamp = Date.now();
			this.history.push(command);
			this.currentIndex++;

			if (this.history.length > this.maxHistory) {
				this.history.shift();
				this.currentIndex--;
			}

			command.execute();
			this.notifyChange();
		} finally {
			this.isExecuting = false;
		}
	}

	undo(): boolean {
		if (!this.canUndo() || this.isExecuting) return false;
		this.isExecuting = true;

		try {
			const command = this.history[this.currentIndex];
			command.undo();
			this.currentIndex--;
			this.notifyChange();
			return true;
		} finally {
			this.isExecuting = false;
		}
	}

	redo(): boolean {
		if (!this.canRedo() || this.isExecuting) return false;
		this.isExecuting = true;

		try {
			this.currentIndex++;
			const command = this.history[this.currentIndex];
			(command.redo ?? command.execute)();
			this.notifyChange();
			return true;
		} finally {
			this.isExecuting = false;
		}
	}

	canUndo(): boolean {
		return this.currentIndex >= 0;
	}

	canRedo(): boolean {
		return this.currentIndex < this.history.length - 1;
	}

	clear(): void {
		this.history = [];
		this.currentIndex = -1;
		this.notifyChange();
	}

	getHistory(): { description: string; timestamp: number; isCurrent: boolean }[] {
		return this.history.map((cmd, i) => ({
			description: cmd.description,
			timestamp: cmd.timestamp,
			isCurrent: i === this.currentIndex
		}));
	}

	getUndoDescription(): string | null {
		return this.canUndo() ? this.history[this.currentIndex].description : null;
	}

	getRedoDescription(): string | null {
		return this.canRedo() ? this.history[this.currentIndex + 1].description : null;
	}

	private notifyChange(): void {
		this.onHistoryChange?.(this.canUndo(), this.canRedo());
	}
}

// Block-specific commands
export class UpdateBlockCommand implements Command {
	description: string;
	timestamp = Date.now();

	constructor(
		private blockId: string,
		private oldValue: any,
		private newValue: any,
		private updateFn: (id: string, value: any) => void
	) {
		this.description = `Update block ${blockId}`;
	}

	execute(): void {
		this.updateFn(this.blockId, this.newValue);
	}

	undo(): void {
		this.updateFn(this.blockId, this.oldValue);
	}
}

export class AddBlockCommand implements Command {
	description: string;
	timestamp = Date.now();

	constructor(
		private block: any,
		private index: number,
		private addFn: (block: any, index: number) => void,
		private removeFn: (id: string) => void
	) {
		this.description = `Add ${block.type} block`;
	}

	execute(): void {
		this.addFn(this.block, this.index);
	}

	undo(): void {
		this.removeFn(this.block.id);
	}
}

export class RemoveBlockCommand implements Command {
	description: string;
	timestamp = Date.now();

	constructor(
		private block: any,
		private index: number,
		private addFn: (block: any, index: number) => void,
		private removeFn: (id: string) => void
	) {
		this.description = `Remove ${block.type} block`;
	}

	execute(): void {
		this.removeFn(this.block.id);
	}

	undo(): void {
		this.addFn(this.block, this.index);
	}
}

export class MoveBlockCommand implements Command {
	description: string;
	timestamp = Date.now();

	constructor(
		private blockId: string,
		private fromIndex: number,
		private toIndex: number,
		private moveFn: (id: string, index: number) => void
	) {
		this.description = `Move block`;
	}

	execute(): void {
		this.moveFn(this.blockId, this.toIndex);
	}

	undo(): void {
		this.moveFn(this.blockId, this.fromIndex);
	}
}

export class BatchCommand implements Command {
	description: string;
	timestamp = Date.now();

	constructor(
		private commands: Command[],
		description?: string
	) {
		this.description = description ?? `Batch: ${commands.length} changes`;
	}

	execute(): void {
		this.commands.forEach((cmd) => cmd.execute());
	}

	undo(): void {
		[...this.commands].reverse().forEach((cmd) => cmd.undo());
	}
}

// Singleton
let commandManager: CommandManager | null = null;

export function getCommandManager(options?: CommandManagerOptions): CommandManager {
	if (!commandManager) {
		commandManager = new CommandManager(options);
	}
	return commandManager;
}
