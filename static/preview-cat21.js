// CAT-21 😺
import { createCatHash, MooncatParser } from '/static/cat21.js';

const el = document.documentElement;
const txid = el.dataset.txid;
const blockHash = el.dataset.blockHash;
const fee = Number(el.dataset.fee);
const weight = Number(el.dataset.weight);

const vsize = weight / 4;
const feeRate = fee / vsize;

const hash = createCatHash(txid, blockHash);
const { svg } = MooncatParser.parseAndGenerateSvg(hash, feeRate);
document.body.innerHTML = svg;
