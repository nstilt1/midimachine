"use client";

import { useState, useEffect } from "react";

const useLocalStorage = (key, initialValue) => {
  const [storedValue, setStoredValue] = useState(initialValue);
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    // Only run on client-side
    if (typeof window !== 'undefined') {
      try {
        const item = localStorage.getItem(key);
        if (item) {
          setStoredValue(JSON.parse(item));
        }
      } catch (error) {
        console.log(error);
      }
      setIsLoaded(true);
    }
  }, [key]);

  const setValue = (value) => {
    try {
      // Allow function updates like setState
      const valueToStore = typeof value === 'function' 
        ? value(storedValue) 
        : value;
      
      setStoredValue(valueToStore);
      
      // Only set in localStorage on client
      if (typeof window !== 'undefined') {
        localStorage.setItem(key, JSON.stringify(valueToStore));
      }
    } catch (error) {
      console.log(error);
    }
  };

  return [isLoaded ? storedValue : initialValue, setValue];
};

export default useLocalStorage;