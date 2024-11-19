"use client";
import {
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableFooter,
    TableHead,
    TableHeader,
    TableRow,
  } from "@/components/ui/table";

  import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
  } from "@/components/ui/accordion";
import Chord from "./Chord";


const ChordTable = ({
    chordData,
    chosenKey
}) => {
    const headers = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

    return (
        <Table>
            <TableCaption>A list of potential chords for the key of {chosenKey}</TableCaption>
            <TableHeader>
                <TableRow>
                    {headers.map((header) => (
                        <TableHead key={header}>{header}</TableHead>
                    ))}
                </TableRow>
            </TableHeader>
            <TableBody>
                {/*columns*/}
                <TableRow className="flex items-start align-top">
                    {chordData && chordData.map((chords, columnIndex) => (
                        <TableCell key={columnIndex}>
                            <div className="flex items-start">
                            <Accordion type="multiple" collapsible>
                                {chords.map((chord, chordIndex) => (
                                    <div key={chordIndex} className="flex items-start">
                                    <Chord
                                        key={chordIndex}
                                        midi={chord['midi']}
                                        chordName={chord['name']}
                                        notes={chord['notes']}
                                        index={chordIndex}
                                    />
                                    </div>
                                ))}
                            </Accordion>
                            </div>
                        </TableCell>
                    ))}
                </TableRow>
            </TableBody>
        </Table>
    );
}

export default ChordTable;