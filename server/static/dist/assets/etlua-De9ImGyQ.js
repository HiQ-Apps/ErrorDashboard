import{g as s}from"./index-Azymtqpk.js";import{r as f}from"./lua-DER4jxlW.js";import{r as p}from"./markup-templating-BxAVv-bL.js";function c(u,n){for(var a=0;a<n.length;a++){const t=n[a];if(typeof t!="string"&&!Array.isArray(t)){for(const e in t)if(e!=="default"&&!(e in u)){const r=Object.getOwnPropertyDescriptor(t,e);r&&Object.defineProperty(u,e,r.get?r:{enumerable:!0,get:()=>t[e]})}}}return Object.freeze(Object.defineProperty(u,Symbol.toStringTag,{value:"Module"}))}var o,l;function d(){if(l)return o;l=1;var u=f(),n=p();o=a,a.displayName="etlua",a.aliases=[];function a(t){t.register(u),t.register(n),function(e){e.languages.etlua={delimiter:{pattern:/^<%[-=]?|-?%>$/,alias:"punctuation"},"language-lua":{pattern:/[\s\S]+/,inside:e.languages.lua}},e.hooks.add("before-tokenize",function(r){var g=/<%[\s\S]+?%>/g;e.languages["markup-templating"].buildPlaceholders(r,"etlua",g)}),e.hooks.add("after-tokenize",function(r){e.languages["markup-templating"].tokenizePlaceholders(r,"etlua")})}(t)}return o}var i=d();const m=s(i),y=c({__proto__:null,default:m},[i]);export{y as e};
