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

if (traits) {
  const colorSwatches = (colors) =>
    colors.map(c =>
      `<span class="cat21-swatch" style="background:${c}" title="${c}"></span> ${c}`
    ).join('<br>');

  let html = '<h2>Traits</h2><dl>';

  html += `<dt>origin</dt><dd>${traits.genesis ? 'Genesis cat' : 'Normal cat'}</dd>`;
  html += `<dt>cat colors</dt><dd>${colorSwatches(traits.catColors)}</dd>`;
  html += `<dt>gender</dt><dd>${traits.gender}</dd>`;
  html += `<dt>pose</dt><dd>${traits.designPose}</dd>`;
  html += `<dt>expression</dt><dd>${traits.designExpression}</dd>`;
  html += `<dt>pattern</dt><dd>${traits.designPattern}</dd>`;
  html += `<dt>facing</dt><dd>${traits.designFacing}</dd>`;
  html += `<dt>laser eyes</dt><dd>${traits.laserEyes}</dd>`;
  html += `<dt>background</dt><dd>${traits.background}</dd>`;
  html += `<dt>background colors</dt><dd>${colorSwatches(traits.backgroundColors)}</dd>`;
  html += `<dt>crown</dt><dd>${traits.crown}</dd>`;
  html += `<dt>glasses</dt><dd>${traits.glasses}</dd>`;

  if (traits.glasses !== 'None') {
    html += `<dt>glasses colors</dt><dd>${colorSwatches(traits.glassesColors)}</dd>`;
  }

  html += '</dl>';
  container.innerHTML = html;
}
