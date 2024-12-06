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
import { 
    Tooltip, 
    TooltipContent, 
    TooltipProvider, 
    TooltipTrigger 
} from "./ui/tooltip";

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
                                This determines what key that the chord table should be in.
                            </p>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
                {chosenKey == "random" && 
                <div className="w-full max-w-sm">
                <p className="text-red-500">
                    This is a valid setting, but be aware that the chords in the 
                    chord table will not be in a specific key. This means that 
                    there will be 12 times as many chords in the chord table.
                </p>
                </div>}
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger asChild className="w-full text-left"><div>
                            <Selector
                                options={chordGroups}
                                selectedOption={chordGroup}
                                onChange={setChordGroup}
                                label="Choose a chord group:"
                            />
                        </div></TooltipTrigger>
                        <TooltipContent>
                            <p className="text-lg max-w-md">
                                These will be the chord types included in the initial vocabulary before 
                                pruning. You can see the chords in each chord group in this vocabulary 
                                menu by inspecting the generated Chord Table and Chord List.
                            </p>
                            <p className="text-lg max-w-md">
                                The Custom (hand-picked) chord group should not need pruning, while 
                                the Custom (use pruning) chord group will simply initialize the 
                                vocabulary to use every possible chord with every root. Then, you can 
                                prune chords that have notes outside of the scale to theoretically 
                                obtain a list of chords that are inside the specified key.
                            </p>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
                {(chordGroup == "custom" || chordGroup == "custom_pruning") && 
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger asChild className="w-full text-left"><div>
                            <MultiSelect
                                options={customChordTypes}
                                selectedOptions={customChords}
                                setSelectedOptions={handleChordTypeSelection}
                            />
                        </div></TooltipTrigger>
                        <TooltipContent>
                            <p className="text-lg max-w-md">
                                These will be the chord types included in the initial vocabulary before 
                                pruning. You can see the chords in each chord group in this vocabulary 
                                menu by inspecting the generated Chord Table and Chord List.
                            </p>
                            <p className="text-lg max-w-md">
                                The Custom (hand-picked) chord group should not need pruning, while 
                                the Custom (use pruning) chord group will simply initialize the 
                                vocabulary to use every possible chord with every root. Then, you can 
                                prune chords that have notes outside of the scale to theoretically 
                                obtain a list of chords that are inside the specified key.
                            </p>
                        </TooltipContent>
                    </Tooltip>
                </TooltipProvider>
                }
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
                    </p>
                    <p className="text-red-500">
                        You are welcome to try it without pruning, but it will likely be 
                        unsatisfactory because the chords will probably not be in a 
                        specific key.
                    </p>
                </div>}
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
            <TooltipProvider>
                <Tooltip>
                    <TooltipTrigger asChild><div>
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
                    </div></TooltipTrigger>
                    <TooltipContent>
                        <p className="text-lg max-w-md">
                            Shows the probabilities of each chord getting picked from the table using 
                            either one-dimensional or two-dimensional chord picking methods. The 2D 
                            probabilities are only valid when the table is arranged by &quot;Contains note.&quot;
                        </p>
                    </TooltipContent>
                </Tooltip>
            </TooltipProvider>
            <br />
            <Button type="submit">Get Chords</Button>
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