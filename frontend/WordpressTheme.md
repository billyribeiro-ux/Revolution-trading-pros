/\*

Theme Name: Hello Elementor Child

Theme URI: https://github.com/elementor/hello-theme/

Description: Hello Elementor Child is a child theme of Hello Elementor,
created by Elementor team

Author: Elementor Team

Author URI: https://elementor.com/

Template: hello-elementor

Version: 1.0.1

Text Domain: hello-elementor-child

License: GNU General Public License v3 or later.

License URI: https://www.gnu.org/licenses/gpl-3.0.html

Tags: flexible-header, custom-colors, custom-menu, custom-logo,
editor-style, featured-images, rtl-language-support, threaded-comments,
translation-ready

\*/

/\*

Add your custom styles here

\*/

/\*.woocommerce-account
.woocommerce-MyAccount-navigation{background-color:#0f2d41;padding:
20px;}\*/

/\*.woocommerce-MyAccount-navigation.colsp.dashboard-sidebar \> ul \> li
\> ul {

position: absolute;

width: 200px;

left: 80px;

top: 0;

z-index: 100011;

display:none;

}

.woocommerce-MyAccount-navigation.colsp.dashboard-sidebar \> ul \> li \>
ul li{

padding: 5px 0px 0px 30px;

height: auto;

}

ul.account-primary-menu.is-collapsed \> li:hover ul.child {

display: block;

} \*/

/\* Icons CSS \*/

\@font-face {

font-family: \'StIcons\';

src: url(\'./assets/fonts/st-icons.ttf\') format(\'truetype\');

font-weight: normal;

font-style: normal;

}

\[class\*=\" st-icon-\"\],

\[class\^=st-icon-\] {

font-family: \'StIcons\' !important;

speak: none;

font-style: normal;

font-weight: 400;

font-variant: normal;

text-transform: none;

line-height: 1;

-webkit-font-smoothing: antialiased;

-moz-osx-font-smoothing: grayscale;

}

body .st-icon-explosive-swings:before {

font-family: \'StIcons\' !important;

content: \"\\E902\";

font-size: 32px;

line-height: 32px;

}

.st-icon-home:before {

font-family: \'StIcons\' !important;

content: \"\\E925\";

font-size: 32px;

line-height: 32px;

color: #8796A0;

}

body .st-icon-swing-trading:before {

font-family: \'StIcons\' !important;

content: \"\\E915\";

font-size: 32px;

line-height: 32px;

}

body .st-icon-small-accounts:before {

font-family: \'StIcons\' !important;

content: \"\\0E99C2\";

font-size: 32px;

line-height: 32px;

}

body .st-icon-day-trading:before {

font-family: \'StIcons\' !important;

content: \"\\E91D\";

font-size: 32px;

line-height: 32px;

}

.st-icon-dashboard:before {

content: \"\\E956\";

font-size: 24px;

line-height: 24px;

}

.st-icon-learning-center:before {

content: \"\\E94D\";

font-size: 24px;

line-height: 24px;

}

.st-icon-chatroom-archive:before {

content: \"\\E942\";

font-size: 24px;

line-height: 24px;

}

.st-icon-forum:before {

content: \"\\E949\";

font-size: 24px;

line-height: 24px;

}

.account-primary-menu .cstooltip:hover .st-icon-home:before,

.account_primary_menu_active .st-icon-home:before {

color: #fff !important;

}

.mbrship-card_icn ::before {

vertical-align: middle;

}

/\* Icons CSS \*/

.pst-grid-innr a {

color: #fff !important;

display: inline-block;

cursor: pointer;

}

div.margn-atu {

margin: 0 auto;

/\*max-width: 660px !important;\*/

padding: 10px 10px;

flex-direction: row;

background-color: #0A2335;

}

div.main-pstinnr {

background-color: #0a2335;

padding: 25px 25px 0;

}

.cselt-row {

display: flex !important;

flex-flow: row wrap;

align-items: flex-end;

column-gap: 30px;

row-gap: 40px;

}

.cselt-row .container {

width: 32%;

}

.cselt-row .col-sm4 {

-webkit-flex: 0 0 auto;

-ms-flex: 0 0 auto;

flex: 0 0 auto;

margin-right: 1%;

width: 100%;

float: none;

-webkit-flex-grow: 0;

-ms-flex-positive: 0;

flex-grow: 0;

-webkit-flex-shrink: 0;

-ms-flex-negative: 0;

flex-shrink: 0;

display: -ms-flexbox;

}

.membership-cards {

margin-top: 30px;

background: #fff;

border-radius: 5px;

-webkit-box-shadow: 0 5px 30px rgb(0 0 0 / 10%);

box-shadow: 0 5px 30px rgb(0 0 0 / 10%);

}

.membership-cards .membership-card\_\_header {

display: block;

padding: 20px;

color: #333;

font-weight: 700;

white-space: normal;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

.membership-card\_\_actions {

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

font-size: 16px;

border-top: 1px solid #ededed;

-webkit-justify-content: center;

-ms-flex-pack: center;

justify-content: center;

}

.membership-card\_\_actions a {

display: block;

-webkit-flex: 0 0 auto;

-ms-flex: 0 0 auto;

flex: 0 0 auto;

-webkit-flex-basis: 50%;

-ms-flex-preferred-size: 50%;

flex-basis: 50%;

width: 50%;

height: 100%;

padding: 15px;

text-align: center;

}

.membership-cards span.mbrship-card_icn {

display: inline-block;

width: 50px;

height: 50px;

margin-right: 9px;

line-height: 50px;

color: #fff;

text-align: center;

border-radius: 50%;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

box-shadow: 0 10px 20px rgba(9, 132, 174, .25);

background-color: #0984ae;

}

.membership-cards .membership-card\_\_header:hover {

color: #0e6ac4;

}

.membership-cards .membership-card\_\_header:hover span.mbrship-card_icn
{

background-color: #076787;

}

.membership-card\_\_actions a+a {

border-left: 1px solid #ededed;

}

.enter-roomclas .fa-graduation-cap:before {

width: 40px !important;

height: 40px !important

}

.woocommerce button.button.alt,

.woocommerce button.button.alt:hover {

background-color: #152341;

}

.courses .elementor-products-grid .woocommerce
ul.products.elementor-grid li.product {

border: 1px solid rgba(0, 0, 0, .125);

position: relative;

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-flex-flow: column nowrap;

-ms-flex-flow: column nowrap;

flex-flow: column nowrap;

align-items: flex-start;

width: 100%;

height: 100%;

background: #fff;

border-radius: 5px;

-webkit-box-shadow: 0 1px 2px rgb(0 0 0 / 15%);

box-shadow: 0 1px 2px rgb(0 0 0 / 15%);

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

.courses .elementor-products-grid .woocommerce
ul.products.elementor-grid li.product
img.attachment-woocommerce_thumbnail {

height: 270px;

object-fit: cover;

border-radius: 5px 5px 0 0;

}

.courses .woocommerce ul.products.elementor-grid li.product
.woocommerce-loop-product\_\_title {

padding-left: 15px;

padding-right: 15px;

margin-bottom: 20px;

font-weight: 400;

line-height: 30px;

font-style: normal;

}

.courses .woocommerce ul span.price {

text-align: right;

margin-right: 15px;

margin-left: 15px;

}

.courses .woocommerce ul.products li.product .button {

border-radius: 5px;

display: inline-block;

margin-top: -2.2em;

margin-left: 15px;

margin-bottom: 15px;

background-color: #f99e31;

color: #fff;

font-style: normal;

font-size: 14px;

font-family: \"open sans\";

padding: 9px 14px;

font-weight: 600;

line-height: 18px;

}

.courses .woocommerce ul.products li.product .price del {

opacity: 1;

}

.woocommerce .courses-related ul.products li.product a img {

width: 24%;

display: inline-block;

}

.woocommerce div.courses-related ul.products
.woocommerce-loop-product\_\_title {

width: 70%;

display: inline-block;

margin-left: 3%;

vertical-align: top;

}

.woocommerce .courses-related ul.products li.product .price,

.woocommerce .courses-related ul.products li.product .button {

display: none;

}

.courses-related .pdrt-panel {

margin-bottom: 1.5em;

background: #fff;

border-radius: 5px;

-webkit-box-shadow: 0 2px 4px rgb(0 0 0 / 35%);

box-shadow: 0 2px 4px rgb(0 0 0 / 35%);

padding: 2px;

}

#add-cart-btn .elementor-button-text {

flex-grow: 0;

}

input.mc-input {

padding: 0 20px;

display: block;

width: 100%;

border-radius: 4px;

height: 50px;

font-family: Poppins;

font-weight: 500;

line-height: 28px;

text-transform: none;

font-size: 14px;

color: #474747;

}

input.mc-button {

background-color: #22adc9;

color: #ffffff;

border-radius: 50px;

min-width: 200px;

height: 50px;

border: none;

}

.sec-relat-post2 .uael-post\_\_content-wrap {

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-flex-flow: column nowrap;

-ms-flex-flow: column nowrap;

flex-flow: column nowrap;

margin: 0px 0px 15px;

position: absolute;

bottom: 0;

flex-wrap: nowrap;

flex-direction: row;

}

.sec-relat-post .uael-post\_\_inner-wrap,

.sec-relat-post2 .uael-post\_\_inner-wrap {

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-flex-direction: column;

-ms-flex-direction: column;

flex-direction: column;

height: 100%;

position: relative;

}

.sec-relat-post .uael-post\_\_content-wrap {

border: 1px solid rgba(0, 0, 0, .125);

border-radius: 25px;

background-color: #fff;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-flex-flow: column nowrap;

-ms-flex-flow: column nowrap;

flex-flow: column nowrap;

justify-content: space-between;

height: 100%;

margin: 0px 1.6rem 15px;

position: relative;

top: -30px;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

.sec-relat-post .uael-post\_\_content-wrap:hover {

-webkit-box-shadow: 0 5px 20px rgba(0, 0, 0, .1);

box-shadow: 0 5px 20px rgba(0, 0, 0, .1);

}

.uael-posts-thumbnail-ratio.sec-relat-post .uael-post\_\_bg-wrap,

.uael-posts-thumbnail-ratio.sec-relat-post2 .uael-post\_\_bg-wrap {

overflow: unset;

}

.uael-posts-thumbnail-ratio.sec-relat-post2 .uael-post\_\_thumbnail {

position: relative;

background-color: #0000006b;

border-radius: 25px;

}

.uael-posts-thumbnail-ratio.sec-relat-post .uael-post\_\_thumbnail img,

.uael-posts-thumbnail-ratio.sec-relat-post2 .uael-post\_\_thumbnail img
{

border-radius: 25px;

}

.uael-posts-thumbnail-ratio.sec-relat-post2 .uael-post\_\_thumbnail img
{

position: absolute;

z-index: -9999;

}

.trader_name i {

color: #707070;

font-size: 15px;

}

.lncntr-exrpt p {

font-size: 14px;

color: #999;

line-height: 1.5;

}

.traders .pst_titl {

color: #707070;

font-weight: 700;

font-size: 32px;

}

.traders .tradrpost {

font-family: \"Open Sans\", sans-serif;

font-weight: 400;

font-size: 20px;

font-style: italic;

}

.traders .lnsprt_box {

margin-top: 10px;

margin-bottom: 20px;

}

.traders .ln_seprt {

border-top-width: 1px;

border-top-style: solid;

border-top-color: #cccccc;

max-width: 80%;

margin: 0 0 0 0;

}

.traders .lncntr-exrpt p {

color: #414141;

font-size: 18px;

line-height: 28px;

}

.cstooltip {

position: relative;

}

body .cstooltip .cstooltiptext {

background-color: #fff;

color: #0984ae;

transform: translate(5px);

transition: all .15s ease-in-out;

border-radius: 6px;

z-index: 2;

box-shadow: 0 10px 30px rgba(0, 0, 0, .15);

left: 50px !important;

top: 15px;

height: 30px;

visibility: hidden;

line-height: 30px;

}

.trdrm-grid-styl .elementor-widget-container {

margin-left: calc(-10px/2);

margin-right: calc(-10px/2);

}

.trdrm-grid-styl .elementor-widget-container {

-ms-flex-wrap: wrap;

-webkit-flex-wrap: wrap;

flex-wrap: wrap;

}

.trdrm-grid-styl .elementor-widget-container {

-js-display: flex;

display: -webkit-box;

display: -webkit-flex;

display: -moz-box;

display: -ms-flexbox;

display: flex;

}

.trdrm-grid-styl .colnm-post {

padding-right: calc(10px/2);

padding-left: calc(10px/2);

margin-bottom: 30px;

}

.trdrm-grid-styl .colnm-post {

text-align: left;

}

.trdrm-grid-styl .colnm-post:nth-child(3n+1):not(.slick-slide) {

clear: left;

}

.trdrm-grid-styl .colnm-post {

width: 33.2%;

float: left;

display: inline-block;

}

\@media screen and (max-width: 1024px) {

.trdrm-grid-styl .colnm-post {

width: 50%;

}

}

\@media screen and (max-width: 430px) {

.trdrm-grid-styl .colnm-post {

width: 100%;

}

}

\@media screen and (max-width: 320px) {

.woocommerce-account div.wc-content-sction {

width: 100%;

}

}

.main-cpost .pst-grid {

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-justify-content: space-between;

-ms-flex-pack: justify;

justify-content: space-between;

-webkit-flex-wrap: wrap;

-ms-flex-wrap: wrap;

flex-wrap: wrap;

background-color: #0a2335;

padding: 25px;

gap: 15px;

}

.pst-grid .pst-grid-innr {

display: inline-block;

z-index: 1;

position: relative;

width: 100%;

min-height: 45vw;

}

div.main-pstinnr\>div {

width: 100% !important;

}

\@media screen and (min-width: 468px) {

.pst-grid .pst-grid-innr {

width: 48%;

min-height: 24vw;

}

}

\@media screen and (min-width: 768px) {

.pst-grid .pst-grid-innr {

width: 31%;

min-height: 16vw;

}

}

\@media screen and (min-width: 1175px) {

.pst-grid .pst-grid-innr {

min-height: auto;

height: 185px;

}

}

.colnm-post img {

border-radius: 5px 5px 0px 0px;

height: 210px !important;

width: 100%;

object-fit: cover;

object-position: top;

}

.pst_title {

margin-top: 10px;

margin-bottom: 0;

}

.colnm-post .pst-img {

margin: -15px -15px 0;

border-radius: 5px;

}

.colnm-post .pst_title a {

color: #666;

font-weight: 700;

font-size: 17px;

}

.cstooltip:hover .cstooltiptext {

visibility: visible;

}

.account-primary-menu.is-collapsed li ul li {

margin-top: 0;

}

.woocommerce-account.woocommerce-page #loading {

position: fixed;

top: 0;

color: #fff;

width: 100%;

height: 100vh;

background: #343436 url(\"img/elite-trading-llive-preloader.png\")
no-repeat center center;

z-index: 100012;

padding-top: 3%;

text-align: center;

}

.csdashboard\_\_toggle {

background-color: #0d2532;

bottom: 0;

height: 50px;

left: 0;

line-height: 50px;

padding: 0;

position: fixed;

right: 0;

z-index: 100010;

display: none;

}

.dashboard\_\_toggle-button {

-webkit-appearance: none;

-moz-appearance: none;

appearance: none;

background: none;

color: #fff !important;

height: 50px;

overflow: hidden;

padding: 0 10px 0 50px;

position: relative;

border-radius: 10px;

border: 1px solid !important;

}

button.dashboard\_\_toggle-button,

button.dashboard\_\_toggle-button {

border: none;

outline: none;

}

button.dashboard\_\_toggle-button:focus,

button.dashboard\_\_toggle-button:hover {

background-color: transparent;

}

.framework\_\_toggle-button-label {

font-size: 12px;

position: relative;

text-transform: uppercase;

top: 0px;

}

.dashboard\_\_toggle-button-icon {

height: 50px;

left: 20px;

position: absolute;

top: 40%;

margin-top: -7px;

width: 50px;

}

.dashboard\_\_toggle-button-icon span {

background-color: #fff;

border-radius: 0;

display: block;

height: 2px;

left: 0;

opacity: 1;

position: absolute;

-webkit-transform: rotate(0);

-ms-transform: rotate(0);

transform: rotate(0);

-webkit-transform-origin: left center;

-ms-transform-origin: left center;

transform-origin: left center;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

width: 20px;

}

.dashboard\_\_toggle-button-icon span:first-child {

top: 0;

}

.dashboard\_\_toggle-button-icon span:nth-child(2) {

top: 6px;

}

.dashboard\_\_toggle-button-icon span:nth-child(3) {

top: 12px;

}

.csdashboard\_\_overlay {

background-color: rgba(0, 0, 0, .65);

bottom: 0;

left: 0;

opacity: 0;

position: fixed;

right: 0;

top: 0;

-webkit-transition: all .3s ease-in-out;

-o-transition: all .3s ease-in-out;

transition: all .3s ease-in-out;

visibility: hidden;

z-index: 100009;

}

.html\--dashboard-menu-open {

overflow: hidden;

}

.html\--dashboard-menu-open .csdashboard\_\_overlay {

opacity: 1;

visibility: visible;

}

\@media screen and (max-width: 980px) {

.woocommerce-account.woocommerce-page.logged-in .csdashboard\_\_toggle {

display: block;

}

}

\@media screen and (max-width: 980px) {

.woocommerce-account
nav.woocommerce-MyAccount-navigation.dashboard-sidebar {

width: 30% !important;

bottom: 50px;

left: 0;

opacity: 0;

overflow-x: hidden;

overflow-y: auto;

position: fixed;

top: 0;

-webkit-transition: all .3s ease-in-out;

-o-transition: all .3s ease-in-out;

transition: all .3s ease-in-out;

visibility: hidden;

z-index: 1000011;

}

.html\--dashboard-menu-open .woocommerce-account
nav.woocommerce-MyAccount-navigation,

.html\--dashboard-menu-open nav.woocommerce-MyAccount-navigation
.account-secondry-menu {

opacity: 1;

visibility: visible;

width: 280px !important;

}

.woocommerce-account div.woocommerce-MyAccount-content {

float: none !important;

width: 100% !important;

}

}

/\* Smartphones (portrait and landscape) \-\-\-\-\-\-\-\-\-\-- \*/

\@media screen and (min-device-width : 451px) and (max-device-width :
670px) {

div.cselt-row div.col-sm4 {

width: 49%;

}

}

\@media screen and (max-device-width: 450px) {

div.cselt-row div.col-sm4 {

width: 100%;

}

}

\@media only screen and (min-device-width : 320px) and (max-device-width
: 980px) {

.woocommerce-account div.wc-content-sction {

width: 100%;

}

.woocommerce-account
nav.woocommerce-MyAccount-navigation.dashboard-sidebar {

width: 70% !important;

bottom: 50px;

left: 0;

opacity: 0;

overflow-x: hidden;

overflow-y: auto;

position: fixed;

top: 0;

-webkit-transition: all .3s ease-in-out;

-o-transition: all .3s ease-in-out;

transition: all .3s ease-in-out;

visibility: hidden;

z-index: 1000011;

}

.html\--dashboard-menu-open .woocommerce-account
nav.woocommerce-MyAccount-navigation,

.html\--dashboard-menu-open nav.woocommerce-MyAccount-navigation
.account-secondry-menu {

opacity: 1;

visibility: visible;

width: 280px !important;

}

.woocommerce-account div.woocommerce-MyAccount-content {

float: none !important;

width: 100% !important;

}

}

/\* \-\-\-\-\-\-\-\-\-\-\-\-\-\--Smartphones (portrait and
landscape)\--End\-\-\-\-\-\-\-\-\-\-- \*/

/\* ul{list-style-type:none;} \*/

ul.accordion ul {

display: none;

padding-left: 20px;

}

.woocommerce-account .woocommerce {

display: flex;

margin: 0 auto;

padding: 50px 15px;

width: 530px;

}

ul.children {

padding: 0;

}

.account-primary-menu .children a {

height: auto;

padding: 0 0 5px 20%;

}

.woocommerce-account .woocommerce.login_container {

display: block;

}

.woocommerce-account .woocommerce h2 {

/\*color: #fff;\*/

font-weight: 700;

font-size: 1.2em;

font-family: Open Sans;

/\* text-align: center; \*/

text-transform: uppercase;

}

.woocommerce-account nav.woocommerce-MyAccount-navigation {

width: 20%;

/\* max-width:360px; \*/

background-color: #0f2d41;

}

\@media Screen and (min-width:980px) and (max-width:1160px) {

.woocommerce-account div nav.woocommerce-MyAccount-navigation {

width: 25%;

}

.woocommerce-account div div.woocommerce-MyAccount-content {

width: 100%;

}

}

.woocommerce-account nav.woocommerce-MyAccount-navigation.colsp {

width: 27%;

}

\@media screen and (min-width: 1520px) {

.woocommerce-account nav.woocommerce-MyAccount-navigation.colsp {

width: 20% !important;

}

.woocommerce-account .wc-content-sction {

width: 100% !important;

}

}

.woocommerce-account .wc-content-sction {

width: 100%;

margin: auto;

margin-top: 10px;

}

.woocommerce-account div.woocommerce-MyAccount-content {

padding: 20px;

width: 100%;

background: #f4f4f4;

position: relative;

}

.woocommerce-account div.wc-accontent-inner {

padding: 4% 2%;

background: #fff;

border-radius: 5px;

-webkit-box-shadow: 0 1px 2px rgb(0 0 0 / 15%);

box-shadow: 0 1px 2px rgb(0 0 0 / 15%);

position: relative;

}

.wc-content-sction .dashb_headr {

background-color: #fff;

border-bottom: 1px solid #dbdbdb;

max-width: 100%;

padding: 20px;

}

.dashb_headr-left,

.dashb_headr-right {

-webkit-align-items: center;

-ms-flex-align: center;

align-items: center;

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-flex-direction: column;

-ms-flex-direction: column;

flex-direction: column;

}

.dashb_headr-right {

-webkit-flex-direction: column-reverse;

-ms-flex-direction: column-reverse;

flex-direction: column-reverse;

margin-top: 10px;

}

.dashb_headr .btn.btn-xs {

font-size: 14px;

line-height: 18px;

padding: 8px 14px;

font-weight: 600;

display: inline-block;

text-align: center;

text-decoration: none;

vertical-align: middle;

cursor: pointer;

-webkit-user-select: none;

-moz-user-select: none;

-ms-user-select: none;

user-select: none;

font-weight: 700;

}

.dashb_headr .btn-link {

color: #0984ae !important;

/\* font-weight: 400!important; \*/

}

.dashb_headr .btn-link,

.dashb_headr .btn-link:focus,

.dashb_headr .btn-link:hover {

background: transparent;

border-color: transparent;

text-decoration: none;

}

.dashb_headr-left .dashb_pg-titl {

color: #333;

font-family: \'Open Sans Condensed Bold\", sans-serif;

font-size: 36px;

font-weight: 700;

margin: 0;

}

\@media screen and (min-width: 577px) {

.dashb_headr-left,

.dashb_headr-right {

-webkit-flex-direction: row;

-ms-flex-direction: row;

flex-direction: row;

}

.dashb_headr-right {

-webkit-flex-direction: row-reverse;

-ms-flex-direction: row-reverse;

flex-direction: row-reverse;

-webkit-justify-content: flex-end;

-ms-flex-pack: end;

justify-content: flex-end;

}

}

\@media screen and (min-width: 820px) {

.dashb_headr {

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-flex-wrap: wrap;

-ms-flex-wrap: wrap;

flex-wrap: wrap;

-webkit-justify-content: space-between;

-ms-flex-pack: justify;

justify-content: space-between;

}

.dashb_headr .dashb_headr-right {

-webkit-flex-direction: row;

-ms-flex-direction: row;

flex-direction: row;

-webkit-justify-content: flex-start;

-ms-flex-pack: start;

justify-content: flex-start;

margin-top: 0;

}

}

\@media screen and (min-width: 1280px) {

.dashboard\_\_header {

padding: 30px;

}

}

.woocommerce-account .catgory-filter form#term_filter {

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

overflow-x: scroll;

-webkit-align-items: center;

-ms-flex-align: center;

align-items: center;

background: #fff;

padding: 20px;

-webkit-box-shadow: 0 3px 6px #00000029;

box-shadow: 0 3px 6px #00000029;

flex-wrap: nowrap;

gap: 15px;

}

.et-pstfilter-loadr {

background: url(\"img/et-postflt-loader.png\") no-repeat center;

display: none;

width: 100%;

height: 95%;

position: absolute;

/\* top: 50%; \*/

/\* right: 50%; \*/

z-index: 9999;

background-color: #fff;

}

.woocommerce-account #term_filter .reset_filter,

.woocommerce-account #term_filter .filter_btn {

margin-right: 10px;

}

#term_filter .filter_btn input {

display: none;

}

#term_filter .filter_btn label {

cursor: pointer;

padding: 11px 32px;

border-radius: 25px;

border: 2px solid;

white-space: nowrap;

font-weight: 700;

color: #666;

display: block;

margin-bottom: 0;

}

#term_filter .filter_btn input:checked+label {

background: #333;

color: #fff;

}

li.woocommerce-MyAccount-navigation-link {

font-family: \"FontAwesome\";

font-weight: 600;

}

.woocommerce-MyAccount-navigation-link.woocommerce-MyAccount-navigation-link\--dashboard\>a:before
{

content: \"\\f0e4\";

font-family: \"FontAwesome\";

font-weight: 400;

font-size: 32px;

}

.woocommerce-MyAccount-navigation-link.woocommerce-MyAccount-navigation-link\--join\>a:before
{

content: \"\\f055\";

font-family: \"FontAwesome\";

font-weight: 400;

font-size: 32px

}

.woocommerce-account div.woocommerce-MyAccount-content.tmp-join {

/\* width: 79%; \*/

}

.wc-content-sction.trd-spreadsht {

width: 73%;

}

.account-primary-menu {

display: block;

bottom: auto;

left: auto;

opacity: 1;

overflow: visible;

position: static;

top: auto;

z-index: auto;

padding: 0 0 30px 0;

font-size: 16px;

width: 100%;

}

\@media screen and (min-width: 980px) {

.account-primary-menu {

visibility: visible;

}

}

\@media screen and (max-width: 980px) {

.account-primary-menu {

overflow-x: hidden;

overflow-y: scroll;

}

}

.account-primary-menu a {

color: hsla(0, 0%, 100%, .5);

min-height: 40px;

/\* padding: 0 20px 0 80px; \*/

display: flex;

-ms-flex-align: center;

font-weight: 300;

margin-bottom: 10px;

}

.account-primary-menu.is-collapsed {

width: 80px;

padding: 30px 0 30px 10px !important

}

.account-primary-menu.is-collapsed a.dashboard-profile-nav-item {

height: 50px;

line-height: 50px;

}

.account-primary-menu.is-collapsed a {

padding: 5px 0 0;

}

.account-primary-menu a,

.account-primary-secondary a {

position: relative;

display: block;

}

.account-primary-menu .dashboard-profile-name {

display: block;

color: #fff;

}

.account-primary-menu a.dashboard-profile-menu-item {

/\* display: flex;

align-items: center; \*/

height: 40px;

line-height: 40px;

margin-top: 20px;

margin-bottom: 20px;

}

.account-primary-menu li,

.account-secondry-menu li {

position: relative;

}

.account-primary-menu .dashboard-menu-category {

font-weight: 700;

margin-bottom: 5px;

color: #fff;

text-transform: uppercase;

}

.account-primary-menu .dashboard-profile-photo img {

position: absolute;

top: 50%;

left: 30px;

margin-top: -17px;

width: 34px;

height: 34px;

border: 2px solid #fff;

border-radius: 50%;

background: no-repeat 50%;

background-size: 32px;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

-moz-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

.account-primary-menu .dashboard-menu-item-icon {

/\* position: absolute;

left: 30px;

width: 32px;

height: 25px;

font-size: 25px; \*/

line-height: 32px;

}

.account-primary-menu.is-collapsed .dashboard-profile-nav-item,

.account-primary-menu.is-collapsed .dashboard-profile-photo {

left: 50%;

margin-left: -16px;

-webkit-transform: scale(1);

-ms-transform: scale(1);

transform: scale(1);

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

.account-primary-menu.is-collapsed .dashboard-profile-photo img {

left: 20px;

}

.account-primary-menu.is-collapsed .dashboard-menu-item-icon {

left: 23px;

}

.account-primary-menu.is-collapsed li {

margin-top: 10px;

}

.fa-graduation-cap:before {}

.account-primary-menu.is-collapsed a:before {

display: none !important;

}

a span.icon-candle-stick {

background: url(img/candle-stick.png) no-repeat;

height: 21px !important;

width: 34px !important;

background-size: contain;

display: inline-block;

}

a span.icon-candle-stick:hover {

background: url(\"img/candle-stick-hover.png\") no-repeat contain;

height: 21px !important;

width: 34px !important;

background-size: contain;

display: inline-block;

}

/\* a span.icon-wc-pkgdashboard {

background: url(img/icon-dashboard.png) no-repeat;

height: 24px !important;

width: 22px !important;

background-size: contain;

display: inline-block;

}

a span.icon-wc-traders {

background: url(img/icon-traders.png) no-repeat;

height: 24px !important;

width: 24px !important;

background-size: contain;

display: inline-block;

}

a span.icon-wc-archive {

background: url(img/icon-archive.png) no-repeat;

height: 24px !important;

width: 24px !important;

background-size: contain;

display: inline-block;

}

a span.icon-wc-ettradesheet {

background: url(img/icon-ettradesheet.png) no-repeat;

height: 24px !important;

width: 24px !important;

background-size: contain;

display: inline-block;

} \*/

.dashboard-sidebar {

display: flex;

-webkit-flex: 0 0 auto;

-ms-flex: 0 0 auto;

flex: 0 0 auto;

-webkit-flex-flow: row no-wrap;

-ms-flex-flow: row no-wrap;

flex-flow: row;

}

.account-secondry-menu {

width: 80%;

font-size: 14px;

font-weight: 600;

background-color: #153e59;

padding-top: 15px;

padding-left: 20px;

-webkit-transition: all .3s ease-in-out;

-o-transition: all .3s ease-in-out;

transition: all .3s ease-in-out;

z-index: auto;

}

\@media screen and (max-width: 980px) {

.account-secondry-menu {

visibility: hidden;

bottom: 50px;

left: 80px;

opacity: 0;

overflow-x: hidden;

overflow-y: auto;

padding-top: 15px;

position: fixed;

top: 0;

width: 240px;

}

}

.account-secondry-menu\>ul {

padding: 20px 15px;

}

/\*span.dashboard-menu-item-icon {

position: absolute;

left: 0;

} \*/

.account-secondry-menu li\>a,

.account-secondry-menu li\>span {

cursor: pointer;

display: inline-flex;

padding: 16px 15px 15px 0;

color: hsla(0, 0%, 100%, .75);

border-radius: 5px;

background-color: transparent;

gap: 10px;

}

.u\--margin-bottom-20 {

margin-bottom: 20px !important;

}

.card-grid {

margin-bottom: 30px;

}

.flex-grid {

display: -webkit-flex !important;

display: -ms-flexbox !important;

display: flex !important;

-webkit-flex-flow: row wrap;

-ms-flex-flow: row wrap;

flex-flow: row wrap;

}

.flex-grid-item {

float: none;

-webkit-flex-grow: 0;

-ms-flex-positive: 0;

flex-grow: 0;

-webkit-flex-shrink: 0;

-ms-flex-negative: 0;

flex-shrink: 0;

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

padding: 0 12px;

}

.flex-grid-panel {

height: 100%;

margin-bottom: 0 !important;

}

.card {

position: relative;

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-flex-flow: column nowrap;

-ms-flex-flow: column nowrap;

flex-flow: column nowrap;

width: 100%;

height: 100%;

background: #fff;

border-radius: 5px;

min-width: 0;

word-wrap: break-word;

-webkit-box-shadow: 0 1px 2px rgb(0 0 0 / 15%);

box-shadow: 0 1px 2px rgb(0 0 0 / 15%);

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

.card-body:last-of-type {

-webkit-flex-grow: 1;

-ms-flex-positive: 1;

flex-grow: 1;

}

section.flex-grid.mod-flex-content-between {

-webkit-justify-content: space-between;

-ms-flex-pack: justify;

justify-content: space-between;

}

.card-body {

-webkit-flex: 1 1 auto;

-ms-flex: 1 1 auto;

flex: 1 1 auto;

padding: 15px;

}

.card-title.h5 {

font-size: 22px;

}

section.flex-grid-column {

-webkit-flex-direction: column;

-ms-flex-direction: column;

flex-direction: column;

}

.btn.btn-tiny {

font-size: 14px;

line-height: 16px;

padding: 6px 8px;

font-weight: 600;

background: #f4f4f4;

border-color: transparent;

}

.card-title {

width: 100%;

border-top-left-radius: 13px;

border-top-right-radius: 13px;

padding: 0.5em;

font-family: Open Sans Condensed;

font-size: 2em;

}

.btn.btn-default,

.btn.btn-default:visited {

color: #0984ae;

}

.card-body h4 strong {

font-family: \"Open Sans\", sans-serif;

font-size: 20px;

color: #666;

}

\@media (min-width: 1200px) {

.col-xl-3 {

width: 25%;

}

}

\@media (min-width: 768px) {

.col-md-4 {

width: 33.33333%;

}

}

ul.enter-roomclas,

ul.enter-roomclas ul {

padding: 0;

margin: 0;

list-style-type: none;

width: 280px;

}

ul.enter-roomclas\>li\>a {

color: #fff;

background: #152341;

border-radius: 5px;

display: inline-block;

font-size: 14px;

text-align: center;

padding: 12px 0px;

font-weight: 600;

width: 242px;

width: 100%;

line-height: 18px;

font-family: \'Open Sans\';

}

ul.enter-roomclas\>li ul {

background: #fff;

display: none;

position: absolute;

width: 100%;

z-index: 99999;

-webkit-box-shadow: 0 10px 30px rgb(0 0 0 / 15%);

box-shadow: 0 10px 30px rgb(0 0 0 / 15%);

padding: 0 10px 20px;

}

ul.enter-roomclas\>li {

position: relative;

}

ul.enter-roomclas\>li ul li {

border-bottom: 1px solid #ededed;

}

ul.enter-roomclas\>li:hover ul {

display: block;

}

ul.enter-roomclas\>li\>a .sub-arrow {

margin-left: 0.255em;

font-size: 15px;

}

ul.enter-roomclas\>li ul a .dashboard-menu-item-icon {

margin-right: 6px;

}

.model_pop_box {

display: none;

width: 100%;

margin: 0 auto;

background-color: #000000cc;

position: fixed;

height: 100%;

top: 0%;

}

.model_pop {

position: absolute;

left: 20%;

top: 16%;

}

.model_pop_close {

cursor: pointer;

display: block;

margin: 0 auto;

padding: 15px 20px;

background-color: #223967;

color: #fff;

}

.crd-img img {

width: 100%;

}

.course-card\_\_header {

font-size: 22px;

font-weight: 400;

padding: 0 13px;

text-align: left;

}

.package-pricing-wrap {

background: #fff;

border-radius: 4px;

overflow: hidden;

font-family: \"Sofia Pro Light\" !important;

font-size: 18px;

}

.pricing-header {

background: #0F6AC4;

height: 85px;

display: flex;

color: #fff;

align-items: center;

justify-content: end;

padding: 0 20px;

}

.package-2 .pricing-header {

background: #3BA5FC;

}

.package-2 .pricing-content-wrap {

border: 3px solid #3BA5FC;

}

.pricing-content-wrap {

padding: 0 30px 30px;

border: 3px solid #0F6AC4;

border-bottom-left-radius: 4px;

border-bottom-right-radius: 4px;

}

.pricing-badge {

background: #fff;

border-radius: 4px;

padding: 10px;

position: absolute;

top: 50px;

width: 82px;

height: 82px;

display: flex;

align-items: center;

justify-content: center;

}

.package-2 .pricing-badge img {

filter: invert(49%) sepia(100%) saturate(527%) hue-rotate(178deg)
brightness(98%) contrast(102%);

}

.package-2 .pricing-badge img {

filter: invert(49%) sepia(100%) saturate(527%) hue-rotate(178deg)
brightness(98%) contrast(102%);

}

.package-details {

display: flex;

align-items: center;

justify-content: space-between;

margin: 60px 0 30px;

}

.package-type {

font-size: 31px;

font-weight: bold;

color: #0F6AC4;

}

.package-2 .package-type {

font-size: 31px;

font-weight: bold;

color: #3BA5FC;

}

.pricing-price {

font-size: 31px;

color: #0F6AC4;

}

.package-2 .pricing-price {

font-size: 31px;

color: #3BA5FC;

}

.pricing-content {

color: #676767;

}

.package-2 a.pricing-button {

background: #E0F1FF;

color: #3BA5FC;

}

a.pricing-button {

position: relative;

background: #E8F4FF;

width: 100%;

display: block;

text-align: center;

padding: 20px 10px;

border-radius: 8px;

color: #0F6AC4;

font-weight: bold;

top: 0;

transition: top ease 0.5s;

}

a.pricing-button:hover {

top: -10px;

}

img.img-circle {

border-radius: 50%;

float: right;

height: 80px;

object-position: top;

object-fit: cover;

width: 80px;

}

img.img-circle.img-siz {

height: 60px;

width: 60px;

}

.authr-info h4,

.authr-info p {

font-family: \'Open Sans\';

margin-bottom: 0 !important;

}

.authr-info h4 {

color: #414141;

font-size: 1.2rem;

font-weight: 500;

}

.authr-info p {

color: #414141;

font-size: 15px;

font-weight: 400;

}

.dwnload-pdf {

text-align: center;

padding-top: 1.5rem !important;

padding-bottom: 1.5rem !important;

}

.dwnload-pdf .btn-group {

position: relative;

display: -webkit-inline-flex;

display: -ms-inline-flexbox;

display: inline-flex;

vertical-align: middle;

}

.dwnload-pdf .btn-group\>.btn {

position: relative;

-webkit-flex: 1 1 auto;

-ms-flex: 1 1 auto;

flex: 1 1 auto;

}

.dwnload-pdf .btn {

color: #fff;

border-radius: 5px;

padding: 10px 20px;

border: 1px solid;

font-weight: 700;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

.dwnload-pdf .btn.btn-orange {

background: #f99e31;

border-color: #f99e31;

font-weight: 700;

font-family: \'Open Sans\';

margin-bottom: 0.5rem !important;

margin-right: 0.5rem !important;

}

form.trad-spread-sheet table tr td {

border: 1px solid #8d8c8c;

}

form.trad-spread-sheet td input,

form.trad-spread-sheet td textarea,

form.trad-spread-sheet td select {

width: 100%;

}

form.trad-spread-sheet td input\[type=\"submit\"\] {

width: auto;

}

table.spradshet_table\>thead\>tr\>th,

table.spradshet_table\>thead\>tr\>td {

border: none;

-webkit-flex: 1 1;

-ms-flex: 1 1;

flex: 1 1;

white-space: nowrap;

}

table.show-records tbody tr {

background-color: #fff;

}

table.spradshet_table\>tbody\>tr:first-child {

margin-top: 15px;

}

table.spradshet_table\>tbody\>tr {

z-index: 1;

position: relative;

display: table-row;

border-radius: 5px;

background-color: #fff;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

table.spradshet_table\>tbody\>tr\>td {

border: none;

padding: 20px;

font-size: 14px;

background-color: #fff;

font-family: \'Open Sans\';

vertical-align: middle;

}

table.spradshet_table {

background: none;

border: 0 !important;

border-collapse: separate;

border-spacing: 0 15px !important;

width: 100% !important;

}

.main_scroll {

overflow-x: auto;

overflow-y: hidden;

}

table.spradshet_table,

table.spradshet_table th,

table.spradshet_table td {

box-sizing: content-box;

}

div.wc-accontent-inner.no-backcolor {

background-color: transparent;

-webkit-box-shadow: none;

box-shadow: none;

}

table.spradshet_table\>tbody\>tr\>td:first-child {

border-top-left-radius: 5px;

border-bottom-left-radius: 5px;

}

table.spreadsheet_table\>tbody\>tr\>td:last-child {

border-top-right-radius: 5px;

border-bottom-right-radius: 5px;

}

/\*digiwised work\*/

:root {

\--bgColorRow: #fff;

\--textColor: #414141;

\--fontFamily: \"Open Sans\", sans-serif;

}

/\* payment method \*/

/\* 1st \*/

div#root .p-LTR.p-Locale-en .p-HeightObserverProvider .p-FadeWrapper
.p-PaymentDetails-group form
.p-Grid.p-CardForm\>.p-GridCell.p-GridCell\--12.p-GridCell\--md6 {

/\* border:2px solid red; \*/

width: 70%;

display: inline-block;

}

div#root .p-LTR.p-Locale-en .p-HeightObserverProvider .p-FadeWrapper
.p-PaymentDetails-group form
.p-Grid.p-CardForm\>.p-GridCell.p-GridCell\--12.p-GridCell\--md6\>.p-Field
{

width: 100%;

display: inline-block;

clear: none;

}

/\* 2nd \*/

div#root .p-LTR.p-Locale-en .p-HeightObserverProvider .p-FadeWrapper
.p-PaymentDetails-group form
.p-Grid.p-CardForm\>.p-GridCell.p-GridCell\--12.p-GridCell\--xs6.p-GridCell\--sm6.p-GridCell\--md3
{

width: 30%;

display: inline-block;

}

div#root .p-LTR.p-Locale-en .p-HeightObserverProvider .p-FadeWrapper
.p-PaymentDetails-group form
.p-Grid.p-CardForm\>.p-GridCell.p-GridCell\--12.p-GridCell\--xs6.p-GridCell\--sm6.p-GridCell\--md3
.p-Field\[data-field\|=\"expiry\"\] {

width: 100%;

display: inline-block;

clear: none;

/\* border:2px solid red; \*/

}

/\* 3rd \*/

div#root .p-LTR.p-Locale-en .p-HeightObserverProvider .p-FadeWrapper
.p-PaymentDetails-group form
.p-Grid.p-CardForm\>.p-GridCell.p-GridCell\--12.p-GridCell\--xs6.p-GridCell\--sm6.p-GridCell\--md3
.p-Field\[data-field\|=\"cvc\"\] {

width: 160%;

display: inline-block;

clear: none;

/\* border:2px solid green; \*/

}

/\* 1st,2nd,3rd input \*/

div#root .p-LTR.p-Locale-en .p-HeightObserverProvider .p-FadeWrapper
.p-PaymentDetails-group form
.p-Grid.p-CardForm\>.p-GridCell.p-GridCell\--12.p-GridCell\--md6\>.p-Field
div .p-CardNumberInput .p-Input input,

div#root .p-LTR.p-Locale-en .p-HeightObserverProvider .p-FadeWrapper
.p-PaymentDetails-group form
.p-Grid.p-CardForm\>.p-GridCell.p-GridCell\--12.p-GridCell\--xs6.p-GridCell\--sm6.p-GridCell\--md3\>.p-Field
div .p-Input input {

padding: 12px 8px;

border-color: #c7c1c6;

border-top-color: #bbb3b9;

margin: 0 1em 0 0;

border-radius: 5px;

/\* padding-top: 1px!important;

padding-bottom: 0!important;

font-size: 17px!important; \*/

}

div#root .p-LTR.p-Locale-en .p-HeightObserverProvider .p-FadeWrapper
.p-PaymentDetails-group form
.p-Grid.p-CardForm\>.p-GridCell.p-GridCell\--12.p-GridCell\--xs6.p-GridCell\--sm6.p-GridCell\--md3:has(.p-Field\[data-field=\"expiry\"\])
{

float: right !important;

}

div#root .p-LTR.p-Locale-en .p-HeightObserverProvider .p-FadeWrapper
.p-PaymentDetails-group form
.p-Grid.p-CardForm\>.p-GridCell.p-GridCell\--12.p-GridCell\--md6:has(.p-Field\[data-field=\"number\"\])
{

width: 68%;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.form-row\>.bar {

width: 100%;

/\* height: 1px; \*/

border-bottom: 1px solid #dbdbdb;

position: absolute;

/\* text-align: center; \*/

right: 0px;

margin-bottom: 10px

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods+.form-row.place-order\>button#wpmc-prev
{

color: #0984ae;

font-size: 17px;

line-height: 24px;

padding: 12px 34px;

background: #f4f4f4;

border-color: transparent;

border: 1px solid #f4f4f4;

font-weight: 700;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

margin-left: 50px;

margin-top: 30px;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods+.form-row.place-order\>button#wpmc-prev:hover
{

color: #0984ae !Important;

background: #e7e7e7 !important;

border-color: transparent;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods+.form-row.place-order\>button#place_order
{

width: 146px;

font-size: 17px;

line-height: 24px;

padding: 12px 24px;

background: #f99e31;

border-color: #f99e31;

font-weight: 700 !important;

color: #fff;

border-radius: 5px;

/\* padding: 10px 20px; \*/

border: 1px solid;

/\* font-weight: 700; \*/

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

float: right;

margin-right: 50px;

/\* padding: 20px; \*/

/\* margin-bottom: 20px;\*/

margin-top: 30px;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods+.form-row.place-order\>button#place_order:hover
{

background: #f88b09;

border-color: #f88b09;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields
.clear {

width: 100%;

/\* height: 1px; \*/

border-bottom: 1px solid #dbdbdb;

position: absolute;

/\* text-align: center; \*/

right: 0px;

margin-bottom: 10px;

}

/\* end payment method \*/

.page-id-193.elementor-page-193 {

background-color: #efefef !important;

}

/\* {

font-family:var(\--fontFamily)!important;

color:var(\--textColor)!important;} \*/

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr
{

background-color: var(\--bgColorRow);

}

.page-id-193.elementor-page-193 form.woocommerce-cart-form\>table tbody
tr:hover\>td,

form.woocommerce-cart-form\>table tbody tr:hover\>th,

form.woocommerce-cart-form\>table tbody\>tr:nth-child(odd)\>td,

form.woocommerce-cart-form\>table tbody\>tr:nth-child(odd)\>th {

background-color: var(\--bgColorRow) !important;

/\*box-shadow: 0 1px 2px rgba(0,0,0,.15);

border-radius: 5px;\*/

transition: all .15s ease-in-out;

/\*border: 1px solid rgba(0,0,0,.125);\*/

}

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>td
{

border: none !important;

font-family: var(\--fontFamily) !important;

color: var(\--textColor) !important;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-thumbnail\>a\>.size-woocommerce_thumbnail
{

width: 100% !important;

height: 100% !important;

border-radius: 5px;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-thumbnail\>a
{

width: 138px !important;

height: auto !important;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-thumbnail
{

width: 162px !important;

height: 110px !important;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-price\>.woocommerce-Price-amount\>bdi
{

font-size: 19px;

line-height: 1.15;

font-weight: 700;

color: var(\--textColor) !important;

position: relative;

top: 14px;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-price\>span.woocommerce-Price-amount.amount\~del,

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-price\>span.woocommerce-Price-amount.amount+del
{

position: relative;

bottom: 25px;

left: 80px;

font-size: 14px;

line-height: 18px;

color: #adadad;

font-weight: 400;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-price
{

width: 190.36px !important;

height: 126px !important;

padding: 20px !important;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-price\>span.woocommerce-Price-amount.amount
{

display: flex;

position: relative;

bottom: 15px;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-price\>a.remove_product
{

font-size: 14px;

line-height: 18px;

padding: 9.5px 18.6px;

font-weight: 600;

color: #0984ae !important;

background: #f4f4f4 !important;

border-color: transparent !important;

transition: all .15s ease-in-out;

border-radius: 5px;

float: right;

/\*position: relative;

top: 13px;\*/

}

/\*making inline\*/

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
{

display: inline;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form.woocommerce-cart-form
{

width: 60%;

display: inline-block;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.cart-collaterals,

.woocommerce-page .cart-collaterals {

width: 200x;

width: 400px;

height: 350px;

background-color: var(\--bgColorRow);

display: inline-block;

float: right !important;

}

.page-id-193.elementor-page-193
.cart-collaterals\>.cart_total\>.order-total\>h2\>strong\>.woocommerce-Price-amount\>bdi
{

font-size: 42px !important;

line-height: 44px !important;

font-weight: 700 !important;

color: var(\--textColor);

}

.page-id-193.elementor-page-193.woocommerce-cart .wc-proceed-to-checkout
a.checkout-button {

font-size: 17px !important;

line-height: 24px !important;

padding: 12px 24px !important;

background: #f99e31 !important;

border-color: #f99e31 !important;

border: 1px solid !important;

font-weight: 700 !important;

border-radius: 5px !important;

transition: all .15s ease-in-out !important;

color: #fff !important;

box-sizing: border-box;

width: 223px !important;

}

.page-id-193.elementor-page-193 .elementor-shortcode\>.woocommerce
.cart-collaterals .cart_totals,

.woocommerce-page .cart-collaterals .cart_totals {

width: 55% !important;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>h3
{

display: block;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>small
{

display: block;

position: absolute;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.cart-collaterals,

.woocommerce-page .cart-collaterals\>.cart_totals {

float: none;

margin: 0 auto;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr.blank
{

height: 15px;

}

/\*22-june\*/

/\*form width set:\*/

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form.woocommerce-cart-form
{

width: 66%;

display: inline-block;

}

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr:hover
{

-webkit-box-shadow: 0 5px 20px rgba(0, 0, 0, .1);

box-shadow: 0 5px 20px rgba(0, 0, 0, .1)

}

/\*right side div set:\*/

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.cart-collaterals,

.woocommerce-page .cart-collaterals {

width: 335px;

height: 350px;

background-color: var(\--bgColorRow);

display: flex;

align-items: center;

float: right !important;

border-radius: 5px;

border: 1px solid rgba(0, 0, 0, .125);

transition: all .20s ease-in-out;

position: relative;

bottom: 20px;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.cart-collaterals:hover {

-webkit-box-shadow: 0 5px 20px rgba(0, 0, 0, .1);

box-shadow: 0 5px 20px rgba(0, 0, 0, .1)

}

.page-id-193.elementor-page-193 .elementor-shortcode\>.woocommerce
.cart-collaterals .cart_totals {

width: 100% !important;

padding: 0px 30px;

}

/\*set h2 in right side div:\*/

.page-id-193.elementor-page-193.woocommerce-page .cart-collaterals
.cart_totals\>h2\>strong {

font-size: 42px;

line-height: 44px;

font-weight: 700 !important;

color: rgb(65, 65, 65);

font-family: \"Open Sans\", sans-serif;

letter-spacing: inherit;

}

/\*button width manage:\*/

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-price\>a.remove_product:hover
{

background: #e7e7e7 !important;

}

.page-id-193.elementor-page-193.woocommerce-cart .wc-proceed-to-checkout
a.checkout-button {

box-sizing: border-box !important;

width: 225px !important;

}

.page-id-193.elementor-page-193.woocommerce-cart .wc-proceed-to-checkout
a.checkout-button:hover {

color: #FFFFFF !important;

background-color: #ff9400 !important;

}

/\*display none to extra tr:\*/

.page-id-193.elementor-page-193.woocommerce-page .cart-collaterals
.cart_totals\>.shop_table_responsive {

display: none;

}

/\*padding remmove from button top:\*/

.page-id-193.elementor-page-193.woocommerce-cart .wc-proceed-to-checkout
{

padding: 0px !important;

height: 76px;

}

/\*set border bottom from the text and also set text:\*/

.page-id-193.elementor-page-193.woocommerce-cart
.wc-proceed-to-checkout+.tax-disclaimer {

color: rgb(102, 102, 102);

font-size: 14px;

font-style: italic;

padding-bottom: 15px;

border-bottom: 1px solid #dbdbdb;

}

/\*display none to extra hr:\*/

.page-id-193.elementor-page-193.woocommerce-cart
.wc-proceed-to-checkout+.tax-disclaimer+hr {

display: none;

}

/\*remove border from table\*/

.page-id-193.elementor-page-193 .woocommerce table.shop_table {

border: none;

}

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>td
{

background: #fff !important;

}

/\*create tr border-radius and border\*/

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>td:first-child
{

border-left-style: solid;

border-top-left-radius: 5px;

border-bottom-left-radius: 5px;

border-left-color: rgba(0, 0, 0, .125) !important;

border-left-width: 1px !important;

border-left-style: solid !important;

}

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>td:last-child
{

border-right-style: solid;

border-top-right-radius: 5px;

border-bottom-right-radius: 5px;

border-right-color: rgba(0, 0, 0, .125) !important;

border-right-width: 1px !important;

border-right-style: solid !important;

}

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>.product-thumbnail,

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>.product-name,

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>.product-price
{

border-top-color: rgba(0, 0, 0, .125) !important;

border-top-width: 1px !important;

border-top-style: solid !important;

}

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>.product-thumbnail,

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>.product-name,

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>.product-price
{

border-bottom-color: rgba(0, 0, 0, .125) !important;

border-bottom-width: 1px !important;

border-bottom-style: solid !important;

}

\@import
url(\'https://fonts.googleapis.com/css2?family=Lobster&family=Open+Sans:wght@300&family=Press+Start+2P&display=swap\');

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>h3
{

/\* width: 300px; \*/

position: relative;

top: 36px;

color: rgb(119, 119, 119);

font-weight: lighter;

font-style: normal;

font-size: 40px;

line-height: 54px;

letter-spacing: -0.4px;

margin-bottom: 38px;

font-weight: 300;

font-family: -apple-system, system-ui, BlinkMacSystemFont, \"Segoe UI\",
Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\",
\"Segoe UI Symbol\";

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>small
{

position: relative;

bottom: 40px;

left: 200px;

width: 300px;

font-style: italic;

font-size: 1rem;

color: rgb(119, 119, 119);

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>small::before
{

content: \"(\";

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>small::after
{

content: \")\";

}

.page-id-193.elementor-page-193
.elementor-element-afd65d2\>.elementor-widget-wrap {

padding: 0px !important;

padding-bottom: 50px !important;

}

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr
{

transition: all .20s ease-in-out;

}

/\*23-june\*/

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-thumbnail\>a\>.size-woocommerce_thumbnail\>img
{

width: 100% !important;

height: 100% !important;

border-radius: 5px;

}

.page-id-193
section.elementor-element.elementor-element-32c3ef4d.elementor-section-boxed.elementor-section-height-default.elementor-top-section
{

display: none;

}

.page-id-193.elementor-page-193.woocommerce-cart
.wc-proceed-to-checkout\~.coupon_add\>.coupon\>#coupon_code {

height: 40px;

font-size: 14px;

line-height: 38px;

padding: 0 12px;

border-radius: 5px;

border-color: #7e7d7e;

font-weight: 400;

color: #666;

background: #fff !important;

border: 1px solid #dbdbdb;

box-shadow: none !important;

transition: all .20s ease-in-out;

}

.page-id-193.elementor-page-193.woocommerce-cart
.wc-proceed-to-checkout\~.coupon_add\>.coupon\>#coupon_code:focus {

border-color: #1e73be;

}

.page-id-193.elementor-page-193.woocommerce-cart
.wc-proceed-to-checkout\~.coupon_add\>.coupon\>#coupon_code:hover {

border-color: #7e7d7e

}

.page-id-193.elementor-page-193.woocommerce-cart
.wc-proceed-to-checkout\~.coupon_add\>.coupon\>.button {

/\* font-size: 17px !important; \*/

line-height: 22px;

padding: 8px 12px;

background: #f99e31 !important;

/\* border-color: #f99e31 !important; \*/

border: 1px solid transparent;

cursor: pointer;

text-align: center;

text-decoration: none;

vertical-align: middle;

transition: all .20s ease-in-out !important;

font-family: \"sofia-pro\", Sans-serif;

font-size: 20px;

font-weight: 400 !important;

color: #FFFFFF;

/\* background-color: var(\--e-global-color-accent ); \*/

border-radius: 8px 8px 8px 8px;

color: #fff !important;

margin-top: 15px;

}

.page-id-193.elementor-page-193.woocommerce-cart
.wc-proceed-to-checkout\~.coupon_add\>.coupon\>,

button:focus {

background: #076787;

border-color: #076787;

color: #FFFFFF;

background-color: #ff9400 !important;

}

.page-id-193.elementor-page-193.woocommerce-cart
.wc-proceed-to-checkout\~.coupon_add\>.coupon\>.button:hover {

background: #076787;

border-color: #076787;

color: #FFFFFF;

background-color: #ff9400 !important;

}

.page-id-193.elementor-page-193.woocommerce-cart .wc-proceed-to-checkout
a.checkout-button:focus {

background: #076787;

border-color: #076787 !important;

color: #FFFFFF;

background-color: #ff9400 !important;

}

.page-id-193.elementor-page-193 .elementor-shortcode\>.woocommerce
.cart-collaterals .cart_totals {

padding: 30px 20px !important;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.cart-collaterals,

.woocommerce-page .cart-collaterals {

height: auto !important;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form.woocommerce-cart-form\>button.clear_cart_button
{

font-family: \"sofia-pro\", Sans-serif;

font-size: 20px;

font-weight: 400;

color: #FFFFFF;

background-color: #f99e31 !important;

border-radius: 8px 8px 8px 8px;

line-height: 24px;

padding: 12px 24px;

border-color: transparent;

cursor: pointer;

border: 1px solid transparent;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

transition: all .20s ease-in-out;

text-align: center;

text-decoration: none;

vertical-align: middle;

float: right;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form.woocommerce-cart-form\>button.clear_cart_button:hover
{

background: #076787;

/\* border-color: #076787; \*/

color: #FFFFFF;

background-color: #ff9400 !important;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form.woocommerce-cart-form\>button.clear_cart_button:focus
{

background: #076787;

border-color: #076787 !important;

color: #FFFFFF;

background-color: #ff9400 !important;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table.shop_table.woocommerce-cart-form\_\_contents
{

margin: 0px -1px 0px 0 !important;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>
{

margin-bottom: 50px;

}

.elementor-menu-cart\_\_wrapper {

display: none;

}

/\* START BILLING FIELDS \*/

.page-id-194.elementor-page-194 {

background-color: #efefef !important;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields
{

position: relative;

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-flex-flow: column nowrap;

-ms-flex-flow: column nowrap;

flex-flow: column nowrap;

width: 760px;

height: auto;

background: #fff;

border-radius: 5px;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

-webkit-transition: all .15s ease-in-out;

-o-transition: all .20s ease-in-out;

transition: all .20s ease-in-out;

/\* border: 2px solid black;\*/

padding: 20px 30px;

display: block;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields:hover
{

-webkit-box-shadow: 0 5px 20px rgba(0, 0, 0, .1);

box-shadow: 0 5px 20px rgba(0, 0, 0, .1)

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields
P.form-row {

padding-top: 0 !important;

padding-left: 15px !important;

padding-right: 15px !important;

position: relative;

min-height: 30px;

display: block;

margin-bottom: 15px !important;

border: 0;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields
P.form-row\>span\>input:focus {

border-color: #1e73be !important;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields
P.form-row\>span\>input:hover {

border-color: #7e7d7e !important;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields
P.form-row\>span\>input {

border-radius: 5px !important;

box-sizing: border-box;

width: 100%;

margin: 0;

outline: 0;

line-height: normal;

display: block;

width: 100%;

height: 50px;

padding: 0 16px;

font-size: 17px;

line-height: 48px;

font-weight: 400;

color: #666;

background: #fff !important;

border: 1px solid #dbdbdb;

-webkit-box-shadow: none !important;

box-shadow: none !important;

border-radius: 5px;

-webkit-transition: all .20s ease-in-out;

-o-transition: all .20s ease-in-out;

transition: all .20s ease-in-out;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields
P.form-row\>span\>label {

color: #7e7d7e !important;

font-weight: 700;

margin-bottom: 8px;

text-transform: none;

font-size: 17px;

display: block;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields
.form-row\>label:not(.required) {

color: #7e7d7e !important;

font-weight: 700;

margin-bottom: 8px;

line-height: 2;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields
.form-row\>label\>.required {

color: #d9534f !important;

font-weight: 700 !important;

font-size: 18px !important;

font-family: \"Open Sans\", sans-serif;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields\>h3
{

font-size: 22px;

/\* margin-bottom: 30px; \*/

font-weight: 500;

line-height: 1.2;

color: rgb(65, 65, 65);

font-family: \"Open Sans\", sans-serif;

padding-bottom: 14px;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields\>#wpmc-back-to-cart
{

color: #0984ae;

font-size: 17px;

line-height: 24px;

padding: 12px 44px;

background: #f4f4f4;

border-color: transparent;

border: 1px solid #f4f4f4;

font-weight: 700;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

margin-left: 50px;

margin-bottom: 20px;

margin-top: 35px;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields\>#wpmc-back-to-cart:hover
{

color: #0984ae;

background: #e7e7e7;

border-color: transparent

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields\>#nect
{

width: 230px;

font-size: 17px;

line-height: 24px;

padding: 12px 24px;

background: #f99e31;

border-color: #f99e31;

font-weight: 700 !important;

color: #fff;

border-radius: 5px;

/\* padding: 10px 20px; \*/

border: 1px solid;

/\* font-weight: 700; \*/

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

float: right;

margin-right: 50px;

/\* padding: 20px; \*/

margin-bottom: 20px;

margin-top: 35px;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.woocommerce-billing-fields\>#nect:hover
{

background: #f88b09;

border-color: #f88b09

}

/\* END BILLING FIELDS \*/

/\* START PAYMENT METHOD \*/

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>.wpmc-tabs-wrapper
{

width: 770px;

float: left;

position: relative;

/\* border: 2px solid red; \*/

right: 2px;

/\*border:2px solid red;\*/

}

.page-id-194.elementor-page-194 .wpmc-step-payment {

position: relative;

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-flex-flow: column nowrap;

-ms-flex-flow: column nowrap;

flex-flow: column nowrap;

width: 760px;

height: auto;

background: #fff !important;

border-radius: 5px;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

-webkit-transition: all .20s ease-in-out;

-o-transition: all .20s ease-in-out;

transition: all .20s ease-in-out;

/\*border:2px solid black;\*/

/\*margin: 0 auto;\*/

}

.page-id-194.elementor-page-194 .wpmc-step-payment:hover {

-webkit-box-shadow: 0 5px 20px rgba(0, 0, 0, .1);

box-shadow: 0 5px 20px rgba(0, 0, 0, .1)

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods {

/\*border:6px solid red;\*/

border-radius: 5px;

margin: 0px 10px !important;

background: #ebe9eb;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>.wpmc-tabs-wrapper
.wpmc-tab-item:before {

display: block;

content: \"\";

/\*border-bottom: 4px solid #dbdbdb;\*/

border-bottom-width: 4px;

height: 0;

position: absolute;

top: 0px;

left: 0;

right: 0;

-webkit-transition: border-color .3s;

-o-transition: border-color .3s;

transition: border-color .3s;

}

/\* END PAYMENT METHOD \*/

/\* start side div \*/

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper {

/\*border:2px solid red;\*/

display: flex;

flex-direction: column;

width: 325px;

height: auto;

background: #fff;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

-webkit-transition: all .20s ease-in-out;

-o-transition: all .20s ease-in-out;

transition: all .20s ease-in-out;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper:hover {

-webkit-box-shadow: 0 5px 20px rgba(0, 0, 0, .1);

box-shadow: 0 5px 20px rgba(0, 0, 0, .1)

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .main_div_cart {

/\*border:2px solid red;\*/

display: flex;

justify-content: space-between;

align-items: center;

padding: 6px 20px;

background: rgba(0, 0, 0, .03);

/\* border-bottom: 1px solid #dbdbdb;\*/

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .main_div_cart\>h3 {

font-weight: 700 !important;

font-size: 1.5rem;

line-height: 1.2;

color: #414141f5;

position: relative;

top: 4px;

font-family: system-ui, -apple-system, \"Segoe UI\", Roboto, \"Helvetica
Neue\", Arial, \"Noto Sans\", \"Liberation Sans\", sans-serif, \"Apple
Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color
Emoji\";

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .main_div_cart\>a {

font-size: 14px;

line-height: 18px;

padding: 8px 14px;

font-weight: 600;

color: #0984ae;

background: #f4f4f4;

border-color: transparent;

border-radius: 5px;

-webkit-transition: all .20s ease-in-out;

-o-transition: all .20s ease-in-out;

transition: all .20s ease-in-out;

text-align: center;

vertical-align: middle;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .main_div_cart\>a:hover {

background: #e7e7e7 !important;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .tax_class {

margin: 0 auto;

border: 1px solid blue;

width: 90%;

height: auto;

display: flex;

justify-content: space-between;

align-items: center;

border: 1px solid #dbdbdb !important;

/\* border-radius: 5px; \*/

font-size: 14px;

height: 42px;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .tax_class\>.tx_text {

width: 50%;

height: 100%;

/\* border:2px solid red; \*/

margin: 0 auto;

display: flex;

padding: 0px 12px;

align-items: center;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .tax_class\>.tx_amount {

width: 50%;

height: 100%;

/\* border:2px solid red; \*/

margin: 0 auto;

display: flex;

padding: 0px 12px;

align-items: center;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .total_checkout_amount {

margin: 0 auto;

width: 100%;

height: 100%;

/\* border:2px solid red; \*/

display: flex;

justify-content: space-between;

align-items: center;

height: 32px;

padding: 40px 20px;

vertical-align: middle;

font-weight: 700;

border-top: 1px solid #dbdbdb;

font-family: \"Open Sans\", sans-serif;

text-transform: none;

font-size: 17px;

line-height: 1.5;

margin-top: 15px;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .total_checkout_amount\>.total_am {

font-size: 20px;

line-height: 32px;

margin: 0;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .total_checkout_amount\>.checkout_cart_total {

font-weight: 700;

margin: 0;

}

/\*24-june\*/

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .total_checkout_amount\>.total_am\>strong {

color: #414141;

font-size: 26px;

line-height: 32px;

ont-family: \"Open Sans\", sans-serif;

font-weight: 700;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper .total_checkout_amount\>.checkout_cart_total {

color: #414141;

font-size: 17px;

line-height: 1.5px;

ont-family: \"Open Sans\", sans-serif;

font-weight: 600;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper\>br {

display: none;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper\>.shop_table_responsive {

background: var(\--bgColorRow) !important;

border-bottom: 1px solid #dbdbdb;

border-top: 0px solid #dbdbdb;

border-radius: 0px !important;

border-left: 0px solid #dbdbdb;

border-right: 0px solid #dbdbdb;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper\>.shop_table_responsive\>tbody\>tr\>td {

background: var(\--bgColorRow) !important;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper\>.shop_table_responsive\>tbody\>tr\>td {

border: none !important;

border-top: 1px solid #dbdbdb !important;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>.wpmc-nav-wrapper\>table\>tbody\>tr\>.product-thumbnail\>.size-woocommerce_thumbnail\>img
{

width: 52px;

height: 52px;

border-radius: 5px;

box-sizing: border-box;

margin-left: 16px;

/\* border: 2px solid red; \*/

margin-bottom: 25px;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper\>.shop_table_responsive\>tbody\>tr\>.product-price {

/\* border:2px solid red !important; \*/

display: flex;

padding: 0px !Important;

padding-top: 10px !important;

vertical-align: top;

/\*margin-right:20px !important;\*/

padding-left: 4px !important;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper\>.shop_table_responsive\>tbody\>tr\>td {

/\* border:2px solid red !important; \*/

/\* padding:18px 20px;\*/

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper\>.shop_table_responsive\>tbody\>tr\>.product-name {

/\* border:2px solid red !important; \*/

/\* display:flex; \*/

padding: 0px !Important;

padding-top: 10px !important;

/\* padding-right:25px !important; \*/

vertical-align: top;

/\* padding-left:10px !important; \*/

padding-bottom: 10px !important;

color: rgb(65, 65, 65);

font-size: 14px;

line-height: 21px;

font-family: \"Open Sans\", sans-serif;

width: 90px;

font-weight: 600;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper\>.shop_table_responsive\>tbody\>tr\>.product-name::after
{

content: \"\";

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper\>.shop_table_responsive\>tbody\>tr\>.product-price\>span\>bdi
{

position: relative;

padding-right: 15px !important;

padding-left: 5px;

font-weight: 700;

color: rgb(65, 65, 65);

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper\>.shop_table_responsive\>tbody\>tr\>.product-thumbnail
{

/\* border:2px solid red !important; \*/

/\* flex-grow:1; \*/

/\*padding-right:15px;\*/

/\*padding-left:15px;\*/

width: 85px;

height: auto;

}

.subscription-details {

display: none;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-steps-wrapper {

float: left;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper {

float: right;

clear: none;

position: relative;

bottom: 98px;

/\* left: 35px; \*/

}

/\* END SIDE DIV \*/

/\*24-june\*/

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-thumbnail\>.size-woocommerce_thumbnail\>img
{

width: 100% !important;

height: 100% !important;

border-radius: 5px;

}

/\*top tab\*/

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-tabs-wrapper .wpmc-tabs-list {

margin-top: 50px;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-tabs-wrapper .wpmc-tab-item .wpmc-tab-text {

display: block;

position: relative;

top: auto;

left: auto;

right: auto;

bottom: 49px !important;

font-size: 17px !important;

color: #999;

font-family: \"Open Sans\", sans-serif;

font-weight: 700 !important;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-tabs-wrapper .wpmc-tab-item {

padding-bottom: 0px !important;

}

.page-id-194.elementor-page-194
.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout\>.wpmc-step-item.wpmc-step-billing.current\>.wc_coupon_message_wrap
{

padding: 0px !important;

padding-top: 7px !important;

}

/\*end top tab\*/

/\*some changes in coupon\*/

.page-id-193.elementor-page-193.woocommerce-cart .coupon\>#coupon_code {

height: 40px;

font-size: 14px;

line-height: 38px;

padding: 0 12px;

border-radius: 5px;

border-color: #7e7d7e;

font-weight: 400;

color: #666;

background: #fff !important;

border: 1px solid #dbdbdb;

box-shadow: none !important;

transition: all .20s ease-in-out;

}

.page-id-193.elementor-page-193.woocommerce-cart
.coupon\>#coupon_code:hover {

border-color: #7e7d7e

}

.page-id-193.elementor-page-193.woocommerce-cart
.coupon\>#coupon_code:focus {

border-color: #1e73be;

}

.page-id-193.elementor-page-193.woocommerce-cart .coupon\>.button {

/\* font-size: 17px !important; \*/

line-height: 22px;

padding: 8px 12px;

background: #f99e31 !important;

/\* border-color: #f99e31 !important; \*/

border: 1px solid transparent;

cursor: pointer;

text-align: center;

text-decoration: none;

vertical-align: middle;

transition: all .20s ease-in-out !important;

font-family: \"sofia-pro\", Sans-serif;

font-size: 20px;

font-weight: 400 !important;

color: #FFFFFF;

/\* background-color: var(\--e-global-color-accent ); \*/

border-radius: 8px 8px 8px 8px;

color: #fff !important;

margin-top: 15px;

}

.page-id-193.elementor-page-193.woocommerce-cart .coupon\>.button:focus
{

background: #076787;

border-color: #076787;

color: #FFFFFF;

background-color: #ff9400 !important;

}

.page-id-193.elementor-page-193.woocommerce-cart .coupon\>.button:hover
{

background: #076787;

border-color: #076787;

color: #FFFFFF;

background-color: #ff9400 !important;

}

.page-id-193.elementor-page-193.woocommerce-cart .coupon {

/\*position:absolute;

right: 30px;

bottom:20px;\*/

}

/\*END some changes in coupon\*/

/\*START PAYMENT METHOD\*/

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment {

background: #fff !important;

padding: 10px 30px;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li
.payment_box.payment_method_woocommerce_payments {

/\* border: 2px solid blue; \*/

background: transparent !Important;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li
.payment_box.payment_method_woocommerce_payments
fieldset#wc-woocommerce_payments-upe-form {

/\* border: 2px solid blue; \*/

background: transparent !Important;

padding: 0px;

border: none;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods {

border: 1px solid #dbdbdb;

border-radius: 5px;

margin: 10px 10px !important;

/\* background: #ebe9eb; \*/

background: #f4f4f4 !important;

padding: 0px !important;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li
.payment_box.payment_method_woocommerce_payments {

border-top: 1px solid #dbdbdb;

/\* border-top:2px solid black; \*/

margin: 0px;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>.wc_payment_method\>input#payment_method_woocommerce_payments,

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>.wc_payment_method\>input#payment_method_woocommerce_payments+label
{

/\* border-bottom:2px solid red; \*/

/\* margin-left:20px;

margin-top:10px; \*/

position: relative;

left: 20px;

top: 10px;

margin: 10px;

color: #414141e8 !important;

font-family: \"Open Sans\", sans-serif;

font-weight: 700;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_bacs\>input#payment_method_bacs,

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_cheque\>input#payment_method_cheque,

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>.wc_payment_method\>input#payment_method_woocommerce_payments,

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>.wc_payment_method\>input#payment_method_woocommerce_payments
{

width: 20px !important;

height: 20px !important;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_bacs
{

/\* border:2px solid red; \*/

padding-left: 30px;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_cheque
{

padding-left: 30px;

/\*padding-bottom:20px;\*/

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_bacs\>input#payment_method_bacs
{

/\*margin: 0 21px 0 0;\*/

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_cheque
input#payment_method_cheque {

/\*margin: 0 21px 0 0;\*/

}

/\*26-june\*/

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_bacs\>input#payment_method_bacs,

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_cheque\>input#payment_method_cheque,

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>.wc_payment_method\>input#payment_method_woocommerce_payments,

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>.wc_payment_method\>input#payment_method_woocommerce_payments
{

width: 20px !important;

height: 20px !important;

/\* -webkit-appearance: none; \*/

/\* padding: 10px 10px; \*/

appearance: none;

-webkit-appearance: none;

border-radius: 50%;

background: #fff;

/\* outline:4px solid #fff; \*/

outline: 1px solid #dbdbdb;

/\* outline: 5px solid #dbdbdb; \*/

position: relative;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_bacs\>input#payment_method_bacs:checked,

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_cheque\>input#payment_method_cheque:checked
{

background: white;

/\* border: px solid black; \*/

/\* outline-offset: 5px; \*/

width: 12px !important;

height: 12px !important;

outline: 5px solid #0984ae;

left: 4px;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>.wc_payment_method\>input#payment_method_woocommerce_payments:checked
{

background: white;

/\* border: px solid black; \*/

/\* outline-offset: 5px; \*/

width: 12px !important;

height: 12px !important;

outline: 5px solid #0984ae;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>.wc_payment_method\>div\>fieldset\>div\>p\>input#wc-woocommerce_payments-new-payment-method
{

width: 20px !important;

height: 20px;

position: relative;

right: 2px;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>.wc_payment_method\>div\>fieldset\>div\>p\>label
{

position: relative;

right: 7px;

}

/\*27-june\*/

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>.wc_payment_method\>input#payment_method_woocommerce_payments
{

top: 22px;

left: 19.5px;

margin-right: 0px !important;

}

/\* .page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>.wc_payment_method\>input#payment_method_woocommerce_payments:checked{

top:20px

} \*/

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_bacs\>input#payment_method_bacs
{

/\*top: 5px !important;\*/

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_cheque\>input#payment_method_cheque
{

left: px !important;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li.wc_payment_method.payment_method_cheque\>input#payment_method_cheque+label
{

/\* color:red; \*/

position: relative;

/\*left:5px;\*/

/\*bottom:3px;\*/

}

/\*27-june\*/

.page-id-194.elementor-page-194 .wpmc-step-payment\>#payment_heading {

margin-left: 30px;

/\* border: 2px solid red; \*/

font-size: 22px;

font-weight: 500;

line-height: 1.2;

color: rgb(65, 65, 65);

font-family: \"Open Sans\", sans-serif;

margin-top: 0px;

padding-top: 30px;

}

/\*END PAYMENT METHOD\*/

/\*27-june\*/

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>.product-name
{

/\* color:red; \*/

transition: color .1s ease-in-out, background-color .1s ease-in-out;

font-weight: 700;

font-size: 19px;

color: inherit;

}

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>.product-name:hover
{

color: #1e73be !important;

}

.page-id-193.elementor-page-193 .elementor-element-afd65d2 {

position: relative;

}

.page-id-193.elementor-page-193.woocommerce-cart .coupon {

/\* position: absolute;

right: 25px;\*/

/\*bottom: 140px;\*/

/\* float: right; \*/

/\* left: 670px; \*/

}

/\* START LOGIN \*/

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p\>label\>span.required
{

color: #d9534f !important;

font-size: 20px;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login {

/\*border: 2px solid red !important;\*/

display: flex;

width: 756px;

/\*height: 475px;\*/

height: auto;

border: 0;

background: #fff;

border-radius: 5px;

-webkit-box-shadow: 0 1px 2px rgb(0 0 0 / 15%);

box-shadow: 0 1px 2px rgb(0 0 0 / 15%);

padding: 30px !important;

flex-direction: row-reverse;

transition: all .20s ease-in-out;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login:hover {

-webkit-box-shadow: 0 5px 20px rgba(0, 0, 0, .1) !important;

box-shadow: 0 5px 20px rgba(0, 0, 0, .1) !important;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>.site-content {

/\*border:2px solid red;\*/

width: 50%;

height: auto;

display: flex;

flex-direction: column;

align-items: center;

text-align: center;

justify-content: center;

padding-left: 40px;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login {

border: none !important;

background: transparent !important;

border-radius: 0px !important;

-webkit-box-shadow: none !important;

box-shadow: none !important;

padding: 0px !important;

/\* border:2px solid red !important;\*/

margin: 0px;

width: 50%;

height: auto;

padding-right: 40px !important;

border-right: 1px solid #dbdbdb !important;

}

/\*.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login:hover{

pointer-events: none;

}\*/

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-last,

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-first
{

/\* width:100%; \*/

display: block;

width: 100%;

/\* height: 50px; \*/

margin-bottom: 15px;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-first\>label,

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-last\>label
{

color: #7e7d7e !important;

font-weight: 700 !important;

margin-bottom: 8px !important;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-first\>.input-text
{

display: block;

width: 100%;

height: 50px;

padding: 0 16px;

font-size: 17px;

line-height: 48px;

font-weight: 400;

color: #666;

background: #fff !important;

border: 1px solid #dbdbdb;

-webkit-box-shadow: none !important;

box-shadow: none !important;

border-radius: 5px;

-webkit-transition: all .20s ease-in-out;

-o-transition: all .20s ease-in-out;

transition: all .20s ease-in-out;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-last\>.password-input
{

display: block;

position: static !important;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-last\>.password-input\>input#password
{

display: block !important;

width: 100%;

height: 50px;

padding: 0 16px;

font-size: 17px;

line-height: 48px;

font-weight: 400;

color: #666;

background: #fff !important;

/\* border: 1px solid #dbdbdb; \*/

-webkit-box-shadow: none !important;

box-shadow: none !important;

border-radius: 5px;

-webkit-transition: all .20s ease-in-out;

-o-transition: all .20s ease-in-out;

transition: all .20s ease-in-out;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-first\>.input-text:hover
{

border-color: #7e7d7e !important;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-first\>.input-text:focus
{

border-color: #1e73be !important;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-last\>.password-input\>input#password:focus
{

border-color: #1e73be !important;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-last\>.password-input\>input#password:hover
{

border-color: #7e7d7e !important;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>.site-content\>h2.new_heading {

font-size: 22px;

margin-bottom: 30px;

font-weight: 500;

line-height: 1.2;

color: rgb(65, 65, 65);

font-family: \"Open Sans\", sans-serif;

/\* font-family: -apple-system,system-ui,BlinkMacSystemFont,\"Segoe
UI\",Helvetica,Arial,sans-serif,\"Apple Color Emoji\",\"Segoe UI
Emoji\",\"Segoe UI Symbol\"; \*/

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>.site-content\>h2.new_heading:after {

display: block;

content: \"\";

width: 130px;

height: 1px;

margin: 20px auto 0;

background: #f99e31;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>.site-content\>h2.new_heading+div+p {

margin-top: 0;

margin-bottom: 1rem;

font-family: \"Open Sans\", sans-serif;

color: rgb(65, 65, 65);

font-size: 17px;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>.site-content\>h2.new_heading+div+p+a
{

font-size: 17px;

line-height: 24px;

padding: 12px 27px;

background: #f99e31;

border-color: #f99e31;

color: #fff;

font-weight: 700 !important;

border-radius: 5px;

/\* padding: 10px 20px; \*/

border: 1px solid;

font-weight: 700;

-webkit-transition: all .20s ease-in-out;

-o-transition: all .20s ease-in-out;

transition: all .20s ease-in-out;

cursor: pointer;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>.site-content\>h2.new_heading+div+p+a:hover
{

background: #f88b09;

border-color: #f88b09;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login
button.woocommerce-button.button.woocommerce-form-login\_\_submit {

width: 110px;

height: 48px;

font-family: \"sofia-pro\", Sans-serif;

font-size: 20px;

font-weight: 400;

color: #FFFFFF;

background-color: #f99e31;

;

border-radius: 8px 8px 8px 8px;

line-height: 24px;

padding: 12px 24px;

border: 1px solid transparent;

box-sizing: border-box;

display: block;

font-weight: 700;

margin-bottom: 6px;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login
button.woocommerce-button.button.woocommerce-form-login\_\_submit:hover
{

background: #f88b09;

border-color: #f88b09;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login
.lost_password {

font-size: 14px !important;

text-align: center;

/\*margin-top:10px;\*/

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login
.lost_password\>a {

color: #1e73be;

font-weight: 400;

text-decoration: none;

transition: color .1s ease-in-out, background-color .1s ease-in-out;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login .form-row
input#rememberme {

width: 20px !important;

height: 20px !important;

border-radius: 3px;

position: relative !important;

display: inline-block !important;

vertical-align: middle;

left: 0;

bottom: 10px;

background: #fff;

-moz-appearance: none;

-webkit-appearance: none;

-o-appearance: none;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

border: 2px solid #222 !important;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login .form-row
input#rememberme:checked:after {

color: white;

font-family: FontAwesome;

content: \"\\F00C\";

display: flex;

align-items: center;

/\* border-radius: 3px; \*/

/\* position:relative !important;

display: inline-block!important;

top: 2px;

left: 0;

margin: 0; \*/

width: 100%;

height: 100%;

/\* line-height: 20px;

padding-top: 20px; \*/

/\* top: -1px;

left: -1px; \*/

/\* font-size: ; \*/

text-align: center;

background: #0984ae;

/\* border-radius: 3px; \*/

overflow: hidden;

-webkit-transition: all .3s cubic-bezier(.48, -.6, .48, 1.65), padding
.3s cubic-bezier(.48, -.6, .48, 1.65) .04s;

-o-transition: all .3s cubic-bezier(.48, -.6, .48, 1.65), padding .3s
cubic-bezier(.48, -.6, .48, 1.65) .04s;

transition: all .3s cubic-bezier(.48, -.6, .48, 1.65), padding .3s
cubic-bezier(.48, -.6, .48, 1.65) .04s;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login .form-row
input#rememberme:checked {

border-color: #0984ae;

background: #0984ae;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login .form-row
input#rememberme+span {

color: #7e7d7e;

font-weight: 400;

margin-left: 8px;

position: relative;

bottom: 8px;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login .form-row
label.woocommerce-form-login\_\_rememberme {

margin-bottom: 16px;

}

.page-id-194.elementor-page-194
.elementor-element-00ff6e8\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
.wpmc-nav-wrapper\>.shop_table_responsive\>tbody\>tr\>.product-price\>del
{

font-size: 12px;

padding-right: 8px;

}

.page-id-193.elementor-page-193 .elementor-shortcode\>.woocommerce
.cart-collaterals .cart_totals\>P\>span.applie_coupon,

.page-id-193.elementor-page-193 .elementor-shortcode\>.woocommerce
.cart-collaterals .cart_totals\>P\>.dis {

font-weight: 600;

color: rgb(65, 65, 65);

}

.page-id-193.elementor-page-193 .elementor-shortcode\>.woocommerce
.cart-collaterals .cart_totals\>P\>span.applie_code,

.page-id-193.elementor-page-193 .elementor-shortcode\>.woocommerce
.cart-collaterals .cart_totals\>P\>span.applie_price {

color: rgb(102, 102, 102);

font-size: 14px;

padding-left: 10px;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p {

color: rgb(65, 65, 65);

font-size: 22px;

font-family: \"Open Sans\", sans-serif;

}

/\* END LOGIN \*/

/\*cart\*/

.elementor-menu-cart\_\_wrapper {

display: block !important;

}

/\*cart\*/

/\*Responsive\*/

\@media only screen and (max-width: 600px) {

.page-id-193.elementor-page-193 {

background-color: lightblue;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce
{

display: flex;

flex-direction: column;

align-items: center;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form.woocommerce-cart-form
{

width: 95%;

display: inline-block;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>h3
{

width: 250px;

position: relative;

top: 20px;

right: 56px;

color: rgb(119, 119, 119);

font-size: 35px;

text-align: center;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>small
{

position: relative;

bottom: 30px;

left: 118px;

width: 150px;

font-size: 13px;

color: rgb(119, 119, 119);

text-align: center;

}

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>.product-name:before
{

content: none;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-thumbnail:before
{

content: none;

}

/\*image\*/

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-thumbnail
{

height: 200px !important;

display: block;

min-width: 100%;

padding: 0px;

margin: 0px;

}

.page-id-193.elementor-page-193
.elementor-widget-shortcode\>.elementor-widget-container\>.elementor-shortcode\>.woocommerce\>form\>table\>tbody\>tr\>.product-thumbnail\>a\>.size-woocommerce_thumbnail\>img
{

width: 100% !important;

height: 100% !important;

border-radius: 5px 5px 0 0;

}

.page-id-193.elementor-page-193
form.woocommerce-cart-form\>table.shop_table.shop_table_responsive.cart.woocommerce-cart-form\_\_contents\>tbody\>tr\>.product-name
{

padding: 0px;

text-align: left !important;

padding: 0px 15px;

padding-top: 25px;

}

}

/\*START REGISTER\*/

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(3),

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(4) {

width: 48%;

/\* clear: none; \*/

display: inline-block;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(4) {

float: right;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(4)\>input#reg_lastname,

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(3)\>input#reg_firstname,

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(5)\>input#reg_email,

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(6)\>span.password-input\>input#reg_password
{

display: block !important;

width: 100%;

height: 50px;

padding: 0 16px;

font-size: 17px;

line-height: 48px;

font-weight: 400;

color: #666;

background: #fff !important;

/\* border: 1px solid #dbdbdb; \*/

-webkit-box-shadow: none !important;

box-shadow: none !important;

border-radius: 5px;

-webkit-transition: all .20s ease-in-out;

-o-transition: all .20s ease-in-out;

transition: all .20s ease-in-out;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(4)\>input#reg_lastname:focus,

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(3)\>input#reg_firstname:focus,

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(5)\>input#reg_email:focus,

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(6)\>span.password-input\>input#reg_password:focus
{

border-color: #1e73be !important;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(4)\>input#reg_lastname:hover,

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(3)\>input#reg_firstname:hover,

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(5)\>input#reg_email:hover,

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>:nth-child(6)\>span.password-input\>input#reg_password:hover
{

border-color: #7e7d7e !important;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>p\>label {

color: #7e7d7e !important;

font-weight: 700 !important;

margin-bottom: 8px !important;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>h6#register_head {

font-size: 22px;

margin-bottom: 30px;

color: rgb(65, 65, 65);

font-family: \"Open Sans\", sans-serif;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>p:nth-child(7) {

margin: auto -30px;

/\* border: 2px solid red; \*/

display: block;

padding: 30px;

border-top: 1px solid #dbdbdb;

margin-top: 40px;

margin-bottom: -30px;

display: flex;

align-items: center;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>p:nth-child(7)\>button.woocommerce-Button.woocommerce-button.button.woocommerce-form-register\_\_submit
{

font-family: \"sofia-pro\", Sans-serif;

font-size: 20px;

font-weight: 400;

color: #FFFFFF;

background-color: #FDAC3B;

border-radius: 8px 8px 8px 8px;

line-height: 24px;

padding: 12px 24px;

-webkit-transition: all .20s ease-in-out;

-o-transition: all .20s ease-in-out;

transition: all .20s ease-in-out;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>p:nth-child(7)\>button.woocommerce-Button.woocommerce-button.button.woocommerce-form-register\_\_submit:hover
{

background: #f88b09;

border-color: #f88b09;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>p:nth-child(7)\>a#backtologin
{

font-size: 17px;

line-height: 24px;

padding: 15px 34px;

background: #f4f4f4;

border-color: transparent;

border-radius: 5px;

font-family: \"sofia-pro\", Sans-serif;

font-weight: 700;

color: #0984ae;

border: 1px solid transparent;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

cursor: pointer;

margin-left: 100px;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>p:nth-child(7)\>a#backtologin:hover
{

color: #0984ae;

background: #e7e7e7;

border-color: transparent;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form#regform\>p\>label\>span.required
{

color: #d9534f !important;

font-size: 20px;

}

/\*END REGISTER\*/

/\*3-july\*/

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li:not(:first-child)
{

padding-left: 30px;

/\* vertical-align:middle; \*/

/\* position:relative; \*/

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li:not(:first-child)\>input
{

width: 20px !important;

height: 20px !important;

/\* -webkit-appearance: none; \*/

/\* padding: 10px 10px; \*/

appearance: none;

-webkit-appearance: none;

border-radius: 50%;

background: #fff;

/\* outline: 4px solid #fff; \*/

outline: 1px solid #dbdbdb;

/\* outline: 5px solid #dbdbdb; \*/

position: relative;

margin-right: 8.5px !important;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li:not(:first-child)\>input:checked
{

background: white;

/\* border: px solid black; \*/

/\* outline-offset: 5px; \*/

width: 12px !important;

height: 12px !important;

outline: 5px solid #0984ae;

left: 4px;

margin-right: 16px !important;

margin-bottom: 3px !important;

}

.page-id-194.elementor-page-194
.wpmc-step-payment\>.woocommerce-checkout-payment\>.wc_payment_methods\>li:not(:first-child)\>label
{

/\* left:0px !important; \*/

/\* bottom:5px !important; \*/

/\* vertical-align:text-top; \*/

padding-bottom: 5px;

height: 25px;

color: #7e7d7e;

font-weight: 400;

font-family: \"Open Sans\", sans-serif;

font-size: 17px;

}

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>p.form-row.form-row-last\>.password-input\>span.show-password-input
{

display: none;

}

/\*5-july\*/

.page-id-194.elementor-page-194 .woocommerce .wpmc-steps-wrapper
.wpmc-step-item\>#checkout_login\>form.woocommerce-form-login\>h3.sign_heading
{

font-size: 22px;

/\* margin-bottom: 30px; \*/

font-weight: 500;

line-height: 1.2;

color: rgb(65, 65, 65);

font-family: \"Open Sans\", sans-serif;

}

/\*6-july\*/

/\*21-july\*/

.postid-11753 .elementor-column {

min-height: 0px;

}

/\*blog-section\*/

/\* 16-aug \*/

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
.ha-pg-classic .ha-pg-thumb-area {

position: relative;

width: 100%;

height: 250px;

height: 588px;

}

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
picture.attachment-2048x2048.size-2048x2048.wp-post-image img {

width: 100%;

height: 100%;

}

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
h4.ha-pg-title a {

color: rgb(255, 255, 255);

font-size: 2.3rem;

}

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
.ha-pg-meta-wrap a.ha-pg-author-text {

color: rgb(255, 255, 255);

font-size: 24px;

font-weight: 600;

line-height: 1.1em;

display: flex;

align-items: center;

}

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
li.ha-pg-date {

display: none;

}

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
h4.ha-pg-title {

position: absolute;

top: 60px;

padding: 30px;

}

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
.ha-pg-meta-wrap {

position: absolute;

top: 240px;

padding: 30px;

}

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
.ha-pg-classic .ha-pg-item {

box-shadow: none;

}

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
.ha-pg-classic .ha-pg-content-area {

padding: 0px;

padding-top: 15px;

}

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
.ha-pg-excerpt\>p {

color: #443f3f;

font-size: 17px;

font-weight: 400;

}

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
.ha-pg-readmore\>a {

color: #0a2335 !important;

font-size: 20px;

font-weight: 400;

}

.elementor-5811
.elementor-element.elementor-element-dc9a205.ha-pg-grid-1.ha-pg-grid-tablet-2.ha-pg-grid-mobile-1.elementor-widget.elementor-widget-ha-post-grid-new.happy-addon.ha-post-grid-new.happy-addon-pro
.ha-pg-classic .ha-pg-badge {

display: none;

}

/\* 16-aug \*/

.elementor-5811 .elementor-element.elementor-element-8fce522
.elementor-heading-title,

.elementor-5811 .elementor-element.elementor-element-b3d08a8
.elementor-heading-title,

.elementor-5811 .elementor-element.elementor-element-a1e3f68
.elementor-heading-title {

font-family: \"Sofia Pro Light\", Sans-serif;

font-size: 1rem;

font-weight: bold;

color: var(\--e-global-color-9107443);

}

.elementor-5811
.elementor-element.elementor-element-8fce522\>.elementor-widget-container,

.elementor-5811
.elementor-element.elementor-element-b3d08a8\>.elementor-widget-container,

.elementor-5811
.elementor-element.elementor-element-a1e3f68\>.elementor-widget-container\>h6.elementor-heading-title.elementor-size-default
{

font-size: 0.9rem;

}

.elementor-5811
.elementor-element.elementor-element-8fce522\>.elementor-widget-container,

.elementor-5811
.elementor-element.elementor-element-b3d08a8\>.elementor-widget-container,

.elementor-5811
.elementor-element.elementor-element-a1e3f68\>.elementor-widget-container
{

padding: 6% 0% 6% 0%;

background-color: var(\--e-global-color-99012f7);

border-radius: 30px 30px 30px 30px;

display: flex;

-webkit-box-pack: center;

-ms-flex-pack: center;

justify-content: center;

padding: 13px 30px 13px 30px;

font-family: \"Sofia Pro Light\", Sans-serif;

font-size: 1rem;

font-weight: bold;

/\* border-radius: 5px 5px 5px 5px; \*/

box-shadow: 0px 0px 10px 0px rgba(0, 0, 0, 0.5);

color: var(\--e-global-color-9107443);

background-color: var(\--e-global-color-47fdcdc);

}

.elementor-5811 .elementor-element.elementor-element-033ccc2
.elementor-heading-title {

color: #0A2335;

font-family: \"Open Sans Condensed\", sans-serif;

font-weight: bold;

margin-bottom: -18px;

margin-left: 5px;

}

.elementor-posts-container.elementor-has-item-ratio
.elementor-post\_\_thumbnail img {

width: 776px;

height: 600px !important;

}

.elementor-5811
.elementor-element.elementor-element-82ba7dc.elementor-widget.elementor-widget-heading
.elementor-widget-container
h4.elementor-heading-title.elementor-size-default {

font-size: 1.5rem;

font-weight: 600;

line-height: 1.1em;

color: #414141;

line-height: 1.2;

}

.elementor-5811 .elementor-element.elementor-element-e9eac5d
.elementor-post\_\_card {

position: relative;

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-flex-flow: column nowrap;

-ms-flex-flow: column nowrap;

flex-flow: column nowrap;

width: 100%;

height: 100%;

background: #fff;

border-radius: 5px;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

border-radius: 25px;

padding: 2px;

cursor: pointer;

}

.elementor-5811
.elementor-posts-container.elementor-posts.elementor-posts\--skin-cards.elementor-grid
{

grid-row-gap: 1.5rem;

}

element.style {}

.elementor-5811 .elementor-element.elementor-element-e9eac5d
.elementor-post\_\_title,

.elementor-5811 .elementor-element.elementor-element-e9eac5d
.elementor-post\_\_title a {

font-family: \"Sofia Pro Light\", Sans-serif;

font-size: 1.25rem;

font-weight: 500;

font-weight: 500;

/\* line-height: 1.2; \*/

color: #0a2335 !important;

font-family: \"Open Sans Condensed\", sans-serif;

line-height: 24px;

margin-bottom: 0px;

}

.elementor-5811 .elementor-posts .elementor-post\_\_card
.elementor-post\_\_meta-data {

border: none !important;

}

.elementor-5811 .elementor-posts .elementor-post\_\_card
.elementor-post\_\_meta-data {

padding: 15px 30px !important;

margin-bottom: 0;

border-top: 1px solid #eaeaea;

border: none;

color: #0a2335 !important;

font-family: \"Open Sans\", sans-serif;

font-size: 17px;

}

.elementor-5811 .elementor-element.elementor-element-e9eac5d
.elementor-post\_\_card:hover {

-webkit-box-shadow: 0 5px 20px rgba(0, 0, 0, .1);

box-shadow: 0 5px 20px rgba(0, 0, 0, .1)

}

.elementor-5811 .elementor-posts .elementor-post\_\_excerpt p {

margin: 0;

line-height: 1.5;

font-size: 14px;

color: #414141;

font-family: \"Open Sans\", sans-serif;

margin-top: 1rem !important;

font-weight: 400;

text-transform: none;

font-size: 17px;

margin-bottom: 0.5rem !important;

}

.elementor-5811 .elementor-posts .elementor-post\_\_read-more {

font-size: 1.25rem;

font-weight: 700;

align-self: flex-start;

color: #0a2335 !important;

font-weight: 400;

text-decoration: none;

transition: color .1s ease-in-out, background-color .1s ease-in-out;

line-height: 1.2;

font-family: \"Open Sans\", sans-serif;

}

.elementor-5811
.elementor-element.elementor-element-d8f42a6\>.elementor-widget-container
{

padding: 3% 0% 3% 0%;

/\* margin: 0 auto; \*/

display: flex;

justify-content: center;

}

.elementor-5811
.elementor-element.elementor-element-d8f42a6\>.elementor-widget-container
h2.elementor-heading-title.elementor-size-default {

color: #0a2335 !important;

font-weight: 700 !important;

font-size: 3rem;

font-family: \"Open Sans\", sans-serif;

line-height: 1.2;

}

.elementor-5811 .elementor-element.elementor-element-26c5d70 {

width: 100%;

}

/\* .new-wrapper-class {

display: flex;

align-items: center;

justify-content: center;

width: 100%;

gap: 10px;

margin-bottom:15px;

} \*/

.elementor-5811
.elementor-element.elementor-element-1734c17.elementor-search-form\--skin-minimal.elementor-widget.elementor-widget-search-form
{

display: flex;

justify-content: center;

width: 60%;

margin: 0 auto;

}

.elementor-5811
.elementor-element.elementor-element-1734c17.elementor-search-form\--skin-minimal.elementor-widget.elementor-widget-search-form
.elementor-widget-container {

width: 100%;

}

.elementor-5811
section.elementor-section.elementor-inner-section.elementor-element.elementor-element-11211e5.elementor-section-content-middle.elementor-section-boxed.elementor-section-height-default.elementor-section-height-default
.elementor-container.elementor-column-gap-default {

display: block;

}

.elementor-5811
.elementor-element.elementor-element-8fce522\>.elementor-widget-container,

.elementor-5811
.elementor-element.elementor-element-b3d08a8\>.elementor-widget-container,

.elementor-5811
.elementor-element.elementor-element-a1e3f68\>.elementor-widget-container
h6.elementor-heading-title.elementor-size-default {

font-size: 0.9rem;

}

.elementor-5811 .ha-pg-classic .ha-pg-thumb img,

.ha-pg-crossroad .ha-pg-thumb img,

.ha-pg-monastic .ha-pg-thumb img,

.ha-pg-outbox .ha-pg-thumb img,

.ha-pg-standard .ha-pg-thumb img,

.ha-pg-stylica .ha-pg-thumb img {

width: 375px !important;

/\* height: 180px !important \*/

}

.elementor-5811 .elementor-element.elementor-element-f91dfc2
.ha-pg-thumb-area {

display: inline-table;

}

.elementor-5811 .elementor-element.elementor-element-8fce522
.elementor-heading-title,

.elementor-5811 .elementor-element.elementor-element-b3d08a8
.elementor-heading-title,

.elementor-5811 .elementor-element.elementor-element-a1e3f68
.elementor-heading-title {

font-size: 13px;

}

/\*error message\*/

div#error-message {

position: absolute;

top: 142px;

right: 30px;

z-index: 1;

border-radius: 10px;

border-top-color: #223967;

}

.woocommerce-error::before {

content: \"\\e016\";

color: #223967;

}

.custom-error-style::before {

content: \"\\e016\";

color: #223967 !important;

}

/\*error message\*/

.elementor-5811 .elementor-element.elementor-element-f91dfc2
.ha-pg-crossroad .ha-pg-thumb-area .ha-pg-thumb {

border-radius: 25px;

margin-bottom: 41px;

}

.elementor-5811 .elementor-element.elementor-element-f91dfc2
.ha-pg-content-area {

padding-right: 1rem !important;

padding-left: 1rem !important;

padding-top: 1.5rem !important;

padding-bottom: 1.5rem !important;

position: relative;

display: -webkit-flex;

display: -ms-flexbox;

display: flex;

-webkit-flex-flow: column nowrap;

-ms-flex-flow: column nowrap;

flex-flow: column nowrap;

width: 325px;

height: 222px;

background: #fff;

border-radius: 25px;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

cursor: pointer;

align-items: center;

justify-content: space-between;

/\* border: 1px solid red; \*/

}

.elementor-5811 .elementor-element.elementor-element-f91dfc2
.ha-pg-content-area:hover {

-webkit-box-shadow: 0 5px 20px rgba(0, 0, 0, .1);

box-shadow: 0 5px 20px rgba(0, 0, 0, .1)

}

.elementor-5811 .elementor-element.elementor-element-f91dfc2
.ha-pg-title a {

color: #0a2335 !important;

font-weight: bolder;

font-size: 1.25rem;

line-height: 1.2;

}

.elementor-5811 .elementor-element.elementor-element-f91dfc2
.ha-pg-title a:hover {

color: #0a2335 !important;

}

.elementor-5811 .ha-pg-meta-wrap ul li a {

font-size: 17px;

color: #000000;

font-family: \"Open Sans\", sans-serif;

}

.elementor-5811 .ha-pg-meta-wrap ul li svg {

fill: #8c8c8c !important;

}

.elementor-5811 .ha-pg-meta-wrap ul li a:hover path {

fill: #8c8c8c !important;

}

.elementor-5811 .elementor-element.elementor-element-f91dfc2
.ha-pg-content-area {

margin: -51px 30px 0px 30px;

}

.elementor-5811 .elementor-element.elementor-element-f91dfc2
.ha-pg-loadmore-wrap .ha-pg-loadmore {

background-color: #0a2335;

color: #fff;

border-radius: 30px;

display: inline-block;

text-align: center;

letter-spacing: 1px;

cursor: pointer;

text-transform: uppercase;

padding: 10px 40px;

-webkit-transition: background-color .2s ease-in-out, border-color .2s
ease-in-out, color .3s ease-in-out;

-o-transition: background-color .2s ease-in-out, border-color .2s
ease-in-out, color .3s ease-in-out;

transition: background-color .2s ease-in-out, border-color .2s
ease-in-out, color .3s ease-in-out;

font-size: 17px;

font-family: \"Open Sans\", sans-serif;

font-weight: 400;

vertical-align: middle;

text-align: center;

}

.elementor-5811 .elementor-element.elementor-element-f91dfc2
h6.ha-pg-title {

border-bottom: 1px solid rgb(30 115 190 / 49%);

padding-bottom: 8%;

}

/\* 1-aug \*/

.elementor-5811
.elementor-element.elementor-element-e6b4597.elementor-widget.elementor-widget-heading
h3.elementor-heading-title.elementor-size-default {

position: absolute;

z-index: 1;

top: 105px;

color: rgb(255, 255, 255);

color: rgba(255, 255, 255)important;

font-size: 2.5rem !important;

left: 35px;

}

.elementor-5811
section.elementor-section.elementor-inner-section.elementor-element.elementor-element-b797158.elementor-section-boxed.elementor-section-height-default.elementor-section-height-default
.elementor-author-box {

position: relative;

right: 35px;

}

th.woocommerce-orders-table\_\_header.woocommerce-orders-table\_\_header-order-number
{

border: none !important;

}

th.woocommerce-orders-table\_\_header {

border: none !important;

}

td.woocommerce-orders-table\_\_cell {

border: none;

}

tr.woocommerce-orders-table\_\_row {

border: none;

}

.woocommerce table.shop_table td {

border: none;

background: white !important;

}

/\* 8-aug \*/

/\* ORDERS PAGE \*/

section.elementor-section.elementor-top-section.elementor-element.elementor-element-bfd36e6.elementor-section-full_width.elementor-section-height-default.elementor-section-height-default
{

font-family: \"Open Sans\", sans-serif;

}

table.woocommerce-orders-table.woocommerce-MyAccount-orders.shop_table.shop_table_responsive.my_account_orders.account-orders-table
{

background: none;

border: 0 !important;

border-collapse: separate;

border-spacing: 0 15px !important;

width: 100% !important;

}

table.woocommerce-orders-table.woocommerce-MyAccount-orders.shop_table.shop_table_responsive.my_account_orders.account-orders-table
tbody tr {

z-index: 1;

position: relative;

display: table-row;

border-radius: 5px;

background-color: transparent;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

table.woocommerce-orders-table.woocommerce-MyAccount-orders.shop_table.shop_table_responsive.my_account_orders.account-orders-table
tbody tr:hover {

z-index: 10;

-webkit-box-shadow: 0 20px 20px rgba(0, 0, 0, .1);

box-shadow: 0 20px 20px rgba(0, 0, 0, .1)

}

table.woocommerce-orders-table.woocommerce-MyAccount-orders.shop_table.shop_table_responsive.my_account_orders.account-orders-table
tbody tr td {

padding: 14.5px;

background-color: transparent;

border-bottom-width: 1px;

-webkit-box-shadow: inset 0 0 0 9999px var(\--bs-table-accent-bg);

box-shadow: inset 0 0 0 9999px var(\--bs-table-accent-bg);

padding-left: 20px;

padding-right: 20px;

font-size: 14px;

color: #212529;

border-bottom: 1px solid rgb(33, 37, 41);

}

table.woocommerce-orders-table.woocommerce-MyAccount-orders.shop_table.shop_table_responsive.my_account_orders.account-orders-table
thead tr th {

padding: 14.5px;

padding-left: 20px;

padding-right: 20px;

font-size: 14px;

font-weight: 700;

color: #212529;

}

table.woocommerce-orders-table.woocommerce-MyAccount-orders.shop_table.shop_table_responsive.my_account_orders.account-orders-table
thead tr th:last-child {

float: right;

}

table.woocommerce-orders-table.woocommerce-MyAccount-orders.shop_table.shop_table_responsive.my_account_orders.account-orders-table
tbody tr td:first-child {

border-top-left-radius: 5px;

border-bottom-left-radius: 5px;

}

table.woocommerce-orders-table.woocommerce-MyAccount-orders.shop_table.shop_table_responsive.my_account_orders.account-orders-table
tbody tr td:last-child {

border-top-right-radius: 5px;

border-bottom-right-radius: 5px;

}

table.woocommerce-orders-table.woocommerce-MyAccount-orders.shop_table.shop_table_responsive.my_account_orders.account-orders-table
tbody tr td:last-child\>a {

float: right;

}

.woocommerce-account div.wc-accontent-inner {

background: transparent;

border: none;

box-shadow: none;

}

td.woocommerce-orders-table\_\_cell.woocommerce-orders-table\_\_cell-order-number\>a
{

color: #1e73be;

font-weight: 400;

text-decoration: none;

}

.woocommerce-pagination.woocommerce-pagination\--without-numbers.woocommerce-Pagination
{

display: flex;

justify-content: center;

}

.woocommerce-pagination.woocommerce-pagination\--without-numbers.woocommerce-Pagination
a {

background: white;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

border-radius: 5px;

}

.woocommerce-pagination.woocommerce-pagination\--without-numbers.woocommerce-Pagination
a:hover {

background-color: #ebe9eb;

color: #0984ae;

}

.specific-div {

display: none;

background-color: #fff;

padding: 20px;

min-width: 260px;

max-width: 280px;

margin: 5px 0 0;

font-size: 14px;

border: none;

border-radius: 5px;

-webkit-box-shadow: 0 10px 30px rgba(0, 0, 0, .15);

box-shadow: 0 10px 30px rgba(0, 0, 0, .15);

position: absolute;

right: 8px;

top: 40px;

z-index: 99999;

}

.dot {

background-color: white !important;

}

.dot:hover {

background-color: #ebe9eb !important;

color: #0984ae !important;

}

.main_view:nth-child(odd) {

margin-bottom: 10px;

}

/\* .main_view:hover{

pointer-events: none !important;

} \*/

table.woocommerce-orders-table.woocommerce-MyAccount-orders.shop_table.shop_table_responsive.my_account_orders.account-orders-table
tbody tr td:last-child {

position: relative;

}

table.woocommerce-orders-table.woocommerce-MyAccount-orders.shop_table.shop_table_responsive.my_account_orders.account-orders-table
tbody tr {

position: static;

}

/\* ADDRESS PAGE \*/

section.elementor-section.elementor-top-section.elementor-element.elementor-element-bfd36e6.elementor-section-full_width.elementor-section-height-default.elementor-section-height-default
.woocommerce-MyAccount-content .u-column1.col-1.woocommerce-Address {

background: #fff;

border-radius: 5px;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

width: 452px;

/\* height: 207px; \*/

padding: 20px;

font-family: \"Open Sans\", sans-serif;

position: relative;

}

section.elementor-section.elementor-top-section.elementor-element.elementor-element-bfd36e6.elementor-section-full_width.elementor-section-height-default.elementor-section-height-default
.woocommerce-MyAccount-content .u-column1.col-1.woocommerce-Address
header.woocommerce-Address-title.title h3 {

font-size: 17px;

font-weight: bolder;

color: #414141;

}

section.elementor-section.elementor-top-section.elementor-element.elementor-element-bfd36e6.elementor-section-full_width.elementor-section-height-default.elementor-section-height-default
.woocommerce-MyAccount-content .u-column1.col-1.woocommerce-Address
header.woocommerce-Address-title.title a.edit {

font-size: 14px;

line-height: 18px;

padding: 8px 14px;

color: #0984ae;

background: #f4f4f4;

border-color: transparent;

border-radius: 5px;

border: 1px solid transparent;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

position: absolute;

top: 25px;

right: 20px;

}

section.elementor-section.elementor-top-section.elementor-element.elementor-element-bfd36e6.elementor-section-full_width.elementor-section-height-default.elementor-section-height-default
.woocommerce-MyAccount-content .u-column1.col-1.woocommerce-Address
header.woocommerce-Address-title.title a.edit:hover {

background-color: #ebe9eb;

}

section.elementor-section.elementor-top-section.elementor-element.elementor-element-bfd36e6.elementor-section-full_width.elementor-section-height-default.elementor-section-height-default
.woocommerce-MyAccount-content .u-column1.col-1.woocommerce-Address
header.woocommerce-Address-title.title+address {

font-weight: 400;

text-transform: none;

font-size: 17px;

line-height: 1.5;

font-style: normal;

color: #504a4adb;

font-family: sans-serif;

}

/\* 11-aug \*/

.woocommerce-edit-address
section.elementor-section.elementor-top-section.elementor-element.elementor-element-bfd36e6.elementor-section-full_width.elementor-section-height-default.elementor-section-height-default
.woocommerce-MyAccount-content .u-column1.col-1.woocommerce-Address
header.woocommerce-Address-title.title+address {

position: relative;

bottom: 14px;

}

.woocommerce-edit-address
section.elementor-section.elementor-top-section.elementor-element.elementor-element-bfd36e6.elementor-section-full_width.elementor-section-height-default.elementor-section-height-default
.woocommerce-MyAccount-content .u-column1.col-1.woocommerce-Address
header.woocommerce-Address-title.title+address\>div:nth-child(odd) {

margin-bottom: 16px;

}

.woocommerce-edit-address
section.elementor-section.elementor-top-section.elementor-element.elementor-element-bfd36e6.elementor-section-full_width.elementor-section-height-default.elementor-section-height-default
.woocommerce-MyAccount-content .u-column1.col-1.woocommerce-Address
header.woocommerce-Address-title.title+address\>div:not(:first-child) {

text-transform: uppercase;

}

.woocommerce-edit-address
section.elementor-section.elementor-top-section.elementor-element.elementor-element-bfd36e6.elementor-section-full_width.elementor-section-height-default.elementor-section-height-default
.woocommerce-MyAccount-content .u-column1.col-1.woocommerce-Address
header.woocommerce-Address-title.title+address\>div {

font-size: 17px;

color: #414141;

font-family: \"Open Sans\", sans-serif;

}

.userImg {

background-image:
url(https://secure.gravatar.com/avatar/4bb1746...?s=50&d=mm&r=g);

background-position: center center;

background-repeat: no-repeat;

height: 50px;

max-width: 50px;

border-radius: 4px;

margin-bottom: 2px;

}

/\* points-in-videos \*/

ul#menu-1-e71767d .sub-menu.elementor-nav-menu\--dropdown li a {

background: white;

}

ul#menu-1-e71767d .sub-menu.elementor-nav-menu\--dropdown li a:hover {

background-color: #E9EBED;

}

#menu-1-e71767d\>li
.elementor-item.elementor-item-active.has-submenu:first-child {

color: #FFFFFF;

fill: #FFFFFF;

}

#menu-1-e71767d\>li
.elementor-item.elementor-item-active.has-submenu:first-child:hover,

#menu-1-e71767d\>li
.elementor-item.elementor-item-active.has-submenu:first-child:focus {

color: #d28a40;

fill: #d28a40;

}

span.dashboard-menu-item-icon.icon-candle-stick {

font-family: \"Font Awesome 5 Free\";

font-weight: 400;

}

/\* span.dashboard-menu-item-icon.icon-candle-stick:after {

content: \"\\f152\";

} \*/

.woocommerce-account nav.woocommerce-MyAccount-navigation {

width: 280px;

}

.woocommerce-account nav.woocommerce-MyAccount-navigation.colsp {

width: 359px;

}

.woocommerce-account .wc-content-sction {

width: 100%;

}

/\* subscription-page \*/

table.my_account_subscriptions.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.shop_table.shop_table_responsive.woocommerce-orders-table\--subscriptions
{

background: none;

border: 0 !important;

border-collapse: separate;

border-spacing: 0 15px;

width: 100%;

}

table.my_account_subscriptions.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.shop_table.shop_table_responsive.woocommerce-orders-table\--subscriptions
tbody tr {

z-index: 1;

position: relative;

display: table-row;

border-radius: 5px;

background-color: transparent;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

table.my_account_subscriptions.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.shop_table.shop_table_responsive.woocommerce-orders-table\--subscriptions
tbody tr:hover {

z-index: 10;

-webkit-box-shadow: 0 20px 20px rgba(0, 0, 0, .1);

box-shadow: 0 20px 20px rgba(0, 0, 0, .1)

}

table.my_account_subscriptions.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.shop_table.shop_table_responsive.woocommerce-orders-table\--subscriptions
tbody tr td {

padding: 22.5px;

background-color: transparent;

border-bottom-width: 1px;

-webkit-box-shadow: inset 0 0 0 9999px var(\--bs-table-accent-bg);

box-shadow: inset 0 0 0 9999px var(\--bs-table-accent-bg);

padding-left: 20px;

padding-right: 20px;

font-size: 14px;

color: #212529;

border-bottom: 1px solid rgb(33, 37, 41);

font-family: \"Open Sans\", sans-serif;

}

table.my_account_subscriptions.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.shop_table.shop_table_responsive.woocommerce-orders-table\--subscriptions
thead tr th {

padding: 14.5px;

padding-left: 20px;

padding-right: 20px;

font-size: 14px;

font-weight: 700;

color: #212529;

line-height: 1.5;

}

table.my_account_subscriptions.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.shop_table.shop_table_responsive.woocommerce-orders-table\--subscriptions
tbody tr td:last-child\>a {

float: right;

background: #f4f4f4;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

border-radius: 5px;

color: #0984ae;

font-weight: 500;

}

table.my_account_subscriptions.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.shop_table.shop_table_responsive.woocommerce-orders-table\--subscriptions
tbody tr td:last-child\>a:hover {

background-color: #ebe9eb;

color: #0984ae;

}

table.my_account_subscriptions.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.shop_table.shop_table_responsive.woocommerce-orders-table\--subscriptions
tbody tr td:first-child {

border-top-left-radius: 5px;

border-bottom-left-radius: 5px;

}

table.my_account_subscriptions.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.shop_table.shop_table_responsive.woocommerce-orders-table\--subscriptions
tbody tr td:last-child {

border-top-right-radius: 5px;

border-bottom-right-radius: 5px;

}

td.subscription-status.order-status.woocommerce-orders-table\_\_cell.woocommerce-orders-table\_\_cell-subscription-status.woocommerce-orders-table\_\_cell-order-status\>span
{

display: inline-block;

height: 21px;

line-height: 21px;

padding: 0 5px;

font-size: 12px;

color: #666;

border-radius: 3px;

background-color: #f4f4f4;

text-transform: uppercase;

}

.cancelled {

color: #eb5757 !important;

background-color: #ffeceb !important;

}

.active {

color: #4bb877 !important;

background-color: #e1faea !important;

}

.onHold {

color: #0984ae !important;

background-color: #e1f5fa !important;

}

/\* subscription-view \*/

/\*10-aug \*/

.woocommerce-view-subscription a.button.reactivate,

.woocommerce-view-subscription a.button.cancel {

color: #0984ae;

font-size: 14px;

line-height: 18px;

font-weight: 500;

background: #f4f4f4;

border-color: transparent;

transition: all .15s ease-in-out;

border-radius: 5px;

}

.woocommerce-view-subscription a.button.reactivate:hover,

.woocommerce-view-subscription a.button.cancel:hover {

color: #0984ae;

background: #e7e7e7;

border-color: transparent

}

/\* 10-aug \*/

.woocommerce-view-subscription .wc-accontent-inner table {

padding: 40px;

background: white;

}

.woocommerce-view-subscription .woocommerce table.shop_table tbody tr
td:first-child {

border-left: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-view-subscription .woocommerce table.shop_table tbody tr
td:last-child {

border-right: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-view-subscription .woocommerce table.shop_table td {

border-top: 1px solid rgba(0, 0, 0, .1) !important;

}

.woocommerce-view-subscription .woocommerce table.shop_table tbody
tr:last-child td {

border-bottom: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-view-subscription table.shop_table.subscription_details
tbody tr td {

font-size: 17px;

color: #666666;

}

.woocommerce-view-subscription table td,

table th {

border: none;

}

.woocommerce-view-subscription table.shop_table.order_details thead tr
th {

border: none;

border-top: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-view-subscription table.shop_table.order_details thead tr
th {

border: none;

border-top: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-view-subscription tr.order_item td.product-name {

border: none;

border-left: none !important;

border-bottom: none !important;

}

.woocommerce-view-subscription tr.order_item td.product-total {

border-right: none !important;

border-bottom: none !important;

}

.woocommerce-view-subscription table.shop_table.order_details thead tr
th:first-child {

border-left: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-view-subscription table.shop_table.order_details thead tr
th:last-child {

border-right: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-view-subscription .woocommerce table.shop_table tbody
tr.order_item td:first-child {

border-left: 1px solid rgba(0, 0, 0, .1) !important;

}

.woocommerce-view-subscription .woocommerce table.shop_table tbody
tr.order_item td:last-child {

border-right: 1px solid rgba(0, 0, 0, .1) !important;

}

.woocommerce-view-subscription .wc-accontent-inner table\>tfoot tr
th:first-child {

border-left: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-view-subscription .wc-accontent-inner table\>tfoot tr
td:last-child {

border-right: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-view-subscription .wc-accontent-inner table\>tfoot
tr:last-child th:first-child {

border-bottom: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-view-subscription .wc-accontent-inner table\>tfoot
tr:last-child td:last-child {

border-bottom: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-view-subscription table.shop_table.order_details thead tr,

.woocommerce-view-subscription table.shop_table.order_details tbody tr,

.woocommerce-view-subscription table.shop_table.order_details tfoot tr {

font-size: 17px;

color: #414141;

}

/\* 3rd-table \*/

.woocommerce-view-subscription
table.shop_table.shop_table_responsive.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-orders.woocommerce-orders-table\--orders
thead,

.woocommerce-view-subscription
table.shop_table.shop_table_responsive.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-orders.woocommerce-orders-table\--orders
tbody {

font-size: 14.45px;

color: #414141;

}

.woocommerce-view-subscription a.woocommerce-button.button.invoice {

display: none;

}

/\* 4th-table \*/

.woocommerce-view-subscription section.woocommerce-customer-details {

padding: 40px;

background: white;

border: 1px solid rgba(0, 0, 0, .1);

border-radius: 5px;

}

.woocommerce-view-subscription section.woocommerce-customer-details
h2.woocommerce-column\_\_title {

display: none;

}

.woocommerce-view-subscription section.woocommerce-customer-details
h2.woocommerce-column\_\_title+address {

border-bottom-width: 1px;

border-right-width: 1px;

color: #666666f0;

font-size: 17px;

line-height: inherit;

font-weight: 400;

text-transform: none;

}

/\* DASHBOARD \*/

/\*
section.elementor-section.elementor-top-section.elementor-element.elementor-element-f966c5c.elementor-section-boxed.elementor-section-height-default.elementor-section-height-default
.woocommerce-account div.woocommerce-MyAccount-content {

border-bottom: 1px solid #d3ccccb3;

} \*/

section.elementor-section.elementor-top-section.elementor-element.elementor-element-f966c5c.elementor-section-boxed.elementor-section-height-default.elementor-section-height-default
.woocommerce-account .wc-content-sction {

width: 100%;

background: #f4f4f4;

}

section.elementor-section.elementor-top-section.elementor-element.elementor-element-f966c5c.elementor-section-boxed.elementor-section-height-default.elementor-section-height-default
.cselt-row {

flex-direction: column;

}

section.elementor-section.elementor-top-section.elementor-element.elementor-element-f966c5c.elementor-section-boxed.elementor-section-height-default.elementor-section-height-default
.cselt-row .col-sm4 {

width: 100%;

padding-bottom: 55px;

}

section.elementor-section.elementor-top-section.elementor-element.elementor-element-f966c5c.elementor-section-boxed.elementor-section-height-default.elementor-section-height-default
.membership-card\_\_actions a {

color: #1e73be;

font-weight: 400;

font-size: 14px;

}

.membership-cards .membership-card\_\_header {

font-size: 17px;

}

/\* .membership-cards .membership-card\_\_header:hover {

color: #0984ae;

} \*/

.elementor-kit-7 a {

font-family: \"Open Sans\", sans-serif;

}

.elementor-element.elementor-element-c581ea8.elementor-widget.elementor-widget-shortcode
.elementor-shortcode\>p {

display: none;

}

.custom-head-0,

.custom-head-1,

.custom-head-2 {

color: #333;

font-weight: 700;

font-size: 20px;

font-family: \"Open Sans\", sans-serif;

}

.cselt-row\>.custom-head-1,

.cselt-row\>.custom-head-2,

.cselt-row\>.custom-head-3 {

color: #333;

font-weight: 700;

font-size: 20px;

margin-top: 30px;

}

.cselt-row\>.custom-head-3 {

margin-top: 0px !important;

}

.custom-hr-0 {

border-top: 1px solid #dbdbdb;

width: 112%;

position: absolute;

left: -52px;

top: 245px;

}

.custom-hr-1 {

border-top: 1px solid #dbdbdb;

width: 112%;

position: absolute;

left: -52px;

top: 535px;

}

.custom-hr-2 {

border-top: 1px solid #dbdbdb;

width: 112%;

position: absolute;

left: -52px;

top: 825px;

}

/\* view 10-aug\*/

.woocommerce-account div.wc-accontent-inner mark {

background-color: #fcf8e3;

}

/\* .woocommerce table.shop_table td {

background: transparent!important;

} \*/

td.subscription-actions.order-actions.woocommerce-orders-table\_\_cell.woocommerce-orders-table\_\_cell-subscription-actions.woocommerce-orders-table\_\_cell-order-actions\>.view,

.woocommerce-view-subscription a.woocommerce-button.button.view {

color: #ffffff;

background-color: #666666;

font-size: 14.45px;

font-weight: 700;

border-radius: 3px;

}

td.subscription-actions.order-actions.woocommerce-orders-table\_\_cell.woocommerce-orders-table\_\_cell-subscription-actions.woocommerce-orders-table\_\_cell-order-actions\>.view:hover,

.woocommerce-view-subscription a.woocommerce-button.button.view:hover {

color: #0a2335;

background-color: #ebe9eb;

}

table.shop_table.shop_table_responsive.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.woocommerce-orders-table\--subscriptions
{

font-size: 14.45px;

color: #414141;

}

table.woocommerce-table.woocommerce-table\--order-details.shop_table.order_details
{

font-size: 17px;

color: #414141;

}

section.woocommerce-customer-details\>address {

font-size: 17px;

color: #666666f0;

}

/\*
table.woocommerce-table.woocommerce-table\--order-details.shop_table.order_details
td,table.shop_table.shop_table_responsive.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.woocommerce-orders-table\--subscriptions
td {

background: transparent !important;

} \*/

section.woocommerce-order-details h2,

section.woocommerce-customer-details h2 {

color: var(\--e-global-color-accent) !important;

}

table.woocommerce-table.woocommerce-table\--order-details.shop_table.order_details+header\>h2
{

margin-bottom: 20px;

margin-top: 40px;

}

/\*

.woocommerce-account .woocommerce h2{

display:none;

}

\*/

table.shop_table.shop_table_responsive.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.woocommerce-orders-table\--subscriptions
thead tr th {

border-bottom: 1px solid rgba(0, 0, 0, .1) !important;

}

.woocommerce-view-order .woocommerce table.shop_table td {

background: transparent !important;

}

/\* ACCOUNT-DETAILS \*/

.woocommerce-edit-account
.woocommerce-MyAccount-content\>.wc-accontent-inner {

padding: 40px;

background: #fff;

border-radius: 5px;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

margin: 0px 20px;

margin-top: 30px;

padding-top: 65px;

margin-bottom: 20px;

}

.woocommerce-edit-account
.woocommerce-MyAccount-content\>.wc-accontent-inner label {

color: #7e7d7e;

font-weight: 700;

font-family: \"Open Sans\", sans-serif;

}

.woocommerce-edit-account .woocommerce form .form-row input.input-text {

width: 100%;

height: 50px;

padding: 0 16px;

font-size: 17px;

line-height: 48px;

font-weight: 400;

color: #666666d1;

border: 1px solid #dbdbdb;

-webkit-box-shadow: none !important;

box-shadow: none !important;

border-radius: 5px;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

.woocommerce-edit-account .woocommerce form .form-row
input.input-text:hover {

border-color: #7e7d7e;

}

.woocommerce-edit-account input#account_display_name+span {

font-size: 17px;

font-family: \"Open Sans\", sans-serif;

color: #414141cc;

}

.woocommerce-edit-account fieldset {

border: none;

padding: 0px;

}

.woocommerce-edit-account fieldset\>legend {

color: #0984ae;

font-weight: 700;

font-size: 14px;

margin-bottom: 25px;

text-transform: uppercase;

/\* margin-top: 23px; \*/

margin-left: -10px;

position: relative;

top: 5px;

font-family: \"Open Sans\", sans-serif;

}

.woocommerce-edit-account button.woocommerce-Button.button {

color: #ffffff;

background: #f88b09;

border-color: #f88b09;

border: solid 1px #f99e31;

border-radius: 5px;

font-size: 16px;

font-weight: 700;

line-height: 1.8;

padding: 10px 20px;

cursor: pointer;

position: relative;

text-decoration: none;

overflow: visible;

font-family: \"sofia-pro\", Sans-serif;

}

.woocommerce-edit-account button.woocommerce-Button.button:focus {

outline: none !important;

border: solid 1px #f99e31;

}

.woocommerce-edit-account select#wcpay_selected_currency+span {

font-size: 17px;

font-family: \"Open Sans\", sans-serif;

color: #414141cc;

}

.woocommerce-edit-account select#wcpay_selected_currency {

color: #666666eb;

border: 1px solid #cbc2c2;

}

.woocommerce-edit-account fieldset\~p {

margin-left: 5px !important;

}

.woocommerce-edit-account span.radioBtnTxt {

margin-left: 15px;

position: relative;

bottom: 1px;

}

.woocommerce-edit-account .woocommerce-MyAccount-content:before {

content: \"Account Details\";

color: #333;

font-weight: 700;

font-size: 20px;

position: relative;

left: 20px;

font-family: \"Open Sans\", sans-serif;

}

.woocommerce-edit-account .woocommerce-MyAccount-content {

padding-top: 40px !important;

}

.woocommerce-edit-account
.woocommerce-MyAccount-content\>.wc-accontent-inner:before {

content: \"personal details\";

text-transform: uppercase;

color: #0984ae;

font-weight: 700;

font-size: 14px;

position: relative;

bottom: 25px;

font-family: \"Open Sans\", sans-serif;

letter-spacing: 1px;

left: -7px;

}

/\* view-table-border \*/

.woocommerce-view-subscription
table.shop_table.shop_table_responsive.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-orders.woocommerce-orders-table\--orders
thead tr th {

border-top: 1px solid rgba(0, 0, 0, .1) !important;

}

.woocommerce-view-subscription
table.shop_table.shop_table_responsive.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-orders.woocommerce-orders-table\--orders
thead tr th:first-child {

border-left: 1px solid rgba(0, 0, 0, .1) !important;

}

.woocommerce-view-subscription
table.shop_table.shop_table_responsive.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-orders.woocommerce-orders-table\--orders
thead tr th:last-child {

border-right: 1px solid rgba(0, 0, 0, .1) !important;

}

.woocommerce-view-order
table.woocommerce-table.woocommerce-table\--order-details.shop_table.order_details
thead tr th {

border: none;

}

.woocommerce-view-order tr.woocommerce-table\_\_line-item.order_item
td.woocommerce-table\_\_product-name.product-name,

.woocommerce-view-order tr.woocommerce-table\_\_line-item.order_item
td.woocommerce-table\_\_product-total.product-total {

border-top: 1px solid rgba(0, 0, 0, .1);

}

.woocommerce-subscriptions
table.shop_table.shop_table_responsive.my_account_orders.woocommerce-orders-table.woocommerce-MyAccount-subscriptions.woocommerce-orders-table\--subscriptions
thead tr th {

border-bottom: 0px solid rgba(0, 0, 0, .1) !important;

}

/\* ADDRESS-EDIT-FORM \*/

.woocommerce-edit-address .woocommerce-address-fields {

background: #fff;

border-radius: 5px;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

font-family: \"Open Sans\", sans-serif;

}

.woocommerce-edit-address .woocommerce-address-fields\_\_field-wrapper {

padding: 40px;

}

.woocommerce-edit-address .woocommerce form .form-row
input.input-text:hover {

border-color: #7e7d7e

}

.woocommerce-edit-address p#update_all_subscriptions_addresses_field {

border-top: 1px solid #dbdbdb;

margin: auto 40px;

}

.woocommerce-edit-address p#update_all_subscriptions_addresses_field\~p
{

/\* margin-top: 20px; \*/

padding: 40px;

}

.woocommerce-edit-address
p#update_all_subscriptions_addresses_field\~p\>button.button {

background: #f88b09;

border-color: #f88b09;

border: solid 1px #f88b09;

border-radius: 5px;

font-size: 16px;

font-weight: 700;

line-height: 1.8;

padding: 10px 20px;

color: #ffffff;

cursor: pointer;

width: 100px;

box-sizing: content-box;

}

.woocommerce-edit-address
p#update_all_subscriptions_addresses_field\~p\>button.button:hover,

.woocommerce-edit-address
p#update_all_subscriptions_addresses_field\~p\>button.button:focus,

.woocommerce-edit-address
p#update_all_subscriptions_addresses_field\~p\>button.button:visited {

color: #ffffff;

outline: none !important;

background: #f99e31;

border: solid 1px #f99e31;

}

.woocommerce-edit-address .woocommerce form .form-row input.input-text {

height: 50px;

color: #444;

height: 50px;

padding: 0 16px;

font-size: 17px;

line-height: 48px;

font-weight: 400;

/\* color: aliceblue; \*/

background: #fff !important;

border: 1px solid #dbdbdb;

-webkit-box-shadow: none !important;

box-shadow: none !important;

border-radius: 5px;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

.woocommerce-edit-address .select2-container\--default
.select2-selection\--single {

border: 1px solid #dbdbdb;

color: #444;

font-size: 17px;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

height: 50px;

line-height: 48px;

font-weight: 400;

display: flex;

align-items: center;

}

.woocommerce-edit-address .select2-container\--default
.select2-selection\--single .select2-selection\_\_arrow {

position: absolute;

top: 12px;

}

.woocommerce-edit-address div.wc-accontent-inner form\>h3 {

color: #333;

font-weight: 700;

font-size: 20px;

margin-bottom: 30px;

font-family: \"Open Sans\", sans-serif;

}

.woocommerce-edit-address p#update_all_subscriptions_addresses_field
span.woocommerce-input-wrapper {

position: relative;

top: 5px;

}

.woocommerce-edit-address .woocommerce form .form-row .input-checkbox {

width: 19px;

height: 19px;

position: relative;

bottom: 3px;

}

/\* Payment-Methods \*/

.woocommerce-payment-methods .wc-accontent-inner {

display: flex;

flex-direction: column-reverse;

}

.woocommerce-payment-methods .wc-content-sction {

background-color: #f4f4f4;

}

.woocommerce-payment-methods .woocommerce-account
div.wc-accontent-inner,

.woocommerce-payment-methods .woocommerce-account
div.woocommerce-MyAccount-content {

background-color: #f4f4f4;

}

.woocommerce-payment-methods
table.woocommerce-MyAccount-paymentMethods.shop_table.shop_table_responsive.account-payment-methods-table+.button
{

font-size: 14px;

line-height: 18px;

padding: 8px 14px;

font-weight: 500;

color: #0984ae !important;

background: #fff;

border-color: #fff;

transition: all .15s ease-in-out;

border-radius: 5px;

width: 147px;

height: 18px;

box-sizing: content-box;

margin-bottom: 8px;

}

.woocommerce-payment-methods
table.woocommerce-MyAccount-paymentMethods.shop_table.shop_table_responsive.account-payment-methods-table+.button:hover
{

background-color: transparent;

}

.woocommerce-payment-methods
table.woocommerce-MyAccount-paymentMethods.shop_table.shop_table_responsive.account-payment-methods-table
{

border: none;

border-spacing: 0 15px !important;

font-size: 0.85em;

}

.woocommerce-payment-methods
table.woocommerce-MyAccount-paymentMethods.shop_table.shop_table_responsive.account-payment-methods-table
thead tr th {

border: none;

padding: 0 20px;

font-size: 14px;

}

.woocommerce-payment-methods tr.payment-method {

z-index: 1;

position: relative;

display: table-row;

border-radius: 5px;

/\* background-color: #fff; \*/

background-color: transparent;

-webkit-box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

box-shadow: 0 1px 2px rgba(0, 0, 0, .15);

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

font-size: 0.85em;

height: 77px;

/\* margin-top: 15px; \*/

}

.woocommerce-payment-methods tr.payment-method:hover {

z-index: 10;

-webkit-box-shadow: 0 20px 20px rgba(0, 0, 0, .1);

box-shadow: 0 20px 20px rgba(0, 0, 0, .1)

}

.woocommerce-payment-methods tr.payment-method td {

padding: 20px !important;

/\* border: 1px solid rgba(0,0,0,.1); \*/

border-bottom: 1px solid #2f4239c7;

color: #212529;

font-size: 14.45px;

font-family: \"Open Sans\", sans-serif;

}

.woocommerce-payment-methods tr.payment-method td:first-child {

border-top-left-radius: 5px;

border-bottom-left-radius: 5px;

}

.woocommerce-payment-methods tr.payment-method td:last-child {

border-top-right-radius: 5px;

border-bottom-right-radius: 5px;

display: flex;

justify-content: flex-end;

height: 77px;

width: 80%;

gap: 5px;

padding-right: 50px !important;

}

.woocommerce-payment-methods
table.woocommerce-MyAccount-paymentMethods.shop_table.shop_table_responsive.account-payment-methods-table\>:not(:first-child)
{

border-top: 2px solid currentColor;

}

.woocommerce-payment-methods tr.payment-method td:last-child\>.button {

font-size: 100%;

margin: 0;

line-height: 1;

cursor: pointer;

position: relative;

text-decoration: none;

padding: 9px 14px;

font-weight: 700;

border-radius: 3px;

left: auto;

color: #515151;

background-color: #ebe9eb;

border: 0;

height: 14px;

box-sizing: content-box;

display: flex;

align-items: center;

justify-content: center;

}

.woocommerce-payment-methods tr.payment-method
td:last-child\>.button:hover {

background-color: #dfdcde;

color: #515151;

}

/\*

icons \*/

/\* .custom-head-0+.col-sm4 span.mbrship-card_icn {

background-image:
url(https://revolutiontradingpros.com/wp-content/uploads/2023/02/icon1.png)!important;

background-color: white;

background-position: center center;

background-repeat: no-repeat;

background-size: cover;

} \*/

.day-trading-card .fa-graduation-cap:before {

content: \"\" !important;

/\* background-image:
url(https://revolutiontradingpros.com/wp-content/uploads/2023/02/61ae223a84381-bpfull-1-fotor-bg-remover-2023081217353.png)
!important;

background-color: white;

background-position: center center;

background-repeat: no-repeat;

background-size: cover;

width: 40px !important;

height: 40px !Important;

box-shadow: 0 10px 20px rgba(60, 34, 241, .25);

display: inline-block;

margin-right: 9px;

line-height: 50px;

color: #fff;

text-align: center;

border-radius: 50%;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

position: relative;

bottom: -8px; \*/

}

.custom-head-0+.col-sm4 .membership-cards span.mbrship-card_icn {

width: 43px;

height: 43px;

background-color: transparent;

-webkit-box-shadow: 0 10px 20px rgba(243, 110, 27, .25);

box-shadow: 0 10px 20px rgba(243, 110, 27, .25);

}

.small-accounts-card .fa-graduation-cap:before {

content: \"\" !important;

/\* background-image:
url(https://revolutiontradingpros.com/wp-content/uploads/2023/02/processed-e88554c1-5f26-426a-82ee-534700fc1ac4_EJnmPH2F1-fotor-bg-remover-20230812162558bg-chng.png)
!important;

background-color: #30a596;

background-position: center center;

background-repeat: no-repeat;

background-size: cover;

width: 40px !important;

height: 40px !important;

box-shadow: 0 10px 20px rgba(60, 34, 241, .25);

display: inline-block;

margin-right: 9px;

line-height: 50px;

color: black;

text-align: center;

border-radius: 50%;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

position: relative;

bottom: -8px;

right: 0; \*/

}

.custom-head-1+.col-sm4 .membership-cards span.mbrship-card_icn {

/\* width:43px;

height:43px; \*/

background-color: transparent;

-webkit-box-shadow: 0 10px 20px rgba(0, 171, 175, .25);

box-shadow: 0 10px 20px rgba(0, 171, 175, .25);

}

.membership-cards .membership-card\_\_header {

display: flex;

align-items: center;

}

/\* span.dashboard-menu-item-icon.icon-candle-stick:after{

content: \"\"!important;

background-image:
url(https://revolutiontradingpros.com/wp-content/uploads/2023/02/icons8-myspace-1.svg)!important;

background-color: #fff;

background-position: center center;

background-repeat: no-repeat;

background-size: cover;

display: inline-block;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

width: 34px;

height: 34px;

border-radius: 50%;

background-color: #0f2d41;

position: absolute;

left: -6px;

/\* font-size: 19px; \*/

bottom: 0px;

opacity:0.5;

}

\*/ li.small-accounts-mp .fa-graduation-cap:before {

content: \"\" !important;

background-image:
url(https://revolutiontradingpros.com/wp-content/uploads/2023/02/processed-e88554c1-5f26-426a-82ee-534700fc1ac4_EJnmPH2F_1\_-removebg-preview.png)
!important;

background-color: #fff;

background-position: center center;

background-repeat: no-repeat;

background-size: cover;

display: inline-block;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

width: 40px;

height: 40px;

border-radius: 50%;

background-color: #0f2d41;

position: absolute;

left: -6px;

bottom: -8px;

opacity: 0.5;

}

li.day-trading-mp .fa-graduation-cap:before {

content: \"\" !important;

background-image:
url(https://revolutiontradingpros.com/wp-content/uploads/2023/02/61ae223a84381-bpfull-1-fotor-bg-remover-2023081217542.png)
!important;

background-color: #fff;

background-position: center center;

background-repeat: no-repeat;

background-size: cover;

display: inline-block;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

width: 40px;

height: 40px;

border-radius: 50%;

background-color: #0f2d41;

position: absolute;

left: -6px;

bottom: -8px;

opacity: 0.6;

}

p.dashboard-menu-category {

margin-bottom: 20px !important;

}

/\* ul.account-primary-menu.is-collapsed li:nth-child(4) {

content: \"\"!important;

background-image:
url(https://revolutiontradingpros.com/wp-content/uploads/2023/02/61ae223a84381-bpfull-1-fotor-bg-remover-2023081217542.png)!important;

background-color: #fff;

background-position: center center;

background-repeat: no-repeat;

background-size: cover;

display: inline-block;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

width: 40px;

height: 40px;

border-radius: 50%;

background-color: #0f2d41;

position: relative;

left: 19px;

opacity:0.6;

} \*/

/\* ul.account-primary-menu.is-collapsed li:nth-child(5){

content: \"\"!important;

background-image:
url(https://revolutiontradingpros.com/wp-content/uploads/2023/02/processed-e88554c1-5f26-426a-82ee-534700fc1ac4_EJnmPH2F_1\_-removebg-preview.png)!important;

background-color: #fff;

background-position: center center;

background-repeat: no-repeat;

background-size: cover;

display: inline-block;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

width: 40px;

height: 40px;

border-radius: 50%;

background-color: #0f2d41;

position: relative;

left: 19px;

opacity:0.6;

} \*/

.fa-graduation-cap:before {

content: \"\" !important;

}

\@import
url(\'https://fonts.googleapis.com/css2?family=Encode+Sans+Condensed&display=swap\');

/\*.dashb_headr:before {

content: \"My Account\";

margin: 0;

color: #333;

font-size: 36px;

font-weight: 700;

letter-spacing: -.01em;

line-height: 1.35;

font-family: \"Open Sans Condensed\", sans-serif!important;

}\*/

.wc-content-sction .dashb_headr {

padding: 20px;

}

.account-secondry-menu {

/\* width: 250px; \*/

width: calc(100% - 20px);

box-sizing: content-box;

padding: 20px 15px;

}

/\* ul.account-secondry-menu li a {

padding-top: 16px!important;

height: 14px;

box-sizing: content-box;

padding-left: 20px;

transition: color .1s ease-in-out,background-color .1s ease-in-out;

border-radius: 5px!important;

display: flex;

align-items: center;

}

ul.account-secondry-menu li {

width: 250px;

height: 45px;

}

ul.account-secondry-menu {

display: flex;

flex-direction: column;

gap: 10px;

} \*/

.blog.page-id-818.theme-hello-elementor.woocommerce-js.wp-custom-logo
.has_ae_slider.elementor-section.elementor-inner-section
.has_ae_slider.elementor-column.elementor-col-100.elementor-inner-column.elementor-element.elementor-element-74b3855.ae-bg-gallery-type-default
{

display: none;

}

/\* svg {

fill: #CACBD2;

}

\*/

.blog h2.elementor-heading-title.elementor-size-default {

display: none;

}

/\*Start CSS By Ghulam\*/

.video-container {

position: relative;

overflow: hidden;

width: 100%;

padding-top: 56.25%;

}

.video-iframe {

position: absolute;

top: 0;

left: 0;

bottom: 0;

right: 0;

width: 100%;

height: 100%;

}

.course-video {

width: 100%;

border: 24px solid #f4f4f4;

}

.course-video h2,

.course-video h3 {

font-weight: 400;

letter-spacing: -.01em;

}

.course-video h2 {

font-size: 200%;

color: #fff;

}

.course-video h3 {

color: #d5d5d5;

font-size: 150%;

}

.loading {

display: none;

top: 0px;

left: 0px;

background: black;

color: white;

width: 100%;

height: 100%;

position: absolute;

z-index: 9999;

}

.loading p {

font-size: 300%;

margin: 0;

padding: 0;

display: inline-block;

position: absolute;

top: 50%;

left: 50%;

transform: translate(-50%, -50%);

}

ul.video_parts {

padding: 0px;

margin-top: 2%;

display: grid;

grid-template-columns: repeat(auto-fill, 200px);

grid-gap: 10px;

justify-content: space-between;

}

ul.video_parts li {

position: relative;

overflow: hidden;

float: left;

width: 100%;

padding-top: 56.25%;

margin-bottom: 0;

}

.video-cards {

padding: 10px;

color: white;

background: black;

position: absolute;

top: 0;

left: 0;

bottom: 0;

right: 0;

width: 100%;

height: 100%;

}

.playbutton {

cursor: pointer;

z-index: 999;

font-size: 300%;

margin: 0;

padding: 0;

display: inline-block;

position: absolute;

top: 50%;

left: 50%;

transform: translate(-50%, -50%);

}

.playbutton:before {

content: \'\\25B6\';

}

.playing:after {

content: \': playing\'

}

.woocommerce-form-coupon {

background: white;

}

.woocommerce-form-coupon p {

font-size: 120%;

}

.elementor-nav-menu a.elementor-sub-item.elementor-item-active {

background: initial;

color: initial;

}

.elementor-5811 .ha-pg-classic .ha-pg-thumb img {

width: initial !important;

height: 100% !important;

}

.ha-pg-item .ha-pg-thumb-area .ha-pg-thumb {

width: initial

}

div.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout {

width: min-content

}

.single-post .elementor-widget-theme-post-content {

width: 85% !important;

margin: 0px auto !important;

}

.woocommerce-checkout table.cart img {

width: 100% !important;

border-radius: 5px;

}

.liOfRelevant a.head {

line-height: 1em !important;

}

.liOfRelevant p.thmb {

position: absolute;

max-height: 95px;

overflow: hidden;

width: 33%;

display: inline-block;

top: 30%;

margin-bottom: 50px;

border-radius: 5px;

}

.liOfRelevant p.desc {

width: 60%;

line-height: 1.2 !important;

text-align: left;

right: 0;

}

.attachment-woocommerce_thumbnail {

border-radius: 5px !important;

}

\@media screen and (max-width: 768px) {

div.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout {

width: 100%;

}

}

div.wpmc-steps-wrapper\>form.checkout.woocommerce-checkout {

width: 100%;

}

.share-icons-wrap {

text-align: right;

min-width: 100%;

}

li.share-title {

font-size: 18px;

font-weight: bold;

text-align: center;

width: 55%;

padding-top: 16px;

}

ul.page-share-list {

list-style: none;

padding: 0;

display: flex;

border-radius: 10px;

overflow: hidden;

border: 1px solid #bebebe;

align-items: center;

justify-content: center;

height: 56px;

width: 100%;

}

ul.page-share-list li {

background: #fff;

border-right: 1px solid #bebebe;

transition: background-color 100ms linear;

background-color: #fff;

background-repeat: no-repeat;

background-position: center center;

background-size: 25px;

cursor: pointer;

height: 100%;

width: 15%;

height: 56px;

}

li.share-title {

width: 55% !important;

pointer-events: none;

}

ul.page-share-list li.fb-wrap:hover {

background-color: #1778f2;

}

ul.page-share-list li.tweet-wrap:hover {

background-color: #00acee;

}

ul.page-share-list li.fb-wrap {

background-image:
url(\'https://revolutiontradingpros.com/wp-content/uploads/2024/01/facebook-icon.svg\');

}

ul.page-share-list li.fb-wrap:hover {

background-image:
url(\'https://revolutiontradingpros.com/wp-content/uploads/2024/01/facebook-icon-white.svg\');

background-color: #0e6ac4;

}

ul.page-share-list li.tweet-wrap {

background-image:
url(\'https://revolutiontradingpros.com/wp-content/uploads/2024/01/x-icon.png\');

}

ul.page-share-list li.tweet-wrap:hover {

background-image:
url(\'https://revolutiontradingpros.com/wp-content/uploads/2024/01/x-icon-white.png\');

background-color: #000;

}

ul.page-share-list li.share-wrap {

background-image:
url(\'https://revolutiontradingpros.com/wp-content/uploads/2024/01/share-icon.png\');

margin-right: -1px;

}

ul.page-share-list li.share-wrap:hover {

background-image:
url(\'https://revolutiontradingpros.com/wp-content/uploads/2024/01/share-icon-white.png\');

background-color: #414141;

}

ul.page-share-list li:last-child {

border-right: none;

}

ul.page-share-list li div {

cursor: pointer;

display: flex;

align-items: center;

justify-content: center;

}

ul.page-share-list img {

height: 28px;

-webkit-transition: -webkit-transform 0.4s;

transition: transform 0.4s;

}

ul.page-share-list img:hover {

-webkit-transform: scale(1.2) rotate(0.01deg);

transform: scale(1.2) rotate(0.01deg);

}

.share-page-link {

pointer-events: none;

user-select: none;

position: absolute;

right: 0;

bottom: -25;

color: #fff;

}

\@media only screen and (max-width: 767px) {

.page-link {

bottom: -15px;

}

.share-title {

display: none;

}

.share-icons-wrap {

text-align: left;

}

ul.page-share-list li {

width: 33%;

}

ul.page-share-list {

justify-content: flext-start;

width: 100%;

}

ul.page-share-list li.fb-wrap,

ul.page-share-list li.share-wrap {

width: 34%;

}

.share-page-link {

font-size: 55%;

}

}

h4.ha-pg-title {

text-align: center;

}

h6.ha-pg-title {

text-align: center;

}

.ha-pg-date-text svg {

display: none;

}

.disclaimer-area {

display: none;

position: fixed;

top: 0;

left: 0;

width: 100%;

height: 100%;

background: rgba(0, 0, 0, 80%);

z-index: 9999999;

}

.disclaimer-box {

display: flex;

flex-direction: column;

width: 100%;

max-width: 500px;

background: white;

position: absolute;

top: 50%;

left: 50%;

transform: translate(-50%, -50%);

padding: 10px;

border-radius: 10px;

font-size: 16px;

height: 80%;

max-height: 300px;

}

.disclaimer-content {

flex-grow: 1;

height: 95%;

max-height: calc(100% - 25px);

overflow-y: auto;

margin-bottom: 5px;

}

.disclaimer-buttons button {

position: relative;

background-color: dodgerblue;

color: white;

border-color: blue;

right: auto;

}

.accept-disclaimer {

float: left;

left: 15px;

}

.deny-disclaimer {

float: right;

right: 15px;

}

/\* Start change the icon color for the learning center from black to
#C5CFD5 and for bullets from black to #153e59 \*/

span.dashboard-menu-item-icon {

fill: #C5CFD5;

}

ul.account-secondry-menu {

color: #153e59;

}

/\* End change the icon color for the learning center from black to
#C5CFD5 and for bullets from black to #153e59 \*/

/\* Changed the icon height for icon-handle-stick \"My Indicators\" on
line# 911 from 34px to 54px \*/

/\* The following CSS is for the icons, under \"Enter a Trading Room\"
dropdown. \*/

.enter-roomclas .room-name {

display: inline-block;

}

.enter-roomclas li {

position: relative

}

/\* line 126 \*/

ul.enter-roomclas\>li ul li a {

padding: 0 10px 10px 10px;

padding-bottom: 10px;

margin-top: 10px;

display: block;

font-size: 15px;

display: flex;

align-items: center;

}

ul.enter-roomclas\>li ul li a:hover {

background-color: #f4f4f4;

}

/\* Following is the CSS for hover effect for memberships \*/

.membership-cards:hover .fa-graduation-cap::before {

box-shadow: 0 5px 20px rgba(0, 0, 0, .5);

}

.page-id-194 .checkout-main-div {

max-width: 1200px;

margin: auto;

padding: 12px

}

/\* hover effects related css for account page\*/

.account-secondry-menu {

list-style: none;

padding: 15px 0

}

.account-secondry-menu li {

display: block;

}

.account-secondry-menu li a {

transition: all 0s;

width: 100%;

}

.woocommerce-MyAccount-navigation-link {

padding: 0px 15px

}

.account-secondry-menu li a:hover {

background: #12354c;

}

.account-secondry-menu .is-active a {

background: #0984ae;

color: white;

opacity: 1 !important

}

/\* .account-secondry-menu li:first-child{

margin-bottom: 5px;

background: #0984ae;

} \*/

.cstooltip a:hover::before,

span.dashboard-menu-item-icon:hover,

.account-primary-menu li .fa-graduation-cap:hover::before {

opacity: 1 !important

}

span.dashboard-menu-item-icon:hover {

color: white;

}

a span.icon-candle-stick:hover {

filter: brightness(1.5)

}

.account-secondry-menu li a {

padding: 15px 15px

}

/\*Start css for the logo on the my-account login page \*/

.login-logo {

text-align: center;

margin-bottom: 20px;

}

.login-logo img {

max-width: 200px;

/\* Adjust as needed \*/

height: auto;

display: block;

margin: 0 auto;

}

/\*End css for the logo on the my-account login page \*/

/\* Start CSS for headings in body 15th March 2025\*/

h1 {

font-size: 28px;

}

h2 {

font-size: 24px;

}

h3 {

font-size: 20px;

}

h4 {

font-size: 18px;

}

h5 {

font-size: 14px;

}

h6 {

font-size: 12px;

}

/\* End CSS for headings in body 15th March 2025\*/

/\* SPX Profit Pluse CSS Starts\*/

/\* menu icon \*/

li.spx-profit-pulse span.cstooltiptext+a:before {

background-image:
url(https://revolutiontradingpros.com/wp-content/uploads/2023/02/unnamed.png)
!important;

background-color: #fff;

}

li.spx-profit-pulse span.cstooltiptext+a:before {

content: \"\" !important;

background-position: center center;

background-repeat: no-repeat;

background-size: cover;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

width: 34px;

height: 34px;

border-radius: 50%;

position: absolute;

left: 47px;

opacity: 0.5;

top: 34px;

}

/\* trading room \*/

.spx-profit-pulse-card .fa-graduation-cap:before {

content: \"\" !important;

background-image:
url(https://revolutiontradingpros.com/wp-content/uploads/2023/02/unnamed-fotor-bg-remover-20230819122532.png)
!important;

background-position: center center;

background-repeat: no-repeat;

background-size: cover;

width: 50px !important;

height: 50px !important;

-webkit-box-shadow: 0 10px 20px rgba(12, 36, 52, .25);

box-shadow: 0 10px 20px rgba(12, 36, 52, .25);

display: inline-block;

margin-right: 9px;

line-height: 50px;

color: black;

text-align: center;

border-radius: 50%;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

position: relative;

bottom: 1px;

right: 5px;

}

/\* not collapsed menu \*/

li.spx-profit-pulse-mp .fa-graduation-cap:before {

content: \"\";

background-image:
url(https://revolutiontradingpros.com/wp-content/uploads/2023/02/unnamed.png)
!important;

background-color: #fff;

background-position: center center;

background-repeat: no-repeat;

background-size: cover;

display: inline-block;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

width: 34px;

height: 34px;

border-radius: 50%;

position: absolute;

left: -2px;

bottom: -8px;

opacity: 0.5;

}

.account-primary-menu:not(.is-collapsed) a:hover
.fa-graduation-cap:before {

opacity: 1;

}

/\* SPX Profit Pluse CSS End\*/

/\* Indicators Page Cards CSS Start\*/

#my-indicators-cards .col-sm4 {

padding: 0;

margin-top: 30px;

margin-right: unset;

}

#my-indicators-cards .membership-cards {

padding: unset !important;

margin-top: unset;

padding: unset;

border: 2px solid rgba(0, 0, 0, .125);

transition: all .3s ease-in-out;

}

#my-indicators-cards .membership-cards\>a {

color: #414141;

padding: 15px;

display: block;

transition: all .5s ease;

}

#my-indicators-cards .membership-cards\>a:hover {

color: #1e73be;

}

#my-indicators-cards .course-card\_\_header {

font-family: \"Open Sans Condensed Bold\", sans-serif;

height: unset;

padding: 11px 11px 4px 0;

margin-bottom: unset;

line-height: 1.2;

}

#my-indicators-cards .membership-card\_\_actions {

display: block;

text-align: left;

padding: 0 15px 15px 15px;

border: unset;

}

#my-indicators-cards .membership-card\_\_actions\>a {

font-size: 14px !important;

font-weight: 600 !important;

line-height: 16px;

padding: 6px 8px !important;

background: #f4f4f4;

border-color: transparent;

transition: all .5s ease;

}

#my-indicators-cards .membership-card\_\_actions\>a:hover {

color: #0984ae;

background: #e7e7e7;

border-color: transparent;

}

/\* Indicators Page Cards CSS End\*/

/\* checkout page terms and conditions styles \*/

.woocommerce-terms-and-conditions-wrapper {

text-transform: none;

font-size: 17px;

line-height: 1.5;

font-family: \\\"Open Sans\\\", sans-serif;

}

.woocommerce-terms-and-conditions-wrapper
.woocommerce-terms-and-conditions-checkbox-text {

font-weight: 400;

}

.woocommerce form .form-row .input-checkbox {

display: inline;

margin: -2px 8px 0 0;

text-align: center;

vertical-align: middle;

}

.woocommerce-terms-and-conditions-wrapper input\[type=checkbox\] {

display: inline;

margin: -2px 8px 0 0;

text-align: center;

vertical-align: middle;

top: 2px;

width: 20px !important;

height: 20px !important;

border-radius: 3px;

outline: none !important;

}

.woocommerce-terms-and-conditions-wrapper a {

color: #1e73be;

font-weight: 400;

text-decoration: none;

}

.form-row.woocommerce-invalid label {

color: inherit !important;

}

/\* checkout page terms and conditions styles end \*/

/\* checkout page side bar \*/

.cart-table-wrapper {

padding: 15px 20px;

border-top: 1px solid #dbdbdb

}

.cart-table td {

border: 0px

}

.cart-table {

border: 1px solid #dbdbdb !important;

border-radius: 5px;

font-size: 14px;

margin: 15px 0 0 !important

}

table.cart-table tbody\>tr:nth-child(odd)\>td,

table tbody\>tr:nth-child(odd)\>th {

background-color: initial !important;

}

table.cart-table tbody tr:hover\>td,

table.cart-table tbody tr:hover\>th {

background-color: initial !important;

}

.cart-table tbody tr:first-child th\[colspan=\"2\"\]:last-child {

border-top-right-radius: 5px;

text-align: left;

background-color: hsla(0, 0%, 50.2%, .0705882353) !important;

}

.cart-table tbody tr td,

.cart-table tbody tr th,

.cart-table tfoot tr td,

.cart-table tfoot tr th,

.cart-table thead tr td,

.cart-table thead tr th {

border-top: 1px solid #ebe9eb !important;

border-right: none !important;

border-bottom: none !important;

padding: 12px !important;

line-height: 18px !important

}

/\* checkout sidebar end \*/

/\*End CSS By Ghulam\*/

/\* Layout Fixes \*/

a.start-here-btn:hover {

color: #0984ae;

background: #e7e7e7;

border-color: transparent;

}

a.start-here-btn {

font-size: 14px;

line-height: 18px;

padding: 8px 14px;

font-weight: 600;

color: #0984ae;

background: #f4f4f4;

border-color: transparent;

margin-left: 10px;

}

.woocommerce-MyAccount-navigation {

margin-top: 0 !important;

}

.account-primary-menu a,

.dash-sidebar-icon {

display: flex !important;

align-items: center;

gap: 10px;

flex-wrap: nowrap;

padding: 0 !important;

}

.account-primary-menu {

padding: 30px 0 30px 30px !important

}

.account-primary-menu p {

padding: 0 !important;

padding-top: 30px !important;

}

.account-primary-menu li,

.account-secondry-menu li {

list-style: none !important;

}

.account-primary-menu.is-collapsed a,

.dash-sidebar-icon.is-collapsed {

justify-content: center;

}

.account_primary_menu_active::after {

top: 4px !important;

}

span.dashboard-profile-photo:before {

content: \"\";

background:
url(\"https://secure.gravatar.com/avatar/3a70a6b6793acd7d0f7aad0908d7bed2?s=32&d=mm&r=g\");

width: 32px;

height: 32px;

border-radius: 30px;

position: unset;

display: block;

border: 1px solid #fff;

background-size: cover;

}

.account-primary-menu.is-collapsed .dashboard-menu-item-icon:before {

position: absolute;

display: block;

content: \"\";

top: 50%;

left: 50%;

width: 50px;

height: 50px;

margin-top: -25px;

margin-left: -25px;

border-radius: 50%;

-webkit-transform: scale(.9);

-ms-transform: scale(.9);

transform: scale(.9);

background: transparent;

-webkit-transition: all .15s ease-in-out;

-o-transition: all .15s ease-in-out;

transition: all .15s ease-in-out;

}

.account-primary-menu.is-collapsed
.dashboard-menu-item-icon:hover:before {

-webkit-transform: scale(1);

-ms-transform: scale(1);

transform: scale(1);

background-color: rgba(0, 0, 0, .2);

}

.woocommerce-MyAccount-content.explosive_swings {

padding: 0 !important;

}

/\* Layout Fixes \*/
