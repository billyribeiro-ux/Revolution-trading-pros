

export const index = 2;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/admin/_layout.svelte.js')).default;
export const universal = {
  "prerender": false,
  "ssr": false
};
export const universal_id = "src/routes/admin/+layout.ts";
export const imports = ["_app/immutable/nodes/2.DPNwn-ua.js","_app/immutable/chunks/C_QN0JMx.js","_app/immutable/chunks/D3S43Cfl.js","_app/immutable/chunks/ByEkyPua.js","_app/immutable/chunks/Div_5PT3.js","_app/immutable/chunks/BpAP-whW.js","_app/immutable/chunks/KfBlM_pv.js","_app/immutable/chunks/2U9ZpxTg.js"];
export const stylesheets = ["_app/immutable/assets/OfflineIndicator.uaYfAfTk.css","_app/immutable/assets/TradingRoomShell.ClgvKLB7.css","_app/immutable/assets/2.DGYt0-YM.css"];
export const fonts = [];
