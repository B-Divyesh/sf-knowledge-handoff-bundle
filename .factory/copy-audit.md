# Landing copy audit

Source: `site/index.html`. Counts treat hyphenated words, commands, and product names as one word. No sentence exceeds 22 words. The banned-word scan found no matches.

| Copy | Words | Result |
| --- | ---: | --- |
| Portable project handoffs | 3 | Pass |
| Build a project handoff | 5 | Pass |
| For departing owners and small teams, give the next owner files, link results, owners, and gaps in one portable bundle. | 20 | Pass |
| Try it with sample data | 6 | Pass |
| Opens a realistic bundle with a broken link and a missing-access gap. | 12 | Pass |
| Install the CLI | 3 | Pass |
| Use your YAML checklist and local files. | 7 | Pass |
| Free under the MIT license | 5 | Pass |
| No account or telemetry | 4 | Pass |
| Built bundles work offline | 4 | Pass |
| Files, links, owners, and gaps in one bundle. | 8 | Pass |
| The problem | 2 | Pass |
| Make missing context visible | 4 | Pass |
| The next owner needs to know which file matters, which link failed, and who still has work to do. | 20 | Pass |
| How it works | 3 | Pass |
| Create a handoff bundle | 4 | Pass |
| List the source of truth | 6 | Pass |
| Add files, public links, owners, expiry dates, and known gaps to a YAML checklist in version control. | 18 | Pass |
| Build and check it | 5 | Pass |
| The CLI copies local files, records SHA-256 hashes, and checks public links only when you ask. | 15 | Pass |
| Send the bundle | 4 | Pass |
| The next owner opens one static folder, reviews artifacts, and exports an acknowledgement tied to the manifest. | 16 | Pass |
| Bundle contents | 2 | Pass |
| Use files you can inspect | 6 | Pass |
| The bundle contains ordinary HTML, JSON, CSS, JavaScript, and copied files. | 10 | Pass |
| It does not need a hosted database or account. | 10 | Pass |
| Copied files include SHA-256 hashes | 5 | Pass |
| Public links are checked only on request | 7 | Pass |
| Credential-like URLs stop before output | 5 | Pass |
| Owners, expiry dates, and gaps remain visible | 7 | Pass |
| Install | 1 | Pass |
| Run the CLI | 3 | Pass |
| Try it with sample data | 6 | Pass |
| Read the source on GitHub | 5 | Pass |
| Privacy and limits | 3 | Pass |
| What the tool does not do | 6 | Pass |
| It does not fetch secrets | 5 | Pass |
| Authenticated URLs are not fetched or written to output. | 9 | Pass |
| Credential-like query parameters stop the build. | 6 | Pass |
| It does not crawl sites | 5 | Pass |
| Link checks use the listed URLs and each origin’s robots.txt. | 11 | Pass |
| They wait between requests from one origin. | 8 | Pass |
| It does not host your bundle | 6 | Pass |
| The generated site works from disk. | 6 | Pass |
| Review progress stays in the recipient browser and exports as local JSON. | 11 | Pass |
| Build a portable project handoff. | 5 | Pass |
| Built by Param Factory · v0.1.0 | 6 | Pass |

## Terminology

| Concept | One word used |
| --- | --- |
| Deliverable sent to the next owner | bundle |
| YAML input | checklist |
| Person receiving work | next owner |
| Items inside a bundle | artifacts |
| Incomplete transfer item | gap |
| Recipient proof file | acknowledgement |
