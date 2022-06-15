
#include "common.h"

void mp_hal_delay_ms(uint32_t delay);

uint32_t mp_hal_ticks_ms(void);

void init_ticks(void);
void get_ticks(void);
void clear_acc(void);