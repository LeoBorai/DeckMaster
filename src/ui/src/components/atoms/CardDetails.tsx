import { useEffect, useState } from "react";

import { Mana } from "./Mana";
import { retrieveImage } from "../../services/DeckMaster";

import type { JSX } from 'react';
import type { Card } from "../../services/DeckMaster/types.gen";

export type Props = {
  card: Card;
}

export function CardDetails({ card }: Props): JSX.Element {
  const [image, setImage] = useState<string>('/images/mtg_card_back.png');

  useEffect(() => {
    asyncLoadImage(card);

    return () => {
      URL.revokeObjectURL(image);
    }
  }, [card, image]);

  const asyncLoadImage = async (card: Card) => {
    const response = await retrieveImage({
      baseUrl: import.meta.env.VITE_DECKMASTER_API_URL,
      path: {
        card_id: card.id,
        deck_id: card.deckId,
      }
    });

    if (response.data) {
      setImage(URL.createObjectURL(response.data as unknown as Blob));
    }
  }

  return (
    <div className="flex flex-col gap-12 justify-center items-center md:grid md:grid-cols-[repeat(2,50%)] w-full md:w-9/12 mx-auto">
      <article className="flex justify-center items-center">
          <img src={image} alt={card?.title} width={300} />
      </article>
      <article className="h-full">
          <div className="flex justify-between items-center pb-4">
            <h2 className="text-lg font-semibold">{card?.title}</h2>
            <Mana mana={card?.mana || []}  />
          </div>
          <p className="pb-4">{card?.description || 'No description available'}</p>
          <dl className="pb-4 space-y-4">
            <div>
              <dt>Kind</dt>
              <dd>{card?.kind}</dd>
            </div>
            <div>
              <dt>Number</dt>
              <dd>{card?.number}</dd>
            </div>
            <div>
              <dt>Power</dt>
              <dd>{card?.power}</dd>
            </div>
            <div>
              <dt>Toughness</dt>
              <dd>{card?.toughness}</dd>
            </div>
            <div>
              <dt>Artist</dt>
              <dd>{card?.artist}</dd>
            </div>
          </dl>
      </article>
    </div>
  )
}
