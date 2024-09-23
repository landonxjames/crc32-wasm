import { hash, crc32Hasher } from "./js-bindings/crc32_wasm";

function bitLength(number) {
  return Math.floor(Math.log2(number)) + 1;
}

function byteLength(number) {
  return Math.ceil(bitLength(number) / 8);
}

function toBytes(number) {
  if (!Number.isSafeInteger(number)) {
    throw new Error("Number is out of range");
  }

  const size = number === 0 ? 0 : byteLength(number);
  const bytes = new Uint8ClampedArray(size);
  let x = number;
  for (let i = size - 1; i >= 0; i--) {
    const rightByte = x & 0xff;
    bytes[i] = rightByte;
    x = Math.floor(x / 0x100);
  }

  return bytes.buffer;
}

let helloWorld = Buffer.from("Hello world");
console.log("Hello World as buf: " + helloWorld);
let foo = hash(helloWorld);
console.log("Hash: " + foo);
let b64encodedchecksum = Buffer.from(toBytes(foo)).toString("base64");
console.log("calculated checksum: " + b64encodedchecksum);
