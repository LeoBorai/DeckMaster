import { ApiClient, ClientConfig, RestApiResponse } from '../http-client';

import type { PaginatedResponse } from '../types';

export interface GetCardsParams {
  page?: number;
  id?: string;
  title?: string;
  unique?: boolean;
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

    getImageUrlByCard(card: Card): string {
      const url = new URL(`${this.baseUrl}/api/v0/mtg/image`);
      url.searchParams.set('card_id', card.id);
      url.searchParams.set('deck_id', card.deckId);
      return url.toString();
    }

    async getCards(params?: GetCardsParams): Promise<RestApiResponse<Card[]>> {
      const queryString = this.buildQueryString(params);
      return this.request<Card[]>(`/api/v0/mtg/cards${queryString}`);
    }

    async getDecks(params?: GetDecksParams): Promise<RestApiResponse<Deck[]>> {
      const queryString = this.buildQueryString(params);
      return this.request<Deck[]>(`/api/v0/mtg/decks${queryString}`);
    }

    async getCardsByPage(page: number): Promise<RestApiResponse<Card[]>> {
        return this.getCards({ page });
    }

    async getDecksByPage(page: number): Promise<RestApiResponse<Deck[]>> {
        return this.getDecks({ page });
    }
}
