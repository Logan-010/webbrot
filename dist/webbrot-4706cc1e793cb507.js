let W=`utf-8`,a3=`function`,a9=`error`,a7=`default`,aa=4,a1=1,a6=`same-origin`,a8=`cors`,V=`undefined`,Z=0,a2=`string`,Y=null,a4=`Object`,$=Array,X=Error,a5=FinalizationRegistry,ac=Object,ae=Object.getPrototypeOf,ad=Promise,ab=Reflect,_=Uint8Array,a0=undefined;var e=((a,c)=>{a=a>>>Z;return b.decode(d().subarray(a,a+ c))});var y=(a=>a===a0||a===Y);var S=((b,d)=>{a=b.exports;U.__wbindgen_wasm_module=d;q=Y;c=Y;a.__wbindgen_start();return a});var U=(async(b)=>{if(a!==a0)return a;if(typeof b!==V){if(ae(b)===ac.prototype){({module_or_path:b}=b)}else{console.warn(`using deprecated parameters for the initialization function; pass a single object instead`)}};if(typeof b===V){b=new URL(`webbrot_bg.wasm`,import.meta.url)};const c=Q();if(typeof b===a2||typeof Request===a3&&b instanceof Request||typeof URL===a3&&b instanceof URL){b=fetch(b)};R(c);const {instance:d,module:e}=await P(await b,c);return S(d,e)});function v(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_export_4(h(b))}}var l=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==Y){return `${a}`};if(b==a2){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==Y){return `Symbol`}else{return `Symbol(${b})`}};if(b==a3){const b=a.name;if(typeof b==a2&&b.length>Z){return `Function(${b})`}else{return `Function`}};if($.isArray(a)){const b=a.length;let c=`[`;if(b>Z){c+=l(a[Z])};for(let d=a1;d<b;d++){c+=`, `+ l(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>a1){d=c[a1]}else{return toString.call(a)};if(d==a4){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return a4}};if(a instanceof X){return `${a.name}: ${a.message}\n${a.stack}`};return d});var k=(a=>{const b=i(a);j(a);return b});var i=(a=>f[a]);var t=((b,c,d,e)=>{const f={a:b,b:c,cnt:a1,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=Z;try{return e(c,f.b,...b)}finally{if(--f.cnt===Z){a.__wbindgen_export_2.get(f.dtor)(c,f.b);s.unregister(f)}else{f.a=c}}};g.original=f;s.register(g,f,f);return g});var Q=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_string_new=((a,b)=>{const c=e(a,b);return h(c)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=i(a);return h(b)});b.wbg.__wbg_value_0cffd86fb9a5a18d=((b,c)=>{const d=i(c).value;const e=p(d,a.__wbindgen_export_0,a.__wbindgen_export_1);const f=m;r().setInt32(b+ aa*a1,f,!0);r().setInt32(b+ aa*Z,e,!0)});b.wbg.__wbindgen_object_drop_ref=(a=>{k(a)});b.wbg.__wbg_body_8e909b791b1745d3=(a=>{const b=i(a).body;return y(b)?Z:h(b)});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return h(b)});b.wbg.__wbg_buffer_ccaed51a635d8a2d=(a=>{const b=i(a).buffer;return h(b)});b.wbg.__wbg_newwithbyteoffsetandlength_7e3eb787208af730=((a,b,c)=>{const d=new _(i(a),b>>>Z,c>>>Z);return h(d)});b.wbg.__wbg_randomFillSync_5c9c955aa56b6049=function(){return v(((a,b)=>{i(a).randomFillSync(k(b))}),arguments)};b.wbg.__wbg_subarray_975a06f9dbd16995=((a,b,c)=>{const d=i(a).subarray(b>>>Z,c>>>Z);return h(d)});b.wbg.__wbg_getRandomValues_3aa56aa6edec874c=function(){return v(((a,b)=>{i(a).getRandomValues(i(b))}),arguments)};b.wbg.__wbg_new_fec2611eb9180f95=(a=>{const b=new _(i(a));return h(b)});b.wbg.__wbg_set_ec2fcf81bc573fd9=((a,b,c)=>{i(a).set(i(b),c>>>Z)});b.wbg.__wbindgen_is_object=(a=>{const b=i(a);const c=typeof b===`object`&&b!==Y;return c});b.wbg.__wbg_crypto_1d1f22824a6a080c=(a=>{const b=i(a).crypto;return h(b)});b.wbg.__wbg_process_4a72847cc503995b=(a=>{const b=i(a).process;return h(b)});b.wbg.__wbg_versions_f686565e586dd935=(a=>{const b=i(a).versions;return h(b)});b.wbg.__wbg_node_104a2ff8d6ea03a2=(a=>{const b=i(a).node;return h(b)});b.wbg.__wbindgen_is_string=(a=>{const b=typeof i(a)===a2;return b});b.wbg.__wbg_require_cca90b1a94a0255b=function(){return v((()=>{const a=module.require;return h(a)}),arguments)};b.wbg.__wbindgen_is_function=(a=>{const b=typeof i(a)===a3;return b});b.wbg.__wbg_msCrypto_eb05e62b530a1508=(a=>{const b=i(a).msCrypto;return h(b)});b.wbg.__wbg_newwithlength_76462a666eca145f=(a=>{const b=new _(a>>>Z);return h(b)});b.wbg.__wbg_get_ef828680c64da212=function(){return v(((a,b)=>{const c=ab.get(i(a),i(b));return h(c)}),arguments)};b.wbg.__wbg_self_bf91bf94d9e04084=function(){return v((()=>{const a=self.self;return h(a)}),arguments)};b.wbg.__wbg_window_52dd9f07d03fd5f8=function(){return v((()=>{const a=window.window;return h(a)}),arguments)};b.wbg.__wbg_globalThis_05c129bf37fcf1be=function(){return v((()=>{const a=globalThis.globalThis;return h(a)}),arguments)};b.wbg.__wbg_global_3eca19bb09e9c484=function(){return v((()=>{const a=global.global;return h(a)}),arguments)};b.wbg.__wbindgen_is_undefined=(a=>{const b=i(a)===a0;return b});b.wbg.__wbg_newnoargs_1ede4bf2ebbaaf43=((a,b)=>{var c=w(a,b);const d=new Function(c);return h(d)});b.wbg.__wbg_call_a9ef466721e824f2=function(){return v(((a,b)=>{const c=i(a).call(i(b));return h(c)}),arguments)};b.wbg.__wbg_call_3bfa248576352471=function(){return v(((a,b,c)=>{const d=i(a).call(i(b),i(c));return h(d)}),arguments)};b.wbg.__wbg_is_4b64bc96710d6a0f=((a,b)=>{const c=ac.is(i(a),i(b));return c});b.wbg.__wbg_set_e864d25d9b399c9f=function(){return v(((a,b,c)=>{const d=ab.set(i(a),i(b),i(c));return d}),arguments)};b.wbg.__wbg_setdata_ccbac292cd5e0fc0=((a,b,c)=>{var d=w(b,c);i(a).data=d});b.wbg.__wbg_previousSibling_5fbe2684a49cc571=(a=>{const b=i(a).previousSibling;return y(b)?Z:h(b)});b.wbg.__wbg_remove_d7a18d9f46bc50fd=(a=>{i(a).remove()});b.wbg.__wbg_before_3350442e02ed9f52=function(){return v(((a,b)=>{i(a).before(i(b))}),arguments)};b.wbg.__wbg_childNodes_87c5e311593a6192=(a=>{const b=i(a).childNodes;return h(b)});b.wbg.__wbg_length_9a6b70327f5f86e1=(a=>{const b=i(a).length;return b});b.wbg.__wbg_document_d7fa2c739c2b191a=(a=>{const b=i(a).document;return y(b)?Z:h(b)});b.wbg.__wbg_createComment_91ba91f80deb16bd=((a,b,c)=>{var d=w(b,c);const e=i(a).createComment(d);return h(e)});b.wbg.__wbg_composedPath_d27a772830ab5dd0=(a=>{const b=i(a).composedPath();return h(b)});b.wbg.__wbg_get_5419cf6b954aa11d=((a,b)=>{const c=i(a)[b>>>Z];return h(c)});b.wbg.__wbindgen_is_falsy=(a=>{const b=!i(a);return b});b.wbg.__wbg_cancelBubble_1fc3632e2ba513ed=(a=>{const b=i(a).cancelBubble;return b});b.wbg.__wbg_parentNode_7e7d8adc9b41ce58=(a=>{const b=i(a).parentNode;return y(b)?Z:h(b)});b.wbg.__wbg_instanceof_ShadowRoot_6d00cedbc919c9a6=(a=>{let b;try{b=i(a) instanceof ShadowRoot}catch(a){b=!1}const c=b;return c});b.wbg.__wbg_host_4a0b95cc36a45cb6=(a=>{const b=i(a).host;return h(b)});b.wbg.__wbindgen_is_null=(a=>{const b=i(a)===Y;return b});b.wbg.__wbg_createDocumentFragment_f0be9d8f1abfac54=(a=>{const b=i(a).createDocumentFragment();return h(b)});b.wbg.__wbg_addEventListener_0ac72681badaf1aa=function(){return v(((a,b,c,d,e)=>{var f=w(b,c);i(a).addEventListener(f,i(d),i(e))}),arguments)};b.wbg.__wbg_addEventListener_4357f9b7b3826784=function(){return v(((a,b,c,d)=>{var e=w(b,c);i(a).addEventListener(e,i(d))}),arguments)};b.wbg.__wbg_createTextNode_3b33c97f8ef3e999=((a,b,c)=>{var d=w(b,c);const e=i(a).createTextNode(d);return h(e)});b.wbg.__wbg_warn_41503a1c2194de89=(a=>{console.warn(i(a))});b.wbg.__wbindgen_throw=((a,b)=>{throw new X(e(a,b))});b.wbg.__wbindgen_debug_string=((b,c)=>{const d=l(i(c));const e=p(d,a.__wbindgen_export_0,a.__wbindgen_export_1);const f=m;r().setInt32(b+ aa*a1,f,!0);r().setInt32(b+ aa*Z,e,!0)});b.wbg.__wbindgen_rethrow=(a=>{throw k(a)});b.wbg.__wbg_then_748f75edfb032440=((a,b)=>{const c=i(a).then(i(b));return h(c)});b.wbg.__wbg_queueMicrotask_c5419c06eab41e73=(a=>{queueMicrotask(i(a))});b.wbg.__wbg_queueMicrotask_848aa4969108a57e=(a=>{const b=i(a).queueMicrotask;return h(b)});b.wbg.__wbg_resolve_0aad7c1484731c99=(a=>{const b=ad.resolve(i(a));return h(b)});b.wbg.__wbindgen_cb_drop=(a=>{const b=k(a).original;if(b.cnt--==a1){b.a=Z;return !0};const c=!1;return c});b.wbg.__wbg_byobRequest_86ac467c94924d3c=(a=>{const b=i(a).byobRequest;return y(b)?Z:h(b)});b.wbg.__wbg_view_de0e81c5c00d2129=(a=>{const b=i(a).view;return y(b)?Z:h(b)});b.wbg.__wbg_byteLength_5d623ba3d92a3a9c=(a=>{const b=i(a).byteLength;return b});b.wbg.__wbg_close_7cda9dd901230214=function(){return v((a=>{i(a).close()}),arguments)};b.wbg.__wbg_new_70a2f23d1565c04c=((a,b)=>{var c=w(a,b);const d=new X(c);return h(d)});b.wbg.__wbg_buffer_95102df5554646dc=(a=>{const b=i(a).buffer;return h(b)});b.wbg.__wbg_byteOffset_ec0928143c619cd7=(a=>{const b=i(a).byteOffset;return b});b.wbg.__wbg_length_9254c4bd3b9f23c4=(a=>{const b=i(a).length;return b});b.wbg.__wbg_close_cfd08d9cf9f36856=function(){return v((a=>{i(a).close()}),arguments)};b.wbg.__wbg_enqueue_e693a6fb4f3261c1=function(){return v(((a,b)=>{i(a).enqueue(i(b))}),arguments)};b.wbg.__wbg_new_1073970097e5a420=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=Z;try{return x(d,c.b,a,b)}finally{c.a=d}};const e=new ad(d);return h(e)}finally{c.a=c.b=Z}});b.wbg.__wbg_instanceof_Window_6575cd7f1322f82f=(a=>{let b;try{b=i(a) instanceof Window}catch(a){b=!1}const c=b;return c});b.wbg.__wbg_createElement_e4523490bd0ae51d=function(){return v(((a,b,c)=>{var d=w(b,c);const e=i(a).createElement(d);return h(e)}),arguments)};b.wbg.__wbg_append_2d052bdf2a44d6e4=function(){return v(((a,b)=>{i(a).append(i(b))}),arguments)};b.wbg.__wbg_setinnerHTML_559d45055154f1d8=((a,b,c)=>{var d=w(b,c);i(a).innerHTML=d});b.wbg.__wbg_removeAttribute_2dc68056b5e6ea3d=function(){return v(((a,b,c)=>{var d=w(b,c);i(a).removeAttribute(d)}),arguments)};b.wbg.__wbg_setAttribute_2a8f647a8d92c712=function(){return v(((a,b,c,d,e)=>{var f=w(b,c);var g=w(d,e);i(a).setAttribute(f,g)}),arguments)};b.wbg.__wbg_target_b0499015ea29563d=(a=>{const b=i(a).target;return y(b)?Z:h(b)});b.wbg.__wbg_nextSibling_46da01c3a2ce3774=(a=>{const b=i(a).nextSibling;return y(b)?Z:h(b)});b.wbg.__wbg_appendChild_bc4a0deae90a5164=function(){return v(((a,b)=>{const c=i(a).appendChild(i(b));return h(c)}),arguments)};b.wbg.__wbg_cloneNode_bd4b7e47afe3ce9f=function(){return v((a=>{const b=i(a).cloneNode();return h(b)}),arguments)};b.wbg.__wbg_respond_ffb6928cd9b79c32=function(){return v(((a,b)=>{i(a).respond(b>>>Z)}),arguments)};b.wbg.__wbindgen_closure_wrapper98=((a,b,c)=>{const d=t(a,b,49,u);return h(d)});b.wbg.__wbindgen_closure_wrapper113=((a,b,c)=>{const d=t(a,b,5,u);return h(d)});b.wbg.__wbindgen_closure_wrapper1471=((a,b,c)=>{const d=t(a,b,104,u);return h(d)});return b});var h=(a=>{if(g===f.length)f.push(f.length+ a1);const b=g;g=f[b];f[b]=a;return b});var T=(b=>{if(a!==a0)return a;if(typeof b!==V){if(ae(b)===ac.prototype){({module:b}=b)}else{console.warn(`using deprecated parameters for \`initSync()\`; pass a single object instead`)}};const c=Q();R(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return S(d,b)});var R=((a,b)=>{});var r=(()=>{if(q===Y||q.buffer.detached===!0||q.buffer.detached===a0&&q.buffer!==a.memory.buffer){q=new DataView(a.memory.buffer)};return q});var P=(async(a,b)=>{if(typeof Response===a3&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===a3){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve Wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var u=((b,c,d)=>{a.__wbindgen_export_3(b,c,h(d))});var j=(a=>{if(a<132)return;f[a]=g;g=a});var d=(()=>{if(c===Y||c.byteLength===Z){c=new _(a.memory.buffer)};return c});var x=((b,c,d,e)=>{a.__wbindgen_export_5(b,c,h(d),h(e))});var w=((a,b)=>{if(a===Z){return i(b)}else{return e(a,b)}});var p=((a,b,c)=>{if(c===a0){const c=n.encode(a);const e=b(c.length,a1)>>>Z;d().subarray(e,e+ c.length).set(c);m=c.length;return e};let e=a.length;let f=b(e,a1)>>>Z;const g=d();let h=Z;for(;h<e;h++){const b=a.charCodeAt(h);if(b>127)break;g[f+ h]=b};if(h!==e){if(h!==Z){a=a.slice(h)};f=c(f,e,e=h+ a.length*3,a1)>>>Z;const b=d().subarray(f+ h,f+ e);const g=o(a,b);h+=g.written;f=c(f,e,h,a1)>>>Z};m=h;return f});let a;const b=typeof TextDecoder!==V?new TextDecoder(W,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw X(`TextDecoder not available`)}};if(typeof TextDecoder!==V){b.decode()};let c=Y;const f=new $(128).fill(a0);f.push(a0,Y,!0,!1);let g=f.length;let m=Z;const n=typeof TextEncoder!==V?new TextEncoder(W):{encode:()=>{throw X(`TextEncoder not available`)}};const o=typeof n.encodeInto===a3?((a,b)=>n.encodeInto(a,b)):((a,b)=>{const c=n.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=Y;const s=typeof a5===V?{register:()=>{},unregister:()=>{}}:new a5(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});const z=[`blob`,`arraybuffer`];const A=[`byob`];const B=[`bytes`];const C=[``,`no-referrer`,`no-referrer-when-downgrade`,`origin`,`origin-when-cross-origin`,`unsafe-url`,a6,`strict-origin`,`strict-origin-when-cross-origin`];const D=[a7,`no-store`,`reload`,`no-cache`,`force-cache`,`only-if-cached`];const E=[`omit`,a6,`include`];const F=[a6,`no-cors`,a8,`navigate`];const G=[`follow`,a9,`manual`];const H=[`basic`,a8,a7,a9,`opaque`,`opaqueredirect`];const I=[`open`,`closed`];const J=typeof a5===V?{register:()=>{},unregister:()=>{}}:new a5(b=>a.__wbg_intounderlyingbytesource_free(b>>>Z,a1));class K{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=Z;J.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingbytesource_free(b,Z)}type(){const b=a.intounderlyingbytesource_type(this.__wbg_ptr);return B[b]}autoAllocateChunkSize(){const b=a.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return b>>>Z}start(b){a.intounderlyingbytesource_start(this.__wbg_ptr,h(b))}pull(b){const c=a.intounderlyingbytesource_pull(this.__wbg_ptr,h(b));return k(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingbytesource_cancel(b)}}const L=typeof a5===V?{register:()=>{},unregister:()=>{}}:new a5(b=>a.__wbg_intounderlyingsink_free(b>>>Z,a1));class M{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=Z;L.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsink_free(b,Z)}write(b){const c=a.intounderlyingsink_write(this.__wbg_ptr,h(b));return k(c)}close(){const b=this.__destroy_into_raw();const c=a.intounderlyingsink_close(b);return k(c)}abort(b){const c=this.__destroy_into_raw();const d=a.intounderlyingsink_abort(c,h(b));return k(d)}}const N=typeof a5===V?{register:()=>{},unregister:()=>{}}:new a5(b=>a.__wbg_intounderlyingsource_free(b>>>Z,a1));class O{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=Z;N.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsource_free(b,Z)}pull(b){const c=a.intounderlyingsource_pull(this.__wbg_ptr,h(b));return k(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingsource_cancel(b)}}export default U;export{K as IntoUnderlyingByteSource,M as IntoUnderlyingSink,O as IntoUnderlyingSource,T as initSync}