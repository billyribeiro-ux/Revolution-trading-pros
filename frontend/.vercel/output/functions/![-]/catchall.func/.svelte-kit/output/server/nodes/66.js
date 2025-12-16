

export const index = 66;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/admin/members/past/_page.svelte.js')).default;
export const universal = {
  "prerender": false,
  "ssr": false,
  "load": null
};
export const universal_id = "src/routes/admin/members/past/+page.ts";
export const imports = ["_app/immutable/nodes/66.BvrQFNPp.js","_app/immutable/chunks/C_QN0JMx.js","_app/immutable/chunks/D3S43Cfl.js","_app/immutable/chunks/ByEkyPua.js","_app/immutable/chunks/CevrrlX8.js"];
export const stylesheets = ["_app/immutable/assets/66.lDEIUvc7.css"];
export const fonts = [];
