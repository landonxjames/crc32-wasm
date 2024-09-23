import { hash, crc32Hasher } from "./js-bindings/crc32_wasm";

//Some util functions to convert numbers to byte arrays
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

//Using the simple exported hash function
let helloWorld = Buffer.from("Hello world");
let hashOut = hash(helloWorld);
let b64encodedchecksum = Buffer.from(toBytes(hashOut)).toString("base64");
console.log("Hash function checksum: " + b64encodedchecksum);

//Using the hasher class for more complex/longer checksums
let hasher = new crc32Hasher.Hasher();
hasher.update(Buffer.from("Hello"));
hasher.update(Buffer.from(" world"));
let hasherOut = hasher.finalize();
let b64encodedHasherchecksum = Buffer.from(toBytes(hasherOut)).toString(
  "base64"
);
console.log("Hasher class checksum: " + b64encodedHasherchecksum);
