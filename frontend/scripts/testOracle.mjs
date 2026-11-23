import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
const root=fileURLToPath(new URL('../../crates/temple-godspeak/',import.meta.url));
const {createOracle}=await import('../src/lib/godspeak.js');
const bytes=readFileSync(root+'assets/oracle.wasm'), vocab=readFileSync(root+'assets/vocabulary.txt','utf8');
const words=vocab.trimEnd().split('\n');
const samples=Array.from({length:91},(_,i)=>BigInt.asUintN(64,BigInt(i+17)*0x9e3779b97f4a7c15n));
const bits=samples.flatMap(n=>Array.from({length:24},(_,b)=>Number((n>>BigInt(b+4))&1n)));
for(const count of [1,4,16,32,128]){
 let index=0;
 const oracle=await createOracle(bytes,vocab,()=>BigInt.asIntN(64,samples[index++]));
 const expected=Array.from({length:count},(_,i)=>words[bits.slice(i*17,i*17+17).reduce((n,b)=>(n<<1)+b,0)%words.length]).join(' ');
 assert.equal(oracle(count),expected);
 assert.throws(()=>oracle(0));assert.throws(()=>oracle(129));
}
const zero=await createOracle(bytes,vocab,()=>0n);
assert.equal(zero(4),Array(4).fill(words[0]).join(' '));
console.log('PASS: compiled HolyC bit order, refill boundaries, original vocabulary, and word counts 1–128 match reference');
