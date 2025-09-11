import { NavLink, Outlet } from "react-router-dom";

import type { JSX } from "react";

export function Layout(): JSX.Element {
  return (
    <div className="bg-zinc-950 text-zinc-100 h-screen">
      <header className="bg-zinc-900 flex items-center justify-between py-4 px-6">
        <div className="flex items-center gap-3">
          <figure>
            <img src="https://via.placeholder.com/50" />
          </figure>
          <h1>DeckMaster</h1>
        </div>
        <nav>
          <NavLink to="/">Search</NavLink>
        </nav>
      </header>
      <main>
        <Outlet />
      </main>
    </div>
  );
}
