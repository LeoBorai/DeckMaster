import { MTG } from "./modules/MTG";

export class DeckMaster {
  readonly mtg: MTG;

    constructor(baseURL: URL) {
        this.mtg = new MTG(baseURL);
    }

    sayHello() {
        return "Hello from DeckMaster!";
    }
}
