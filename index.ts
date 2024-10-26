import {
  crc32Hash,
  crc32Hasher,
  crc64NvmeHash,
  crc64NvmeHasher,
} from "./js-bindings/crc32_wasm";

//Some util functions to convert numbers/bigints to byte arrays
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

function bnToBuf(bn: bigint) {
  var hex = BigInt(bn).toString(16);
  if (hex.length % 2) {
    hex = "0" + hex;
  }

  var len = hex.length / 2;
  var u8 = new Uint8Array(len);

  var i = 0;
  var j = 0;
  while (i < len) {
    u8[i] = parseInt(hex.slice(j, j + 2), 16);
    i += 1;
    j += 2;
  }

  return u8;
}

//Using the simple exported crc32Hash function
let helloWorld = Buffer.from("Hello world");
let hashOut = crc32Hash(helloWorld);
let b64encodedchecksum = Buffer.from(toBytes(hashOut)).toString("base64");
console.log("Crc32Hash function checksum: " + b64encodedchecksum);

//Using the crc32Hasher class for more complex/longer checksums
let hasher = new crc32Hasher.Hasher();
hasher.update(Buffer.from("Hello"));
hasher.update(Buffer.from(" world"));
let hasherOut = hasher.finalize();
let b64encodedHasherchecksum = Buffer.from(toBytes(hasherOut)).toString(
  "base64"
);
console.log("Crc32Hasher class checksum: " + b64encodedHasherchecksum);

//Using the simple exported crc64NvmeHash function
let crc64HashOut = crc64NvmeHash(helloWorld);
let b64EncodedCrc64Checksum = Buffer.from(bnToBuf(crc64HashOut)).toString(
  "base64"
);
console.log("Crc32Hash function checksum: " + b64EncodedCrc64Checksum);

//Using the crc64NvmeHasher class for more complex/longer checksums
let crc64 = new crc64NvmeHasher.Hasher();
crc64.update(Buffer.from("Hello"));
crc64.update(Buffer.from(" world"));
let crc64HasherOut = crc64.finalize();
let b64encodedcrc64Hasherchecksum = Buffer.from(
  bnToBuf(crc64HasherOut)
).toString("base64");
console.log("crc64NvmeHasher class checksum: " + b64encodedcrc64Hasherchecksum);
