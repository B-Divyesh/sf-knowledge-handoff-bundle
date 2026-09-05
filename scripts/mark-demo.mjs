import { readFile, writeFile } from 'node:fs/promises';

const [file] = process.argv.slice(2);
if (!file) throw new Error('Usage: node scripts/mark-demo.mjs <index.html>');

const html = await readFile(file, 'utf8');
const banner = `<div class="demo-banner" role="status"><p>Demo — sample data, nothing is saved</p><div class="demo-banner-actions"><button type="button" data-demo-reset>Reset demo</button><a href="/" data-demo-start-real>Start for real</a></div></div>`;
const skip = `<a class="skip" href="#main">Skip to handoff</a>`;
const header = `<header class="demo-site-header"><a href="/" aria-label="Knowledge Handoff Bundle home">KHB / 01</a><nav aria-label="Primary navigation"><a href="/demo/" aria-current="page">Demo</a><a href="/#how">How it works</a><a href="/privacy/">Privacy</a><a href="/#install">Install</a></nav>${banner}</header>`;
const footer = `<footer class="demo-site-footer"><p>Build a portable project handoff.</p><nav aria-label="Footer"><a href="/privacy/">Privacy</a><a href="/terms/">Terms</a><a href="https://github.com/B-Divyesh/sf-knowledge-handoff-bundle">GitHub</a></nav><p>Built by Param Factory · v0.1.0</p></footer>`;
if (!html.includes('<body>')) throw new Error('Could not mark the generated demo page');
const metadata = `<link rel="canonical" href="https://knowledge-handoff-bundle.sociobot.in/demo/"><meta property="og:title" content="Demo — Knowledge Handoff Bundle"><meta property="og:description" content="Try a sample project handoff bundle."><meta property="og:image" content="https://knowledge-handoff-bundle.sociobot.in/handoff-preview.webp"><meta name="twitter:card" content="summary_large_image">`;
await writeFile(
  file,
  html
    .replace(/<title>[^<]*<\/title>/, '<title>Demo — Knowledge Handoff Bundle</title>')
    .replace('</head>', `${metadata}</head>`)
    .replace(skip, '')
    .replace('<body>', `<body data-demo="true">${skip}${header}`)
    .replace(/<footer>.*?<\/footer>/, footer),
);
