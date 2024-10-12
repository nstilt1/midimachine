"use client";

import React, { useEffect, useState } from 'react';
import dynamic from 'next/dynamic';
import MidiForm from '../components/MidiForm';

const MidiApp: React.FC = () => {
  const [wasmModule, setWasmModule] = useState<any | null>(null);

  useEffect(() => {
    const loadWasm = async () => {
      try {
        const wasm = await import('../../public/musicgen.js');
        console.log(wasm);
        await wasm.default();
        setWasmModule(wasm);
      } catch (error) {
        console.error("Error loading WASM module", error);
      }
    };
    loadWasm();
  }, []);

  return (
    <div>
      <h1>MIDI File Generator</h1>
      {wasmModule ? <MidiForm wasmModule={wasmModule} /> : <p>Loading WASM...</p>}
    </div>
  );
};

export default MidiApp;
