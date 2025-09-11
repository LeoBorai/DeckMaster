import { ApiClient, ClientConfig } from '../http-client';

import type { PaginatedResponse } from '../types';

export interface GetCardsParams {
  page?: number;
}

export interface GetDecksParams {
  page?: number;
}

export interface Card {
  id: string;
  title: string;
  number: number;
  kind: string;
  rarity: string;
  deckId: string;
  artist?: string | null;
  description?: string | null;
  mana?: string[] | null;
  power?: string | null;
  toughness?: string | null;
}

export interface Deck {
  id: string;
  name: string;
  code: string;
  release: string;
}

export type PaginatedCardsResponse = PaginatedResponse<Card>;

export type PaginatedDecksResponse = PaginatedResponse<Deck>;

export class MTG extends ApiClient {
    constructor(config?: ClientConfig) {
      super(config);
    }

    async getCards(params?: GetCardsParams): Promise<Card[]> {
      const queryString = this.buildQueryString(params);
      return this.request<Card[]>(`/api/v0/mtg/cards${queryString}`);
    }

    async getDecks(params?: GetDecksParams): Promise<Deck[]> {
      const queryString = this.buildQueryString(params);
      return this.request<Deck[]>(`/api/v0/mtg/decks${queryString}`);
    }

    async getCardsByPage(page: number): Promise<Card[]> {
        return this.getCards({ page });
    }

    async getDecksByPage(page: number): Promise<Deck[]> {
        return this.getDecks({ page });
    }
}
