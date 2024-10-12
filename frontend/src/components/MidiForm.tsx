"use client";

import Image from "next/image";
import React, { ChangeEvent, FormEvent, useState } from 'react';
import MidiPlayer from "./MidiPlayer";

interface MidiFormProps {
  wasmModule: any;
}

const MidiForm: React.FC<MidiFormProps> = ({ wasmModule }) => {
  const [textInput, setTextInput] = useState<string>('');
  const [fileInput, setFileInput] = useState<File | null>(null);
  const [selectedOption, setSelectedOption] = useState<string>('');
  const [midiFile, setMidiFile] = useState<string | null>(null);

  const handleTextChange = (event: ChangeEvent<HTMLInputElement>) => {
    setTextInput(event.target.value);
  }

  const handleFileChange = (event: ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      setFileInput(event.target.files[0]);
    }
  }

  const handleOptionChange = (event: ChangeEvent<HTMLInputElement>) => {
    setSelectedOption(event.target.value);
  }

  const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if((!fileInput && !textInput) || !selectedOption) {
      alert("Please provide an input.");
      return;
    }

    try {
        let fileBinary;
        if (fileInput === null) {
            fileBinary = new Uint8Array(0);
        } else {
            const fileArrayBuffer = await fileInput.arrayBuffer();
            fileBinary = new Uint8Array(fileArrayBuffer);
        }
      
      const textBinary = new TextEncoder().encode(textInput);

      const combinedBinary = new Uint8Array(fileBinary.length + textBinary.length);
      combinedBinary.set(fileBinary);
      combinedBinary.set(textBinary, fileBinary.length);

      const midiBinary = wasmModule.generate_midi(combinedBinary, selectedOption);

      const midiBlob = new Blob([midiBinary], { type: 'audio/midi' });
      const midiUrl = URL.createObjectURL(midiBlob);

      setMidiFile(midiUrl);
    } catch (error) {
      console.error("Error processing file", error);
      alert("An error occurred while generating the MIDI file.");
    }
  };

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <div>
          <label>Text Input:</label>
          <input type="text" value={textInput} onChange={handleTextChange} />
        </div>
        <div>
          <label>Upload File:</label>
          <input type="file" onChange={handleFileChange} accept=".jpg,.png" />
        </div>
        <div>
          <label>Select an Option:</label>
          <div>
            <input
              type="radio"
              value="melody"
              checked={selectedOption === 'melody'}
              onChange={handleOptionChange}
              required
            />
            Melody
            <input
              type="radio"
              value="chords"
              checked={selectedOption === 'chords'}
              onChange={handleOptionChange}
              required
            />
            Chords
          </div>
        </div>
        <button type="submit">Submit</button>
      </form>

      {midiFile && (
        <div>
      {/* Using the midi-player custom element for MIDI playback */}
      {midiFile && (
        <MidiPlayer
          midiFileUrl={midiFile}
        ></MidiPlayer>
      )}
        </div>
      )}
    </div>
  );
};

export default MidiForm;