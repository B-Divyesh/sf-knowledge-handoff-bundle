# Demo sandbox

## Website demo

Open `/demo/` or `https://knowledge-handoff-bundle.sociobot.in/demo/`.

The first page displays a persistent `Demo — sample data, nothing is saved` banner. The Atlas sample has two copied local files, one recorded reachable public reference, one recorded broken link, an expiry warning, and a production-access gap.

Demo review state uses a browser key beginning with `demo:`. It never reads or writes the normal `khb:` review key. **Reset demo** clears that `demo:` key and resets the fields. **Start for real** clears it before returning to the landing page.

The sample link outcomes are recorded fixtures. The demo makes no link-check request at page load.

## CLI demo

Run:

```sh
khb demo
```

The command writes the bundled Atlas sample to a new temporary directory and prints the bundle path. It makes no network request. The bundled input is in `examples/atlas/` and is also compiled into the binary so the installed artifact can run the same sample.

For the website build, `khb demo --output site/public/demo --force` writes the sample and `scripts/mark-demo.mjs` adds the sandbox banner and site navigation.
