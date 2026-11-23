"use client";

import { useEffect, useRef, useState } from "react";
import OracleSeed from "./OracleSeed";
import { DrumAudio } from "@/lib/drumAudio";
import { ZipStore } from "@/lib/zipStore";

const names = ["Kick", "Closed HH", "Open HH", "Pedal HH", "Low snare", "High snare", "Clap", "Low tom", "Mid tom", "High tom"];
const files = ["kick", "hihat_closed", "hihat_open", "hihat_pedal", "snare_low", "snare_high", "clap", "tom_low", "tom_mid", "tom_high"];
const defaults = [36, 42, 46, 44, 38, 40, 39, 45, 47, 50];
const randomText = () => `drums-${crypto.randomUUID()}`;
const midiUrl = bytes => URL.createObjectURL(new Blob([bytes], { type: "audio/midi" }));
const download = (url, name) => {
  const link = document.createElement("a");
  link.href = url;
  link.download = name;
  link.click();
};

export default function DrumGenerator({ wasmModule }) {
  const [config, setConfig] = useState({
    bpm: 90, bars: 4, style: "boom_bap", complexity: .5, density: .5,
    swing: .15, humanization: .12, variation: .4, fill_probability: .35,
    ghost_probability: .2, roll_probability: .25, seed_text: "drum groove",
    enabled: [true, true, true, false, true, false, true, false, false, false],
    notes: defaults, channel: 9,
  });
  const audio = useRef(null);
  const [pattern, setPattern] = useState(null);
  const [playing, setPlaying] = useState(false);
  const [samples, setSamples] = useState({});
  const [loop, setLoop] = useState(true);
  const getAudio = () => audio.current || (audio.current = new DrumAudio());
  useEffect(() => () => audio.current?.close(), []);
  const play = async () => {
    try { let current = pattern;
      if (!current) current = prepare(config);
      await getAudio().play(current, loop); setPlaying(true);
    } catch(e) {setError(String(e));}
  };
  const [preview, setPreview] = useState("");
  const [players, setPlayers] = useState(null);
  const [error, setError] = useState("");
  const [count, setCount] = useState(1);
  const [metadata, setMetadata] = useState(true);
  const [busy, setBusy] = useState(false);
  const [progress, setProgress] = useState(0);
  const update = (key, value) => setConfig(current => ({ ...current, [key]: value }));

  useEffect(() => () => {
    if (players) {
      URL.revokeObjectURL(players.full);
      players.layers.forEach(layer => URL.revokeObjectURL(layer.url));
    }
  }, [players]);

  const prepare = (nextConfig, variationText = null) => {
    audio.current?.stop(); setPlaying(false);
    const json = JSON.stringify(nextConfig);
    const rendered = JSON.parse(wasmModule.drum_pattern_json(json, variationText || ""));
    setPattern(rendered);
    const full = variationText === null
      ? wasmModule.generate_drum_midi(json)
      : wasmModule.vary_drum_midi(json, variationText);
    const layers = files.flatMap((name, index) => {
      if (!nextConfig.enabled[index]) return [];
      const bytes = variationText === null
        ? wasmModule.generate_drum_instrument(json, index)
        : wasmModule.vary_drum_instrument(json, variationText, index);
      return [{ name: names[index], url: midiUrl(bytes) }];
    });
    setPlayers({ full: midiUrl(full), layers });
    setPreview(variationText === null
      ? wasmModule.preview_drum_pattern(json)
      : wasmModule.preview_drum_variation(json, variationText));
    setError("");
    return rendered;
  };

  const generate = (fresh = false) => {
    try {
      const nextConfig = fresh ? { ...config, seed_text: randomText() } : config;
      if (fresh) setConfig(nextConfig);
      prepare(nextConfig);
    } catch (cause) { setError(String(cause)); }
  };
  const vary = () => {
    try { prepare(config, randomText()); }
    catch (cause) { setError(String(cause)); }
  };
  const batch = async () => {
    try {
      if (!Number.isInteger(count) || count < 1 || count > 10000) throw Error("Batch count must be 1–10,000");
      setBusy(true);
      setProgress(0);
      const zip = new ZipStore();
      const seedLabels = [];
      for (let index = 0; index < count; index++) {
        const seedText = config.seed_text || "0";
        const label = index === 0 ? seedText : `${seedText}#${String(index + 1).padStart(6, "0")}`;
        const nextConfig = { ...config, seed_text: label };
        const json = JSON.stringify(nextConfig);
        const folder = `pattern_${String(index + 1).padStart(6, "0")}/`;
        zip.add(`${folder}full_pattern.mid`, wasmModule.generate_drum_midi(json));
        for (let instrument = 0; instrument < files.length; instrument++) {
          if (nextConfig.enabled[instrument]) {
            zip.add(`${folder}${files[instrument]}.mid`, wasmModule.generate_drum_instrument(json, instrument));
          }
        }
        if (metadata) zip.add(`${folder}metadata.json`, JSON.stringify(nextConfig, null, 2));
        seedLabels.push(label);
        if (index % 20 === 0) {
          setProgress(index + 1);
          await new Promise(resolve => setTimeout(resolve, 0));
        }
      }
      zip.add("manifest.json", JSON.stringify({
        generator_version: "0.1.0", generated_at: new Date().toISOString(),
        count, config, seed_labels: seedLabels,
      }, null, 2));
      const url = URL.createObjectURL(zip.blob());
      download(url, `drums_${config.style}_${config.bpm}bpm.zip`);
      setTimeout(() => URL.revokeObjectURL(url), 30000);
      setProgress(count);
      setError("");
    } catch (cause) { setError(String(cause)); }
    finally { setBusy(false); }
  };

  return <div className="space-y-4 max-w-3xl">
    <h2 className="text-xl font-bold">Hip-hop drums</h2>
    <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
      <label>BPM<input className="w-full" type="number" min="30" max="300" value={config.bpm} onChange={event => update("bpm", Number(event.target.value))}/></label>
      <label>Bars<input className="w-full" type="number" min="1" max="1024" value={config.bars} onChange={event => update("bars", Number(event.target.value))}/></label>
      <label>Style<select className="w-full" value={config.style} onChange={event => update("style", event.target.value)}>
        <option value="boom_bap">Boom bap</option><option value="trap">Trap</option><option value="lo_fi">Lo-fi</option><option value="modern">Modern</option>
      </select></label>
      <label>Seed text<input className="w-full" type="text" value={config.seed_text} onChange={event => update("seed_text", event.target.value)}/></label>
      {[["complexity", "Complexity"], ["density", "Density"], ["swing", "Swing"], ["humanization", "Humanization"], ["variation", "Variation"], ["fill_probability", "Fills"], ["ghost_probability", "Ghost snares"], ["roll_probability", "Hat rolls"]].map(([key, label]) =>
        <label key={key}>{label}: {Math.round(config[key] * 100)}%<input className="w-full" type="range" min="0" max="1" step="0.01" value={config[key]} onChange={event => update(key, Number(event.target.value))}/></label>
      )}
    </div>
    <OracleSeed onSeed={text => update("seed_text", text)}/>
    <div className="grid grid-cols-2 md:grid-cols-5 gap-2">{names.map((name, index) =>
      <label key={name} className="flex gap-1 items-center"><input type="checkbox" checked={config.enabled[index]} onChange={event => update("enabled", config.enabled.map((value, i) => i === index ? event.target.checked : value))}/>{name}<input className="w-12" type="number" min="0" max="127" value={config.notes[index]} onChange={event => update("notes", config.notes.map((value, i) => i === index ? Number(event.target.value) : value))}/></label>
    )}</div>
    <div className="flex gap-2 flex-wrap">
      <button className="border rounded p-2" onClick={() => generate(false)}>Generate / regenerate</button>
      <button className="border rounded p-2" onClick={() => generate(true)}>New pattern</button>
      <button className="border rounded p-2" onClick={vary}>Generate variation</button>
    </div>
    <section className="border rounded p-3 space-y-3">
      <h3 className="font-semibold">Local drum kit / your one-shots</h3>
      <div className="flex gap-3"><button className="border rounded p-2" onClick={play}>{playing ? "Restart" : "Play loop"}</button><button className="border rounded p-2" onClick={()=>{audio.current?.stop();setPlaying(false);}}>Stop</button><label><input type="checkbox" checked={loop} onChange={e=>setLoop(e.target.checked)}/> Loop</label></div>
      <p>Preview uses the generated MIDI timing and velocities. Each lane keeps its sound when you change its MIDI pitch. Pitch numbers are authoritative: DAWs may label octaves differently. Map these same notes to your FL drum instruments.</p>
      <div className="grid md:grid-cols-2 gap-2">{names.map((name,index)=><label key={name}>{name} · MIDI {config.notes[index]}<input type="file" accept="audio/*" onChange={async e=>{const file=e.target.files[0];if(!file)return;try{await getAudio().load(index,file);setSamples(s=>({...s,[index]:file.name}));setError("");}catch(err){setError(`Cannot decode ${file.name}: ${err}`);}e.target.value="";}}/><span className="text-xs">{samples[index] || "Built-in synthesized kit"}</span>{samples[index]&&<button onClick={()=>{audio.current?.samples.delete(index);setSamples(s=>{const next={...s};delete next[index];return next;});}}>Reset sound</button>}</label>)}</div>
      <p className="text-xs">One-shots stay in memory for this session. MIDI contains notes, not audio; load the same samples in FL. Regenerate after changing groove or MIDI mapping controls.</p>
    </section>
    {players && <div className="space-y-3">
      <h3 className="font-semibold">MIDI downloads</h3>
      <div><span>Full pattern</span> <button className="border rounded p-1" onClick={() => download(players.full, "full_pattern.mid")}>Download MIDI</button></div>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-3">{players.layers.map(layer => <div key={layer.name}><span>{layer.name}</span> <a className="underline" href={layer.url} download={`${layer.name.replace(/ /g,"_")}.mid`}>Download MIDI</a></div>)}</div>
    </div>}
    <div className="flex gap-3 items-center flex-wrap">
      <label>Patterns<input className="w-24 ml-2" type="number" min="1" max="10000" value={count} onChange={event => setCount(Number(event.target.value))}/></label>
      <label><input type="checkbox" checked={metadata} onChange={event => setMetadata(event.target.checked)}/> Metadata</label>
      <button disabled={busy} className="border rounded p-2" onClick={batch}>Export all MIDI files as ZIP</button>
      {busy && <span>Generated {progress}/{count}</span>}
    </div>
    {error && <p role="alert">{error}</p>}
    {preview && <pre className="text-xs overflow-auto max-h-80">{preview}</pre>}
    <p className="text-sm">Each enabled drum has a separate MIDI file in the ZIP. For large batches or a chosen output directory, use the drum_batch command.</p>
  </div>;
}
