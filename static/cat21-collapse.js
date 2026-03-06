// CAT-21 😺 — Override collapse text truncation for Public Pixel font.
// The upstream index.js resize() calculates how many characters fit, but with
// Public Pixel the result is too wide — text overflows the container because
// list markers, padding etc. aren't accounted for. This script re-runs the
// same logic with a 0.8x capacity multiplier so text fits cleanly.
// Note: index.js's resize() also runs on DOMContentLoaded and window resize,
// but we can't remove it (scoped inside a closure). Our fonts.ready handler
// overwrites its results, so the visual double-run is brief.
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

  // Only run after font is ready — avoids a wasted run on DOMContentLoaded
  // when the font isn't loaded yet (canvas would measure with fallback font).
  // document.fonts.ready resolves immediately if the font is already cached.
  document.fonts.ready.then(cat21Resize);
});
