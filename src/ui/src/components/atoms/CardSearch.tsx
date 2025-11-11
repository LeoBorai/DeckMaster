import { useEffect, useRef, useState } from "react";

import { retrieveCards } from "../../services/DeckMaster";

import type { ChangeEventHandler, JSX } from "react";
import type { Card } from "../../services/DeckMaster/types.gen";
import { MagnifyingGlassIcon } from "@heroicons/react/16/solid";

type Props = {
  onCardSelect(card: Card): void;
};

export function CardSearch(props: Props): JSX.Element {
  const [inputValue, setInputValue] = useState("");
  const [suggestions, setSuggestions] = useState<Card[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [showSuggestions, setShowSuggestions] = useState(false);
  const debounceRef = useRef<number | null>(null);

  // eslint-disable-next-line @typescript-eslint/no-unsafe-function-type
  const debounce = (func: Function, delay: number) => {
    return (...args: unknown[]) => {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }
      debounceRef.current = setTimeout(() => func(...args), delay);
    };
  };

  // Search function that calls the API
  const searchCards = async (searchValue: string) => {
    if (!searchValue.trim()) {
      setSuggestions([]);
      setIsLoading(false);
      return;
    }

    try {
      setIsLoading(true);
      const results = await retrieveCards({
        baseUrl: import.meta.env.VITE_DECKMASTER_API_URL,
        query: {
          title: searchValue,
          unique: true,
        },
      });

      setSuggestions(results.data?.data || []);
    } catch (error) {
      console.error("Error fetching cards:", error);
      setSuggestions([]);
    } finally {
      setIsLoading(false);
    }
  };

  const debouncedSearch = debounce(searchCards, 300);

  const handleInputChange: ChangeEventHandler = (e) => {
    const value = (e.target as unknown as { value: string }).value;
    setInputValue(value);
    setShowSuggestions(true);

    if (value.trim()) {
      setIsLoading(true);
      debouncedSearch(value);
    } else {
      setSuggestions([]);
      setIsLoading(false);
    }
  };

  const handleSuggestionClick = (card: Card) => {
    setInputValue(card.title);
    setShowSuggestions(false);
    if (props.onCardSelect) {
      props.onCardSelect(card);
    }
  };

  const handleFocus = () => {
    if (suggestions.length > 0) {
      setShowSuggestions(true);
    }
  };

  const handleBlur = () => {
    setTimeout(() => {
      setShowSuggestions(false);
    }, 200);
  };

  useEffect(() => {
    return () => {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }
    };
  }, []);

  return (
    <div className="relative w-full max-w-md">
      <div className="relative">
        <div className="flex flex-1 justify-center px-2 lg:ml-6 lg:justify-end">
          <div className="grid w-full max-w-lg grid-cols-1 lg:max-w-xs">
            <input
              name="search"
              placeholder="Search"
              aria-label="Search"
              className="col-start-1 row-start-1 block w-full rounded-md bg-emerald-700/50 py-1.5 pr-3 pl-10 text-base text-white outline-1 -outline-offset-1 outline-indigo-400/25 placeholder:text-white/50 focus:outline-2 focus:-outline-offset-2 focus:outline-white sm:text-sm/6"
              value={inputValue}
              onChange={handleInputChange}
              onFocus={handleFocus}
              onBlur={handleBlur}
            />
            <MagnifyingGlassIcon
              aria-hidden="true"
              className="pointer-events-none col-start-1 row-start-1 ml-3 size-5 self-center text-white/50"
            />
          </div>
        </div>

        {isLoading && (
          <div className="absolute right-3 top-2.5">
            <div className="animate-spin h-4 w-4 border-2 border-blue-500 border-t-transparent rounded-full"></div>
          </div>
        )}
      </div>
      {showSuggestions && (
        <div className="absolute top-full left-0 right-0 mt-1 bg-white border border-gray-300 rounded-lg shadow-lg max-h-60 overflow-y-auto z-50">
          {isLoading && suggestions.length === 0 ? (
            <div className="px-4 py-2 text-gray-500 text-center">
              Searching...
            </div>
          ) : suggestions.length > 0 ? (
            suggestions.map((card, index) => (
              <div
                key={card.id || index}
                onClick={() => handleSuggestionClick(card)}
                className="px-4 py-2 hover:bg-gray-100 cursor-pointer border-b border-gray-100 last:border-b-0"
              >
                <div className="font-medium text-gray-900">{card.title}</div>
              </div>
            ))
          ) : inputValue.trim() && !isLoading ? (
            <div className="px-4 py-2 text-gray-500 text-center">
              No cards found
            </div>
          ) : null}
        </div>
      )}
    </div>
  );
}
