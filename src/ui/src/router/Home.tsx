import { useNavigate } from "react-router-dom";
import { useEffect } from "react";

import { CardSearch } from "../components/atoms/CardSearch";

import type { JSX } from "react";

export function Home(): JSX.Element {
  const navigate = useNavigate();

  useEffect(() => {
    document.title = "DeckMaster";
  }, []);

  return (
    <div className="flex justify-center items-center py-6 px-4 my-auto w-full">
      <article className="flex flex-col justify-center items-center gap-4">
        <h1 className="text-3xl font-semibold">DeckMaster</h1>
        <p>Magic The Gathering - Cards Database and Deck Builder</p>
        <hr />
        <CardSearch onCardSelect={(card) => navigate(`/cards/${card.id}`)} />
      </article>
    </div>
  );
}
