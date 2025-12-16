import * as universal from '../entries/pages/account/_page.js';

export const index = 8;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/account/_page.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/account/+page.js";
export const imports = ["_app/immutable/nodes/8.cJ1SbZzN.js","_app/immutable/chunks/C_QN0JMx.js","_app/immutable/chunks/D3S43Cfl.js","_app/immutable/chunks/ByEkyPua.js","_app/immutable/chunks/1Dlf-fUy.js"];
export const stylesheets = [];
export const fonts = [];
