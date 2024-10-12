import React, { useEffect } from 'react';

// Define the interface for props, expecting midiFileUrl as a string
interface MidiPlayerComponentProps {
  midiFileUrl: string | null;
}

const MidiPlayerComponent: React.FC<MidiPlayerComponentProps> = ({ midiFileUrl }) => {
    useEffect(() => {
        console.log("Midi player defined: ", customElements.get('midi-player'));
        console.log("Midi-visualizer defined: ", customElements.get('midi-visualizer'));
    }, []);
    return (
        <div>
            <h3>Generated MIDI File:</h3>
            {/* Using the midi-player custom element for MIDI playback */}
            {midiFileUrl && (
                <div>
                    <midi-player
                        src={midiFileUrl}
                        style={{ width: '600px', height: '200px' }}
                        visualizer="#mainVisualizer"
                        sound-font="https://storage.googleapis.com/magentadata/js/soundfonts/sgm_plus"
                    ></midi-player>
                    <midi-visualizer 
                        type="piano-roll" 
                        id="mainVisualizer"
                        style={{ width: '600px', height: '200px', border: '1px solid black' }}
                    ></midi-visualizer>
                </div>
            )}
        </div>
  );
};

export default MidiPlayerComponent;
