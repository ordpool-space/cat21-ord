// CAT-21 😺 — Override collapse text truncation for Public Pixel font.
// The upstream index.js resize() calculates how many characters fit, but with
// Public Pixel the result is too wide — text overflows the container because
// list markers, padding etc. aren't accounted for. This script re-runs the
// same logic with a 0.8x capacity multiplier so text fits cleanly.
// Note: This executes right after index.js's resize(), so collapse runs twice.
// We can't remove the original because its resize function is scoped inside
// a DOMContentLoaded closure with no external reference.
addEventListener("DOMContentLoaded", () => {
  const MULTIPLIER = 0.8;
  const ctx = document.createElement('canvas').getContext('2d');

  function cat21Resize() {
    for (const node of document.getElementsByClassName('collapse')) {
      const original = node.dataset.original || node.textContent.trim();
      node.dataset.original = original;
      const length = original.length;
      let width = node.clientWidth;
      if (width === 0) {
        width = node.parentNode.getBoundingClientRect().width;
      }
      ctx.font = window.getComputedStyle(node).font;
      const capacity = (width / (ctx.measureText(original).width / length)) * MULTIPLIER;
      if (capacity >= length) {
        node.textContent = original;
      } else {
        const count = Math.floor((capacity - 1) / 2);
        node.textContent = `${original.substring(0, count)}…${original.substring(length - count)}`;
      }
    }
  }

  addEventListener('resize', cat21Resize);
  cat21Resize();

  // Re-run after Public Pixel font loads — canvas.measureText uses the
  // fallback font if the webfont isn't ready yet, producing wrong capacity.
  document.fonts.ready.then(cat21Resize);
});
