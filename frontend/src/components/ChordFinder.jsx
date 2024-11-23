"use client";

import useLocalStorage from "@/hooks/useLocalStorage";
import { useState } from "react";
import MultiSelect from "./MultiSelector";
import Selector from "./Selector";
import { Button } from "./ui/button";
import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
  } from "@/components/ui/accordion";
import ChordTable from "./ChordTable";
import Chord from "./Chord";

const ChordFinder = ({
    wasmModule,
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
    scales,
    showExtraControls
}) => {
    const [chords, setChords] = useState([]);
    const [allChords, setAllChords] = useState([]);
    const [notes, setNotes] = useLocalStorage("notes", []);

    const handleNoteSelection = (option) => {
        if (notes.includes(option)) {
            setNotes(notes.filter((item) => item !== option));
        } else {
            setNotes([...notes, option]);
        }
    }
    
    const handleSubmit = async (event) => {
        event.preventDefault();

        if(chosenKey == "random") {
            alert("Please choose a key.");
            return;
        }

        try {
            const json = wasmModule.chord_finder(chosenKey, customChords, chordGroup, scale, notes);
            const data = JSON.parse(json);
            setChords(data['chord_table']);
            setAllChords(data['chord_list']);
        } catch (error) {
            console.error("Error getting chords", error);
            //alert("An error occurred while looking for chords.");
        }
    }

    const noteChoices = [
        "A",
        "A#",
        "B",
        "C",
        "C#",
        "D",
        "D#",
        "E",
        "F",
        "F#",
        "G",
        "G#"
    ];

    return (
        <div>
            <form onSubmit={handleSubmit}>
                <div>
                {showExtraControls && <div>
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
                {(chordGroup == "custom" || chordGroup == "custom_pruning") && <MultiSelect
                    options={customChordTypes}
                    selectedOptions={customChords}
                    setSelectedOptions={handleChordTypeSelection}
                />}
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
                <MultiSelect
                    options={noteChoices}
                    selectedOptions={notes}
                    setSelectedOptions={handleNoteSelection}
                />

            </div>}
            <Button type="submit">Find Chords</Button>
            {chords && <div>
                <Accordion type="multiple" collapsible>
                    <AccordionItem value="table">
                        <AccordionTrigger>Chord table</AccordionTrigger>
                        <AccordionContent><ChordTable chordData={chords} chosenKey={chosenKey}/></AccordionContent>
                    </AccordionItem>
                    <AccordionItem value="list">
                        <AccordionTrigger>Chord list</AccordionTrigger>
                        <AccordionContent>
                            <Accordion type="multiple" collapsible>
                            {allChords.map((chord, index) => (
                                <Chord
                                    key={index}
                                    midi={chord['midi']}
                                    chordName={chord['name']}
                                    notes={chord['notes']}
                                    index={index}
                                />
                            ))}
                            </Accordion>
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>
            </div>}
                </div>
            </form>
        </div>
    );
}

export default ChordFinder;