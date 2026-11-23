// Host adapter for the HolyC module built by crates/temple-godspeak/build_wasm.sh.
export async function createOracle(bytes, vocabulary, pick) {
  const { instance } = await WebAssembly.instantiate(bytes, { env: { HostPick: pick } });
  const api = instance.exports;
  api.__rt_init();
  const words = vocabulary.trimEnd().split('\n');
  const encoder = new TextEncoder();
  const packed = encoder.encode(words.join('\0') + '\0');
  if (words.length > 8000 || packed.length > 100000) throw Error('Oracle vocabulary exceeds its buffers');
  const memory = new Uint8Array(api.memory.buffer);
  const view = new DataView(api.memory.buffer);
  let offset = Number(api.OracleVocabAddress());
  memory.set(packed, offset);
  const slots = Number(api.OracleWordsAddress());
  for (let i = 0; i < words.length; i++) {
    view.setBigUint64(slots + i * 8, BigInt(offset), true);
    offset += encoder.encode(words[i]).length + 1;
  }
  return count => {
    if (!Number.isInteger(count) || count < 1 || count > 128) throw Error('Choose 1–128 words');
    api.OracleInit(BigInt(words.length));
    const result = [];
    for (let i = 0; i < count; i++) {
      const start = Number(api.OracleNext());
      const end = memory.indexOf(0, start);
      if (start <= 0 || end < start) throw Error('Invalid oracle output');
      result.push(new TextDecoder().decode(memory.subarray(start, end)));
    }
    return result.join(' ');
  };
}
let pending;
export async function generateOracleSeed(count = 16) {
  if (!pending) pending = (async () => {
    const [wasm, vocabulary] = await Promise.all([fetch('/godspeak.wasm'), fetch('/godspeak-vocabulary.txt')]);
    if (!wasm.ok || !vocabulary.ok) throw Error('Cannot load the HolyC oracle. Run its build_wasm.sh script.');
    return createOracle(await wasm.arrayBuffer(), await vocabulary.text(), () => {
      const sample = crypto.getRandomValues(new Uint32Array(2));
      return BigInt.asIntN(64, (BigInt(sample[0]) << 32n) | BigInt(sample[1]));
    });
  })().catch(error => { pending = null; throw error; });
  return (await pending)(count);
}
