import { useEffect, useRef, useState } from "react";

import { retrieveCards } from "../../services/DeckMaster";

import type { ChangeEventHandler, JSX } from "react";
import type { Card } from "../../services/DeckMaster/types.gen";

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
        <input
          type="text"
          value={inputValue}
          onChange={handleInputChange}
          onFocus={handleFocus}
          onBlur={handleBlur}
          placeholder="Search for a card..."
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-colors"
        />
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
