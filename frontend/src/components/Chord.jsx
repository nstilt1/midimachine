"use client";

import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";
import { Button } from "./ui/button";
import React, { useEffect, useRef, useState } from 'react';

const Chord = ({
    midi,
    chordName,
    notes,
    index
}) => {
    const [midiFileUrl, setMidiFileUrl] = useState(null);

    useEffect(() => {
        const byteChars = atob(midi);
        const byteNumbers = new Array(byteChars.length);

        for (let i = 0; i < byteChars.length; i++) {
            byteNumbers[i] = byteChars.charCodeAt(i);
        }

        const byteArray = new Uint8Array(byteNumbers);
        const midiBlob = new Blob([byteArray], { type: 'audio/midi' });
        const url = URL.createObjectURL(midiBlob);

        setMidiFileUrl(url);

        // Cleanup the URL when the component unmounts
        return () => {
            URL.revokeObjectURL(url);
        };
    }, [midi]);

    return (
        <AccordionItem value={index + '' + chordName}>
            <AccordionTrigger>{chordName}</AccordionTrigger>
            <AccordionContent>
                <div>
                    {notes}
                </div>
                <div>
                    {midiFileUrl && (
                        <midi-player
                            src={midiFileUrl}
                            sound-font="https://storage.googleapis.com/magentadata/js/soundfonts/sgm_plus"
                            style={{ width: '75px', height: '20px' }}
                        ></midi-player>
                    )}
                </div>
            </AccordionContent>
        </AccordionItem>
    );
};

export default Chord;
