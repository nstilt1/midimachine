const { default: useLocalStorage } = require("@/hooks/useLocalStorage")

import { Trash, ArrowUp, ArrowDown, Plus, Save, Play, Square, LoaderPinwheel } from "lucide-react";
import { forwardRef, useEffect, useImperativeHandle, useRef, useState } from "react";

import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";

import Chord from "./Chord";
import { Button } from "./ui/button";

const ChordProgressionBuilder = forwardRef(({ initialChordTable, wasmModule }, ref) => {
    const [chords, setChords] = useLocalStorage("chordProgressionBuilderChords", []);
    const [isPlaying, setIsPlaying] = useState(false);
    const playerRef = useRef(null);
    const [midiFileUrl, setMidiFileUrl] = useState(null);
    const [downloadUrl, setDownloadUrl] = useState(null);
    const [isMidiUpToDate, setIsMidiUpToDate] = useState(false);
    const [isLoading, setIsLoading] = useState(false);

    const updateMidi = async () => {
        if (chords.length === 0) {
            console.warn("No chords to generate MIDI");
            return;
        }
    
        try {
            console.time("generate_chord_progression_midi");
            let chordsArr = chords.map(chord => chord['note_vec']);
            const midiBinary = wasmModule.generate_midi_chord_progression(chordsArr);
            console.timeEnd("generate_chord_progression_midi");
            
            const midiBlob = new Blob([midiBinary], { type: 'audio/midi' });
            const midiUrl = URL.createObjectURL(midiBlob);
            
            // Revoke previous URL to prevent memory leaks
            if (midiFileUrl) {
                URL.revokeObjectURL(midiFileUrl);
            }
            
            setMidiFileUrl(midiUrl);
        } catch (error) {
            console.error("Error generating chord progression MIDI file:", error);
            throw error; // Rethrow to be caught in togglePlayOrPause
        }
    }

    const togglePlayOrPause = async () => {
        if (isPlaying) {
            playerRef.current.stop();
            console.log("Stopping playback");
            setIsPlaying(false);
        } else {
            playerRef.current.loop = true;
            
            // If MIDI is not up to date, generate it before playing
            if (!isMidiUpToDate) {
                try {
                    setIsLoading(true);
                    await updateMidi();
                    setIsMidiUpToDate(true);
                } catch (error) {
                    console.error("Failed to generate MIDI", error);
                    setIsLoading(false);
                    return;
                }
            }
    
            // Wait for a short moment to ensure the player is ready
            await new Promise(resolve => setTimeout(resolve, 100));
            
            setIsLoading(false);
            playerRef.current.start();
            setIsPlaying(true);
        }
    }

    useEffect(() => {
        if(midiFileUrl) {
            fetch(midiFileUrl)
                .then((response) => response.blob())
                .then((blob) => {
                    const url = URL.createObjectURL(blob);
                    setDownloadUrl(url);
                });

            // clean up previous url if it exists
            return () => {
                if(downloadUrl) {
                    URL.revokeObjectURL(downloadUrl);
                }
            };
        }
    }, [midiFileUrl, downloadUrl]);

    // expose handleAddChord to parent component
    useImperativeHandle(ref, () => ({
        handleAddChord: (chord) => {
            setChords(prevChords => [...prevChords, chord]);
            setIsMidiUpToDate(false);
            setIsPlaying(false);
            playerRef.current.stop();
        }
    }));

    const handleDeleteChord = (index) => {
        setChords(chords.filter((_, i) => i !== index));
        setIsMidiUpToDate(false);
        setIsPlaying(false);
        playerRef.current.stop();
    };

    const handleMoveUp = (index) => {
        if (index === 0) return;
        const newChords = [...chords];
        [newChords[index - 1], newChords[index]] = [newChords[index], newChords[index - 1]];
        setChords(newChords);
        setIsMidiUpToDate(false);
        setIsPlaying(false);
        playerRef.current.stop();
    };

    const handleMoveDown = (index) => {
        if (index === chords.length - 1) return;
        const newChords = [...chords];
        [newChords[index], newChords[index + 1]] = [newChords[index + 1], newChords[index]];
        setChords(newChords);
        setIsMidiUpToDate(false);
        setIsPlaying(false);
        playerRef.current.stop();
    };

    return (
        <div className="fixed right-0 top-0 w-64 h-full flex flex-col">
            <midi-player
                ref={playerRef}
                src={midiFileUrl}
                id="cpbPlayer"
                style={{ width: '0px', height: '0px'}}
            ></midi-player>
            <div className="flex-grow overflow-y-auto">
                <div className="p-4">
                    <h2 className="text-xl font-bold mb-4">Chord Progression</h2>
                    <Accordion type="multiple" collapsible>
                        {chords.map((chord, index) => (
                            <Chord
                                key={index}
                                json={chord}
                                index={index}
                                onDelete={handleDeleteChord}
                                onMoveUp={handleMoveUp}
                                onMoveDown={handleMoveDown}
                                isInProgression={true}
                            />
                        ))}
                    </Accordion>
                </div>
            </div>
            <div className="bg-white border-t p-2 flex space-x-2">
            <a href={downloadUrl} download="chordProgression.mid"><Button 
                    variant="outline" 
                    className="flex-grow"
                >
                    <Save className="mr-2 h-4 w-4" /> Save
                </Button></a>
                {isLoading ? 
                    (<Button 
                        variant="outline"
                        className="flex-grow"
                    >
                        <LoaderPinwheel className="mr-2 h-4 w-4" /> Loading
                    </Button>
                    ) : (
                        !isPlaying ? 
                            (<Button 
                                variant="outline" 
                                className="flex-grow"
                                onClick={togglePlayOrPause}
                            >
                                <Play className="mr-2 h-4 w-4" /> Play 
                            </Button>
                            ) : (
                                <Button
                                    variant="outline"
                                    className="flex-grow"
                                    onClick={togglePlayOrPause}
                                >
                                <Square className="mr-2 h-4 w-4" /> Stop
                                </Button>
                            )
                        
                    )}
                
                
            </div>
        </div>
    );
});

export default ChordProgressionBuilder;