import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

import { DeckMaster } from "@deckmaster/client";

import type { JSX } from 'react';
import type { Card } from '@deckmaster/client/src/modules/MTG';

export function Card(): JSX.Element {
  const [card, setCard] = useState<Card | null>(null);
  const [isLoading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);
  const params = useParams();

  useEffect(() => {
    loadCard();
  }, []);

  const loadCard = async () => {
    try {
      const id = params.id;
      const dm = new DeckMaster();
      const response = await dm.mtg.getCards({
        id
      });
      const first = response.data[0];

      setCard(first || null);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="flex justify-center items-center py-6 px-4 my-auto w-full">
      {
        isLoading ? (
          <h2>Loading</h2>
        ) : (
          <article>
              <h1>{card?.title}</h1>
          </article>
        )
      }
    </div>
  )
}
