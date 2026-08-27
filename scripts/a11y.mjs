import AxeBuilder from '@axe-core/playwright';
import { chromium } from 'playwright';

const baseUrl = process.argv[2] ?? 'http://127.0.0.1:4173';
const routes = ['/', '/demo/', '/privacy/', '/terms/'];
const browser = await chromium.launch();
let failed = false;

for (const route of routes) {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  await page.goto(new URL(route, baseUrl).href, { waitUntil: 'networkidle' });
  const results = await new AxeBuilder({ page }).analyze();
  const serious = results.violations.filter(({ impact }) => impact === 'serious' || impact === 'critical');
  console.log(`${route}: ${results.violations.length} total, ${serious.length} serious/critical`);
  for (const violation of serious) console.error(`  ${violation.id}: ${violation.help}`);
  failed ||= serious.length > 0;
  await context.close();
}

await browser.close();
process.exitCode = failed ? 1 : 0;
