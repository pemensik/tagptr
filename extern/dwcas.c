#include <stdatomic.h>
#include <stdint.h>

struct dwcas_uint128_t {
  uint64_t first, second;
};

static memory_order mem_order_from_uint8_t(uint8_t order) {
  switch (order) {
    case 0: return memory_order_relaxed;
    case 1: return memory_order_acquire;
    case 2: return memory_order_release;
    case 3: return memory_order_acq_rel;
    default: return memory_order_seq_cst;
  }
}

bool dwcas_compare_exchange_128(
  volatile struct dwcas_uint128_t* dst,
  uint64_t* old,
  const uint64_t* new,
  uint8_t success,
  uint8_t failure,
) {
  return atomic_compare_exchange_strong(
    dst, old, new,
    mem_order_from_uint8_t(success),
    mem_order_from_uint8_t(failure),
  );
}
