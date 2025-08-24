import { MTG } from "./modules/MTG";

export class DeckMaster {
  readonly mtg: MTG;

    constructor() {
        this.mtg = new MTG();
    }

    sayHello() {
        return "Hello from DeckMaster!";
    }
}
