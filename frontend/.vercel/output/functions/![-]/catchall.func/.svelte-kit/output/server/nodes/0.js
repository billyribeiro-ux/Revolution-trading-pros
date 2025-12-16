import * as universal from '../entries/pages/_layout.js';
import * as server from '../entries/pages/_layout.server.ts.js';

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/+layout.js";
export { server };
export const server_id = "src/routes/+layout.server.ts";
export const imports = ["_app/immutable/nodes/0.VEjxNYAz.js","_app/immutable/chunks/C_QN0JMx.js","_app/immutable/chunks/D3S43Cfl.js","_app/immutable/chunks/ByEkyPua.js","_app/immutable/chunks/BGaU0aUD.js","_app/immutable/chunks/BnUILK9F.js","_app/immutable/chunks/2U9ZpxTg.js","_app/immutable/chunks/rZTkdfva.js","_app/immutable/chunks/3r80v9Z5.js","_app/immutable/chunks/CPVBir7Q.js","_app/immutable/chunks/PPVm8Dsz.js"];
export const stylesheets = ["_app/immutable/assets/TradingRoomShell.ClgvKLB7.css","_app/immutable/assets/TemplateEditor.DjG4lWv8.css","_app/immutable/assets/ContentPlaceholder.tz5q3GR-.css","_app/immutable/assets/0.D3EZphZE.css"];
export const fonts = [];
