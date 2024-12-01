import React, { useState } from "react";

const MultiSelect = ({ options, selectedOptions, setSelectedOptions }) => {
  return (
    <div className="flex flex-col">
      {options.map((option) => (
        <button
          key={option}
          className={`${
            selectedOptions.includes(option)
              ? "bg-blue-500 text-white"
              : "bg-gray-200 text-black"
          } p-2 m-1 rounded`}
          onClick={(event) => {
            event.preventDefault();
            setSelectedOptions(option);
            }
        }
        >
          {option}
        </button>
      ))}
    </div>
  );
};

export default MultiSelect