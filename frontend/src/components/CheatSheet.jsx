"use client";

import { useState } from "react";
import Selector from "./Selector";
import { Button } from "./ui/button";
import ChordTable from "./ChordTable";
import MultiSelect from "./MultiSelector";

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
    showExtraControls
}) => {
    const [chords, setChords] = useState([]);
    const handleSubmit = async (event) => {
        event.preventDefault();

        if(chosenKey == "random") {
            alert("Please choose a key to view the chords of.");
            return;
        }

        try {
            const json = wasmModule.get_chords_of_key(chosenKey, customChords, chordGroup, scale);
            const data = JSON.parse(json);
            setChords(data);
            //console.log(data);
        } catch (error) {
            console.error("Error getting chords", error);
            alert("An error occurred while computing valid chords.");
        }
    }

    return (
        <div>
            <form onSubmit={handleSubmit}>
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
                {chordGroup == "custom" || chordGroup == "custom_pruning" && <MultiSelect
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
                        The "Custom (use pruning)" chord group is intended to be used with pruning. 
                        You are welcome to try it without pruning, but it will likely be 
                        unsatisfactory because the chords will probably not be in a 
                        specific key.
                    </p>
                </div>}
            </div>}
            <Button type="submit">Get Chords</Button>
            {chords && <ChordTable chordData={chords} chosenKey={chosenKey}/>}
            </form>
        </div>
    );
}

export default CheatSheet;