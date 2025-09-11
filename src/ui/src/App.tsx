import { useEffect } from 'react';
import { RouterProvider } from 'react-router-dom';

import { DeckMaster } from '@deckmaster/client';
import { router } from './router';

import type { JSX } from 'react';

export function App(): JSX.Element {
  useEffect(() => {
    const dm = new DeckMaster({ baseUrl: 'http://localhost:7878' });

    (async () => {
      await dm.mtg.getCards();
    })();
  }, []);

  return (
    <RouterProvider router={router} />
  )
}
