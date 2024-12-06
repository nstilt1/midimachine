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
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "./ui/tooltip";

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
    showExtraControls,
    tableScheme,
    setTableScheme,
    tableSchemes,
    cpbRef
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

        try {
            console.time("chord_finder");
            const json = wasmModule.chord_finder(chosenKey, customChords, chordGroup, scale, notes, tableScheme);
            console.timeEnd("chord_finder");

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
                <div>
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
                                This determines what key the chord table should be in. Leaving it set to "any" will check every 
                                possible chord.
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
                            </div>
                        </TooltipTrigger>
                        <TooltipContent>
                            <p className="text-lg max-w-md">
                                These will be the chord types that are included in the lookup. 
                                You can see a list of all the chords in each group in the &quot;Chord Vocabulary&quot; menu.
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
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger asChild className="w-full text-left"><div>
                        <label htmlFor="noteSelector" className="block text-gray-700 text-sm font-bold mb-2">Select Notes:</label>
                        <MultiSelect
                            id="noteSelector"
                            options={noteChoices}
                            selectedOptions={notes}
                            setSelectedOptions={handleNoteSelection}
                        />
                        </div></TooltipTrigger>
                        <TooltipContent>
                            <p className="text-lg max-w-md">
                                The notes that you want to find chords for.
                            </p>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger asChild className="w-full text-left"><div>
                        <Selector 
                            options={tableSchemes}
                            selectedOption={tableScheme}
                            onChange={setTableScheme}
                            label="Chord table arranged by:"
                        />
                    </div></TooltipTrigger>
                    <TooltipContent>
                        <p className="text-lg max-w-md">
                            Rearranges the Chord Table by this scheme.
                        </p>
                        <p className="text-lg max-w-md">
                        &quot;Contains note&quot; arranges each column so that the chords in the &quot;C&quot; 
                            column all contain the note C. Chords in the &quot;D&quot; column will contain the 
                            note D.
                        </p>
                        <p className="text-lg max-w-md">
                        &quot;Highest note&quot; arranges each column so that the chords in the &quot;C&quot; column 
                            will all have C as their highest note. This is likewise for the &quot;Lowest Note&quot;
                            arrangement.
                        </p>
                    </TooltipContent>
                </Tooltip>
            </TooltipProvider>

            </div>
            <Button type="submit">Find Chords</Button>
            {chords && <div>
                <Accordion type="multiple" defaultValue={["table", "list"]} collapsible>
                    <AccordionItem value="table">
                        <AccordionTrigger>Chord table</AccordionTrigger>
                        <AccordionContent><ChordTable chordData={chords} chosenKey={chosenKey} cpbRef={cpbRef}/></AccordionContent>
                    </AccordionItem>
                    <AccordionItem value="list">
                        <AccordionTrigger>Chord list</AccordionTrigger>
                        <AccordionContent>
                            <Accordion type="multiple" collapsible>
                            {allChords.map((chord, index) => (
                                <Chord
                                    key={index}
                                    json={chord}
                                    index={index}
                                    onAdd={() => {
                                        if (cpbRef.current) {
                                            cpbRef.current.handleAddChord(chord)
                                        }
                                    }}
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