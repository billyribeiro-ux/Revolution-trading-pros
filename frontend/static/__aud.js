window.__AUD=function(){const parse=c=>{const m=c.match(/rgba?\(([^)]+)\)/);if(!m)return null;const p=m[1].split(/[ ,\/]+/).filter(Boolean).map(Number);return {r:p[0],g:p[1],b:p[2],a:p.length>3?p[3]:1}};
const lum=({r,g,b})=>{const f=v=>{v/=255;return v<=.03928?v/12.92:Math.pow((v+.055)/1.055,2.4)};return .2126*f(r)+.7152*f(g)+.0722*f(b)};
const hsl=({r,g,b})=>{r/=255;g/=255;b/=255;const mx=Math.max(r,g,b),mn=Math.min(r,g,b);let h=0,s=0,l=(mx+mn)/2;if(mx!==mn){const d=mx-mn;s=l>.5?d/(2-mx-mn):d/(mx+mn);h=mx===r?(g-b)/d+(g<b?6:0):mx===g?(b-r)/d+2:(r-g)/d+4;h*=60}return {h,s,l}};
const mix=(t,b)=>({r:t.r*t.a+b.r*(1-t.a),g:t.g*t.a+b.g*(1-t.a),b:t.b*t.a+b.b*(1-t.a),a:1});
const effBg=el=>{let e=el,acc=null;while(e){const c=parse(getComputedStyle(e).backgroundColor);if(c&&c.a>0){if(c.a>=.95)return acc?mix(acc,c):c;acc=acc?mix(acc,c):c}e=e.parentElement}const base={r:13,g:17,b:23,a:1};return acc?mix(acc,base):base};
const cr=(a,b)=>{const l1=lum(a),l2=lum(b);return (Math.max(l1,l2)+.05)/(Math.min(l1,l2)+.05)};
const emo=/[\u{1F300}-\u{1FAFF}☀-➿⭐⚡]/u;
const out={light:[],lowc:[],emoji:[],fx:[],hue:[],empty:[]};
const main=document.querySelector('main#main-content')||document.body;
for(const el of main.querySelectorAll('*')){const cs=getComputedStyle(el);if(cs.display==='none'||cs.visibility==='hidden')continue;const r=el.getBoundingClientRect();if(r.width<2||r.height<2)continue;
const name=(typeof el.className==='string'&&el.className?'.'+el.className.split(' ')[0].replace(/svelte-\w+/,''):'')+el.tagName.toLowerCase();
const bg=parse(cs.backgroundColor);const hh=bg?hsl(bg):null;
if(bg&&bg.a>.6&&r.width*r.height>3000&&lum(bg)>.35&&!(hh.s>.6&&hh.h>30&&hh.h<60))out.light.push(name+' '+cs.backgroundColor);
if(bg&&bg.a>.5&&hh.s>.3&&hh.h>235&&hh.h<320&&hh.l>.15&&hh.l<.8&&r.width*r.height>600)out.hue.push(name+' '+cs.backgroundColor);
const bi=cs.backgroundImage;if(bi.includes('gradient')&&!/transparent/.test(bi))out.fx.push(name+' grad');
if(cs.backdropFilter!=='none')out.fx.push(name+' blur');
if(cs.animationName!=='none'&&cs.animationIterationCount!=='1'&&!/spin|shimmer|skeleton|loading|progress|indeterminate/i.test(cs.animationName))out.fx.push(name+' anim:'+cs.animationName);
for(const p of ['::before','::after']){const c=getComputedStyle(el,p);if(c.content!=='none'&&c.content!=='normal'&&(c.backgroundImage.includes('gradient')||c.backdropFilter!=='none'))out.fx.push(name+p+' grad')}
if(el.children.length===0&&!el.textContent.trim()&&bg&&bg.a>.5&&r.width>150&&r.height>10&&!['INPUT','TEXTAREA','SELECT','IMG','SVG','CANVAS','HR','PROGRESS'].includes(el.tagName.toUpperCase())&&!/skeleton|bar|fill|track|progress|divider|spinner|avatar|dot|line|thumb/i.test(el.className))out.empty.push(name+' '+Math.round(r.width)+'x'+Math.round(r.height));
for(const n of el.childNodes){if(n.nodeType===3){const t=n.textContent.trim();if(!t)continue;if(emo.test(t))out.emoji.push(name+' '+t.slice(0,8));const fg=parse(cs.color);if(!fg)continue;const b=effBg(el);const f=fg.a<1?mix(fg,b):fg;const c=cr(f,b);if(c<3&&parseFloat(cs.fontSize)<40&&r.height>8)out.lowc.push(name+' '+c.toFixed(1)+' '+t.slice(0,16));break}}}
const u=a=>[...new Set(a.map(x=>String(x)))];const s=k=>{const a=u(out[k]);return a.length?k+':'+a.length+' ['+a.slice(0,3).join(' | ')+']':''};
return (location.pathname+' => '+(['light','hue','lowc','emoji','fx','empty'].map(s).filter(Boolean).join(' ; ')||'CLEAN')).replace(/rgb\(/g,'(').replace(/[^A-Za-z0-9 .:_()\-\[\]|;,>]/g,'~');};
