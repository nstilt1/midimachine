This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app).

## Getting Started

Install dependencies and run the development server (Node.js 20+ must be installed):

```bash
bun install
bun run dev
```

Bun manages dependencies and runs scripts; Next.js itself runs under Node.js.
Forcing this Next.js version to run under Bun can leave development startup stuck
at `Starting...`. Both `bun run dev` and `npm run dev` use Node.js.

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.js`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/basic-features/font-optimization) to automatically optimize and load Inter, a custom Google Font.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js/) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/deployment) for more details.

Drums: generate a pattern, then use **Play loop** for the local synthesized kit.
Load audio one-shots per lane to preview your own sounds; files stay in memory for
this session. MIDI exports use the lane's numeric MIDI pitch, independent of its
preview sound. Use the same samples and note mapping in FL Studio (octave labels
can differ between applications). ZIP export includes individual drum lanes.

Chord Vocabulary includes a fretboard editor with one root per custom type,
required/optional note selection, browser storage, and individual/all-custom JSON
export and import. Shapes are normalized to root-relative intervals, then voiced
in the generator's register. Choose a Custom chord group and select your saved
types to use them in generation, vocabulary, finder, and scale pruning. Optional
notes expand into voicings even with pruning disabled. Custom (use pruning)
tries all 12 root transpositions; Custom (hand-picked) keeps each saved root. Natural + blues uses the
union of natural minor and minor blues before transposition to the selected key.

Build WASM with `RUSTUP_TOOLCHAIN=nightly-2026-05-28 bash build_wasm.sh` from
`musicgen/`; the script also installs the generated files into `frontend/public/`.

Chord sampling removes duplicate voicings after optional-note expansion using a
hash set of actual pitches. Chord names, note order, and root labels do not give
identical note sets extra sampling weight. Different octave voicings stay distinct.

Use **Generate HolyC oracle seed** beside the melody or drum seed input to fill
it with 4, 8, 16, or 32 words from Terry Davis's vocabulary. This runs the original
HolyC word-selection core in a separate WASM module and uses browser Web Crypto
for entropy. It replaces the text; MIDI generation remains a separate action.
The generated text is a random word sequence, not grammatical prose. Keep the
text and turn off randomization to reproduce a MIDI result.

The standalone reusable Rust crate lives at `../crates/temple-godspeak/`. Its
README documents the native API, WASM host interface, and source provenance.
