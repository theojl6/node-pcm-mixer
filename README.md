# node-pcm-mixer
Mix two PCM buffers into one

## Features
* 16 bit LPCM mixing

## Installation
```bash
npm install node-pcm-mixer
```

## Usage
```js
const { mixLpcm } = require("@theojl6/node-pcm-mixer");

const buf1 = fs.readFileSync("0.pcm");
const buf2 = fs.readFileSync("1.pcm");

const mixedLpcm = mixLpcm(buf1, buf2);
```


This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

