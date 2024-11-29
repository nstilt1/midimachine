"use client";

import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";
import { Button } from "./ui/button";

import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger,
} from "@/components/ui/alert-dialog"

import React, { useEffect, useRef, useState } from 'react';

import { Trash, ArrowUp, ArrowDown, Plus } from "lucide-react";

const Chord = ({
    json,
    index,
    onDelete,
    onMoveUp,
    onMoveDown,
    onAdd,
    isInProgression = false
}) => {
    const [midiFileUrl, setMidiFileUrl] = useState(null);

    useEffect(() => {
        const byteChars = atob(json['midi']);
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
    }, [json]);

    return (
        <AccordionItem value={index + '' + json['name']}>
            <AccordionTrigger className="group">{json['name']}</AccordionTrigger>
            <AccordionContent>
                <div>
                    {json['notes']}
                </div>
                {isInProgression ? (
                    <>
                    <AlertDialog>
                        <AlertDialogTrigger asChild>
                            <Button
                                variant="ghost"
                                size="sm"
                            >
                                <Trash className="h-4 w-4" />
                            </Button>
                        </AlertDialogTrigger>
                        <AlertDialogContent>
                            <AlertDialogHeader>
                            <AlertDialogTitle>Are you sure you want to delete this chord?</AlertDialogTitle>
                            <AlertDialogDescription>
                                This action cannot be undone. This will permanently delete this chord.
                            </AlertDialogDescription>
                            </AlertDialogHeader>
                            <AlertDialogFooter>
                            <AlertDialogCancel>Cancel</AlertDialogCancel>
                            <AlertDialogAction onClick={() => {onDelete?.(index);}}>Continue</AlertDialogAction>
                            </AlertDialogFooter>
                        </AlertDialogContent>
                    </AlertDialog>
                        <Button
                            variant="ghost"
                            size="sm"
                            onClick={(e) => {
                                e.stopPropagation();
                                onMoveUp?.(index);
                            }}
                            disabled={index === 0}
                        >
                            <ArrowUp className="h-4 w-4" />
                        </Button>
                        <Button 
                            variant="ghost"
                            size="sm"
                            onClick={(e) => {
                                e.stopPropagation();
                                onMoveDown?.(index);
                            }}
                        >
                            <ArrowDown className="h-4 w-4" />
                        </Button>
                    </>
                ) : (
                    <Button
                        variant="ghost"
                        size="sm"
                        onClick={(e) => {
                            e.stopPropagation();
                            onAdd?.(json);
                        }}
                    >
                        <Plus className="h-4 w-4" />
                    </Button>
                )}
                {json['probability_2d'] && <div>
                    <div>
                        P(2D): {json['probability_2d']}
                    </div>
                    <div>
                        P(1D): {json['probability_1d']}
                    </div>
                </div>}
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
