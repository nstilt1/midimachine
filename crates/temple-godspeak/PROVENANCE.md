# Source provenance and adaptations

TempleOS source: https://github.com/cia-foundation/TempleOS
Commit: c26482bb6ad3f80106d28504ec5db3c6a360732c (archive branch).
Terry A. Davis released TempleOS into the public domain. The original files are
retained in holyc/: HolySpirit.HC, GodExt.HC, Vocab.DD, and Language.DD (Doc/HolyC.DD).
This snapshot contains 7,569 vocabulary tokens; versions of TempleOS may differ.

Compiler: https://github.com/ParkerrDev/HolyC-wasm
Commit: 7e7213397c5db64ffba6e4ebbf29cf8aa26b86a8 (master).
Vendored unchanged under vendor/holyc-wasm, including its README and package
metadata identifying the compiler/runtime as MIT licensed. No network is needed
to rebuild. The wrapper and Rust host are MIT licensed.

The build extracts GodBitsIns, GodBits, and GodWordStr from the original HolyC.
It compiles with noPrelude:true, lenient:false, and rejects all warnings and
parser diagnostics. The compiler's C-like shift precedence differs from HolyC
(see Language.DD), so `res=res<<1+b` is explicitly parenthesized as
`res=(res<<1)+b` before compiling. Tests verify low-bit-first FIFO insertion,
high-bit-first accumulation, 24-bit refills, and 17-bit word selection.

holyc/host.HC replaces the TempleOS popup/timer, FIFO allocation, and filesystem
vocabulary loading with fixed buffers and one host import, env.HostPick() -> i64.
GodPick still shifts the host sample right by four bits. The next 24 bits enter
the FIFO. The FIFO needs at most 24 entries here because it is refilled only when
empty. The original modulo selection is preserved (including its modulo bias).

The Rust host accepts raw u64 samples. The browser host supplies crypto.getRandomValues.
Neither reproduces TempleOS hardware timing or its popup UI. This is the original
word-selection core with an adapted host, not an emulated TempleOS installation,
not Terry's separate Linux Happy.TXT script, and not a grammatical sentence model.
