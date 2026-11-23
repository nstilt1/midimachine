import {readFileSync} from 'node:fs';
import assert from 'node:assert/strict';
const bytes=readFileSync(new URL('../assets/oracle.wasm',import.meta.url));
const module=new WebAssembly.Module(bytes);
assert.deepEqual(WebAssembly.Module.imports(module),[{module:'env',name:'HostPick',kind:'function'}]);
const values=Array.from({length:91},(_,i)=>BigInt.asUintN(64,BigInt(i+17)*0x9e3779b97f4a7c15n));
const bits=values.flatMap(n=>Array.from({length:24},(_,b)=>(n>>BigInt(b+4))&1n));
let index=0;
const {exports:api}=await WebAssembly.instantiate(module,{env:{HostPick:()=>BigInt.asIntN(64,values[index++])}});
api.__rt_init();api.OracleInit(0n);
for(let word=0;word<128;word++){
 const expected=bits.slice(word*17,word*17+17).reduce((n,b)=>(n<<1n)+b,0n);
 assert.equal(api.GodBits(17n,0n),expected);
}
assert.equal(index,Math.ceil(128*17/24));
console.log('PASS: HolyC WASM bit order, shift precedence, refill boundaries, and strict host import contract');
