// CAT-21 😺
import { createCatHash, MooncatParser } from '/static/cat21.js';

const container = document.getElementById('cat21-traits');
const txid = container.dataset.txid;
const blockHash = container.dataset.blockHash;
const fee = Number(container.dataset.fee);
const weight = Number(container.dataset.weight);

const vsize = weight / 4;
const feeRate = fee / vsize;
const hash = createCatHash(txid, blockHash);
const { traits } = MooncatParser.parseAndGenerateSvg(hash, feeRate);

function createSwatch(color) {
  const span = document.createElement('span');
  span.className = 'cat21-swatch';
  span.title = color;
  span.style.background = color;
  return span;
}

function createColorDD(colors) {
  const dd = document.createElement('dd');
  colors.forEach((c, i) => {
    if (i > 0) dd.appendChild(document.createElement('br'));
    dd.appendChild(createSwatch(c));
    dd.appendChild(document.createTextNode(' ' + c));
  });
  return dd;
}

function addRow(dl, label, value) {
  const dt = document.createElement('dt');
  dt.textContent = label;
  dl.appendChild(dt);

  if (value instanceof HTMLElement) {
    dl.appendChild(value);
  } else {
    const dd = document.createElement('dd');
    dd.textContent = value;
    dl.appendChild(dd);
  }
}

if (traits) {
  const h2 = document.createElement('h2');
  h2.textContent = 'Traits';
  container.appendChild(h2);

  const dl = document.createElement('dl');

  addRow(dl, 'origin', traits.genesis ? 'Genesis cat' : 'Normal cat');
  addRow(dl, 'cat colors', createColorDD(traits.catColors));
  addRow(dl, 'gender', traits.gender);
  addRow(dl, 'pose', traits.designPose);
  addRow(dl, 'expression', traits.designExpression);
  addRow(dl, 'pattern', traits.designPattern);
  addRow(dl, 'facing', traits.designFacing);
  addRow(dl, 'laser eyes', traits.laserEyes);
  addRow(dl, 'background', traits.background);
  addRow(dl, 'background colors', createColorDD(traits.backgroundColors));
  addRow(dl, 'crown', traits.crown);
  addRow(dl, 'glasses', traits.glasses);

  if (traits.glasses !== 'None') {
    addRow(dl, 'glasses colors', createColorDD(traits.glassesColors));
  }

  container.appendChild(dl);
}
