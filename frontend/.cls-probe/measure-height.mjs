import { chromium } from '@playwright/test';
const URL=process.argv[2], W=Number(process.argv[3]||390), H=Number(process.argv[4]||844);
const b=await chromium.launch();const ctx=await b.newContext({viewport:{width:W,height:H},isMobile:W<700});
const p=await ctx.newPage();
await p.goto(URL,{waitUntil:'networkidle',timeout:30000});await p.waitForTimeout(1000);
let prev=await p.evaluate(()=>document.body.scrollHeight); const start=prev;
const ht=prev; const jumps=[];
for(let y=0;y<ht;y+=H*0.5){
  await p.evaluate(_y=>scrollTo(0,_y),y);await p.waitForTimeout(250);
  const cur=await p.evaluate(()=>document.body.scrollHeight);
  if(Math.abs(cur-prev)>10){jumps.push({scrollY:y,delta:cur-prev,newHeight:cur});} prev=cur;
}
// which element changed: snapshot section heights
const sections=await p.evaluate(()=>Array.from(document.querySelectorAll('section,footer')).map(s=>({c:(s.className||'').toString().split(/\s+/).slice(0,2).join('.')||s.tagName,h:Math.round(s.getBoundingClientRect().height)})));
console.log('startHeight',start,'jumps',JSON.stringify(jumps));
await b.close();
