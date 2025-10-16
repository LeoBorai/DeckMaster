import { useEffect, useState } from "react";

import { retrieveImage } from "../../services/DeckMaster";

import type { JSX } from "react";
import type { Card } from "../../services/DeckMaster/types.gen";
import { Mana } from "./Mana";
import { NavLink } from "react-router-dom";

export type Props = {
  card: Card;
};

export function CardMiniature({ card }: Props): JSX.Element {
  const [image, setImage] = useState<string>("/images/mtg_card_back.png");
  const [isFlipped, setIsFlipped] = useState<boolean>(false);

  useEffect(() => {
    asyncLoadImage(card);
  }, [card]);

  useEffect(() => {
    return () => {
      if (image && image.startsWith("blob:")) {
        URL.revokeObjectURL(image);
      }
    };
  }, [image]);

  const asyncLoadImage = async (card: Card) => {
    const response = await retrieveImage({
      baseUrl: import.meta.env.VITE_DECKMASTER_API_URL,
      path: {
        card_id: card.id,
        deck_id: card.deckId,
      },
    });

    if (response.data) {
      setImage(URL.createObjectURL(response.data as unknown as Blob));
    }
  };

  const handleFlip = () => {
    setIsFlipped(!isFlipped);
  };

  return (
    <div
      key={card.id}
      className="border rounded-lg p-2 hover:shadow-lg transition-shadow duration-200 cursor-pointer perspective-1000"
      style={{
        transformStyle: "preserve-3d",
        transform: isFlipped ? "rotateY(180deg)" : "rotateY(0deg)",
        transition: "transform 0.6s",
      }}
    >
      <div
        className="w-full backface-hidden"
        style={{ backfaceVisibility: "hidden" }}
        onClick={handleFlip}
      >
        <img
          src={image}
          alt={card.title}
          className="w-full h-auto mb-2 rounded"
        />
      </div>
      <div
        className="absolute top-0 left-0 w-full backface-hidden"
        style={{
          backfaceVisibility: "hidden",
          transform: "rotateY(180deg)",
        }}
      >
        <div className="rounded p-4 h-full">
          <header>
            <h3 className="text-lg font-bold mb-2">{card.title}</h3>
            <Mana mana={card.mana || []} />
          </header>
          <article>
            <p className="mb-4 text-sm">
              <strong>Number:</strong> {card.number || "N/A"}
            </p>
            <p className="mb-4 text-sm">
              {card.description || "No description available"}
            </p>
          </article>
          <NavLink
            to={`/cards/${card.id}`}
            onClick={() => {
              setIsFlipped(false);
            }}
          >
            Go to details
          </NavLink>
          <button type="button" onClick={handleFlip}>
            Flip back
          </button>
        </div>
      </div>
    </div>
  );
}
