"use client";

import React, { useRef, useEffect, useState } from 'react';
import { Button } from '@/components/ui/button';

const MidiPlayerComponent = ({ midiFileUrl, textInput }) => {
  const playerRef = useRef(null);
  const visualizerRef = useRef(null);
  const [downloadUrl, setDownloadUrl] = useState(null);

  // Sanitizes filenames
  const sanitizeFilename = (str) => {
    const maxNameLength = 100;
    let sanitized = str.replace(/[^a-z0-9_\-]/gi, '_');
    if (sanitized.length > maxNameLength) {
      sanitized = sanitized.substring(0, maxNameLength) + '-';
    }
    return "midimachine-" + sanitized + ".mid";
  };

  useEffect(() => {
    if (playerRef.current && visualizerRef.current) {
      playerRef.current.addVisualizer(visualizerRef.current);
    }
  }, [playerRef, visualizerRef]);

  useEffect(() => {
    if (midiFileUrl) {
      fetch(midiFileUrl)
        .then((response) => response.blob())
        .then((blob) => {
          const url = URL.createObjectURL(blob);
          setDownloadUrl(url);
        });

      // Clean up the previous URL if it exists
      return () => {
        if (downloadUrl) {
          URL.revokeObjectURL(downloadUrl);
        }
      };
    }
  }, [midiFileUrl, downloadUrl]);

  return (
    <div>
      <h3>Generated MIDI File:</h3>
      {midiFileUrl && (
        <div>
          {/* MIDI Player */}
          <midi-player
            ref={playerRef}
            src={midiFileUrl}
            id="player"
            visualizer="#mainVisualizer"
            sound-font="https://storage.googleapis.com/magentadata/js/soundfonts/sgm_plus"
            style={{ width: '600px', height: '100px' }}
          ></midi-player>

          {/* MIDI Visualizer */}
          <midi-visualizer
            ref={visualizerRef}
            src={midiFileUrl}
            type="piano-roll"
            id="mainVisualizer"
            style={{ width: '600px', border: '1px solid black' }}
          ></midi-visualizer>

          {/* MIDI Download Button */}
          {downloadUrl && (
            <a href={downloadUrl} download={sanitizeFilename(textInput)}>
              <Button>Download Midi</Button>
            </a>
          )}
        </div>
      )}
    </div>
  );
};

export default MidiPlayerComponent;
