import './style.css';

const copyButton = document.querySelector<HTMLButtonElement>('[data-copy]');
const copyStatus = document.querySelector<HTMLElement>('#copy-status');

copyButton?.addEventListener('click', async () => {
  const value = copyButton.dataset.copy ?? '';
  try {
    await navigator.clipboard.writeText(value);
    copyStatus!.textContent = 'Copied.';
  } catch {
    copyStatus!.textContent = 'Copy failed. Select the command above.';
  }
});

if ('serviceWorker' in navigator && location.protocol === 'https:') {
  window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js'));
}
