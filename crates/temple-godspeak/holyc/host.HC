// Host adaptation, not Terry's TempleOS UI/filesystem. All integers use i64.
#define NULL 0
#define GOD_BAD_BITS 4
#define GOD_GOOD_BITS 24
import I64 HostPick();
class COracle {
  I64 fifo;
  I64 num_words;
  U8 **words;
};
COracle god;
U8 fifo_bits[64];
I64 fifo_head, fifo_tail;
U8 *word_slots[8000];
U8 vocab_data[100000];
I64 FifoU8Ins(I64 fifo,U8 bit) {
  fifo_bits[fifo_tail%64]=bit; fifo_tail++; return 1;
}
I64 FifoU8Rem(I64 fifo,U8 *bit) {
  if (fifo_head==fifo_tail) return 0;
  *bit=fifo_bits[fifo_head%64]; fifo_head++; return 1;
}
I64 GodPick(U8 *msg=NULL) { return HostPick()>>GOD_BAD_BITS; }
U0 OracleInit(I64 count) {
  fifo_head=0; fifo_tail=0; god.num_words=count; god.words=word_slots;
}
I64 OracleWordsAddress() { return word_slots; }
I64 OracleVocabAddress() { return vocab_data; }
