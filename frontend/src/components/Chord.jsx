"use client";

import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
  } from "@/components/ui/accordion";
import { Button } from "./ui/button";
import React, { useRef, useState } from 'react';

const Chord = ({
    midi,
    chordName,
    notes,
    index
}) => {

    const midiFileURL = () => {
        const byteChars = atob(midi);
        const byteNumbers = new Array(byteChars.length);

        for (let i = 0; i < byteChars.length; i++) {
            byteNumbers[i] = byteChars.charCodeAt(i);
        }

        const byteArray = new Uint8Array(byteNumbers);

        const midiBlob = new Blob([byteArray], { type: 'audio/midi' });
        return URL.createObjectURL(midiBlob);
    }

    const playMidi = () => {

    }

    return (
        <AccordionItem value={index + '' + chordName}>
            <AccordionTrigger>{chordName}</AccordionTrigger>
            <AccordionContent>
                <div>
                {notes}
                </div>
                <div>
                    <midi-player
                        src={midiFileURL()}
                        sound-font="https://storage.googleapis.com/magentadata/js/soundfonts/sgm_plus"
                        style={{ width: '75px', height: '20px' }}
                    ></midi-player>    
                </div>
            </AccordionContent>
        </AccordionItem>
    )
}

export default Chord;