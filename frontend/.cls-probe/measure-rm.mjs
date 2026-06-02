import { chromium } from '@playwright/test';
const URL=process.argv[2], W=Number(process.argv[3]||390), H=Number(process.argv[4]||844), RM=process.argv[5]==='reduce';
const init=()=>{window.__c=0;new PerformanceObserver(l=>{for(const e of l.getEntries()){if(!e.hadRecentInput)window.__c+=e.value;}}).observe({type:'layout-shift',buffered:true});};
const b=await chromium.launch();
const ctx=await b.newContext({viewport:{width:W,height:H},isMobile:W<700,hasTouch:W<700,reducedMotion:RM?'reduce':'no-preference'});
const p=await ctx.newPage();await p.addInitScript(init);
await p.goto(URL,{waitUntil:'networkidle',timeout:30000});await p.waitForTimeout(1000);
const ht=await p.evaluate(()=>document.body.scrollHeight);
for(let y=0;y<ht;y+=H*0.5){await p.evaluate(_y=>scrollTo(0,_y),y);await p.waitForTimeout(250);}
await p.waitForTimeout(600);
console.log('reducedMotion='+RM, 'CLS='+(await p.evaluate(()=>window.__c)).toFixed(4));
await b.close();
