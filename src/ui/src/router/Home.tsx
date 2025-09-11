import type { JSX } from "react";
import { CardSearch } from "../components/atoms/CardSearch";

export function Home(): JSX.Element {
  return (
    <div className="flex justify-center items-center py-6 px-4 my-auto w-full">
      <article className="flex flex-col justify-center items-center gap-4">
        <h1 className="text-3xl font-semibold">
          DeckMaster
        </h1>
        <p>Magic The Gathering - Cards Database and Deck Builder</p>
        <hr />
        <CardSearch />
      </article>
    </div>
  )
}
