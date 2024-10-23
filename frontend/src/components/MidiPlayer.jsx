"use client";

import React, { useRef, useEffect, useState } from 'react';

const MidiPlayerComponent = ({ midiFileUrl }) => {
  const playerRef = useRef(null);
  const visualizerRef = useRef(null);
  const [downloadUrl, setDownloadUrl] = useState(null);

  useEffect(() => {
    if (playerRef.current && visualizerRef.current) {
      visualizerRef.current.player = playerRef.current;
      //playerRef.current.addVisualizer(playerRef.current);
    }

    if (midiFileUrl) {
      fetch(midiFileUrl)
        .then((response) => response.blob())
        .then((blob) => {
          const url = URL.createObjectURL(blob);
          setDownloadUrl(url);
        });
    }
  }, [midiFileUrl]);

  // Cleanup the created URL
  useEffect(() => {
    return () => {
      if (downloadUrl) {
        URL.revokeObjectURL(downloadUrl);
      }
    };
  }, [downloadUrl]);

  useEffect(() => {
    if (playerRef.current && visualizerRef.current) {
      playerRef.current.addVisualizer(visualizerRef.current);
    }
  })

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
            //src={midiFileUrl}
            style={{ width: '600px', border: '1px solid black' }}
          ></midi-visualizer>

          {/* MIDI Download Button */}
          {downloadUrl && (
            <a href={downloadUrl} download="generated-midi-file.mid">
              <button>Download MIDI</button>
            </a>
          )}
        </div>
      )}
    </div>
  );
};

export default MidiPlayerComponent;
