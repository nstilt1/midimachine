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

import { 
  Tooltip, 
  TooltipContent, 
  TooltipProvider, 
  TooltipTrigger 
} from "./ui/tooltip";
import DropdownWithNavigation from "./DropdownWithNavigation";

const MidiForm = ({ 
  wasmModule, 
  showExtraControls, 
  toggleExtraControls,
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
  const [useSameChords, setUseSameChords] = useLocalStorage("useSameChords", true);
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
  const [patternToUse, setPatternToUse] = useLocalStorage("patternToUse", "--");
  const [duration, setDuration] = useLocalStorage("duration", 4);
  const [patterns, setPatterns] = useLocalStorage("patterns", ["--", "1-2-3-4", "1-1-2-3"]);

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
      isRandom,
      patternToUse,
      duration,
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
    if(settings.pattern) {
      setPatternToUse(settings.pattern);
    } else {
      setPatternToUse("");
    }
    if(settings.duration) {
      setDuration(settings.duration);
    } else {
      setDuration(4);
    }
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

  const handleDurationChange = (value) => {
    setDuration(value);
    if (value > 0) {
      setDuration(Math.round(value));
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
        !isRandom,
        patternToUse,
        duration,
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
    <div className="max-w-md">
      <form onSubmit={handleSubmit}>
        <div>
          {showExtraControls && 
            <Button onClick={toggleExtraControls}>Hide advanced controls</Button>
          }
          {!showExtraControls && 
            <Button onClick={toggleExtraControls}>Show advanced controls</Button>
          }
        {showExtraControls && <div>
          <div>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild className="w-full text-left"><div>
                  <Selector 
                    options={modes}
                    selectedOption={mode}
                    onChange={setMode}
                    label="Choose a mode:"
                  />
                </div></TooltipTrigger>
                <TooltipContent>
                  <p className="text-lg max-w-md">
                    The modes represent different chord placement algorithms.
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild className="w-full text-left"><div>
                  <input 
                    type="checkbox"
                    id="useSameChords"
                    checked={useSameChords}
                    onChange={handleUseSameChordsChange}
                  />
                  <label htmlFor="useSameChords">Use same chords for all modes?</label>
                </div></TooltipTrigger>
                <TooltipContent>
                  <p className="text-lg max-w-md">
                    When checked, this ensures that the same exact set of chords 
                    will be used when generating chords with different modes. 
                    Otherwise, the chords will likely be different.
                  </p>
                  <p className="text-lg max-w-md">
                    This is primarily useful for comparing the different chord 
                    placement algorithms (modes).
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild className="w-full text-left"><div>
                <NumberInput
                  value={numChords}
                  onChange={handleNumChordsChange}
                  id="numChords"
                  labelText="# of chords:"
                />
                </div></TooltipTrigger>
                <TooltipContent>
                  <p className="text-lg max-w-md">
                    Determines the number of chords that will be generated and 
                    placed into the MIDI file.
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild className="w-full text-left"><div>
                  <NumberInput
                    value={numUniqueChords}
                    onChange={handleNumUniqueChordsChange}
                    id="numUniqueChords"
                    labelText="# unique chords:"
                  />
                </div></TooltipTrigger>
                <TooltipContent>
                  <p className="text-lg max-w-md">
                    This setting attempts to ensure that the last N chords will 
                    be unique. There are some situations where there will be duplicate 
                    chords, such as when the value in the &quot;# unique chords&quot; 
                    is greater than the total amount of chords to pick from.
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild className="w-full text-left"><div>
                  <Selector 
                    options={keys} 
                    selectedOption={chosenKey}
                    onChange={setKey}
                    label="Choose a key:"
                  />
                </div></TooltipTrigger>
                <TooltipContent>
                  <p className="text-lg max-w-md">
                    This determines what key that the generated MIDI is supposed 
                    to be in.
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild className="w-full text-left"><div>
                <Selector 
                  options={chordGroups}
                  selectedOption={chordGroup}
                  onChange={setChordGroup}
                  label="Choose a chord group:"
                />
                {(chordGroup == "custom" || chordGroup == "custom_pruning") && <MultiSelect
                  options={customChordTypes}
                  selectedOptions={customChords}
                  setSelectedOptions={handleChordTypeSelection}
                />}
                </div></TooltipTrigger>
                <TooltipContent>
                  <p className="text-lg max-w-md">
                    The chord group determines which chords will be included in 
                    the initial vocabulary before pruning. You can view the chords 
                    in the vocabulary using the &quot;Chord Vocabulary&quot; menu.
                  </p>
                  <p className="text-lg max-w-md">
                    The Custom chord groups require you to select which chord types 
                    to be included. The Custom (use pruning) chord group initializes 
                    the vocabulary to have every possible chord with all roots. This 
                    is supposed to be pruned to a specific scale so as to find the 
                    potential chords of a key.
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild className="w-full text-left"><div>
                  <Selector
                    options={chordPickingMethods}
                    selectedOption={chord_picking_method}
                    onChange={setChordPickingMethod}
                    label="Choose a chord picking method:"
                  />
                </div></TooltipTrigger>
                <TooltipContent>
                  <p className="text-lg max-w-md">
                    Test different chord picking methods. The 2D method picks a 
                    random column in the Chord Table, then picks a random chord in 
                    that column.
                  </p>
                  <p className="text-lg max-w-md">
                    The 1D method simply picks a random chord from the Chord List.
                  </p>
                  <p className="text-lg max-w-md">
                    The probability of each chord being picked can be observed in 
                    the Chord Vocabulary menu. The Chord Table used by the MIDI 
                    Machine is arranged by &quot;Contains note&quot; if you want to see the 
                    same exact probabilities as the ones that are present in the 
                    MIDI Machine&apos;s output.
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>

            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild className="w-full text-left"><div>
                  <Selector 
                    options={scales}
                    selectedOption={scale}
                    onChange={setScale}
                    label="Prune chords to fit this scale:"
                  />
                </div></TooltipTrigger>
                <TooltipContent>
                  <p className="text-lg max-w-md">
                    Pruning chords removes all chords from the chord table that contain 
                    notes that are outside of the given scale. This theoretically can 
                    ensure that the resulting chords will all be in key.
                  </p>
                  <p className="text-lg max-w-md">
                    The &quot;No pruning, but clone chords with optional notes&quot; option copies 
                    chords that were defined with optional notes so that different variations 
                    of chords may show up in the chord table.
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
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
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger asChild className="w-full text-left"><div>
                    <input 
                      type="checkbox"
                      id="isReproducible"
                      checked={isRandom}
                      onChange={handleIsRandomChange}
                    />
                    <label htmlFor="isReproducible">Randomize output? (Not reproducible)</label>
                  </div></TooltipTrigger>
                  <TooltipContent>
                    <p className="text-lg max-w-md">
                      This checkbox can partially randomize the output and is not 
                      very reproducible. This works by <b>not</b> sorting an array 
                      created from a Hash Set, which is in a random order.
                    </p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
              
            </div>}
            {/* Duration and Patterns/sequences */}
            {false && (
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild className="w-full text-left"><div>
                <NumberInput
                  value={duration}
                  onChange={handleDurationChange}
                  id="duration"
                  labelText="Max duration of each chord (in beats):"
                />
                </div></TooltipTrigger>
                <TooltipContent>
                  <p className="text-lg max-w-md">
                    Determines the maximum amount of beats for each bar&apos;s 
                    chord. This should be less than 4, and the played chord will 
                    only last for 1 bar before it switches to the next chord.
                  </p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
            )}
            </div>
            
          </div>}
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild className="w-full text-left"><div>
                <Selector
                  options={vibes}
                  selectedOption={vibe}
                  onChange={setVibe}
                  label="Choose a vibe:"
                />
              </div></TooltipTrigger>
              <TooltipContent>
                <p className="text-lg max-w-md">
                  The vibe selector is a way to try to choose a completely different 
                  vibe and set of chords.
                </p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>
        <div>
          <ChatBar  
            onSubmit={handleSubmit} 
            onTextChange={handleTextChange}
            fileInputRef={fileInputRef}
            textInput={textInput}
          />
        </div>
        <DropdownWithNavigation
              value={patternToUse}
              setValue={setPatternToUse}
              options={patterns}
              setOptions={setPatterns}
              id="pattern"
              labelText="The pattern/sequence to use:"
              enabled={useSameChords && mode === "chords"}
            />
      </form>

      {midiFile && (
        <div>
      {/* Using the midi-player custom element for MIDI playback */}
      {midiFile && (
        <MidiPlayer
          midiFileUrl={midiFile}
          textInput={textInput}
          vibe={vibe}
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