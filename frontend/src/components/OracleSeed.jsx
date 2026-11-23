"use client";
import { useState } from 'react';
import { generateOracleSeed } from '@/lib/godspeak';
export default function OracleSeed({ onSeed }) {
  const [count, setCount] = useState(16);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  const generate = async () => {
    setBusy(true); setError('');
    try { onSeed(await generateOracleSeed(count)); }
    catch (cause) { setError(String(cause)); }
    finally { setBusy(false); }
  };
  return <div className="my-3 space-y-1">
    <div className="flex gap-2 items-center flex-wrap">
      <button type="button" className="border rounded p-2" disabled={busy} onClick={generate}>{busy ? 'Loading oracle…' : 'Generate HolyC oracle seed'}</button>
      <label>Words <select value={count} onChange={e=>setCount(Number(e.target.value))}>{[4,8,16,32].map(n=><option key={n} value={n}>{n}</option>)}</select></label>
    </div>
    <p className="text-xs">Terry Davis’s original word-selection core compiled to WASM, with browser randomness. Replaces the seed text; generate the MIDI when ready.</p>
    {error && <p role="alert">{error}</p>}
  </div>;
}
