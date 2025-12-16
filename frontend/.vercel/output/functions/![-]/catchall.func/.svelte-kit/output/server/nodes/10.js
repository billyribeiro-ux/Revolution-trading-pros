

export const index = 10;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/admin/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/10.Db0fQIRO.js","_app/immutable/chunks/C_QN0JMx.js","_app/immutable/chunks/lN6oCIqA.js","_app/immutable/chunks/D3S43Cfl.js","_app/immutable/chunks/ByEkyPua.js","_app/immutable/chunks/KfBlM_pv.js"];
export const stylesheets = ["_app/immutable/assets/10.C-8FIb6P.css"];
export const fonts = [];
