import { useEffect, useState } from "react";

import { retrieveCards } from "../../services/DeckMaster";
import { CardMiniature } from "../atoms/CardMiniature";

import type { JSX } from "react";
import type { Card } from "../../services/DeckMaster/types.gen";

export type Props = {
  card: Card;
};

export function RelatedCards({ card }: Props): JSX.Element {
  const [relatedCards, setRelatedCards] = useState<Card[]>([]);

  useEffect(() => {
    loadRelatedCards(card);
  }, [card]);

  const loadRelatedCards = async (card: Card) => {
    const response = await retrieveCards({
      baseUrl: import.meta.env.VITE_DECKMASTER_API_URL,
      query: {
        deck_id: card.deckId,
        skip: card.id,
      },
    });

    if (response.data) {
      setRelatedCards(response.data.data || []);
    }
  };

  return (
    <div>
      {relatedCards.length > 0 && (
        <div className="mt-6">
          <h2 className="text-lg font-semibold mb-4">Related Cards</h2>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
            {relatedCards.map((relatedCard) => (
              <CardMiniature key={relatedCard.id} card={relatedCard} />
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
