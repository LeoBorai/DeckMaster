import { MTG } from "./modules/MTG";
import { ClientConfig } from "./http-client";

export type { ClientConfig } from './http-client';

export * from './types';

export class DeckMaster {
  readonly mtg: MTG;

  constructor(config?: ClientConfig) {
    this.mtg = new MTG(config);
  }
}
