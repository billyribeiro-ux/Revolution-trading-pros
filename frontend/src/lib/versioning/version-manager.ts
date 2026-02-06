/**
 * Version Manager
 * ═══════════════════════════════════════════════════════════════════════════
 * Track document versions with diffs for history and restore
 */

import { diff_match_patch } from 'diff-match-patch';
import type { Block } from '$lib/components/cms/blocks/types';

export interface Version {
	id: string;
	timestamp: number;
	blocks: Block[];
	diff?: string;
	author: string;
	authorEmail?: string;
	message: string;
	size: number;
}

export interface VersionComparison {
	html: string;
	addedLines: number;
	removedLines: number;
	changedBlocks: string[];
}

export class VersionManager {
	private dmp = new diff_match_patch();
	private versions: Version[] = [];
	private maxVersions: number;

	constructor(maxVersions = 50) {
		this.maxVersions = maxVersions;
	}

	createVersion(blocks: Block[], author: string, message: string, authorEmail?: string): Version {
		const previous = this.versions[this.versions.length - 1];
		const current = JSON.stringify(blocks, null, 2);

		const version: Version = {
			id: `v_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`,
			timestamp: Date.now(),
			blocks: JSON.parse(JSON.stringify(blocks)),
			author,
			authorEmail,
			message,
			size: new Blob([current]).size
		};

		if (previous) {
			const previousContent = JSON.stringify(previous.blocks, null, 2);
			const diffs = this.dmp.diff_main(previousContent, current);
			this.dmp.diff_cleanupSemantic(diffs);
			version.diff = this.dmp.diff_toDelta(diffs);
		}

		this.versions.push(version);

		if (this.versions.length > this.maxVersions) {
			this.versions.shift();
		}

		return version;
	}

	getVersions(): Version[] {
		return [...this.versions];
	}

	getVersion(id: string): Version | null {
		return this.versions.find((v) => v.id === id) || null;
	}

	getLatestVersion(): Version | null {
		return this.versions[this.versions.length - 1] || null;
	}

	getVersionCount(): number {
		return this.versions.length;
	}

	revertToVersion(id: string): Block[] {
		const version = this.getVersion(id);
		if (!version) throw new Error('Version not found');
		return JSON.parse(JSON.stringify(version.blocks));
	}

	compareVersions(v1Id: string, v2Id: string): VersionComparison {
		const v1 = this.getVersion(v1Id);
		const v2 = this.getVersion(v2Id);

		if (!v1 || !v2) throw new Error('Version not found');

		const content1 = JSON.stringify(v1.blocks, null, 2);
		const content2 = JSON.stringify(v2.blocks, null, 2);

		const diffs = this.dmp.diff_main(content1, content2);
		this.dmp.diff_cleanupSemantic(diffs);

		const html = this.dmp.diff_prettyHtml(diffs);

		let addedLines = 0;
		let removedLines = 0;

		for (const [op, text] of diffs) {
			const lines = text.split('\n').length - 1;
			if (op === 1) addedLines += lines;
			if (op === -1) removedLines += lines;
		}

		const changedBlocks = this.findChangedBlocks(v1.blocks, v2.blocks);

		return {
			html,
			addedLines,
			removedLines,
			changedBlocks
		};
	}

	private findChangedBlocks(blocks1: Block[], blocks2: Block[]): string[] {
		const changed: string[] = [];
		const map1 = new Map(blocks1.map((b) => [b.id, JSON.stringify(b)]));
		const map2 = new Map(blocks2.map((b) => [b.id, JSON.stringify(b)]));

		for (const [id, content] of map1) {
			if (!map2.has(id)) {
				changed.push(`removed:${id}`);
			} else if (map2.get(id) !== content) {
				changed.push(`modified:${id}`);
			}
		}

		for (const id of map2.keys()) {
			if (!map1.has(id)) {
				changed.push(`added:${id}`);
			}
		}

		return changed;
	}

	getVersionHistory(limit = 10): Array<{
		id: string;
		timestamp: number;
		author: string;
		message: string;
		timeAgo: string;
	}> {
		return this.versions
			.slice(-limit)
			.reverse()
			.map((v) => ({
				id: v.id,
				timestamp: v.timestamp,
				author: v.author,
				message: v.message,
				timeAgo: this.getTimeAgo(v.timestamp)
			}));
	}

	private getTimeAgo(timestamp: number): string {
		const seconds = Math.floor((Date.now() - timestamp) / 1000);

		if (seconds < 60) return 'just now';
		if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
		if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;
		if (seconds < 604800) return `${Math.floor(seconds / 86400)}d ago`;

		return new Date(timestamp).toLocaleDateString();
	}

	clear(): void {
		this.versions = [];
	}

	exportVersions(): string {
		return JSON.stringify(this.versions);
	}

	importVersions(data: string): void {
		try {
			const versions = JSON.parse(data);
			if (Array.isArray(versions)) {
				this.versions = versions;
			}
		} catch {
			throw new Error('Invalid version data');
		}
	}

	getTotalSize(): number {
		return this.versions.reduce((sum, v) => sum + v.size, 0);
	}
}

let versionManager: VersionManager | null = null;

export function getVersionManager(): VersionManager {
	if (!versionManager) {
		versionManager = new VersionManager();
	}
	return versionManager;
}
