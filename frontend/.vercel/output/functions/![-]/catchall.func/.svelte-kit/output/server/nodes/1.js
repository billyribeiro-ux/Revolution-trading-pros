

export const index = 1;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_error.svelte.js')).default;
export const imports = ["_app/immutable/nodes/1.w-m8hfrI.js","_app/immutable/chunks/C_QN0JMx.js","_app/immutable/chunks/BttL13M3.js"];
export const stylesheets = ["_app/immutable/assets/1.HO3cackX.css"];
export const fonts = [];
