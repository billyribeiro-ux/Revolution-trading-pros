

export const index = 4;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/admin/analytics/_layout.svelte.js')).default;
export const imports = ["_app/immutable/nodes/4.BQ1TOXp9.js","_app/immutable/chunks/C_QN0JMx.js"];
export const stylesheets = [];
export const fonts = [];
