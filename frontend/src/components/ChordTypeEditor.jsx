"use client";
import { useState } from "react";
const pitch = n => `${["C","C#","D","D#","E","F","F#","G","G#","A","A#","B"][n % 12]}${Math.floor(n / 12)-1}`;
const save = (data, name) => {
  const url = URL.createObjectURL(new Blob([JSON.stringify(data, null, 2)], {type:"application/json"}));
  const a = document.createElement("a"); a.href=url; a.download=name; a.click(); setTimeout(()=>URL.revokeObjectURL(url),1000);
};
export default function ChordTypeEditor({ types, onChange, wasmModule }) {
  const [root,setRoot]=useState(0);
  const [name,setName]=useState("");
  const [notes,setNotes]=useState([]);
  const [optional,setOptional]=useState([]);
  const [mode,setMode]=useState("required");
  const [error,setError]=useState("");
  const commit = next => { try { wasmModule.set_custom_chord_types(JSON.stringify(next)); onChange(next); setError(""); } catch(e){setError(String(e));} };
  const toggle = n => {
    if(mode === "required") { setNotes(notes.includes(n)?notes.filter(x=>x!==n):[...notes,n]); setOptional(optional.filter(x=>x!==n)); }
    else {setOptional(optional.includes(n)?optional.filter(x=>x!==n):[...optional,n]); setNotes(notes.filter(x=>x!==n));}
  };
  const add = () => {
    if(!name.trim() || !notes.length) {setError("Name the chord and select at least one required note.");return;}
    const lowest = Math.min(...notes, ...optional);
    const base = root + 12 * Math.floor((lowest-root)/12);
    const chord = {name:name.trim(),note_intervals:notes.map(n=>n-base).sort((a,b)=>a-b),optional_notes:optional.map(n=>n-base).sort((a,b)=>a-b),roots:[root]};
    commit([...types.filter(t=>t.name!==chord.name),chord]);
  };
  return <section className="space-y-3 border rounded p-4 my-4">
    <h3 className="text-lg font-bold">Build a chord type</h3>
    <p>Standard guitar tuning, open string through fret 24. Click circles to select pitches. Shapes become intervals above the chosen root. Hand-picked mode keeps this root; pruning mode tries all 12 roots before filtering. Optional notes create additional voicings during pruning; choose “all notes” to expand without filtering.</p>
    <label>Root <select value={root} onChange={e=>setRoot(Number(e.target.value))}>{["C","C#","D","D#","E","F","F#","G","G#","A","A#","B"].map((n,i)=><option key={n} value={i}>{n}</option>)}</select></label>
    <label>Name <input value={name} maxLength={80} onChange={e=>setName(e.target.value)}/></label>
    <select className="ml-3" value={mode} onChange={e=>setMode(e.target.value)}><option value="required">Required notes</option><option value="optional">Optional notes</option></select>
    <div className="overflow-auto" aria-label="Guitar fretboard">{[64,59,55,50,45,40].map((open,i)=><div key={i} className="flex gap-1 min-w-max items-center"><span className="w-8">{pitch(open)}</span>{Array.from({length:25},(_,f)=>{const n=open+f;return <button key={f} title={`String ${i+1}, fret ${f}, ${pitch(n)}`} aria-pressed={notes.includes(n)||optional.includes(n)} onClick={()=>toggle(n)} className={`rounded-full border w-12 h-12 text-xs ${notes.includes(n)?"bg-blue-700 text-white":optional.includes(n)?"bg-amber-600 text-white":""}`}>{pitch(n)}<br/>{f}</button>;})}</div>)}</div>
    <p>Required: {[...notes].sort((a,b)=>a-b).map(pitch).join(", ") || "none"} · Optional: {optional.map(pitch).join(", ") || "none"}</p>
    <div className="flex gap-3"><button onClick={add}>Save / replace chord type</button><button onClick={()=>{setNotes([]);setOptional([]);setName("");}}>Clear</button><button onClick={()=>save(types,"chord-types.json")}>Export all custom types</button>
    <label>Import JSON <input type="file" accept=".json,application/json" onChange={async e=>{try{const data=JSON.parse(await e.target.files[0].text());const imported=Array.isArray(data)?data:[data];commit([...types.filter(t=>!imported.some(x=>x.name===t.name)),...imported]);}catch(err){setError(String(err));}e.target.value="";}}/></label></div>
    {types.map(t=><div key={t.name} className="flex gap-3"><strong>{t.name}</strong><span>{t.note_intervals.join(", ")}</span><button onClick={()=>{setName(t.name);setRoot(t.roots[0]);const base=48+t.roots[0];setNotes(t.note_intervals.map(n=>n+base));setOptional(t.optional_notes.map(n=>n+base));}}>Edit</button><button onClick={()=>save(t,`${t.name.replace(/[^a-z0-9]/gi,"_")}.json`)}>Export JSON</button><button onClick={()=>commit(types.filter(x=>x.name!==t.name))}>Delete</button></div>)}
    {error&&<p role="alert">{error}</p>}
    <p>Saved locally in this browser. Select Custom in the chord controls to use these types.</p>
  </section>;
}
