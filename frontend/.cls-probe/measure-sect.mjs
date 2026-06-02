import { chromium } from '@playwright/test';
const URL=process.argv[2], W=Number(process.argv[3]||390), H=Number(process.argv[4]||844);
const b=await chromium.launch();const ctx=await b.newContext({viewport:{width:W,height:H},isMobile:W<700});
const p=await ctx.newPage();
await p.goto(URL,{waitUntil:'networkidle',timeout:30000});await p.waitForTimeout(1500);
const snap=()=>p.evaluate(()=>Array.from(document.querySelectorAll('section,footer')).map(s=>({c:(typeof s.className==='string'?s.className:'').split(/\s+/).filter(x=>/section|footer|cta|social/i.test(x)).slice(0,1).join('')||s.tagName.toLowerCase(),h:Math.round(s.getBoundingClientRect().height)})));
const before=await snap();
const ht=await p.evaluate(()=>document.body.scrollHeight);
for(let y=0;y<ht;y+=H*0.5){await p.evaluate(_y=>scrollTo(0,_y),y);await p.waitForTimeout(200);}
await p.waitForTimeout(800);
const after=await snap();
const diffs=before.map((s,i)=>({c:s.c,beforeH:s.h,afterH:after[i]?.h,delta:(after[i]?.h??0)-s.h})).filter(d=>Math.abs(d.delta)>5);
console.log(JSON.stringify(diffs,null,1));
await b.close();
