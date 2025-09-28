import { NavLink, Outlet } from "react-router-dom";

import type { JSX } from "react";

export function Layout(): JSX.Element {
  return (
    <div className="bg-zinc-900 text-zinc-100 min-h-screen">
      <header className="bg-zinc-800 text-sm flex items-center justify-between py-2 px-4 h-9 border-b border-zinc-600">
        <h1>DeckMaster</h1>
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
