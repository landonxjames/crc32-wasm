export namespace ComponentCrcWasmCrc32Hasher {
  export { Hasher };
}

export class Hasher {
  constructor()
  update(input: Uint8Array): void;
  finalize(): number;
  reset(): void;
}
