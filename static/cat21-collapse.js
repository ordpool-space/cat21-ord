// CAT-21 😺 — Override collapse text truncation for Public Pixel font.
// The upstream index.js resize() calculates how many characters fit, but with
// Public Pixel the result is too wide — text overflows the container because
// list markers, padding etc. aren't accounted for.
//
// This script loads BEFORE index.js and:
// 1. Intercepts addEventListener('resize', ...) to block index.js's handler
// 2. Registers our own resize handler with a 0.8x capacity multiplier
// 3. Runs after document.fonts.ready so canvas.measureText uses Public Pixel

// Block index.js from registering its resize handler.
// index.js's DOMContentLoaded callback calls addEventListener('resize', resize)
// where resize() references a 'collapse' variable. We intercept that one call.
const _origAddEventListener = window.addEventListener;
window.addEventListener = function(type, fn, ...args) {
  if (type === 'resize' && fn.toString().includes('collapse')) {
    return; // swallow index.js's resize handler
  }
  return _origAddEventListener.call(this, type, fn, ...args);
};

addEventListener("DOMContentLoaded", () => {
  // Restore addEventListener now that index.js has registered its handlers
  window.addEventListener = _origAddEventListener;

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

  // Run after font is ready — canvas.measureText needs the actual Public Pixel
  // metrics, not the fallback font. Resolves immediately if font is cached.
  document.fonts.ready.then(cat21Resize);
});
