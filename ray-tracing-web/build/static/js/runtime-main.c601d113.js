!function(e){function t(t){for(var r,u,a=t[0],c=t[1],s=t[2],f=0,l=[];f<a.length;f++)u=a[f],Object.prototype.hasOwnProperty.call(o,u)&&o[u]&&l.push(o[u][0]),o[u]=0;for(r in c)Object.prototype.hasOwnProperty.call(c,r)&&(e[r]=c[r]);for(p&&p(t);l.length;)l.shift()();return i.push.apply(i,s||[]),n()}function n(){for(var e,t=0;t<i.length;t++){for(var n=i[t],r=!0,u=1;u<n.length;u++){var a=n[u];0!==o[a]&&(r=!1)}r&&(i.splice(t--,1),e=c(c.s=n[0]))}return e}var r={},o={2:0},i=[];var u={};var a={141:function(){return{"./index_bg.js":{__wbg_new_59cb74e423758ede:function(){return r[140].exports.f()},__wbg_stack_558ba5917b466edd:function(e,t){return r[140].exports.j(e,t)},__wbg_error_4bb6c2a97407129a:function(e,t){return r[140].exports.c(e,t)},__wbindgen_object_drop_ref:function(e){return r[140].exports.l(e)},__wbg_getRandomValues_f5e14ab7ac8e995d:function(e,t,n){return r[140].exports.e(e,t,n)},__wbg_randomFillSync_d5bd2d655fdf256a:function(e,t,n){return r[140].exports.g(e,t,n)},__wbg_self_1b7a39e3a92c949c:function(){return r[140].exports.i()},__wbg_require_604837428532a733:function(e,t){return r[140].exports.h(e,t)},__wbg_crypto_968f1772287e2df0:function(e){return r[140].exports.b(e)},__wbindgen_is_undefined:function(e){return r[140].exports.k(e)},__wbg_getRandomValues_a3d34b4fee3c2869:function(e){return r[140].exports.d(e)},__wbindgen_throw:function(e,t){return r[140].exports.m(e,t)}}}}};function c(t){if(r[t])return r[t].exports;var n=r[t]={i:t,l:!1,exports:{}};return e[t].call(n.exports,n,n.exports,c),n.l=!0,n.exports}c.e=function(e){var t=[],n=o[e];if(0!==n)if(n)t.push(n[2]);else{var r=new Promise((function(t,r){n=o[e]=[t,r]}));t.push(n[2]=r);var i,s=document.createElement("script");s.charset="utf-8",s.timeout=120,c.nc&&s.setAttribute("nonce",c.nc),s.src=function(e){return c.p+"static/js/"+({}[e]||e)+"."+{0:"d56b4d64"}[e]+".chunk.js"}(e);var f=new Error;i=function(t){s.onerror=s.onload=null,clearTimeout(l);var n=o[e];if(0!==n){if(n){var r=t&&("load"===t.type?"missing":t.type),i=t&&t.target&&t.target.src;f.message="Loading chunk "+e+" failed.\n("+r+": "+i+")",f.name="ChunkLoadError",f.type=r,f.request=i,n[1](f)}o[e]=void 0}};var l=setTimeout((function(){i({type:"timeout",target:s})}),12e4);s.onerror=s.onload=i,document.head.appendChild(s)}return({0:[141]}[e]||[]).forEach((function(e){var n=u[e];if(n)t.push(n);else{var r,o=a[e](),i=fetch(c.p+""+{141:"2f12cff2dc7a378c4317"}[e]+".module.wasm");if(o instanceof Promise&&"function"===typeof WebAssembly.compileStreaming)r=Promise.all([WebAssembly.compileStreaming(i),o]).then((function(e){return WebAssembly.instantiate(e[0],e[1])}));else if("function"===typeof WebAssembly.instantiateStreaming)r=WebAssembly.instantiateStreaming(i,o);else{r=i.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,o)}))}t.push(u[e]=r.then((function(t){return c.w[e]=(t.instance||t).exports})))}})),Promise.all(t)},c.m=e,c.c=r,c.d=function(e,t,n){c.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:n})},c.r=function(e){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},c.t=function(e,t){if(1&t&&(e=c(e)),8&t)return e;if(4&t&&"object"===typeof e&&e&&e.__esModule)return e;var n=Object.create(null);if(c.r(n),Object.defineProperty(n,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var r in e)c.d(n,r,function(t){return e[t]}.bind(null,r));return n},c.n=function(e){var t=e&&e.__esModule?function(){return e.default}:function(){return e};return c.d(t,"a",t),t},c.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},c.p="/rust-ray-tracing-demo/ray-tracing-web/build/",c.oe=function(e){throw console.error(e),e},c.w={};var s=this["webpackJsonpray-tracing-web"]=this["webpackJsonpray-tracing-web"]||[],f=s.push.bind(s);s.push=t,s=s.slice();for(var l=0;l<s.length;l++)t(s[l]);var p=f;n()}([]);
//# sourceMappingURL=runtime-main.c601d113.js.map