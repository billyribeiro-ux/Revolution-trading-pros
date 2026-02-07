declare module 'd3' {
	export function select(element: any): any;
	export function scaleLinear(): any;
	export function scaleBand(): any;
	export function scaleOrdinal(): any;
	export function scaleSequential(interpolator: any): any;
	export function line<_T>(): any;
	export function area<_T>(): any;
	export function arc<_T>(): any;
	export function pie<_T>(): any;
	export function lineRadial<_T>(): any;
	export function rgb(color: string): any;
	export function interpolateBlues(t: number): string;
	export const curveMonotoneX: any;
	export const curveLinearClosed: any;
}
