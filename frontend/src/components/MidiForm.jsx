"use client";

import Image from "next/image";
import React, { useRef, useState } from 'react';
import MidiPlayer from "./MidiPlayer";
import ChatBar from "./ChatBar";
import Selector from "./Selector";

const MidiForm = ({ wasmModule }) => {
  const [textInput, setTextInput] = useState('');
  const [selectedOption, setSelectedOption] = useState('melody');
  const [useSameChords, setUseSameChords] = useState(false);
  const [midiFile, setMidiFile] = useState(null);
  const [numChords, setNumChords] = useState(20);
  const [key, setKey] = useState('random');

  const fileInputRef = useRef(null);

  const handleTextChange = (event) => {
    setTextInput(event.target.value);
  }

  const handleOptionChange = (event) => {
    setSelectedOption(event.target.value);
  }

  const handleUseSameChordsChange = (event) => {
    setUseSameChords(!useSameChords);
  }

  const handleNumChordsChange = (event) => {
    // ensure the number stored in `numChords` is greater than 0
    if (event.target.value > 0) {
      setNumChords(Math.round(event.target.value));
    }
  }

  const handleSubmit = async (event) => {
    event.preventDefault();

    if(fileInputRef.current.files.length == 0 && !textInput) {
      alert("Please provide an input.");
      return;
    }
    if(!selectedOption) {
      alert("Please select an option.");
      return;
    }

    try {
        let fileBinary;
        if (fileInputRef.current.files.length == 0) {
            fileBinary = new Uint8Array(0);
        } else {
            const fileArrayBuffer = await fileInputRef.current.files[0].arrayBuffer();
            fileBinary = new Uint8Array(fileArrayBuffer);
        }
      
      const textBinary = new TextEncoder().encode(textInput);

      const combinedBinary = new Uint8Array(fileBinary.length + textBinary.length);
      combinedBinary.set(fileBinary);
      combinedBinary.set(textBinary, fileBinary.length);

      //console.log("useSameChords = " + useSameChords);
      //console.log("key: " + key);
      const midiBinary = wasmModule.generate_midi(combinedBinary, selectedOption, useSameChords, numChords, key);

      const midiBlob = new Blob([midiBinary], { type: 'audio/midi' });
      const midiUrl = URL.createObjectURL(midiBlob);

      setMidiFile(midiUrl);
    } catch (error) {
      console.error("Error processing file", error);
      alert("An error occurred while generating the MIDI file.");
    }
  };

  const keys = [
    { label: "Pick one for me", value: "random" },
    { label: "C minor", value: "Cmin" },
    { label: "C# minor", value: "C#min" },
    { label: "D minor", value: "Dmin" },
    { label: "D# minor", value: "D#min" },
    { label: "E minor", value: "Emin" },
    { label: "F minor", value: "Fmin" },
    { label: "F# minor", value: "F#min" },
    { label: "G minor", value: "Gmin" },
    { label: "G# minor", value: "G#min" },
    { label: "A minor", value: "Amin" },
    { label: "A# minor", value: "A#min" },
    { label: "B minor", value: "Bmin" },
    { label: "C major", value: "Cmaj" },
    { label: "C# major", value: "C#maj" },
    { label: "D major", value: "Dmaj" },
    { label: "D# major", value: "D#maj" },
    { label: "E major", value: "Emaj" },
    { label: "F major", value: "Fmaj" },
    { label: "F# major", value: "F#maj" },
    { label: "G major", value: "Gmaj" },
    { label: "G# major", value: "G#maj" },
    { label: "A major", value: "Amaj" },
    { label: "A# major", value: "A#maj" },
    { label: "B minor", value: "Bmaj" }
  ];

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <div>
          <label>Select an Option:</label>
          <div>
            <input
              type="radio"
              value="melody"
              checked={selectedOption === 'melody'}
              onChange={handleOptionChange}
              required
            />
            Melody
            <input
              type="radio"
              value="chords"
              checked={selectedOption === 'chords'}
              onChange={handleOptionChange}
              required
            />
            Chords
            <input 
              type="checkbox"
              id="useSameChords"
              checked={useSameChords}
              onChange={handleUseSameChordsChange}
            />
            <label htmlFor="useSameChords">Use same chords for melody and chords?</label>
            <br/>
            <input
              type="number"
              id="numChords"
              onChange={handleNumChordsChange}
            />
            Num Chords
            <p>Key</p>
            <Selector 
              options={keys} 
              selectedOption={key}
              onChange={setKey}
              label="Choose a key:"
            />
          </div>
        </div>
        <div>
          <ChatBar  
            onSubmit={handleSubmit} 
            onTextChange={handleTextChange}
            fileInputRef={fileInputRef}
          />
        </div>
      </form>

      {midiFile && (
        <div>
      {/* Using the midi-player custom element for MIDI playback */}
      {midiFile && (
        <MidiPlayer
          midiFileUrl={midiFile}
        ></MidiPlayer>
      )}
        </div>
      )}
    </div>
  );
};

export default MidiForm;