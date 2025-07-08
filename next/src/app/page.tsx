"use client";

import { useState, useMemo } from "react";
import { gen_random_number } from "randomizer";

export default function Home() {
  const [inputValue, setInputValue] = useState("");

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    if (/^\d*$/.test(value)) {
      setInputValue(value);
    }
  };

  const randomNumber = useMemo(() => {
    if (inputValue === "") return "????";
    const seed = parseInt(inputValue, 10);
    return gen_random_number(seed);
  }, [inputValue]);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen p-8 font-[family-name:var(--font-geist-sans)]">
      <main className="flex flex-col gap-6 items-center">
        <input
          type="text"
          value={inputValue}
          onChange={handleInputChange}
          className="px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-800 dark:border-gray-600 dark:text-white"
          placeholder="seed (number)"
        />
        <p className="text-lg font-medium">
          Generated Random Number: {randomNumber}
        </p>
      </main>
    </div>
  );
}
