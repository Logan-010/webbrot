let $=4,N=null,W=`string`,O=`undefined`,U=`function`,P=`utf-8`,T=1,X=`Object`,R=0,L=Array,Q=Error,V=FinalizationRegistry,a1=Object,a0=Object.getPrototypeOf,Z=Promise,Y=Reflect,S=Uint8Array,_=globalThis,M=undefined;var x=((b,c,d,e)=>{a.__wbindgen_export_6(b,c,i(d),i(e))});var J=(b=>{if(a!==M)return a;if(typeof b!==O){if(a0(b)===a1.prototype){({module:b}=b)}else{console.warn(`using deprecated parameters for \`initSync()\`; pass a single object instead`)}};const c=G();H(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return I(d,b)});var m=(a=>{const b=c(a);l(a);return b});var F=(async(a,b)=>{if(typeof Response===U&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===U){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve Wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var f=(()=>{if(e===N||e.byteLength===R){e=new S(a.memory.buffer)};return e});var l=(a=>{if(a<132)return;b[a]=h;h=a});var k=(a=>a===M||a===N);var I=((b,c)=>{a=b.exports;K.__wbindgen_wasm_module=c;r=N;e=N;a.__wbindgen_start();return a});var w=((b,c,d)=>{a.__wbindgen_export_5(b,c,i(d))});var H=((a,b)=>{});var v=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==N){return `${a}`};if(b==W){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==N){return `Symbol`}else{return `Symbol(${b})`}};if(b==U){const b=a.name;if(typeof b==W&&b.length>R){return `Function(${b})`}else{return `Function`}};if(L.isArray(a)){const b=a.length;let c=`[`;if(b>R){c+=v(a[R])};for(let d=T;d<b;d++){c+=`, `+ v(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c&&c.length>T){d=c[T]}else{return toString.call(a)};if(d==X){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return X}};if(a instanceof Q){return `${a.name}: ${a.message}\n${a.stack}`};return d});var q=((a,b,c)=>{if(c===M){const c=o.encode(a);const d=b(c.length,T)>>>R;f().subarray(d,d+ c.length).set(c);n=c.length;return d};let d=a.length;let e=b(d,T)>>>R;const g=f();let h=R;for(;h<d;h++){const b=a.charCodeAt(h);if(b>127)break;g[e+ h]=b};if(h!==d){if(h!==R){a=a.slice(h)};e=c(e,d,d=h+ a.length*3,T)>>>R;const b=f().subarray(e+ h,e+ d);const g=p(a,b);h+=g.written;e=c(e,d,h,T)>>>R};n=h;return e});var c=(a=>b[a]);function j(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_export_0(i(b))}}var g=((a,b)=>{a=a>>>R;return d.decode(f().subarray(a,a+ b))});var K=(async(b)=>{if(a!==M)return a;if(typeof b!==O){if(a0(b)===a1.prototype){({module_or_path:b}=b)}else{console.warn(`using deprecated parameters for the initialization function; pass a single object instead`)}};if(typeof b===O){b=new URL(`webbrot_bg.wasm`,import.meta.url)};const c=G();if(typeof b===W||typeof Request===U&&b instanceof Request||typeof URL===U&&b instanceof URL){b=fetch(b)};H(c);const {instance:d,module:e}=await F(await b,c);return I(d,e)});var u=((b,c,d,e)=>{const f={a:b,b:c,cnt:T,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=R;try{return e(c,f.b,...b)}finally{if(--f.cnt===R){a.__wbindgen_export_4.get(f.dtor)(c,f.b);t.unregister(f)}else{f.a=c}}};g.original=f;t.register(g,f,f);return g});var i=(a=>{if(h===b.length)b.push(b.length+ T);const c=h;h=b[c];b[c]=a;return c});var G=(()=>{const b={};b.wbg={};b.wbg.__wbg_addEventListener_90e553fdce254421=function(){return j(((a,b,d,e)=>{c(a).addEventListener(g(b,d),c(e))}),arguments)};b.wbg.__wbg_body_942ea927546a04ba=(a=>{const b=c(a).body;return k(b)?R:i(b)});b.wbg.__wbg_buffer_09165b52af8c5237=(a=>{const b=c(a).buffer;return i(b)});b.wbg.__wbg_buffer_609cc3eee51ed158=(a=>{const b=c(a).buffer;return i(b)});b.wbg.__wbg_byobRequest_77d9adf63337edfb=(a=>{const b=c(a).byobRequest;return k(b)?R:i(b)});b.wbg.__wbg_byteLength_e674b853d9c77e1d=(a=>{const b=c(a).byteLength;return b});b.wbg.__wbg_byteOffset_fd862df290ef848d=(a=>{const b=c(a).byteOffset;return b});b.wbg.__wbg_call_672a4d21634d4a24=function(){return j(((a,b)=>{const d=c(a).call(c(b));return i(d)}),arguments)};b.wbg.__wbg_call_7cccdd69e0791ae2=function(){return j(((a,b,d)=>{const e=c(a).call(c(b),c(d));return i(e)}),arguments)};b.wbg.__wbg_cancelBubble_2e66f509cdea4d7e=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_cloneNode_a8ce4052a2c37536=function(){return j(((a,b)=>{const d=c(a).cloneNode(b!==R);return i(d)}),arguments)};b.wbg.__wbg_cloneNode_e35b333b87d51340=function(){return j((a=>{const b=c(a).cloneNode();return i(b)}),arguments)};b.wbg.__wbg_close_304cc1fef3466669=function(){return j((a=>{c(a).close()}),arguments)};b.wbg.__wbg_close_5ce03e29be453811=function(){return j((a=>{c(a).close()}),arguments)};b.wbg.__wbg_composedPath_977ce97a0ef39358=(a=>{const b=c(a).composedPath();return i(b)});b.wbg.__wbg_content_537e4105afcd9cee=(a=>{const b=c(a).content;return i(b)});b.wbg.__wbg_createComment_8b540d4b9d22f212=((a,b,d)=>{const e=c(a).createComment(g(b,d));return i(e)});b.wbg.__wbg_createElement_8c9931a732ee2fea=function(){return j(((a,b,d)=>{const e=c(a).createElement(g(b,d));return i(e)}),arguments)};b.wbg.__wbg_createTextNode_42af1a9f21bb3360=((a,b,d)=>{const e=c(a).createTextNode(g(b,d));return i(e)});b.wbg.__wbg_crypto_ed58b8e10a292839=(a=>{const b=c(a).crypto;return i(b)});b.wbg.__wbg_deleteProperty_96363d4a1d977c97=function(){return j(((a,b)=>{const d=Y.deleteProperty(c(a),c(b));return d}),arguments)};b.wbg.__wbg_document_d249400bd7bd996d=(a=>{const b=c(a).document;return k(b)?R:i(b)});b.wbg.__wbg_enqueue_bb16ba72f537dc9e=function(){return j(((a,b)=>{c(a).enqueue(c(b))}),arguments)};b.wbg.__wbg_firstElementChild_d75d385f5abd1414=(a=>{const b=c(a).firstElementChild;return k(b)?R:i(b)});b.wbg.__wbg_getRandomValues_bcb4912f16000dc4=function(){return j(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbg_get_67b2ba62fc30de12=function(){return j(((a,b)=>{const d=Y.get(c(a),c(b));return i(d)}),arguments)};b.wbg.__wbg_get_b9b93047fe3cf45b=((a,b)=>{const d=c(a)[b>>>R];return i(d)});b.wbg.__wbg_host_166cb082dae71d08=(a=>{const b=c(a).host;return i(b)});b.wbg.__wbg_insertBefore_c181fb91844cd959=function(){return j(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return i(e)}),arguments)};b.wbg.__wbg_instanceof_Element_0af65443936d5154=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_instanceof_ShadowRoot_726578bcd7fa418a=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_instanceof_Window_def73ea0955fc569=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_length_a446193dc22c12f8=(a=>{const b=c(a).length;return b});b.wbg.__wbg_log_0cc1b7768397bcfe=((b,c,d,e,f,h,i,j)=>{let k;let l;try{k=b;l=c;console.log(g(b,c),g(d,e),g(f,h),g(i,j))}finally{a.__wbindgen_export_1(k,l,T)}});b.wbg.__wbg_log_cb9e190acc5753fb=((b,c)=>{let d;let e;try{d=b;e=c;console.log(g(b,c))}finally{a.__wbindgen_export_1(d,e,T)}});b.wbg.__wbg_mark_7438147ce31e9d4b=((a,b)=>{performance.mark(g(a,b))});b.wbg.__wbg_measure_fb7825c11612c823=function(){return j(((b,c,d,e)=>{let f;let h;let i;let j;try{f=b;h=c;i=d;j=e;performance.measure(g(b,c),g(d,e))}finally{a.__wbindgen_export_1(f,h,T);a.__wbindgen_export_1(i,j,T)}}),arguments)};b.wbg.__wbg_msCrypto_0a36e2ec3a343d26=(a=>{const b=c(a).msCrypto;return i(b)});b.wbg.__wbg_new_23a2665fac83c611=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=R;try{return x(d,c.b,a,b)}finally{c.a=d}};const e=new Z(d);return i(e)}finally{c.a=c.b=R}});b.wbg.__wbg_new_a12002a7f91c75be=(a=>{const b=new S(c(a));return i(b)});b.wbg.__wbg_new_c68d7209be747379=((a,b)=>{const c=new Q(g(a,b));return i(c)});b.wbg.__wbg_newnoargs_105ed471475aaf50=((a,b)=>{const c=new Function(g(a,b));return i(c)});b.wbg.__wbg_newwithbyteoffsetandlength_d97e637ebe145a9a=((a,b,d)=>{const e=new S(c(a),b>>>R,d>>>R);return i(e)});b.wbg.__wbg_newwithlength_a381634e90c276d4=(a=>{const b=new S(a>>>R);return i(b)});b.wbg.__wbg_node_02999533c4ea02e3=(a=>{const b=c(a).node;return i(b)});b.wbg.__wbg_parentNode_9de97a0e7973ea4e=(a=>{const b=c(a).parentNode;return k(b)?R:i(b)});b.wbg.__wbg_process_5c1d670bc53614b8=(a=>{const b=c(a).process;return i(b)});b.wbg.__wbg_queueMicrotask_97d92b4fcc8a61c5=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_d3219def82552485=(a=>{const b=c(a).queueMicrotask;return i(b)});b.wbg.__wbg_randomFillSync_ab2cfe79ebbf2740=function(){return j(((a,b)=>{c(a).randomFillSync(m(b))}),arguments)};b.wbg.__wbg_removeEventListener_056dfe8c3d6c58f9=function(){return j(((a,b,d,e)=>{c(a).removeEventListener(g(b,d),c(e))}),arguments)};b.wbg.__wbg_remove_e2d2659f3128c045=(a=>{c(a).remove()});b.wbg.__wbg_remove_efb062ab554e1fbd=(a=>{c(a).remove()});b.wbg.__wbg_require_79b1e9274cde3c87=function(){return j((()=>{const a=module.require;return i(a)}),arguments)};b.wbg.__wbg_resolve_4851785c9c5f573d=(a=>{const b=Z.resolve(c(a));return i(b)});b.wbg.__wbg_respond_1f279fa9f8edcb1c=function(){return j(((a,b)=>{c(a).respond(b>>>R)}),arguments)};b.wbg.__wbg_setAttribute_2704501201f15687=function(){return j(((a,b,d,e,f)=>{c(a).setAttribute(g(b,d),g(e,f))}),arguments)};b.wbg.__wbg_set_65595bdd868b3009=((a,b,d)=>{c(a).set(c(b),d>>>R)});b.wbg.__wbg_set_bb8cecf6a62b9f46=function(){return j(((a,b,d)=>{const e=Y.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbg_setinnerHTML_31bde41f835786f7=((a,b,d)=>{c(a).innerHTML=g(b,d)});b.wbg.__wbg_setnodeValue_58cb1b2f6b6c33d2=((a,b,d)=>{c(a).nodeValue=b===R?M:g(b,d)});b.wbg.__wbg_static_accessor_GLOBAL_88a902d13a557d07=(()=>{const a=typeof global===O?N:global;return k(a)?R:i(a)});b.wbg.__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0=(()=>{const a=typeof _===O?N:_;return k(a)?R:i(a)});b.wbg.__wbg_static_accessor_SELF_37c5d418e4bf5819=(()=>{const a=typeof self===O?N:self;return k(a)?R:i(a)});b.wbg.__wbg_static_accessor_WINDOW_5de37043a91a9c40=(()=>{const a=typeof window===O?N:window;return k(a)?R:i(a)});b.wbg.__wbg_subarray_aa9065fa9dc5df96=((a,b,d)=>{const e=c(a).subarray(b>>>R,d>>>R);return i(e)});b.wbg.__wbg_target_0a62d9d79a2a1ede=(a=>{const b=c(a).target;return k(b)?R:i(b)});b.wbg.__wbg_then_44b73946d2fb3e7d=((a,b)=>{const d=c(a).then(c(b));return i(d)});b.wbg.__wbg_value_91cbf0dd3ab84c1e=((b,d)=>{const e=c(d).value;const f=q(e,a.__wbindgen_export_2,a.__wbindgen_export_3);const g=n;s().setInt32(b+ $*T,g,!0);s().setInt32(b+ $*R,f,!0)});b.wbg.__wbg_versions_c71aa1626a93e0a1=(a=>{const b=c(a).versions;return i(b)});b.wbg.__wbg_view_fd8a56e8983f448d=(a=>{const b=c(a).view;return k(b)?R:i(b)});b.wbg.__wbindgen_cb_drop=(a=>{const b=m(a).original;if(b.cnt--==T){b.a=R;return !0};const c=!1;return c});b.wbg.__wbindgen_closure_wrapper1528=((a,b,c)=>{const d=u(a,b,86,w);return i(d)});b.wbg.__wbindgen_closure_wrapper1552=((a,b,c)=>{const d=u(a,b,104,w);return i(d)});b.wbg.__wbindgen_debug_string=((b,d)=>{const e=v(c(d));const f=q(e,a.__wbindgen_export_2,a.__wbindgen_export_3);const g=n;s().setInt32(b+ $*T,g,!0);s().setInt32(b+ $*R,f,!0)});b.wbg.__wbindgen_is_falsy=(a=>{const b=!c(a);return b});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===U;return b});b.wbg.__wbindgen_is_null=(a=>{const b=c(a)===N;return b});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==N;return d});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===W;return b});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===M;return b});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return i(b)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return i(b)});b.wbg.__wbindgen_object_drop_ref=(a=>{m(a)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=g(a,b);return i(c)});b.wbg.__wbindgen_throw=((a,b)=>{throw new Q(g(a,b))});return b});var s=(()=>{if(r===N||r.buffer.detached===!0||r.buffer.detached===M&&r.buffer!==a.memory.buffer){r=new DataView(a.memory.buffer)};return r});let a;const b=new L(128).fill(M);b.push(M,N,!0,!1);const d=typeof TextDecoder!==O?new TextDecoder(P,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Q(`TextDecoder not available`)}};if(typeof TextDecoder!==O){d.decode()};let e=N;let h=b.length;let n=R;const o=typeof TextEncoder!==O?new TextEncoder(P):{encode:()=>{throw Q(`TextEncoder not available`)}};const p=typeof o.encodeInto===U?((a,b)=>o.encodeInto(a,b)):((a,b)=>{const c=o.encode(a);b.set(c);return {read:a.length,written:c.length}});let r=N;const t=typeof V===O?{register:()=>{},unregister:()=>{}}:new V(b=>{a.__wbindgen_export_4.get(b.dtor)(b.a,b.b)});const y=[`bytes`];const z=typeof V===O?{register:()=>{},unregister:()=>{}}:new V(b=>a.__wbg_intounderlyingbytesource_free(b>>>R,T));class A{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=R;z.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingbytesource_free(b,R)}type(){const b=a.intounderlyingbytesource_type(this.__wbg_ptr);return y[b]}autoAllocateChunkSize(){const b=a.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return b>>>R}start(b){a.intounderlyingbytesource_start(this.__wbg_ptr,i(b))}pull(b){const c=a.intounderlyingbytesource_pull(this.__wbg_ptr,i(b));return m(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingbytesource_cancel(b)}}const B=typeof V===O?{register:()=>{},unregister:()=>{}}:new V(b=>a.__wbg_intounderlyingsink_free(b>>>R,T));class C{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=R;B.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsink_free(b,R)}write(b){const c=a.intounderlyingsink_write(this.__wbg_ptr,i(b));return m(c)}close(){const b=this.__destroy_into_raw();const c=a.intounderlyingsink_close(b);return m(c)}abort(b){const c=this.__destroy_into_raw();const d=a.intounderlyingsink_abort(c,i(b));return m(d)}}const D=typeof V===O?{register:()=>{},unregister:()=>{}}:new V(b=>a.__wbg_intounderlyingsource_free(b>>>R,T));class E{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=R;D.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsource_free(b,R)}pull(b){const c=a.intounderlyingsource_pull(this.__wbg_ptr,i(b));return m(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingsource_cancel(b)}}export default K;export{A as IntoUnderlyingByteSource,C as IntoUnderlyingSink,E as IntoUnderlyingSource,J as initSync}