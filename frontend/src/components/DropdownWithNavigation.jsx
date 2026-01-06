import { useState, useCallback } from "react";
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "./ui/select";
import { Button } from "./ui/button";
import { Plus, Trash, ChevronLeft, ChevronRight } from "lucide-react";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from "./ui/dialog";
import { Input } from "./ui/input";

export default function DropdownWithNavigation({
  value,
  setValue,
  options,
  setOptions,
  id,
  labelText,
}) {
  const [showDialog, setShowDialog] = useState(false)
  const [newPatternInput, setNewPatternInput] = useState("")

  const currentIndex = options.indexOf(value)

  const addPattern = useCallback(() => {
    setShowDialog(true)
  }, [])

  const confirmAddPattern = () => {
    if (!newPatternInput.trim()) return
    if (options.contains(newInputPattern.trim())) return;

    const updated = [...options, newPatternInput.trim()]
    setOptions(updated)
    setValue(newPatternInput.trim())

    setNewPatternInput("")
    setShowDialog(false)
  }

  const removePattern = useCallback(() => {
    if (options.length <= 1) return

    const idx = currentIndex
    const updated = options.filter((p) => p !== value)

    setOptions(updated)

    const next = updated[idx] || updated[updated.length - 1]
    setValue(next)
  }, [options, value, currentIndex, setOptions, setValue])

  const moveLeft = useCallback(() => {
    const nextIndex = (currentIndex - 1 + options.length) % options.length
    setValue(options[nextIndex])
  }, [currentIndex, options, setValue])

  const moveRight = useCallback(() => {
    const nextIndex = (currentIndex + 1) % options.length
    setValue(options[nextIndex])
  }, [currentIndex, options, setValue])

  return (
    <>
      <div>
        <label htmlFor={id} className="mr-2 whitespace-nowrap">
          {labelText}
        </label>

        <div className="w-full mb-2">
            <Select value={value} onValueChange={setValue} className="w-full">
            <SelectTrigger>
                <SelectValue placeholder="Select pattern" />
            </SelectTrigger>
            <SelectContent>
                {options.map((opt) => (
                <SelectItem key={opt} value={opt}>
                    {opt}
                </SelectItem>
                ))}
            </SelectContent>
            </Select>
        </div>

        <div className="flex gap-2 w-full">
            <Button variant="outline" className="flex-1" size="icon" onClick={addPattern}>
                <Plus className="h-4 w-4" />
            </Button>
        

            <Button
                variant="outline"
                size="icon"
                className="flex-1"
                onClick={removePattern}
                disabled={options.length <= 1}
            >
                <Trash className="h-4 w-4" />
            </Button>

            <Button variant="outline" className="flex-1" size="icon" onClick={moveLeft}>
                <ChevronLeft className="h-4 w-4" />
            </Button>

            <Button variant="outline" className="flex-1" size="icon" onClick={moveRight}>
                <ChevronRight className="h-4 w-4" />
            </Button>
        </div>
      </div>

      <Dialog open={showDialog} onOpenChange={setShowDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Add New Pattern</DialogTitle>
          </DialogHeader>

          <Input
            placeholder="e.g. 1-2-1-3"
            value={newPatternInput}
            onChange={(e) => setNewPatternInput(e.target.value)}
          />

          <DialogFooter>
            <Button onClick={confirmAddPattern}>Add</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  )
}
