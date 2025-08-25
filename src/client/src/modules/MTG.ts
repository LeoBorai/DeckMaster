export type FindCardsFilter = {
  page?: number;
  deckId?: string;
}

type Card = {
    id: string; // UUID in TypeScript is typically represented as a string
    title: string;
    number: number; // TypeScript uses number for both integer and float
    description?: string;
    mana?: string[];
    kind: string;
    rarity: string;
    artist?: string;
    power?: string;
    toughness?: string;
    deckId: string;
}

type Deck = {
    id: string;
    name: string;
    code: string;
    release: Date;
}

export class MTG {
    private readonly baseURL: URL;

    constructor(baseURL: URL) {
        this.baseURL = baseURL;
    }

    async getCards(filter: FindCardsFilter): Promise<Card[]> {
      const url = new URL(this.baseURL);
      url.pathname = '/api/v0/mtg/cards';

      url.searchParams.append('page', (filter.page ?? 1).toString());

      if (filter.deckId) {
        url.searchParams.append('deckId', filter.deckId);
      }

      const response = await fetch(url.toString());

      if (!response.ok) {
        throw new Error(`Error fetching cards: ${response.statusText}`);
      }

      const data = await response.json();

      return data;
    }
}
