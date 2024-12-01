import React, { useState, useEffect } from 'react';
import { 
  Sheet, 
  SheetContent, 
  SheetHeader, 
  SheetTitle, 
  SheetDescription 
} from "@/components/ui/sheet";
import { 
  AlertDialog, 
  AlertDialogAction, 
  AlertDialogCancel, 
  AlertDialogContent, 
  AlertDialogDescription, 
  AlertDialogFooter, 
  AlertDialogHeader, 
  AlertDialogTitle 
} from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button";
import { Trash, Upload } from "lucide-react";
import useLocalStorage from '@/hooks/useLocalStorage';

const SavedChords = ({ 
    isOpen, 
    onClose, 
    currentChords, 
    onLoadProgression,
    filterType,
    midiFileUrl
}) => {
  const [savedProgressions, setSavedProgressions] = useLocalStorage("savedProgressions", {});
  const [saveDialogOpen, setSaveDialogOpen] = useState(false);
  const [saveProgressionName, setSaveProgressionName] = useState('');
  const [saveMethod, setSaveMethod] = useState('browser');

  const saveChordProgression = (name, method = 'browser') => {
    // Determine contents based on whether it's a function or direct object
    const contents =
      typeof currentChords === 'function' ? currentChords(name) : currentChords;
  
    if (method === 'browser' || method === 'everywhere') {
      const updatedProgressions = {
        ...savedProgressions,
        [name]: {
          type: filterType || 'chordProgression',
          contents,
          timestamp: new Date().toISOString(),
        },
      };
      setSavedProgressions(updatedProgressions);
    }
  
    if (method === 'computer' || method === 'everywhere') {
        if (midiFileUrl) {
            const link = document.createElement('a');
            link.href = midiFileUrl; // Use the prop directly
            link.download = `${filterType || 'progression'}-${name}.midi`;
            link.click();
        } else {
            console.error("No MIDI file available to download");
        }
    }
  
    setSaveDialogOpen(false);
  };

  const handleSave = (saveMethod) => {
    switch(saveMethod) {
      case 'computer':
        // Create and trigger download
        const blob = new Blob([JSON.stringify(currentChords)], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.download = `chordProgression-${saveProgressionName}.json`;
        link.click();
        URL.revokeObjectURL(url);
        break;
      case 'browser':
        saveChordProgression(saveProgressionName);
        break;
      case 'everywhere':
        saveChordProgression(saveProgressionName);
        // Create and trigger download (same as 'computer')
        const everywhereBlob = new Blob([JSON.stringify(currentChords)], { type: 'application/json' });
        const everywhereUrl = URL.createObjectURL(everywhereBlob);
        const everywhereLink = document.createElement('a');
        everywhereLink.href = downloadUrl;
        everywhereLink.download = `chordProgression-${saveProgressionName}.mid`;
        everywhereLink.click();
        console.log("URLs: " + {midiFileUrl, downloadUrl})
        URL.revokeObjectURL(everywhereUrl);
        break;
    }
    setSaveDialogOpen(false);
  };

  const handleLoadProgression = (name) => {
    onLoadProgression(savedProgressions[name].contents);
    setSelectedProgressionToLoad(null);
  };

  const handleDeleteProgression = (name) => {
    const updatedProgressions = { ...savedProgressions };
    delete updatedProgressions[name];
    setSavedProgressions(updatedProgressions);
  };

  // Safely filter saved items based on the filterType prop
  const savedItems = Object.entries(savedProgressions)
    .filter(([_, item]) => {
      // If no filterType is provided, show all items
      if (!filterType) return true;
      // Otherwise, filter by the specified type
      return item?.type === filterType;
    })
    .reverse();

  return (
    <>
      <Sheet open={isOpen} onOpenChange={onClose}>
        <SheetContent className="w-[400px]">
          <SheetHeader>
            <SheetTitle>
              {filterType === 'generated' ? 'Saved Settings' : 'Saved Chord Progressions'}
            </SheetTitle>
            <SheetDescription>
              Manage your saved {filterType === 'generated' ? 'settings' : 'chord progressions'}
            </SheetDescription>
          </SheetHeader>
          
          <div className="mt-4 space-y-2">
            <Button 
              variant="outline" 
              className="w-full" 
              onClick={() => setSaveDialogOpen(true)}
            >
              Save Current {filterType === 'generated' ? 'Settings' : 'Progression'}
            </Button>

            {savedItems.length === 0 ? (
              <p className="text-center text-gray-500 mt-4">
                No saved {filterType === 'generated' ? 'settings' : 'chord progressions'}
              </p>
            ) : (
              savedItems.map(([name, item]) => (
                <div 
                  key={name} 
                  className="border p-2 rounded flex justify-between items-center"
                >
                  <span>{name}</span>
                  <div className="space-x-2">
                    <Button 
                      size="sm" 
                      variant="outline"
                      onClick={() => onLoadProgression(item.contents)}
                    >
                      <Upload className="h-4 w-4 mr-2" /> Load
                    </Button>
                    <Button 
                      size="sm" 
                      variant="destructive"
                      onClick={() => handleDeleteProgression(name)}
                    >
                      <Trash className="h-4 w-4" />
                    </Button>
                  </div>
                </div>
              ))
            )}
          </div>
        </SheetContent>
      </Sheet>

      {/* Save Dialog */}
      {saveDialogOpen && (
        <AlertDialog open={saveDialogOpen} onOpenChange={setSaveDialogOpen}>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>
                Save Current {filterType === 'generated' ? 'Settings' : 'Progression'}
              </AlertDialogTitle>
              <AlertDialogDescription>
                Enter a name for your saved {filterType === 'generated' ? 'settings' : 'progression'}
              </AlertDialogDescription>
            </AlertDialogHeader>
            <div className="space-y-2">
              <input
                type="text"
                value={saveProgressionName}
                onChange={(e) => setSaveProgressionName(e.target.value)}
                placeholder={`Enter ${filterType === 'generated' ? 'settings' : 'progression'} name`}
                className="w-full p-2 border rounded"
              />
              {filterType === 'generated' ? 
                (
                    <div></div>
                ) : (
                <div className="flex space-x-2">
                <Button 
                  variant={saveMethod === 'browser' ? 'default' : 'outline'}
                  onClick={() => setSaveMethod('browser')}
                >
                  Browser
                </Button>
                <Button 
                  variant={saveMethod === 'computer' ? 'default' : 'outline'}
                  onClick={() => setSaveMethod('computer')}
                >
                  Computer
                </Button>
                <Button 
                  variant={saveMethod === 'everywhere' ? 'default' : 'outline'}
                  onClick={() => setSaveMethod('everywhere')}
                >
                  Everywhere
                </Button>
              </div>
                )}
            </div>
            <AlertDialogFooter>
              <AlertDialogCancel>Cancel</AlertDialogCancel>
              <AlertDialogAction 
                onClick={() => saveChordProgression(saveProgressionName, saveMethod)}
                disabled={!saveProgressionName}
              >
                Save
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      )}
    </>
  );
};

export default SavedChords;