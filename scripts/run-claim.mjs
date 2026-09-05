import { spawn } from 'node:child_process';

const index = process.argv.indexOf('--test-name-pattern');
const pattern = index >= 0 ? process.argv[index + 1] : undefined;
if (!pattern?.startsWith('@claim:')) {
  throw new Error('Pass one claim tag: npm run test:claims -- --test-name-pattern @claim:<id>');
}

const child = spawn(process.execPath, ['--test', '--test-name-pattern', pattern, 'tests/claims.test.mjs'], { stdio: 'inherit' });
child.on('close', (code) => { process.exitCode = code ?? 1; });
