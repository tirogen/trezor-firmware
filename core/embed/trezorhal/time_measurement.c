
#include "common.h"
#include "irq.h"

extern __IO uint32_t uwTick;



__IO uint32_t ticks = 0;
__IO uint32_t ticks_diff = 0;
__IO uint32_t ticks_acc = 0;
__IO uint32_t total_acc = 0;

void init_ticks(void) {
  ticks = (hal_ticks_ms() * 180000) + SysTick->VAL;
}

void get_ticks(void) {
  uint32_t ticks_now = (hal_ticks_ms() * 180000) + SysTick->VAL;
  uint32_t ticks_diff_tmp = ticks_now - ticks;
  ticks = ticks_now;
  ticks_diff = ticks_diff_tmp;
  ticks_acc += ticks_diff_tmp;
  total_acc++;
}

void clear_acc(void) {
  ticks_acc = 0;
  total_acc = 0;
}