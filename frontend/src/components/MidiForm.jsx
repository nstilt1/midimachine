"use client";

import Image from "next/image";
import React, { useRef, useState } from 'react';
import MidiPlayer from "./MidiPlayer";
import ChatBar from "./ChatBar";
import Selector from "./Selector";
import NumberInput from "./NumberInput";
import MultiSelect from "./MultiSelector";
import useLocalStorage from "@/hooks/useLocalStorage";
import { Button } from "@/components/ui/button";
import { Save } from "lucide-react";
import SavedChords from "./SavedChords";

const MidiForm = ({ 
  wasmModule, 
  showExtraControls, 
  chosenKey, 
  setKey,
  chordGroup,
  setChordGroup,
  customChords,
  scale,
  setScale,
  handleChordTypeSelection,
  keys,
  chordGroups,
  customChordTypes,
  scales
}) => {
  const [textInput, setTextInput] = useLocalStorage("textInput", '');
  const [mode, setMode] = useLocalStorage("mode", "melody");
  const [useSameChords, setUseSameChords] = useLocalStorage("useSameChords", false);
  const [midiFile, setMidiFile] = useState(null);
  const [numChords, setNumChords] = useLocalStorage("numChords", 20);
  const [sanitizedNumChords, setSanitizedNumChords] = useLocalStorage("sanitizedNumChords", 20);
  const [vibe, setVibe] = useLocalStorage("vibe", 'default');
  const [chord_picking_method, setChordPickingMethod] = useLocalStorage("chord_picking_method", 'original');
  const [numUniqueChords, setNumUniqueChords] = useLocalStorage("numUniqueChords", 0);
  const [sanitizedNumUniqueChords, setSanitizedNumUniqueChords] = useLocalStorage("sanitizedNumUniqueChords", 0);
  const [savedChordsOpen, setSavedChordsOpen] = useState(false);
  const fileInputRef = useRef(null);
  const [isRandom, setIsRandom] = useLocalStorage("isRandom", true);

  // Function to save current form settings
  const saveCurrentSettings = (name) => {
    const settingsToSave = {
      chosenKey,
      chordGroup,
      customChords,
      scale,
      textInput,
      mode,
      useSameChords,
      sanitizedNumChords,
      numChords,
      vibe,
      chord_picking_method,
      numUniqueChords,
      sanitizedNumUniqueChords,
      isRandom
    };
  
    // Use useLocalStorage to save
    const savedProgressions = JSON.parse(localStorage.getItem('savedProgressions') || '{}');
    savedProgressions[name] = {
      type: 'generated',
      contents: settingsToSave,
      timestamp: new Date().toISOString()
    };
    localStorage.setItem('savedProgressions', JSON.stringify(savedProgressions));
  };

  // Function to load saved settings
  const handleLoadSettings = (settings) => {
    // Update each state variable from the loaded settings
    setKey(settings.chosenKey);
    setChordGroup(settings.chordGroup);
    handleChordTypeSelection(settings.customChords);
    setScale(settings.scale);
    setTextInput(settings.textInput);
    setMode(settings.mode);
    setUseSameChords(settings.useSameChords);
    setNumChords(settings.numChords);
    setSanitizedNumChords(settings.sanitizedNumChords);
    setVibe(settings.vibe);
    setChordPickingMethod(settings.chord_picking_method);
    setNumUniqueChords(settings.numUniqueChords);
    setSanitizedNumUniqueChords(settings.sanitizedNumUniqueChords);
    setIsRandom(settings.isRandom);
  };

  const handleTextChange = (event) => {
    setTextInput(event.target.value);
  }

  const handleUseSameChordsChange = (event) => {
    setUseSameChords(!useSameChords);
  }

  const handleNumChordsChange = (value) => {
    // ensure the number stored in `numChords` is greater than 0
    setNumChords(value);
    if (value > 0) {
      setSanitizedNumChords(Math.round(value));
    }
  }

  const handleNumUniqueChordsChange = (value) => {
    setNumUniqueChords(value);
    if (value >= 0) {
      setSanitizedNumUniqueChords(Math.round(value));
    }
  }

  const handleIsRandomChange = (event) => {
    setIsRandom(!isRandom);
  }

  const handleSubmit = async (event) => {
    event.preventDefault();

    if(fileInputRef.current.files.length == 0 && !textInput) {
      alert("Please provide an input.");
      return;
    }

    if((chordGroup == "custom" || chordGroup == "custom_pruning") && customChords.length == 0) {
      alert("Please choose some chord types");
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
      //console.log("key: " + chosenKey);
      console.time("generate_midi");
      const midiBinary = wasmModule.generate_midi(
        combinedBinary, 
        mode, 
        useSameChords, 
        sanitizedNumChords, 
        chosenKey, 
        customChords, 
        chordGroup,
        chord_picking_method,
        sanitizedNumUniqueChords,
        scale,
        !isRandom
      );
      console.timeEnd("generate_midi");

      const midiBlob = new Blob([midiBinary], { type: 'audio/midi' });
      const midiUrl = URL.createObjectURL(midiBlob);

      setMidiFile(midiUrl);
    } catch (error) {
      console.error("Error processing file", error);
      alert("An error occurred while generating the MIDI file.");
    }
  };

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

  const chordPickingMethods = [
    { label: "Original - 2D", value: "original" },
    { label: "1D", value: "1D" }
  ];

  const modes = [
    { label: "Melody", value: "melody" },
    { label: "Chords", value: "chords" },
    { label: "Melody v2", value: "melody v2" },
    { label: "Melody v3", value: "melody v3" },
    { label: "Intended Placement", value: "intended" }
  ];

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <div>
        {showExtraControls && <div>
          <div>
            <Selector 
              options={modes}
              selectedOption={mode}
              onChange={setMode}
              label="Choose a mode:"
            />
            <input 
              type="checkbox"
              id="useSameChords"
              checked={useSameChords}
              onChange={handleUseSameChordsChange}
            />
            <label htmlFor="useSameChords">Use same chords for all modes?</label>
            <NumberInput
              value={numChords}
              onChange={handleNumChordsChange}
              id="numChords"
              labelText="# of chords:"
            />
            <NumberInput
              value={numUniqueChords}
              onChange={handleNumUniqueChordsChange}
              id="numUniqueChords"
              labelText="# unique chords:"
            />
            <Selector 
              options={keys} 
              selectedOption={chosenKey}
              onChange={setKey}
              label="Choose a key:"
            />
            <Selector 
              options={chordGroups}
              selectedOption={chordGroup}
              onChange={setChordGroup}
              label="Choose a chord group:"
            />
            {chordGroup == "custom" || chordGroup == "custom_pruning" && <MultiSelect
              options={customChordTypes}
              selectedOptions={customChords}
              setSelectedOptions={handleChordTypeSelection}
            />}
            <Selector
              options={chordPickingMethods}
              selectedOption={chord_picking_method}
              onChange={setChordPickingMethod}
              label="Choose a chord picking method:"
            />
            <Selector 
              options={scales}
              selectedOption={scale}
              onChange={setScale}
              label="Prune chords to fit this scale:"
            />
            {(scale == "all_notes" || scale == "disabled") && chordGroup == "custom_pruning" && 
            <div className="w-full max-w-sm">
              <p className="text-red-500">
                The &quot;Custom (use pruning)&quot; chord group is intended to be used with pruning. 
                You are welcome to try it without pruning, but it will likely be 
                unsatisfactory because the chords will probably not be in a 
                specific key.
              </p>
            </div>}
            {scale != "disabled" && 
            <div>
              <input 
              type="checkbox"
              id="isReproducible"
              checked={isRandom}
              onChange={handleIsRandomChange}
            />
              <label htmlFor="isReproducible">Randomize output? (Not reproducible)</label>
            </div>}

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
            textInput={textInput}
          />
        </div>
      </form>

      {midiFile && (
        <div>
      {/* Using the midi-player custom element for MIDI playback */}
      {midiFile && (
        <MidiPlayer
          midiFileUrl={midiFile}
          textInput={textInput}
        ></MidiPlayer>
      )}
      <div className="mb-4">
          <Button 
            type="button"
            variant="outline" 
            onClick={() => setSavedChordsOpen(true)}
          >
            <Save className="mr-2 h-4 w-4" /> Save/Load Settings
          </Button>
        </div>
        </div>
      )}
      <SavedChords 
        isOpen={savedChordsOpen}
        onClose={() => setSavedChordsOpen(false)}
        currentChords={(name) => {
          // Create a function that returns the current settings
          const settingsToSave = {
            chosenKey,
            chordGroup,
            customChords,
            scale,
            textInput,
            mode,
            useSameChords,
            sanitizedNumChords,
            numChords,
            vibe,
            chord_picking_method,
            numUniqueChords,
            sanitizedNumUniqueChords,
            isRandom
          };
          return settingsToSave;
        }}
        onLoadProgression={handleLoadSettings}
        filterType="generated"
      />
    </div>
  );
};

export default MidiForm;