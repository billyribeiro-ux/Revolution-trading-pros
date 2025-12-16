import{z as Qt,p as Xt,o as Zt,a3 as ea,f as b,a9 as ta,e as a,N as ee,cC as aa,q as t,i as w,w as x,h as u,j as sa,E as na,aN as ra,c as dt,aP as ot,K as ct,B as Te,at as da,r as e,Q as te,g as i,H as ae,x as n,cD as oa,d as Me,O as R,k as ca,n as ia,d0 as la}from"../chunks/C_QN0JMx.js";import{a as va,x as pa,A as ga,b as _a,C as ua,E as ha,d as ba,F as ya}from"../chunks/3r80v9Z5.js";import{g as ma}from"../chunks/CPVBir7Q.js";import{g as xa,s as fa}from"../chunks/Cefv-2g3.js";function qa(){const r=Date.now().toString(36),_=Math.random().toString(36).substring(2,10);return`RCP-${r}-${_}`.toUpperCase()}function ka(r){let _=0;for(let d=0;d<r.length;d++){const s=r.charCodeAt(d);_=(_<<5)-_+s,_=_&_}return Math.abs(_).toString(16).padStart(8,"0")}function it(r,_="Revolution Trading Pros"){const d={receiptId:qa(),generatedAt:new Date().toISOString(),websiteUrl:window.location.origin,websiteName:_,consentId:r.consentId||"N/A",consentDate:r.updatedAt,consentMethod:r.consentMethod||"unknown",expiresAt:r.expiresAt,categories:{necessary:r.necessary,analytics:r.analytics,marketing:r.marketing,preferences:r.preferences},privacySignals:r.privacySignals?{gpc:r.privacySignals.gpc,dnt:r.privacySignals.dnt,region:r.privacySignals.region}:void 0,auditTrail:va().slice(-10),userAgent:navigator.userAgent,schemaVersion:"1.0.0",checksum:""},s=JSON.stringify({consentId:d.consentId,consentDate:d.consentDate,categories:d.categories});return d.checksum=ka(s),d}function wa(r){return JSON.stringify(r,null,2)}function $a(r){const _=wa(r),d=new Blob([_],{type:"application/json"}),s=URL.createObjectURL(d),q=document.createElement("a");q.href=s,q.download=`consent-receipt-${r.receiptId}.json`,document.body.appendChild(q),q.click(),document.body.removeChild(q),URL.revokeObjectURL(s)}function Sa(r){const _=Object.entries(r.categories).map(([s,q])=>`
      <tr>
        <td style="padding: 8px; border-bottom: 1px solid #eee; text-transform: capitalize;">${s}</td>
        <td style="padding: 8px; border-bottom: 1px solid #eee;">
          <span style="color: ${q?"#22c55e":"#ef4444"}; font-weight: bold;">
            ${q?"✓ Enabled":"✗ Disabled"}
          </span>
        </td>
      </tr>
    `).join(""),d=r.auditTrail.map(s=>`
      <tr>
        <td style="padding: 6px; font-size: 12px;">${new Date(s.timestamp).toLocaleString()}</td>
        <td style="padding: 6px; font-size: 12px;">${s.action}</td>
      </tr>
    `).join("");return`
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Consent Receipt - ${r.receiptId}</title>
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      max-width: 800px;
      margin: 0 auto;
      padding: 40px 20px;
      color: #333;
    }
    .header {
      text-align: center;
      border-bottom: 2px solid #0ea5e9;
      padding-bottom: 20px;
      margin-bottom: 30px;
    }
    .header h1 { margin: 0; color: #0ea5e9; }
    .header p { color: #666; margin: 5px 0; }
    .section { margin-bottom: 25px; }
    .section h2 {
      font-size: 16px;
      color: #0ea5e9;
      border-bottom: 1px solid #eee;
      padding-bottom: 8px;
    }
    table { width: 100%; border-collapse: collapse; }
    .info-row td { padding: 8px 0; }
    .info-row td:first-child { font-weight: 500; width: 180px; }
    .footer {
      margin-top: 40px;
      padding-top: 20px;
      border-top: 1px solid #eee;
      font-size: 12px;
      color: #666;
      text-align: center;
    }
    .checksum {
      font-family: monospace;
      background: #f5f5f5;
      padding: 10px;
      border-radius: 4px;
      text-align: center;
    }
    @media print {
      body { padding: 20px; }
    }
  </style>
</head>
<body>
  <div class="header">
    <h1>🔒 Consent Receipt</h1>
    <p>${r.websiteName}</p>
    <p style="font-size: 12px;">Receipt ID: ${r.receiptId}</p>
  </div>

  <div class="section">
    <h2>Consent Details</h2>
    <table class="info-row">
      <tr>
        <td>Consent ID:</td>
        <td><code>${r.consentId}</code></td>
      </tr>
      <tr>
        <td>Date Given:</td>
        <td>${new Date(r.consentDate).toLocaleString()}</td>
      </tr>
      <tr>
        <td>Method:</td>
        <td style="text-transform: capitalize;">${r.consentMethod}</td>
      </tr>
      <tr>
        <td>Expires:</td>
        <td>${r.expiresAt?new Date(r.expiresAt).toLocaleString():"N/A"}</td>
      </tr>
    </table>
  </div>

  <div class="section">
    <h2>Cookie Categories</h2>
    <table>
      ${_}
    </table>
  </div>

  ${r.privacySignals?`
  <div class="section">
    <h2>Privacy Signals Detected</h2>
    <table class="info-row">
      <tr>
        <td>Global Privacy Control:</td>
        <td>${r.privacySignals.gpc?"Yes (Respected)":"No"}</td>
      </tr>
      <tr>
        <td>Do Not Track:</td>
        <td>${r.privacySignals.dnt?"Yes (Respected)":"No"}</td>
      </tr>
      <tr>
        <td>Detected Region:</td>
        <td>${r.privacySignals.region||"Unknown"}</td>
      </tr>
    </table>
  </div>
  `:""}

  ${r.auditTrail.length>0?`
  <div class="section">
    <h2>Consent History</h2>
    <table>
      <thead>
        <tr>
          <th style="text-align: left; padding: 6px;">Date</th>
          <th style="text-align: left; padding: 6px;">Action</th>
        </tr>
      </thead>
      <tbody>
        ${d}
      </tbody>
    </table>
  </div>
  `:""}

  <div class="section">
    <h2>Verification</h2>
    <div class="checksum">
      Checksum: <strong>${r.checksum}</strong>
    </div>
    <p style="font-size: 12px; color: #666; text-align: center; margin-top: 10px;">
      This checksum can be used to verify the integrity of this receipt.
    </p>
  </div>

  <div class="footer">
    <p>Generated on ${new Date(r.generatedAt).toLocaleString()}</p>
    <p>${r.websiteUrl}</p>
    <p>Schema Version: ${r.schemaVersion}</p>
  </div>
</body>
</html>
  `.trim()}function Ca(r){const _=Sa(r),d=window.open("","_blank");d&&(d.document.write(_),d.document.close(),d.focus(),setTimeout(()=>{d.print()},250))}const Da=!1,Ia=!1,Xa=Object.freeze(Object.defineProperty({__proto__:null,prerender:Da,ssr:Ia},Symbol.toStringTag,{value:"Module"}));var Ra=b('<meta name="description"/>'),La=b("<option> </option>"),Pa=b('<button class="btn btn-secondary svelte-qdcndg"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg> </button> <button class="btn btn-secondary svelte-qdcndg"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="6 9 6 2 18 2 18 9"></polyline><path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"></path><rect x="6" y="14" width="12" height="8"></rect></svg> Print Receipt</button>',1),Aa=b('<p class="consent-id svelte-qdcndg"> <code class="svelte-qdcndg"> </code></p>'),Na=b('<section class="section svelte-qdcndg"><h2 class="svelte-qdcndg">Your Current Consent</h2> <div class="consent-status svelte-qdcndg"><div><span class="status-label"> </span> <span class="status-badge enabled svelte-qdcndg"> </span></div> <div><span class="status-label"> </span> <span> </span></div> <div><span class="status-label"> </span> <span> </span></div> <div><span class="status-label"> </span> <span> </span></div></div> <!></section>'),Ta=b('<tr><td class="cookie-name svelte-qdcndg"><code class="svelte-qdcndg"> </code></td><td class="svelte-qdcndg"> </td><td class="svelte-qdcndg"> </td><td class="svelte-qdcndg"><span> </span></td></tr>'),Ma=b('<div class="cookie-category-section svelte-qdcndg"><h3 class="category-title svelte-qdcndg"><span> </span> </h3> <div class="cookie-table-wrapper svelte-qdcndg"><table class="cookie-table svelte-qdcndg"><thead><tr><th class="svelte-qdcndg"> </th><th class="svelte-qdcndg"> </th><th class="svelte-qdcndg"> </th><th class="svelte-qdcndg"> </th></tr></thead><tbody></tbody></table></div></div>'),Ua=b('<div class="cookie-summary svelte-qdcndg"><div class="summary-stat svelte-qdcndg"><span class="stat-value svelte-qdcndg"> </span> <span class="stat-label svelte-qdcndg">Total Cookies</span></div> <div class="summary-stat svelte-qdcndg"><span class="stat-value svelte-qdcndg"> </span> <span class="stat-label svelte-qdcndg">Categorized</span></div> <div class="summary-stat svelte-qdcndg"><span class="stat-value svelte-qdcndg"> </span> <span class="stat-label svelte-qdcndg">Unknown</span></div></div> <!>',1),ja=b('<p class="loading svelte-qdcndg">Loading cookie information...</p>'),za=b('<span class="vendor-locations"> </span>'),Oa=b('<a target="_blank" rel="noopener noreferrer" class="vendor-link svelte-qdcndg">Privacy Policy →</a>'),Ea=b('<div class="vendor-card svelte-qdcndg"><h3 class="svelte-qdcndg"> </h3> <p class="svelte-qdcndg"> </p> <div class="vendor-meta svelte-qdcndg"><span class="vendor-categories"> </span> <!></div> <!></div>'),Va=b('<section class="section svelte-qdcndg"><h2 class="svelte-qdcndg">Third-Party Services</h2> <div class="vendor-grid svelte-qdcndg"></div></section>'),Ba=b('<div class="signal detected svelte-qdcndg"><span class="signal-icon svelte-qdcndg">🛡️</span> <span> </span></div>'),Fa=b('<div class="signal detected svelte-qdcndg"><span class="signal-icon svelte-qdcndg">🚫</span> <span> </span></div>'),Ha=b('<div class="signal svelte-qdcndg"><span class="signal-icon svelte-qdcndg">🌍</span> <span> </span></div>'),Ja=b('<section class="section svelte-qdcndg"><h2 class="svelte-qdcndg">Privacy Signals Detected</h2> <div class="privacy-signals svelte-qdcndg"><!> <!> <!></div></section>'),Ya=b('<div class="cookie-policy svelte-qdcndg"><div class="container svelte-qdcndg"><header class="header svelte-qdcndg"><div class="header-top svelte-qdcndg"><h1 class="svelte-qdcndg"> </h1> <select class="language-select svelte-qdcndg"></select></div> <p class="subtitle svelte-qdcndg"> </p> <p class="last-updated svelte-qdcndg"> </p></header> <section class="quick-actions svelte-qdcndg"><button class="btn btn-primary svelte-qdcndg"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"></path><circle cx="12" cy="12" r="3"></circle></svg> </button> <!></section> <!> <section class="section svelte-qdcndg"><h2 class="svelte-qdcndg"> </h2> <p class="svelte-qdcndg"> </p></section> <section class="section svelte-qdcndg"><h2 class="svelte-qdcndg"> </h2> <div class="category-grid svelte-qdcndg"><div class="category-card necessary svelte-qdcndg"><div class="category-header svelte-qdcndg"><span class="category-icon svelte-qdcndg">🔒</span> <h3 class="svelte-qdcndg"> </h3> <span class="badge required svelte-qdcndg"> </span></div> <p class="svelte-qdcndg"> </p></div> <div class="category-card analytics svelte-qdcndg"><div class="category-header svelte-qdcndg"><span class="category-icon svelte-qdcndg">📊</span> <h3 class="svelte-qdcndg"> </h3> <span class="badge optional svelte-qdcndg"> </span></div> <p class="svelte-qdcndg"> </p></div> <div class="category-card marketing svelte-qdcndg"><div class="category-header svelte-qdcndg"><span class="category-icon svelte-qdcndg">📢</span> <h3 class="svelte-qdcndg"> </h3> <span class="badge optional svelte-qdcndg"> </span></div> <p class="svelte-qdcndg"> </p></div> <div class="category-card preferences svelte-qdcndg"><div class="category-header svelte-qdcndg"><span class="category-icon svelte-qdcndg">⚙️</span> <h3 class="svelte-qdcndg"> </h3> <span class="badge optional svelte-qdcndg"> </span></div> <p class="svelte-qdcndg"> </p></div></div></section> <section class="section svelte-qdcndg"><h2 class="svelte-qdcndg"> </h2> <!></section> <!> <!> <footer class="policy-footer svelte-qdcndg"><p>For questions about our cookie policy, please contact us at <a href="mailto:privacy@revolutiontradingpros.com" class="svelte-qdcndg">privacy@revolutiontradingpros.com</a></p></footer></div></div>');function Za(r,_){Xt(_,!1);const d=()=>Te(ba,"$consentStore",se),s=()=>Te(ha,"$t",se),q=()=>Te(ya,"$currentLanguage",se),[se,lt]=na();let U=ot(null),ne=ot([]),Ue=xa();const vt={en:"English",de:"Deutsch",fr:"Français",es:"Español",it:"Italiano",nl:"Nederlands",pt:"Português",pl:"Polski",sv:"Svenska",da:"Dansk",fi:"Suomi",no:"Norsk",cs:"Čeština",ro:"Română",hu:"Magyar",el:"Ελληνικά",bg:"Български",hr:"Hrvatski",sk:"Slovenčina",sl:"Slovenščina",et:"Eesti",lv:"Latviešu",lt:"Lietuvių",ja:"日本語",zh:"中文",ko:"한국어"};Zt(()=>{dt(U,fa()),_a(),dt(ne,ma())});function pt(){pa()}function gt(){const o=it(d());$a(o)}function _t(){const o=it(d());Ca(o)}function ut(o){const c=o.target;ga(c.value)}const je={necessary:"bg-green-500/20 text-green-400 border-green-500/30",analytics:"bg-blue-500/20 text-blue-400 border-blue-500/30",marketing:"bg-purple-500/20 text-purple-400 border-purple-500/30",preferences:"bg-amber-500/20 text-amber-400 border-amber-500/30",unknown:"bg-gray-500/20 text-gray-400 border-gray-500/30"};ea();var re=Ya();ta("qdcndg",o=>{var c=Ra();x(()=>ct(c,"content",s().cookiePolicyIntro)),ra(()=>{da.title=`${s().cookiePolicyTitle??""} | Revolution Trading Pros`}),u(o,c)});var ze=t(re),de=t(ze),oe=t(de),ce=t(oe),ht=t(ce,!0);e(ce);var L=a(ce,2);L.__change=ut,ee(L,5,()=>ua().slice(0,7),te,(o,c)=>{var l=La(),y=t(l,!0);e(l);var g={};x(()=>{n(y,vt[i(c)]),g!==(g=i(c))&&(l.value=(l.__value=i(c))??"")}),u(o,l)}),e(L);var Oe;aa(L),e(oe);var ie=a(oe,2),bt=t(ie,!0);e(ie);var Ee=a(ie,2),yt=t(Ee);e(Ee),e(de);var le=a(de,2),Q=t(le);Q.__click=pt;var mt=a(t(Q));e(Q);var xt=a(Q,2);{var ft=o=>{var c=Pa(),l=Me(c);l.__click=gt;var y=a(t(l));e(l);var g=a(l,2);g.__click=_t,x(()=>n(y,` ${s().downloadReceipt??""}`)),u(o,c)};w(xt,o=>{d().hasInteracted&&o(ft)})}e(le);var Ve=a(le,2);{var qt=o=>{var c=Na(),l=a(t(c),2),y=t(l);let g;var k=t(y),$=t(k,!0);e(k);var C=a(k,2),D=t(C,!0);e(C),e(y);var v=a(y,2);let p;var h=t(v),S=t(h,!0);e(h);var P=a(h,2);let j;var z=t(P,!0);e(P),e(v);var I=a(v,2);let m;var f=t(I),B=t(f,!0);e(f);var A=a(f,2);let O;var F=t(A,!0);e(A),e(I);var N=a(I,2);let T;var H=t(N),De=t(H,!0);e(H);var E=a(H,2);let J;var Y=t(E,!0);e(E),e(N),e(l);var X=a(l,2);{var G=W=>{var M=Aa(),Z=t(M),V=a(Z),Ie=t(V,!0);e(V),e(M),x(()=>{n(Z,`${s().consentId??""}: `),n(Ie,d().consentId)}),u(W,M)};w(X,W=>{d().consentId&&W(G)})}e(c),x(()=>{g=R(y,1,"status-item svelte-qdcndg",null,g,{enabled:d().necessary}),n($,s().necessary),n(D,s().required),p=R(v,1,"status-item svelte-qdcndg",null,p,{enabled:d().analytics}),n(S,s().analytics),j=R(P,1,"status-badge svelte-qdcndg",null,j,{enabled:d().analytics}),n(z,d().analytics?s().enabled:s().disabled),m=R(I,1,"status-item svelte-qdcndg",null,m,{enabled:d().marketing}),n(B,s().marketing),O=R(A,1,"status-badge svelte-qdcndg",null,O,{enabled:d().marketing}),n(F,d().marketing?s().enabled:s().disabled),T=R(N,1,"status-item svelte-qdcndg",null,T,{enabled:d().preferences}),n(De,s().preferences),J=R(E,1,"status-badge svelte-qdcndg",null,J,{enabled:d().preferences}),n(Y,d().preferences?s().enabled:s().disabled)}),u(o,c)};w(Ve,o=>{d().hasInteracted&&o(qt)})}var ve=a(Ve,2),pe=t(ve),kt=t(pe,!0);e(pe);var Be=a(pe,2),wt=t(Be,!0);e(Be),e(ve);var ge=a(ve,2),_e=t(ge),$t=t(_e,!0);e(_e);var Fe=a(_e,2),ue=t(Fe),he=t(ue),be=a(t(he),2),St=t(be,!0);e(be);var He=a(be,2),Ct=t(He,!0);e(He),e(he);var Je=a(he,2),Dt=t(Je,!0);e(Je),e(ue);var ye=a(ue,2),me=t(ye),xe=a(t(me),2),It=t(xe,!0);e(xe);var Ye=a(xe,2),Rt=t(Ye,!0);e(Ye),e(me);var Ge=a(me,2),Lt=t(Ge,!0);e(Ge),e(ye);var fe=a(ye,2),qe=t(fe),ke=a(t(qe),2),Pt=t(ke,!0);e(ke);var We=a(ke,2),At=t(We,!0);e(We),e(qe);var Ke=a(qe,2),Nt=t(Ke,!0);e(Ke),e(fe);var Qe=a(fe,2),we=t(Qe),$e=a(t(we),2),Tt=t($e,!0);e($e);var Xe=a($e,2),Mt=t(Xe,!0);e(Xe),e(we);var Ze=a(we,2),Ut=t(Ze,!0);e(Ze),e(Qe),e(Fe),e(ge);var Se=a(ge,2),Ce=t(Se),jt=t(Ce,!0);e(Ce);var zt=a(Ce,2);{var Ot=o=>{var c=Ua(),l=Me(c),y=t(l),g=t(y),k=t(g,!0);e(g),ae(2),e(y);var $=a(y,2),C=t($),D=t(C,!0);e(C),ae(2),e($);var v=a($,2),p=t(v),h=t(p,!0);e(p),ae(2),e(v),e(l);var S=a(l,2);ee(S,1,()=>Object.entries(i(U).byCategory),te,(P,j)=>{var z=ia(()=>la(i(j),2));let I=()=>i(z)[0],m=()=>i(z)[1];var f=ca(),B=Me(f);{var A=O=>{var F=Ma(),N=t(F),T=t(N),H=t(T,!0);e(T);var De=a(T);e(N);var E=a(N,2),J=t(E),Y=t(J),X=t(Y),G=t(X),W=t(G,!0);e(G);var M=a(G),Z=t(M,!0);e(M);var V=a(M),Ie=t(V,!0);e(V);var tt=a(V),Ht=t(tt,!0);e(tt),e(X),e(Y);var at=a(Y);ee(at,5,m,te,(Jt,K)=>{var Re=Ta(),Le=t(Re),st=t(Le),Yt=t(st,!0);e(st),e(Le);var Pe=a(Le),Gt=t(Pe,!0);e(Pe);var Ae=a(Pe),Wt=t(Ae,!0);e(Ae);var nt=a(Ae),Ne=t(nt);let rt;var Kt=t(Ne,!0);e(Ne),e(nt),e(Re),x(()=>{n(Yt,i(K).name),n(Gt,i(K).purpose||"Not specified"),n(Wt,i(K).duration||"Session"),rt=R(Ne,1,"type-badge svelte-qdcndg",null,rt,{"first-party":i(K).type==="first-party"}),n(Kt,i(K).type||"Unknown")}),u(Jt,Re)}),e(at),e(J),e(E),e(F),x(()=>{R(T,1,`category-badge ${(je[I()]||je.unknown)??""}`,"svelte-qdcndg"),n(H,I()),n(De,` (${m().length??""} cookies)`),n(W,s().cookieName),n(Z,s().cookiePurpose),n(Ie,s().cookieDuration),n(Ht,s().cookieType)}),u(O,F)};w(B,O=>{m().length>0&&O(A)})}u(P,f)}),x(()=>{n(k,i(U).totalCookies),n(D,i(U).categorizedCookies),n(h,i(U).uncategorizedCookies)}),u(o,c)},Et=o=>{var c=ja();u(o,c)};w(zt,o=>{i(U)?o(Ot):o(Et,!1)})}e(Se);var et=a(Se,2);{var Vt=o=>{var c=Va(),l=a(t(c),2);ee(l,5,()=>i(ne),te,(y,g)=>{var k=Ea(),$=t(k),C=t($,!0);e($);var D=a($,2),v=t(D,!0);e(D);var p=a(D,2),h=t(p),S=t(h);e(h);var P=a(h,2);{var j=m=>{var f=za(),B=t(f);e(f),x(A=>n(B,`Data: ${A??""}`),[()=>i(g).dataLocations.join(", ")]),u(m,f)};w(P,m=>{i(g).dataLocations&&m(j)})}e(p);var z=a(p,2);{var I=m=>{var f=Oa();x(()=>ct(f,"href",i(g).privacyPolicyUrl)),u(m,f)};w(z,m=>{i(g).privacyPolicyUrl&&m(I)})}e(k),x(m=>{n(C,i(g).name),n(v,i(g).description),n(S,`Categories: ${m??""}`)},[()=>i(g).requiredCategories.join(", ")]),u(y,k)}),e(l),e(c),u(o,c)};w(et,o=>{i(ne).length>0&&o(Vt)})}var Bt=a(et,2);{var Ft=o=>{var c=Ja(),l=a(t(c),2),y=t(l);{var g=v=>{var p=Ba(),h=a(t(p),2),S=t(h,!0);e(h),e(p),x(()=>n(S,s().gpcDetected)),u(v,p)};w(y,v=>{d().privacySignals.gpc&&v(g)})}var k=a(y,2);{var $=v=>{var p=Fa(),h=a(t(p),2),S=t(h,!0);e(h),e(p),x(()=>n(S,s().dntDetected)),u(v,p)};w(k,v=>{d().privacySignals.dnt&&v($)})}var C=a(k,2);{var D=v=>{var p=Ha(),h=a(t(p),2),S=t(h);e(h),e(p),x(()=>n(S,`${s().regionDetected??""}: ${d().privacySignals.region??""}`)),u(v,p)};w(C,v=>{d().privacySignals.region&&v(D)})}e(l),e(c),u(o,c)};w(Bt,o=>{d().privacySignals&&o(Ft)})}ae(2),e(ze),e(re),x(()=>{n(ht,s().cookiePolicyTitle),Oe!==(Oe=q())&&(L.value=(L.__value=q())??"",oa(L,q())),n(bt,s().cookiePolicyIntro),n(yt,`${s().lastUpdated??""}: ${Ue.formattedDate??""} (v${Ue.version})`),n(mt,` ${s().cookieSettings??""}`),n(kt,s().whatAreCookies),n(wt,s().whatAreCookiesDescription),n($t,s().howWeUseCookies),n(St,s().necessary),n(Ct,s().required),n(Dt,s().necessaryDescription),n(It,s().analytics),n(Rt,s().optional),n(Lt,s().analyticsDescription),n(Pt,s().marketing),n(At,s().optional),n(Nt,s().marketingDescription),n(Tt,s().preferences),n(Mt,s().optional),n(Ut,s().preferencesDescription),n(jt,s().cookieDeclaration)}),u(r,re),sa(),lt()}Qt(["change","click"]);export{Za as component,Xa as universal};
