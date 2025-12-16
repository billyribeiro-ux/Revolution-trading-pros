

export const index = 110;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/blog/_slug_/_page.svelte.js')).default;
export const universal = {
  "prerender": false,
  "ssr": false,
  "load": null
};
export const universal_id = "src/routes/blog/[slug]/+page.ts";
export const imports = ["_app/immutable/nodes/110.DCa04jdj.js","_app/immutable/chunks/C_QN0JMx.js","_app/immutable/chunks/lN6oCIqA.js","_app/immutable/chunks/D3S43Cfl.js","_app/immutable/chunks/ByEkyPua.js","_app/immutable/chunks/BttL13M3.js","_app/immutable/chunks/SRGU3HV7.js"];
export const stylesheets = ["_app/immutable/assets/110.BuMPGfHq.css"];
export const fonts = [];
