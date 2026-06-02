import { chromium } from '@playwright/test';
const URL = process.argv[2] || 'http://localhost:5173/';
const W = Number(process.argv[3] || 390), H = Number(process.argv[4] || 844);
const init = () => { window.__cls=0; window.__s=[];
  new PerformanceObserver(l=>{for(const e of l.getEntries()){ if(e.hadRecentInput)continue; window.__cls+=e.value;
    const src=(e.sources||[]).map(s=>{const n=s.node;let d='?';if(n&&n.nodeType===1){d=n.tagName.toLowerCase();if(n.className&&typeof n.className==='string')d+='.'+n.className.trim().split(/\s+/).slice(0,2).join('.');}return d;});
    window.__s.push({v:+e.value.toFixed(4),src});}}).observe({type:'layout-shift',buffered:true}); };
const b=await chromium.launch();
const ctx=await b.newContext({viewport:{width:W,height:H},isMobile:W<700,hasTouch:W<700});
const p=await ctx.newPage(); await p.addInitScript(init);
await p.goto(URL,{waitUntil:'networkidle',timeout:30000}); await p.waitForTimeout(1000);
// slow scroll through the whole page
const height=await p.evaluate(()=>document.body.scrollHeight);
for(let y=0;y<height;y+=H*0.5){ await p.evaluate(_y=>window.scrollTo(0,_y),y); await p.waitForTimeout(250); }
await p.waitForTimeout(800);
const r=await p.evaluate(()=>({cls:window.__cls,s:window.__s}));
const agg={}; for(const sh of r.s){ for(const n of sh.src){ agg[n]=(agg[n]||0)+sh.v; } }
console.log(JSON.stringify({url:URL,viewport:`${W}x${H}`,totalCLS:+r.cls.toFixed(4),shiftCount:r.s.length,
  topSources:Object.entries(agg).sort((a,b2)=>b2[1]-a[1]).slice(0,6).map(([n,v])=>({node:n,cls:+v.toFixed(4)}))},null,2));
await b.close();
