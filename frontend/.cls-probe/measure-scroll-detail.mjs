import { chromium } from '@playwright/test';
const URL=process.argv[2]||'http://localhost:5173/', W=Number(process.argv[3]||390), H=Number(process.argv[4]||844);
const init=()=>{window.__s=[];new PerformanceObserver(l=>{for(const e of l.getEntries()){if(e.hadRecentInput)continue;
 for(const s of (e.sources||[])){const n=s.node;let d='?';if(n&&n.nodeType===1){d=n.tagName.toLowerCase()+(n.id?'#'+n.id:'')+(typeof n.className==='string'?'.'+n.className.trim().split(/\s+/).slice(0,2).join('.'):'');}
 window.__s.push({v:+e.value.toFixed(4),node:d,fromY:s.previousRect?Math.round(s.previousRect.y):null,toY:s.currentRect?Math.round(s.currentRect.y):null,dH:s.currentRect&&s.previousRect?Math.round(s.currentRect.height-s.previousRect.height):null});}}}).observe({type:'layout-shift',buffered:true});};
const b=await chromium.launch();const ctx=await b.newContext({viewport:{width:W,height:H},isMobile:W<700,hasTouch:W<700});
const p=await ctx.newPage();await p.addInitScript(init);
await p.goto(URL,{waitUntil:'networkidle',timeout:30000});await p.waitForTimeout(1000);
const ht=await p.evaluate(()=>document.body.scrollHeight);
for(let y=0;y<ht;y+=H*0.5){await p.evaluate(_y=>scrollTo(0,_y),y);await p.waitForTimeout(250);}
await p.waitForTimeout(600);
const s=await p.evaluate(()=>window.__s);
console.log(JSON.stringify(s.sort((a,b2)=>b2.v-a.v).slice(0,8),null,2));
await b.close();
