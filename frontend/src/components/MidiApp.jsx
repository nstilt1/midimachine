"use client";

import React, { useEffect, useState } from 'react';
import dynamic from 'next/dynamic';
import MidiForm from './MidiForm';

const MidiApp = ({ showExtraControls }) => {
  const [wasmModule, setWasmModule] = useState(null);

  useEffect(() => {
    const loadWasm = async () => {
      try {
        const wasm = await import('../../public/musicgen.js');
        // console.log(wasm);
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
      <h1><span class="blend">&quot;</span>AI<span class="blend">&quot;</span> MIDI File Generator</h1>
      {wasmModule ? <MidiForm
        wasmModule={wasmModule}
        showExtraControls={showExtraControls}
      /> : <p>Loading...</p>}
    </div>
  );
};

export default MidiApp;
