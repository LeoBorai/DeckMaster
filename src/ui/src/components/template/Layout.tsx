import { Outlet, useNavigate } from "react-router-dom";

import { Disclosure } from "@headlessui/react";

import type { JSX } from "react";
import { CardSearch } from "../atoms/CardSearch";

export function Layout(): JSX.Element {
  const navigate = useNavigate();

  return (
    <>
      <div className="min-h-full">
        <div className="bg-emerald-800 pb-32">
          <Disclosure
            as="nav"
            className="border-b border-emerald-400/25 bg-emerald-800 lg:border-none"
          >
            <div className="mx-auto max-w-7xl px-2 sm:px-4 lg:px-8">
              <div className="relative flex h-16 items-center justify-between lg:border-b lg:border-indigo-400/25">
                <div className="flex items-center px-2 lg:px-0">
                  <div className="shrink-0">
                    <img
                      alt="DeckMaster"
                      src="https://placehold.co/400"
                      className="block size-8"
                    />
                  </div>
                </div>
                <CardSearch
                  onCardSelect={(card) => navigate(`/cards/${card.id}`)}
                />
              </div>
            </div>
          </Disclosure>
        </div>

        <main className="-mt-28">
          <div className="mx-auto max-w-7xl px-4 pb-12 sm:px-6 lg:px-8">
            <div className="rounded-lg bg-zinc-800 text-white px-5 py-6 outline-1 -outline-offset-1 outline-white/10 sm:px-6">
              <Outlet />
            </div>
          </div>
        </main>
      </div>
    </>
  );
}
