"use client";

import React, { useEffect, useState } from 'react';
import dynamic from 'next/dynamic';
import useLocalStorage from '@/hooks/useLocalStorage';
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger
} from '@/components/ui/tabs';
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import MidiForm from './MidiForm';
import CheatSheet from './CheatSheet';
import ChordFinder from './ChordFinder';

const MidiApp = ({ showExtraControls, cpbRef, wasmModule }) => {
  const [key, setKey] = useLocalStorage("key", 'random');
  const [chordGroup, setChordGroup] = useLocalStorage("chordGroup", 'default');
  const [customChords, setCustomChords] = useLocalStorage("customChords", []);
  const [scale, setScale] = useLocalStorage("scale", "disabled");
  const [tableScheme, setTableScheme] = useLocalStorage("tableScheme", "contains_note");

  const handleChordTypeSelection = (option) => {
    if (customChords.includes(option)) {
      setCustomChords(customChords.filter((item) => item !== option));
    } else {
      setCustomChords([...customChords, option]);
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

  const chordGroups = [
    { label: "Default", value: "default" },
    { label: "Original", value: "original" },
    { label: "Custom (hand-picked)", value: "custom" },
    { label: "Custom (use pruning)", value: "custom_pruning" }
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

  const scales = [
    { label: "Disable chord pruning", value: "disabled" },
    { label: "Natural", value: "natural" },
    { label: "Melodic", value: "melodic" },
    { label: "Harmonic", value: "harmonic" },
    { label: "Pentatonic", value: "pentatonic" },
    { label: "Romanian", value: "romanian" },
    { label: "Hungarian", value: "hungarian" },
    { label: "No pruning, but clone chords with optional notes", value: "all_notes" }
  ];

  const tableSchemes = [
    { label: "Contains note", value: "contains_note" },
    { label: "Highest note", value: "highest_note" },
    { label: "Lowest note", value: "lowest_note" }
  ];

  return (
    <div>
      {wasmModule ? 
        <div>
          <Tabs defaultValue="generator">
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="generator">Generator</TabsTrigger>
              <TabsTrigger value="cheat-sheet">Chord Vocabulary</TabsTrigger>
              <TabsTrigger value="chord-finder">Chord Finder</TabsTrigger>
            </TabsList>
            <TabsContent value="generator">
              <Card>
                <CardHeader>
                  <CardTitle><span className="blend">&quot;</span>AI<span className="blend">&quot;</span> MIDI File Generator</CardTitle>
                </CardHeader>
                <CardContent>
              <MidiForm
                wasmModule={wasmModule}
                showExtraControls={showExtraControls}
                chosenKey={key}
                setKey={setKey}
                chordGroup={chordGroup}
                setChordGroup={setChordGroup}
                customChords={customChords}
                scale={scale}
                setScale={setScale}
                handleChordTypeSelection={handleChordTypeSelection}
                keys={keys}
                chordGroups={chordGroups}
                customChordTypes={customChordTypes}
                scales={scales}
              />
              </CardContent>
              </Card>
            </TabsContent>
            <TabsContent value="cheat-sheet">
              <Card>
                <CardHeader>
                  <CardTitle>Chord Vocabulary</CardTitle>
                </CardHeader>
                <CardContent>
              <CheatSheet
                wasmModule={wasmModule}
                chosenKey={key}
                setKey={setKey}
                chordGroup={chordGroup}
                setChordGroup={setChordGroup}
                customChords={customChords}
                scale={scale}
                setScale={setScale}
                handleChordTypeSelection={handleChordTypeSelection}
                keys={keys}
                chordGroups={chordGroups}
                customChordTypes={customChordTypes}
                scales={scales}
                showExtraControls={showExtraControls}
                tableScheme={tableScheme}
                setTableScheme={setTableScheme}
                tableSchemes={tableSchemes}
                cpbRef={cpbRef}
              />
              </CardContent>
              </Card>
            </TabsContent>
            <TabsContent value="chord-finder">
              <Card>
                <CardHeader>
                  <CardTitle>Chord Finder</CardTitle>
                </CardHeader>
                <CardContent>
                  <ChordFinder
                    wasmModule={wasmModule}
                    chosenKey={key}
                    setKey={setKey}
                    chordGroup={chordGroup}
                    setChordGroup={setChordGroup}
                    customChords={customChords}
                    scale={scale}
                    setScale={setScale}
                    handleChordTypeSelection={handleChordTypeSelection}
                    keys={keys}
                    chordGroups={chordGroups}
                    customChordTypes={customChordTypes}
                    scales={scales}
                    showExtraControls={showExtraControls}
                    tableScheme={tableScheme}
                    setTableScheme={setTableScheme}
                    tableSchemes={tableSchemes}
                    cpbRef={cpbRef}
                  />
                </CardContent>
              </Card>
            </TabsContent>
          </Tabs>
          </div> : <p>Loading...</p>}
    </div>
  );
};

export default MidiApp;
