declare module 'd3' {
  export function select(element: any): any;
  export function scaleLinear(): any;
  export function scaleBand(): any;
  export function scaleOrdinal(): any;
  export function scaleSequential(interpolator: any): any;
  export function line<T>(): any;
  export function area<T>(): any;
  export function arc<T>(): any;
  export function pie<T>(): any;
  export function lineRadial<T>(): any;
  export function rgb(color: string): any;
  export function interpolateBlues(t: number): string;
  export const curveMonotoneX: any;
  export const curveLinearClosed: any;
}
