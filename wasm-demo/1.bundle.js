(window.webpackJsonp=window.webpackJsonp||[]).push([[1],[,,function(t,n,e){"use strict";e.r(n);var r=e(3);e.d(n,"normalize_text",(function(){return r.i})),e.d(n,"AskalonoStore",(function(){return r.a})),e.d(n,"LicenseInfo",(function(){return r.b})),e.d(n,"MatchResult",(function(){return r.c})),e.d(n,"__wbg_new_0d50725e1ae68303",(function(){return r.d})),e.d(n,"__wbindgen_string_new",(function(){return r.g})),e.d(n,"__wbg_push_46274b393147c746",(function(){return r.e})),e.d(n,"__wbindgen_object_drop_ref",(function(){return r.f})),e.d(n,"__wbindgen_throw",(function(){return r.h}))},function(t,n,e){"use strict";(function(t){e.d(n,"i",(function(){return g})),e.d(n,"a",(function(){return v})),e.d(n,"b",(function(){return x})),e.d(n,"c",(function(){return j})),e.d(n,"d",(function(){return O})),e.d(n,"g",(function(){return k})),e.d(n,"e",(function(){return m})),e.d(n,"f",(function(){return A})),e.d(n,"h",(function(){return T}));var r=e(4);let u=new("undefined"==typeof TextDecoder?(0,t.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});u.decode();let o=null;function c(){return null!==o&&o.buffer===r.o.buffer||(o=new Uint8Array(r.o.buffer)),o}function i(t,n){return u.decode(c().subarray(t,t+n))}const f=new Array(32).fill(void 0);f.push(void 0,null,!0,!1);let s=f.length;function l(t){s===f.length&&f.push(f.length+1);const n=s;return s=f[n],f[n]=t,n}function a(t){return f[t]}function d(t){const n=a(t);return function(t){t<36||(f[t]=s,s=t)}(t),n}let p=null;function h(){return null!==p&&p.buffer===r.o.buffer||(p=new Int32Array(r.o.buffer)),p}let _=0;let b=new("undefined"==typeof TextEncoder?(0,t.require)("util").TextEncoder:TextEncoder)("utf-8");const w="function"==typeof b.encodeInto?function(t,n){return b.encodeInto(t,n)}:function(t,n){const e=b.encode(t);return n.set(e),{read:t.length,written:e.length}};function y(t,n,e){if(void 0===e){const e=b.encode(t),r=n(e.length);return c().subarray(r,r+e.length).set(e),_=e.length,r}let r=t.length,u=n(r);const o=c();let i=0;for(;i<r;i++){const n=t.charCodeAt(i);if(n>127)break;o[u+i]=n}if(i!==r){0!==i&&(t=t.slice(i)),u=e(u,r,r=i+3*t.length);const n=c().subarray(u+i,u+r);i+=w(t,n).written}return _=i,u}function g(t){try{var n=y(t,r.e,r.f),e=_;r.p(8,n,e);var u=h()[2],o=h()[3];return i(u,o)}finally{r.d(u,o)}}class v{static __wrap(t){const n=Object.create(v.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,r.a(t)}constructor(){var t=r.j();return v.__wrap(t)}identify(t){var n=y(t,r.e,r.f),e=_,u=r.h(this.ptr,n,e);return j.__wrap(u)}licenses(){return d(r.i(this.ptr))}get_license(t){var n=y(t,r.e,r.f),e=_,u=r.g(this.ptr,n,e);return 0===u?void 0:x.__wrap(u)}}class x{static __wrap(t){const n=Object.create(x.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,r.b(t)}text(){try{r.k(8,this.ptr);var t=h()[2],n=h()[3];return i(t,n)}finally{r.d(t,n)}}}class j{static __wrap(t){const n=Object.create(j.prototype);return n.ptr=t,n}free(){const t=this.ptr;this.ptr=0,r.c(t)}name(){try{r.m(8,this.ptr);var t=h()[2],n=h()[3];return i(t,n)}finally{r.d(t,n)}}score(){return r.n(this.ptr)}license_text(){try{r.l(8,this.ptr);var t=h()[2],n=h()[3];return i(t,n)}finally{r.d(t,n)}}}const O=function(){return l(new Array)},k=function(t,n){return l(i(t,n))},m=function(t,n){return a(t).push(a(n))},A=function(t){d(t)},T=function(t,n){throw new Error(i(t,n))}}).call(this,e(5)(t))},function(t,n,e){"use strict";var r=e.w[t.i];t.exports=r;e(3);r.q()},function(t,n){t.exports=function(t){if(!t.webpackPolyfill){var n=Object.create(t);n.children||(n.children=[]),Object.defineProperty(n,"loaded",{enumerable:!0,get:function(){return n.l}}),Object.defineProperty(n,"id",{enumerable:!0,get:function(){return n.i}}),Object.defineProperty(n,"exports",{enumerable:!0}),n.webpackPolyfill=1}return n}}]]);