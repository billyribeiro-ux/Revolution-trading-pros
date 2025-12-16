const a="https://revolutiontradingpros.com";function s(e,t){switch(e){case"youtube":return`https://img.youtube.com/vi/${t}/maxresdefault.jpg`;case"vimeo":return`https://vumbnail.com/${t}.jpg`;case"dailymotion":return`https://www.dailymotion.com/thumbnail/video/${t}`;case"ted":return"https://pi.tedcdn.com/r/talkstar-photos.s3.amazonaws.com/uploads/placeholder/placeholder.jpg";case"wistia":return`https://embed-ssl.wistia.com/deliveries/${t}.jpg`;default:return""}}function l(e,t){switch(e){case"youtube":return`https://www.youtube.com/embed/${t}`;case"vimeo":return`https://player.vimeo.com/video/${t}`;case"dailymotion":return`https://www.dailymotion.com/embed/video/${t}`;case"ted":return`https://embed.ted.com/talks/${t}`;case"wistia":return`https://fast.wistia.net/embed/iframe/${t}`;default:return""}}function d(){return[{pageUrl:"/courses/day-trading-masterclass",title:"Day Trading Masterclass - Complete Course Overview",description:"Learn professional day trading strategies from industry experts. This comprehensive course covers technical analysis, risk management, and live trading techniques.",thumbnailUrl:"/images/courses/day-trading-thumbnail.jpg",playerUrl:"https://www.youtube.com/embed/dQw4w9WgXcQ",duration:3600,publicationDate:"2025-10-15T10:00:00Z",familyFriendly:!0,tags:["day trading","trading course","stock market","technical analysis"],category:"Education",platform:"youtube",videoId:"dQw4w9WgXcQ",uploader:"Revolution Trading Pros",uploaderUrl:`${a}/about`},{pageUrl:"/courses/swing-trading-pro",title:"Swing Trading Pro - Master Multi-Day Positions",description:"Master swing trading with our professional course. Learn to identify trends, manage positions overnight, and maximize profits on multi-day trades.",thumbnailUrl:"/images/courses/swing-trading-thumbnail.jpg",playerUrl:"https://player.vimeo.com/video/123456789",duration:5400,publicationDate:"2025-09-20T14:00:00Z",familyFriendly:!0,tags:["swing trading","position trading","trend analysis"],category:"Education",platform:"vimeo",videoId:"123456789",uploader:"Revolution Trading Pros",uploaderUrl:`${a}/about`},{pageUrl:"/courses/options-trading",title:"Options Trading Fundamentals - Calls, Puts & Strategies",description:"Comprehensive options trading education covering calls, puts, spreads, iron condors, and advanced strategies for consistent profits.",thumbnailUrl:"/images/courses/options-thumbnail.jpg",playerUrl:"https://www.youtube.com/embed/OPTIONS123",duration:7200,publicationDate:"2025-11-01T09:00:00Z",familyFriendly:!0,tags:["options trading","calls","puts","spreads","iron condor"],category:"Education",platform:"youtube",videoId:"OPTIONS123",uploader:"Revolution Trading Pros",uploaderUrl:`${a}/about`},{pageUrl:"/live-trading-rooms/day-trading",title:"Live Day Trading Room - Daily Market Analysis",description:"Join our live trading room for real-time market analysis, trade ideas, and interactive Q&A with professional traders.",thumbnailUrl:"/images/live-rooms/day-trading-live.jpg",playerUrl:"https://www.youtube.com/embed/LIVE123",publicationDate:"2025-11-25T13:30:00Z",familyFriendly:!0,tags:["live trading","day trading","market analysis","trading room"],category:"Education",live:!0,platform:"youtube",videoId:"LIVE123",uploader:"Revolution Trading Pros",uploaderUrl:`${a}/about`},{pageUrl:"/blog/weekly-market-recap",title:"Weekly Market Recap - SPY, QQQ, & Key Sectors Analysis",description:"Our weekly video recap covering major index movements, sector rotation, and key trading opportunities for the upcoming week.",thumbnailUrl:"/images/blog/weekly-recap-thumbnail.jpg",playerUrl:"https://www.youtube.com/embed/RECAP123",duration:1800,publicationDate:"2025-11-24T18:00:00Z",familyFriendly:!0,tags:["market recap","SPY","QQQ","stock market","weekly analysis"],category:"News",platform:"youtube",videoId:"RECAP123",uploader:"Revolution Trading Pros",uploaderUrl:`${a}/about`}]}function r(e){return e.replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&apos;")}function p(e){let t=e.thumbnailUrl;if(e.platform&&e.videoId&&!t.startsWith("http")){const n=s(e.platform,e.videoId);n?t=n:t=`${a}${t}`}else t.startsWith("http")||(t=`${a}${t}`);let o=e.playerUrl;!o&&e.platform&&e.videoId&&(o=l(e.platform,e.videoId));let i=`
	<url>
		<loc>${a}${e.pageUrl}</loc>
		<video:video>
			<video:thumbnail_loc>${t}</video:thumbnail_loc>
			<video:title>${r(e.title)}</video:title>
			<video:description>${r(e.description)}</video:description>`;return e.contentUrl&&(i+=`
			<video:content_loc>${e.contentUrl}</video:content_loc>`),o&&(i+=`
			<video:player_loc allow_embed="yes">${o}</video:player_loc>`),e.duration&&(i+=`
			<video:duration>${e.duration}</video:duration>`),e.expirationDate&&(i+=`
			<video:expiration_date>${e.expirationDate}</video:expiration_date>`),e.rating!==void 0&&(i+=`
			<video:rating>${e.rating.toFixed(1)}</video:rating>`),e.viewCount!==void 0&&(i+=`
			<video:view_count>${e.viewCount}</video:view_count>`),e.publicationDate&&(i+=`
			<video:publication_date>${e.publicationDate}</video:publication_date>`),i+=`
			<video:family_friendly>${e.familyFriendly!==!1?"yes":"no"}</video:family_friendly>`,e.tags&&e.tags.length>0&&e.tags.slice(0,32).forEach(n=>{i+=`
			<video:tag>${r(n)}</video:tag>`}),e.category&&(i+=`
			<video:category>${r(e.category)}</video:category>`),e.restrictedCountries&&e.restrictedCountries.length>0?i+=`
			<video:restriction relationship="deny">${e.restrictedCountries.join(" ")}</video:restriction>`:e.allowedCountries&&e.allowedCountries.length>0&&(i+=`
			<video:restriction relationship="allow">${e.allowedCountries.join(" ")}</video:restriction>`),e.requiresSubscription&&(i+=`
			<video:requires_subscription>yes</video:requires_subscription>`),e.live&&(i+=`
			<video:live>yes</video:live>`),e.uploader&&(i+=`
			<video:uploader${e.uploaderUrl?` info="${e.uploaderUrl}"`:""}>${r(e.uploader)}</video:uploader>`),e.platform&&(i+=`
			<video:platform relationship="allow">web mobile tv</video:platform>`),i+=`
		</video:video>
	</url>`,i}function u(){const e=d(),t=e.map(i=>p(i)).join("");return`<?xml version="1.0" encoding="UTF-8"?>
<?xml-stylesheet type="text/xsl" href="/video-sitemap.xsl"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:video="http://www.google.com/schemas/sitemap-video/1.1"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9
                            http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd
                            http://www.google.com/schemas/sitemap-video/1.1
                            http://www.google.com/schemas/sitemap-video/1.1/sitemap-video.xsd">
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->
	<!-- Revolution Trading Pros - Google Video Sitemap -->
	<!-- Generated: ${new Date().toISOString()} -->
	<!-- Total Videos: ${e.length} -->
	<!-- Platforms: YouTube, Vimeo, Dailymotion, TED, Wistia supported -->
	<!-- ═══════════════════════════════════════════════════════════════════════════ -->${t}
</urlset>`}const c=async()=>{const e=u();return new Response(e,{headers:{"Content-Type":"application/xml; charset=utf-8","Cache-Control":"public, max-age=3600, s-maxage=86400, stale-while-revalidate=604800","X-Content-Type-Options":"nosniff","X-Robots-Tag":"noindex"}})},m=!1;export{c as GET,m as prerender};
