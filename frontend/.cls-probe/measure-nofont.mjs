import { chromium } from '@playwright/test';
const URL=process.argv[2], W=Number(process.argv[3]||390), H=Number(process.argv[4]||844), BLOCK=process.argv[5]==='block';
const init=()=>{window.__c=0;window.__t=[];new PerformanceObserver(l=>{for(const e of l.getEntries()){if(!e.hadRecentInput){window.__c+=e.value;window.__t.push(Math.round(e.startTime));}}}).observe({type:'layout-shift',buffered:true});};
const b=await chromium.launch();const ctx=await b.newContext({viewport:{width:W,height:H},isMobile:W<700});
if(BLOCK){await ctx.route('**/*',r=>{const u=r.request().url();if(/\.(woff2?|ttf|otf|eot)(\?|$)/i.test(u)||r.request().resourceType()==='font')return r.abort();return r.continue();});}
const p=await ctx.newPage();await p.addInitScript(init);
await p.goto(URL,{waitUntil:'networkidle',timeout:30000});await p.waitForTimeout(1000);
const ht=await p.evaluate(()=>document.body.scrollHeight);
for(let y=0;y<ht;y+=H*0.5){await p.evaluate(_y=>scrollTo(0,_y),y);await p.waitForTimeout(250);}
await p.waitForTimeout(600);
const r=await p.evaluate(()=>({c:window.__c,t:window.__t}));
console.log('fontsBlocked='+BLOCK,'CLS='+r.c.toFixed(4),'shiftTimes(ms)='+JSON.stringify(r.t));
await b.close();
