import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {compileHolyC} from '../vendor/holyc-wasm/src/compiler.js';
const root=fileURLToPath(new URL('../',import.meta.url));
const original=readFileSync(root+'holyc/HolySpirit.HC','utf8');
const extract=(start,end)=>original.slice(original.indexOf(start),original.indexOf(end,original.indexOf(start)));
let core=extract('public U0 GodBitsIns','public U0 GodHexIns')+
  extract('public I64 GodBits(', 'public I64 GodInit(')+
  extract('public U8 *GodWordStr(', 'public U0 GodWord(');
// HolyC shifts bind more tightly than addition; the third-party parser uses C
// precedence. Parenthesize explicitly to retain Terry's HolyC semantics.
if(!core.includes('res=res<<1+b;'))throw Error('Upstream GodBits changed');
core=core.replace('res=res<<1+b;','res=(res<<1)+b;');
const source=readFileSync(root+'holyc/host.HC','utf8')+'\n'+core+ '\nI64 OracleNext() { return GodWordStr(); }\n';
const result=compileHolyC(source,{noPrelude:true,lenient:false,exports:['OracleInit','OracleWordsAddress','OracleVocabAddress','OracleNext','GodBitsIns','GodBits']});
if(result.warnings.length||result.diagnostics.length)throw Error(JSON.stringify({warnings:result.warnings,diagnostics:result.diagnostics}));
if(!WebAssembly.validate(result.bytes))throw Error('Invalid WASM');
writeFileSync(root+'assets/oracle.wasm',result.bytes);
// Use the original char_bmp_word definition: apostrophe, digits, ASCII letters,
// underscore, and bytes 128..255. Vocab.DD in this snapshot is ASCII.
const vocab=readFileSync(root+'holyc/Vocab.DD');
let words=[],word=[];
const isWord=c=>c===39||(c>=48&&c<=57)||(c>=65&&c<=90)||c===95||(c>=97&&c<=122)||c>=128;
for(const c of [...vocab,0]){if(isWord(c))word.push(c);else if(word.length){words.push(Buffer.from(word).toString('utf8'));word=[];}}
if(words.length>8000||Buffer.byteLength(words.join('\0'))+1>100000)throw Error('Vocabulary exceeds static buffers');
writeFileSync(root+'assets/vocabulary.txt',words.join('\n')+'\n');
console.log(`Compiled Terry's oracle core in strict mode; ${words.length} vocabulary entries`);
