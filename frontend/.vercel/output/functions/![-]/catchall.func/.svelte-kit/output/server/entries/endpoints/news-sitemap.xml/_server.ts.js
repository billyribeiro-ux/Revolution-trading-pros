const o="https://revolutiontradingpros.com",a="Revolution Trading Pros";function r(){const e=new Date,t=[{url:"/blog/market-analysis-november-2025",title:"S&P 500 Technical Analysis: Key Levels to Watch This Week",publicationDate:new Date(e.getTime()-7200*1e3).toISOString(),keywords:["S&P 500","technical analysis","stock market","trading"],stockTickers:["SPX","SPY","ES"],genres:["Blog"],imageUrl:"/images/blog/sp500-analysis.jpg",imageTitle:"S&P 500 Technical Analysis Chart",author:"Revolution Trading Team"},{url:"/blog/options-trading-strategies-2025",title:"Best Options Trading Strategies for Volatile Markets",publicationDate:new Date(e.getTime()-480*60*1e3).toISOString(),keywords:["options trading","volatility","trading strategies","VIX"],stockTickers:["VIX","UVXY"],genres:["Blog"],author:"Revolution Trading Team"},{url:"/blog/day-trading-tips-beginners",title:"Essential Day Trading Tips for Beginners: A Complete Guide",publicationDate:new Date(e.getTime()-1440*60*1e3).toISOString(),keywords:["day trading","beginners","trading education","stock market"],genres:["Blog"],author:"Revolution Trading Team"},{url:"/blog/swing-trading-momentum-stocks",title:"How to Identify Momentum Stocks for Swing Trading",publicationDate:new Date(e.getTime()-2160*60*1e3).toISOString(),keywords:["swing trading","momentum stocks","technical analysis"],genres:["Blog"],author:"Revolution Trading Team"}],s=new Date(e.getTime()-2880*60*1e3);return t.filter(i=>new Date(i.publicationDate)>s)}function n(e){return e.replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&apos;")}function g(e){let t=`
	<url>
		<loc>${o}${e.url}</loc>
		<news:news>
			<news:publication>
				<news:name>${n(a)}</news:name>
				<news:language>en</news:language>
			</news:publication>
			<news:publication_date>${e.publicationDate}</news:publication_date>
			<news:title>${n(e.title)}</news:title>`;if(e.keywords&&e.keywords.length>0){const s=e.keywords.slice(0,10).join(", ");t+=`
			<news:keywords>${n(s)}</news:keywords>`}if(e.stockTickers&&e.stockTickers.length>0){const s=e.stockTickers.join(", ");t+=`
			<news:stock_tickers>${n(s)}</news:stock_tickers>`}return e.genres&&e.genres.length>0&&(t+=`
			<news:genres>${e.genres.join(", ")}</news:genres>`),t+=`
		</news:news>`,e.imageUrl&&(t+=`
		<image:image>
			<image:loc>${o}${e.imageUrl}</image:loc>`,e.imageTitle&&(t+=`
			<image:title>${n(e.imageTitle)}</image:title>`),t+=`
		</image:image>`),t+=`
	</url>`,t}function l(){const e=r(),t=e.map(i=>g(i)).join(""),s=new Date().toISOString();return`<?xml version="1.0" encoding="UTF-8"?>
<?xml-stylesheet type="text/xsl" href="/news-sitemap.xsl"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:news="http://www.google.com/schemas/sitemap-news/0.9"
        xmlns:image="http://www.google.com/schemas/sitemap-image/1.1"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9
                            http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd
                            http://www.google.com/schemas/sitemap-news/0.9
                            http://www.google.com/schemas/sitemap-news/0.9/sitemap-news.xsd">
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- ${a} - Google News Sitemap -->
	<!-- Generated: ${s} -->
	<!-- Total Articles: ${e.length} -->
	<!-- Articles within last 48 hours per Google News requirements -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->${t}
</urlset>`}const m=async()=>{const e=l();return new Response(e,{headers:{"Content-Type":"application/xml; charset=utf-8","Cache-Control":"public, max-age=900, s-maxage=1800, stale-while-revalidate=3600","X-Content-Type-Options":"nosniff","X-Robots-Tag":"noindex"}})},c=!1;export{m as GET,c as prerender};
