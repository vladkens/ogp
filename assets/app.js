const getOGIUrl = () => {
  const els = document.querySelector("form[hx-target='#ogpi']").querySelectorAll("input, select");
  const ops = Array.from(els)
    .map((x) => `${x.name}=${encodeURIComponent(x.value)}`)
    .join("&");

  return `${window.location.origin}/v0/png?${ops}`;
};

document.getElementById("copy_url").addEventListener("click", (event) => {
  navigator.clipboard.writeText(getOGIUrl());

  const wasContent = event.target.textContent;
  event.target.textContent = "Copied!";
  setTimeout(() => {
    event.target.textContent = wasContent;
  }, 1000);
});

document.getElementById("open_url").addEventListener("click", () => {
  window.open(getOGIUrl());
});

document.addEventListener("wheel", (e) => {
  const el = document.activeElement;
  if (!el || el.tagName.toLowerCase() !== "select") return;

  const change = e.deltaY < 0 ? 1 : -1;
  const newIndex = Math.max(0, Math.min(el.length - 1, el.selectedIndex + change));
  if (el.selectedIndex !== newIndex) {
    el.options[newIndex].selected = true;
    el.dispatchEvent(new InputEvent("input", { bubbles: true }));
  }
});

hljs.highlightAll();
