import fs from 'fs';
import { compressToBase64 } from 'lz-string';

const data = fs.readFileSync('./albums.json').toString();

const compressed = compressToBase64(data);

fs.writeFileSync('./albums-compressed-b64.json', compressed);
