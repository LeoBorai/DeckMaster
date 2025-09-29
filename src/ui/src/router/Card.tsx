import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

import { CardDetails } from "../components/atoms/CardDetails";
import { retrieveCards } from "../services/DeckMaster";

import type { JSX } from 'react';
import type { Card } from "../services/DeckMaster/types.gen";


export function Card(): JSX.Element {
  const [card, setCard] = useState<Card | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);
  const params = useParams();

  useEffect(() => {
    (async () => {
      try {
        const id = params.id;

        if (id) {
          const cards = await retrieveCards({
            baseUrl: import.meta.env.VITE_DECKMASTER_API_URL,
            query: {
              id,
            }
          });

          setCard(cards?.data?.data[0] as Card || null);
        }
      } catch (err) {
        setError(err as Error);
      } finally {
        setLoading(false);
      }
    })();
  }, [params.id]);

  return (
    <div className="flex justify-center items-center py-6 px-4 my-auto w-full">
      {loading && <p>Loading...</p>}
      {error && <p className="text-red-500">Error: {error.message}</p>}
      {card && <CardDetails card={card} />}
    </div>
  )
}
