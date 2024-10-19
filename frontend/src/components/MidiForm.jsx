"use client";

import Image from "next/image";
import React, { useRef, useState } from 'react';
import MidiPlayer from "./MidiPlayer";
import ChatBar from "./ChatBar";
import Selector from "./Selector";
import NumberInput from "./NumberInput";
import MultiSelect from "./MultiSelector";
import useLocalStorage from "@/hooks/useLocalStorage";

const MidiForm = ({ wasmModule, showExtraControls }) => {
  const [textInput, setTextInput] = useLocalStorage("textInput", '');
  const [selectedOption, setSelectedOption] = useLocalStorage("selectedOption", 'melody');
  const [useSameChords, setUseSameChords] = useLocalStorage("useSameChords", false);
  const [midiFile, setMidiFile] = useState(null);
  const [numChords, setNumChords] = useLocalStorage("numChords", 20);
  const [key, setKey] = useLocalStorage("key", 'random');
  const [vibe, setVibe] = useLocalStorage("vibe", 'default');
  const [chordGroup, setChordGroup] = useLocalStorage("chordGroup", 'default');
  const [customChords, setCustomChords] = useLocalStorage("customChords", []);
  const [chord_picking_method, setChordPickingMethod] = useLocalStorage("chord_picking_method", 'original')

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

  const handleChordTypeSelection = (option) => {
    if (customChords.includes(option)) {
      setCustomChords(customChords.filter((item) => item !== option));
    } else {
      setCustomChords([...customChords, option]);
    }
  };

  const handleNumChordsChange = (value) => {
    // ensure the number stored in `numChords` is greater than 0
    if (value > 0) {
      setNumChords(Math.round(value));
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
      let textBinary;
      if (vibe == "default") {
        textBinary = new TextEncoder().encode(textInput);
      } else {
        textBinary = new TextEncoder().encode(textInput + vibe);
      }

      const combinedBinary = new Uint8Array(fileBinary.length + textBinary.length);
      combinedBinary.set(fileBinary);
      combinedBinary.set(textBinary, fileBinary.length);

      //console.log("useSameChords = " + useSameChords);
      //console.log("key: " + key);
      const midiBinary = wasmModule.generate_midi(
        combinedBinary, 
        selectedOption, 
        useSameChords, 
        numChords, key, 
        customChords, 
        chordGroup,
        chord_picking_method
      );

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

  const vibes = [
    { label: "Default vibe", value: "default"},
    { label: "Vibe 1", value: "1" },
    { label: "Vibe 2", value: "2" },
    { label: "Vibe 3", value: "3" },
    { label: "Vibe 4", value: "4" },
    { label: "Vibe 5", value: "5" },
    { label: "Vibe 6", value: "6" },
    { label: "Vibe 7", value: "7" },
    { label: "Vibe 8", value: "8" },
    { label: "Vibe 9", value: "9" },
    { label: "Vibe 10", value: "10" }
  ];

  const chordGroups = [
    { label: "Default", value: "default" },
    { label: "Original", value: "original" },
    { label: "Custom", value: "custom" }
  ];

  const customChordTypes = [
    "major",
    "minor",
    "minor7",
    "major7",
    "diminished",
    "augmented",
    "major6",
    "minor6",
    "major9",
    "minor9",
    "major7sharp9",
    "major7flat5sharp9",
    "major9flat5",
    "major7flat9",
    "major13",
    "dominant9",
    "add9"
  ];

  const chordPickingMethods = [
    { label: "Original", value: "original" },
    { label: "1D", value: "1D" }
  ]

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <div>
        {showExtraControls && <div><label>Select an Option:</label>
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
            <br/>
            <input 
              type="checkbox"
              id="useSameChords"
              checked={useSameChords}
              onChange={handleUseSameChordsChange}
            />
            <label htmlFor="useSameChords">Use same chords for melody and chords?</label>
            <NumberInput
              value={numChords}
              onChange={handleNumChordsChange}
              id="numChords"
              labelText="# of chords:"
            />
            Num Chords
            <Selector 
              options={keys} 
              selectedOption={key}
              onChange={setKey}
              label="Choose a key:"
            />
            <Selector 
              options={chordGroups}
              selectedOption={chordGroup}
              onChange={setChordGroup}
              label="Choose a chord group:"
            />
            {chordGroup == "custom" && <MultiSelect
              options={customChordTypes}
              selectedOptions={customChords}
              setSelectedOptions={handleChordTypeSelection}
            />}
            <Selector
              options={chordPickingMethods}
              selectedOptions={chord_picking_method}
              onChange={setChordPickingMethod}
              label="Choose a chord picking method:"
            />
            </div>
            
          </div>}
            <Selector
              options={vibes}
              selectedOption={vibe}
              onChange={setVibe}
              label="Choose a vibe:"
            />
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