"use client";

import Image from "next/image";
import MidiApp from "@/components/MidiApp";
import { useEffect, useRef, useState } from "react";
import useLocalStorage from "@/hooks/useLocalStorage";
import Navbar from "@/components/NavbarMainSite";
import ChordProgressionBuilder from "@/components/ChordProgressionBuilder";

export default function Home() {
  const [clickedLogo, setClickedLogo] = useLocalStorage("clickedLogo", false);
  const cpbRef = useRef(null);
  const [buttonText, setButtonText] = useState(clickedLogo);
  const [wasmModule, setWasmModule] = useState(null);
  const [showBuilder, setShowBuilder] = useLocalStorage("showBuilder", false);

  useEffect(() => {
    setButtonText(clickedLogo ? "Hide advanced controls" : "Show advanced controls")
  }, [clickedLogo]);

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
      <Navbar whenClickedLogo={() => setClickedLogo(!clickedLogo)} />
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
        <MidiApp showExtraControls={clickedLogo} cpbRef={cpbRef} wasmModule={wasmModule}></MidiApp>
      </main>
      {showBuilder && <ChordProgressionBuilder ref={cpbRef} wasmModule={wasmModule} />}
      <footer className="row-start-3 flex gap-6 flex-wrap items-center justify-center p-8">
        <a
          className="flex items-center gap-2 hover:underline hover:underline-offset-4"
          target="_blank"
          href="https://alteredbrainchemistry.com"
          rel="noopener noreferrer"
          onClick={(event) => {
            event.preventDefault();
            setClickedLogo(!clickedLogo);
          }}
        >
          <Image
            aria-hidden
            src="https://nextjs.org/icons/file.svg"
            alt="File icon"
            width={16}
            height={16}
          />
          {buttonText}
        </a>
        <a
          className="flex items-center gap-2 hover:underline hover:underline-offset-4"
          href="https://alteredbrainchemistry.com"
          target="_blank"
          rel="noopener noreferrer"
          onClick={(event) => {
            event.preventDefault();
            setShowBuilder(!showBuilder);
          }}
        >
          <Image
            aria-hidden
            src="https://nextjs.org/icons/window.svg"
            alt="Window icon"
            width={16}
            height={16}
          />
          Toggle Progression Builder
        </a>
        <a
          className="flex items-center gap-2 hover:underline hover:underline-offset-4"
          href="https://youtu.be/9elEPFb5f-A"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Image
            aria-hidden
            src="https://nextjs.org/icons/globe.svg"
            alt="Globe icon"
            width={16}
            height={16}
          />
          How this works →
        </a>
      </footer>
    </div>
    </div>
  );
}
