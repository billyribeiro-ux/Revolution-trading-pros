

export const index = 27;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/admin/consent/_page.svelte.js')).default;
export const universal = {
  "prerender": false,
  "ssr": false
};
export const universal_id = "src/routes/admin/consent/+page.ts";
export const imports = ["_app/immutable/nodes/27.BtT6Rz_1.js","_app/immutable/chunks/C_QN0JMx.js","_app/immutable/chunks/rZTkdfva.js","_app/immutable/chunks/3r80v9Z5.js","_app/immutable/chunks/CPVBir7Q.js","_app/immutable/chunks/PPVm8Dsz.js","_app/immutable/chunks/Cefv-2g3.js"];
export const stylesheets = ["_app/immutable/assets/TemplateEditor.DjG4lWv8.css","_app/immutable/assets/ContentPlaceholder.tz5q3GR-.css","_app/immutable/assets/27.vHSzdPnc.css"];
export const fonts = [];
