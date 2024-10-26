export namespace ComponentCrcWasmCrc64NvmeHasher {
  export { Hasher };
}

export class Hasher {
  constructor()
  update(input: Uint8Array): void;
  finalize(): bigint;
  reset(): void;
}
