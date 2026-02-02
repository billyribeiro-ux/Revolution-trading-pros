declare module 'diff-match-patch' {
	export class diff_match_patch {
		Diff_Timeout: number;
		Diff_EditCost: number;
		Match_Threshold: number;
		Match_Distance: number;
		Patch_DeleteThreshold: number;
		Patch_Margin: number;
		Match_MaxBits: number;

		diff_main(text1: string, text2: string, opt_checklines?: boolean): Diff[];
		diff_commonPrefix(text1: string, text2: string): number;
		diff_commonSuffix(text1: string, text2: string): number;
		diff_cleanupSemantic(diffs: Diff[]): void;
		diff_cleanupSemanticLossless(diffs: Diff[]): void;
		diff_cleanupEfficiency(diffs: Diff[]): void;
		diff_cleanupMerge(diffs: Diff[]): void;
		diff_xIndex(diffs: Diff[], loc: number): number;
		diff_prettyHtml(diffs: Diff[]): string;
		diff_text1(diffs: Diff[]): string;
		diff_text2(diffs: Diff[]): string;
		diff_levenshtein(diffs: Diff[]): number;
		diff_toDelta(diffs: Diff[]): string;
		diff_fromDelta(text1: string, delta: string): Diff[];

		match_main(text: string, pattern: string, loc: number): number;

		patch_make(a: string | Diff[], opt_b?: string | Diff[], opt_c?: string): Patch[];
		patch_deepCopy(patches: Patch[]): Patch[];
		patch_apply(patches: Patch[], text: string): [string, boolean[]];
		patch_addPadding(patches: Patch[]): string;
		patch_splitMax(patches: Patch[]): void;
		patch_toText(patches: Patch[]): string;
		patch_fromText(textline: string): Patch[];
	}

	export type Diff = [number, string];

	export interface Patch {
		diffs: Diff[];
		start1: number | null;
		start2: number | null;
		length1: number;
		length2: number;
	}

	export const DIFF_DELETE: -1;
	export const DIFF_INSERT: 1;
	export const DIFF_EQUAL: 0;
}
