"use client";

import { useState } from "react";
import Selector from "./Selector";
import { Button } from "./ui/button";
import ChordTable from "./ChordTable";
import MultiSelect from "./MultiSelector";
import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
  } from "@/components/ui/accordion";
import Chord from "./Chord";
import { Checkbox } from "./ui/checkbox";
import useLocalStorage from "@/hooks/useLocalStorage";

const CheatSheet = ({
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
    const [showProbabilities, setShowProbabilities] = useLocalStorage("showProbability", false);

    const handleShowProbabilitiesChange = () => {
        setShowProbabilities(!showProbabilities);
    }

    const handleSubmit = async (event) => {
        event.preventDefault();

        if(chosenKey == "random") {
            alert("Please choose a key to view the chords of.");
            return;
        }

        try {
            console.time("get_vocabulary");
            const json = wasmModule.get_chords_of_key(chosenKey, customChords, chordGroup, scale, tableScheme, showProbabilities);
            console.timeEnd("get_vocabulary");

            const data = JSON.parse(json);
            setChords(data['chord_table']);
            setAllChords(data['chord_list']);
            //console.log(data);
        } catch (error) {
            console.error("Error getting chords", error);
            alert("An error occurred while computing valid chords.");
        }
    }

    return (
        <div>
            <form onSubmit={handleSubmit}>
            <div>
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
                <Selector 
                    options={tableSchemes}
                    selectedOption={tableScheme}
                    onChange={setTableScheme}
                    label="Chord table arranged by:"
                />
            </div>
            <input 
                type="checkbox"
                id="showProbabilities" 
                checked={showProbabilities} 
                onChange={handleShowProbabilitiesChange} 
            />
            <label 
                htmlFor="showProbabilities"
                className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >
                Show probabilities?
            </label>
            <br />
            <Button type="submit">Get Chords</Button>
            {chords && <div>
                <Accordion type="multiple" collapsible>
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
                                            cpbRef.current.handleAddChord(chord);
                                        }
                                    }}
                                />
                            ))}
                            </Accordion>
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>
            </div>}
            </form>
        </div>
    );
}

export default CheatSheet;