/*
 * This file is part of the Trezor project, https://trezor.io/
 *
 * Copyright (c) SatoshiLabs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <stdint.h>
#include TREZOR_BOARD
#include "display_interface.h"
#include "memzero.h"
#include STM32_HAL_H

// using const volatile instead of #define results in binaries that change
// only in 1-byte when the flag changes.
// using #define leads compiler to over-optimize the code leading to bigger
// differencies in the resulting binaries.
const volatile uint8_t DISPLAY_ST7789V_INVERT_COLORS = 0;

// FSMC/FMC Bank 1 - NOR/PSRAM 1
#define DISPLAY_MEMORY_BASE 0x60000000
#define DISPLAY_MEMORY_PIN 16

__IO uint8_t *const DISPLAY_CMD_ADDRESS =
    (__IO uint8_t *const)((uint32_t)DISPLAY_MEMORY_BASE);
__IO uint8_t *const DISPLAY_DATA_ADDRESS =
    (__IO uint8_t *const)((uint32_t)DISPLAY_MEMORY_BASE |
                          (1 << DISPLAY_MEMORY_PIN));

#define LED_PWM_TIM_PERIOD \
  (255)  // little less than 4kHz with PSC = (SystemCoreClock / 1000000) - 1)
#define LED_PWM_SLOW_TIM_PERIOD \
  (10000)  // about 10Hz (with PSC = (SystemCoreClock / 1000000) - 1)

// section "9.1.3 RDDID (04h): Read Display ID"
// of ST7789V datasheet
#define DISPLAY_ID_ST7789V 0x858552U

// section "6.2.1. Read display identification information (04h)"
// of GC9307 datasheet
#define DISPLAY_ID_GC9307 0x009307U

// section "8.3.23 Read ID4 (D3h)"
// of ILI9341V datasheet
#define DISPLAY_ID_ILI9341V 0x009341U

static int DISPLAY_BACKLIGHT = -1;
static int DISPLAY_ORIENTATION = -1;
static int pwm_period = LED_PWM_TIM_PERIOD;

void display_pixeldata(uint16_t c) { PIXELDATA(c); }

static uint32_t read_display_id(uint8_t command) {
  volatile uint8_t c = 0;
  uint32_t id = 0;
  CMD(command);
  c = *DISPLAY_DATA_ADDRESS;  // first returned value is a dummy value and
  // should be discarded
  c = *DISPLAY_DATA_ADDRESS;
  id |= (c << 16);
  c = *DISPLAY_DATA_ADDRESS;
  id |= (c << 8);
  c = *DISPLAY_DATA_ADDRESS;
  id |= c;
  return id;
}

static uint32_t display_identify(void) {
  static uint32_t id = 0x000000U;
  static char id_set = 0;

  if (id_set) return id;  // return if id has been already set

  id = read_display_id(0x04);  // RDDID: Read Display ID
  // the default RDDID for ILI9341 should be 0x8000.
  // some display modules return 0x0.
  // the ILI9341 has an extra id, let's check it here.
  if ((id != DISPLAY_ID_ST7789V) &&
      (id != DISPLAY_ID_GC9307)) {         // if not ST7789V and not GC9307
    uint32_t id4 = read_display_id(0xD3);  // Read ID4
    if (id4 == DISPLAY_ID_ILI9341V) {      // definitely found a ILI9341
      id = id4;
    }
  }
  id_set = 1;
  return id;
}

void display_reset_state() {}

static void __attribute__((unused)) display_sleep(void) {
  uint32_t id = display_identify();
  if ((id == DISPLAY_ID_ILI9341V) || (id == DISPLAY_ID_GC9307) ||
      (id == DISPLAY_ID_ST7789V)) {
    CMD(0x28);     // DISPOFF: Display Off
    CMD(0x10);     // SLPIN: Sleep in
    HAL_Delay(5);  // need to wait 5 milliseconds after "sleep in" before
    // sending any new commands
  }
}

static void display_unsleep(void) {
  uint32_t id = display_identify();
  if ((id == DISPLAY_ID_ILI9341V) || (id == DISPLAY_ID_GC9307) ||
      (id == DISPLAY_ID_ST7789V)) {
    CMD(0x11);     // SLPOUT: Sleep Out
    HAL_Delay(5);  // need to wait 5 milliseconds after "sleep out" before
    // sending any new commands
    CMD(0x29);  // DISPON: Display On
  }
}

static struct { uint16_t x, y; } BUFFER_OFFSET;

void display_set_window(uint16_t x0, uint16_t y0, uint16_t x1, uint16_t y1) {
  x0 += BUFFER_OFFSET.x;
  x1 += BUFFER_OFFSET.x;
  y0 += BUFFER_OFFSET.y;
  y1 += BUFFER_OFFSET.y;
  uint32_t id = display_identify();
  if ((id == DISPLAY_ID_ILI9341V) || (id == DISPLAY_ID_GC9307) ||
      (id == DISPLAY_ID_ST7789V)) {
    CMD(0x2A);
    DATA(x0 >> 8);
    DATA(x0 & 0xFF);
    DATA(x1 >> 8);
    DATA(x1 & 0xFF);  // column addr set
    CMD(0x2B);
    DATA(y0 >> 8);
    DATA(y0 & 0xFF);
    DATA(y1 >> 8);
    DATA(y1 & 0xFF);  // row addr set
    CMD(0x2C);
  }
}

int display_orientation(int degrees) {
  if (degrees != DISPLAY_ORIENTATION) {
    if (degrees == 0 || degrees == 90 || degrees == 180 || degrees == 270) {
      DISPLAY_ORIENTATION = degrees;

      char BX = 0, BY = 0;
      uint32_t id = display_identify();
      if ((id == DISPLAY_ID_ILI9341V) || (id == DISPLAY_ID_GC9307) ||
          (id == DISPLAY_ID_ST7789V)) {
#define RGB (1 << 3)
#define MV (1 << 5)
#define MX (1 << 6)
#define MY (1 << 7)
        // MADCTL: Memory Data Access Control - reference:
        // section 9.3 in the ILI9341 manual
        // section 6.2.18 in the GC9307 manual
        // section 8.12 in the ST7789V manual
        uint8_t display_command_parameter = 0;
        switch (degrees) {
          case 0:
            display_command_parameter = 0;
            BY = (id == DISPLAY_ID_GC9307);
            break;
          case 90:
            display_command_parameter = MV | MX;
            BX = (id == DISPLAY_ID_GC9307);
            break;
          case 180:
            display_command_parameter = MX | MY;
            BY = (id != DISPLAY_ID_GC9307);
            break;
          case 270:
            display_command_parameter = MV | MY;
            BX = (id != DISPLAY_ID_GC9307);
            break;
        }
        if (id == DISPLAY_ID_GC9307) {
          display_command_parameter ^= RGB | MY;  // XOR RGB and MY settings
        }
        CMD(0x36);
        DATA(display_command_parameter);
        // reset the column and page extents
        display_set_window(0, 0, DISPLAY_RESX - 1, DISPLAY_RESY - 1);
      }
      BUFFER_OFFSET.x = BX ? (MAX_DISPLAY_RESY - DISPLAY_RESY) : 0;
      BUFFER_OFFSET.y = BY ? (MAX_DISPLAY_RESY - DISPLAY_RESY) : 0;
    }
  }
  return DISPLAY_ORIENTATION;
}

int display_get_orientation(void) { return DISPLAY_ORIENTATION; }

int display_backlight(int val) {
  if (DISPLAY_BACKLIGHT != val && val >= 0 && val <= 255) {
    DISPLAY_BACKLIGHT = val;
    TIM1->CCR1 = pwm_period * val / 255;
  }
  return DISPLAY_BACKLIGHT;
}

static void send_init_seq_GC9307(void) {
  // Inter Register Enable1
  CMD(0xFE);

  // Inter Register Enable2
  CMD(0xEF);

  // TEON: Tearing Effect Line On; V-blanking only
  CMD(0x35);
  DATA(0x00);

  // COLMOD: Interface Pixel format; 65K color: 16-bit/pixel (RGB 5-6-5 bits
  // input)
  CMD(0x3A);
  DATA(0x55);

  // Frame Rate
  // CMD(0xE8); DATA(0x12); DATA(0x00);

  // Power Control 2
  CMD(0xC3);
  DATA(0x27);

  // Power Control 3
  CMD(0xC4);
  DATA(0x18);

  // Power Control 4
  CMD(0xC9);
  DATA(0x1F);

  CMD(0xC5);
  DATA(0x0F);

  CMD(0xC6);
  DATA(0x00);

  CMD(0xC7);
  DATA(0x10);

  CMD(0xC8);
  DATA(0x01);

  CMD(0xFF);
  DATA(0x62);

  CMD(0x99);
  DATA(0x3E);

  CMD(0x9D);
  DATA(0x4B);

  CMD(0x8E);
  DATA(0x0F);

  // SET_GAMMA1
  CMD(0xF0);
  DATA(0x8F);
  DATA(0x1B);
  DATA(0x05);
  DATA(0x06);
  DATA(0x07);
  DATA(0x42);

  // SET_GAMMA3
  CMD(0xF2);
  DATA(0x5C);
  DATA(0x1F);
  DATA(0x12);
  DATA(0x10);
  DATA(0x07);
  DATA(0x43);

  // SET_GAMMA2
  CMD(0xF1);
  DATA(0x59);
  DATA(0xCF);
  DATA(0xCF);
  DATA(0x35);
  DATA(0x37);
  DATA(0x8F);

  // SET_GAMMA4
  CMD(0xF3);
  DATA(0x58);
  DATA(0xCF);
  DATA(0xCF);
  DATA(0x35);
  DATA(0x37);
  DATA(0x8F);
}

static void send_init_seq_ST7789V(uint8_t invert_colors) {
  // most recent manual:
  // https://www.newhavendisplay.com/appnotes/datasheets/LCDs/ST7789V.pdf
  // TEON: Tearing Effect Line On; V-blanking only
  CMD(0x35);
  DATA(0x00);

  // COLMOD: Interface Pixel format; 65K color: 16-bit/pixel (RGB 5-6-5 bits
  // input)
  CMD(0x3A);
  DATA(0x55);

  // CMD2EN: Commands in command table 2 can be executed when EXTC level is Low
  CMD(0xDF);
  DATA(0x5A);
  DATA(0x69);
  DATA(0x02);
  DATA(0x01);

  // LCMCTRL: LCM Control: XOR RGB setting
  CMD(0xC0);
  DATA(0x20);

  // GATECTRL: Gate Control; NL = 240 gate lines, first scan line is gate 80.;
  // gate scan direction 319 -> 0
  CMD(0xE4);
  DATA(0x1D);
  DATA(0x0A);
  DATA(0x11);

  // INVOFF (20h): Display Inversion Off
  // INVON  (21h): Display Inversion On
  CMD(0x20 | invert_colors);

  // the above config is the most important and definitely necessary

  // PWCTRL1: Power Control 1
  CMD(0xD0);
  DATA(0xA4);
  DATA(0xA1);

  // gamma curve 1
  // CMD(0xE0); DATA(0x70); DATA(0x2C); DATA(0x2E); DATA(0x15); DATA(0x10);
  // DATA(0x09); DATA(0x48); DATA(0x33); DATA(0x53); DATA(0x0B); DATA(0x19);
  // DATA(0x18); DATA(0x20); DATA(0x25);

  // gamma curve 2
  // CMD(0xE1); DATA(0x70);
  // DATA(0x2C); DATA(0x2E); DATA(0x15); DATA(0x10); DATA(0x09); DATA(0x48);
  // DATA(0x33); DATA(0x53); DATA(0x0B); DATA(0x19); DATA(0x18); DATA(0x20);
  // DATA(0x25);
}

static void send_init_seq_ILI9341V(void) {
  // most recent manual: https://www.newhavendisplay.com/app_notes/ILI9341.pdf
  // TEON: Tearing Effect Line On; V-blanking only
  CMD(0x35);
  DATA(0x00);

  // COLMOD: Interface Pixel format; 65K color: 16-bit/pixel (RGB 5-6-5 bits
  // input)
  CMD(0x3A);
  DATA(0x55);

  // Display Function Control: gate scan direction 319 -> 0
  CMD(0xB6);
  DATA(0x0A);
  DATA(0xC2);
  DATA(0x27);
  DATA(0x00);

  // Interface Control: XOR BGR as ST7789V does
  CMD(0xF6);
  DATA(0x09);
  DATA(0x30);
  DATA(0x00);

  // the above config is the most important and definitely necessary

  CMD(0xCF);
  DATA(0x00);
  DATA(0xC1);
  DATA(0x30);

  CMD(0xED);
  DATA(0x64);
  DATA(0x03);
  DATA(0x12);
  DATA(0x81);

  CMD(0xE8);
  DATA(0x85);
  DATA(0x10);
  DATA(0x7A);

  CMD(0xF7);
  DATA(0x20);

  CMD(0xEA);
  DATA(0x00);
  DATA(0x00);

  // power control VRH[5:0]
  CMD(0xC0);
  DATA(0x23);

  // power control SAP[2:0] BT[3:0]
  CMD(0xC1);
  DATA(0x12);

  // vcm control 1
  CMD(0xC5);
  DATA(0x60);
  DATA(0x44);

  // vcm control 2
  CMD(0xC7);
  DATA(0x8A);

  // framerate
  CMD(0xB1);
  DATA(0x00);
  DATA(0x18);

  // 3 gamma func disable
  CMD(0xF2);
  DATA(0x00);

  // gamma curve 1
  CMD(0xE0);
  DATA(0x0F);
  DATA(0x2F);
  DATA(0x2C);
  DATA(0x0B);
  DATA(0x0F);
  DATA(0x09);
  DATA(0x56);
  DATA(0xD9);
  DATA(0x4A);
  DATA(0x0B);
  DATA(0x14);
  DATA(0x05);
  DATA(0x0C);
  DATA(0x06);
  DATA(0x00);

  // gamma curve 2
  CMD(0xE1);
  DATA(0x00);
  DATA(0x10);
  DATA(0x13);
  DATA(0x04);
  DATA(0x10);
  DATA(0x06);
  DATA(0x25);
  DATA(0x26);
  DATA(0x3B);
  DATA(0x04);
  DATA(0x0B);
  DATA(0x0A);
  DATA(0x33);
  DATA(0x39);
  DATA(0x0F);
}

void display_init_seq(void) {
  HAL_GPIO_WritePin(GPIOC, GPIO_PIN_14, GPIO_PIN_RESET);  // LCD_RST/PC14
  // wait 10 milliseconds. only needs to be low for 10 microseconds.
  // my dev display module ties display reset and touch panel reset together.
  // keeping this low for max(display_reset_time, ctpm_reset_time) aids
  // development and does not hurt.
  HAL_Delay(10);
  HAL_GPIO_WritePin(GPIOC, GPIO_PIN_14, GPIO_PIN_SET);  // LCD_RST/PC14
  // max wait time for hardware reset is 120 milliseconds
  // (experienced display flakiness using only 5ms wait before sending commands)
  HAL_Delay(120);

  // identify the controller we will communicate with
  uint32_t id = display_identify();
  if (id == DISPLAY_ID_GC9307) {
    send_init_seq_GC9307();
  } else if (id == DISPLAY_ID_ST7789V) {
    send_init_seq_ST7789V(DISPLAY_ST7789V_INVERT_COLORS);
  } else if (id == DISPLAY_ID_ILI9341V) {
    send_init_seq_ILI9341V();
  }

  display_unsleep();
}

void display_setup_fmc(void) {
  // Reference UM1725 "Description of STM32F4 HAL and LL drivers",
  // section 64.2.1 "How to use this driver"
  SRAM_HandleTypeDef external_display_data_sram;
  external_display_data_sram.Instance = FMC_NORSRAM_DEVICE;
  external_display_data_sram.Init.NSBank = FMC_NORSRAM_BANK1;
  external_display_data_sram.Init.DataAddressMux = FMC_DATA_ADDRESS_MUX_DISABLE;
  external_display_data_sram.Init.MemoryType = FMC_MEMORY_TYPE_SRAM;
  external_display_data_sram.Init.MemoryDataWidth = FMC_NORSRAM_MEM_BUS_WIDTH_8;
  external_display_data_sram.Init.BurstAccessMode =
      FMC_BURST_ACCESS_MODE_DISABLE;
  external_display_data_sram.Init.WaitSignalPolarity =
      FMC_WAIT_SIGNAL_POLARITY_LOW;
  external_display_data_sram.Init.WrapMode = FMC_WRAP_MODE_DISABLE;
  external_display_data_sram.Init.WaitSignalActive = FMC_WAIT_TIMING_BEFORE_WS;
  external_display_data_sram.Init.WriteOperation = FMC_WRITE_OPERATION_ENABLE;
  external_display_data_sram.Init.WaitSignal = FMC_WAIT_SIGNAL_DISABLE;
  external_display_data_sram.Init.ExtendedMode = FMC_EXTENDED_MODE_DISABLE;
  external_display_data_sram.Init.AsynchronousWait =
      FMC_ASYNCHRONOUS_WAIT_DISABLE;
  external_display_data_sram.Init.WriteBurst = FMC_WRITE_BURST_DISABLE;
  external_display_data_sram.Init.ContinuousClock =
      FMC_CONTINUOUS_CLOCK_SYNC_ONLY;
  external_display_data_sram.Init.PageSize = FMC_PAGE_SIZE_NONE;

  // reference RM0090 section 37.5 Table 259, 37.5.4, Mode 1 SRAM, and 37.5.6
  FMC_NORSRAM_TimingTypeDef normal_mode_timing;
  normal_mode_timing.AddressSetupTime = 5;
  normal_mode_timing.AddressHoldTime = 1;  // don't care
  normal_mode_timing.DataSetupTime = 6;
  normal_mode_timing.BusTurnAroundDuration = 0;  // don't care
  normal_mode_timing.CLKDivision = 2;            // don't care
  normal_mode_timing.DataLatency = 2;            // don't care
  normal_mode_timing.AccessMode = FMC_ACCESS_MODE_A;

  HAL_SRAM_Init(&external_display_data_sram, &normal_mode_timing, NULL);
}

void display_init(void) {
  // init peripherals
  __HAL_RCC_GPIOE_CLK_ENABLE();
  __HAL_RCC_TIM1_CLK_ENABLE();
  __HAL_RCC_FMC_CLK_ENABLE();

  GPIO_InitTypeDef GPIO_InitStructure;

  // LCD_PWM/PA7 (backlight control)
  GPIO_InitStructure.Mode = GPIO_MODE_AF_PP;
  GPIO_InitStructure.Pull = GPIO_NOPULL;
  GPIO_InitStructure.Speed = GPIO_SPEED_FREQ_VERY_HIGH;
  GPIO_InitStructure.Alternate = GPIO_AF1_TIM1;
  GPIO_InitStructure.Pin = GPIO_PIN_7;
  HAL_GPIO_Init(GPIOA, &GPIO_InitStructure);

  // enable PWM timer
  TIM_HandleTypeDef TIM1_Handle;
  TIM1_Handle.Instance = TIM1;
  TIM1_Handle.Init.Period = LED_PWM_TIM_PERIOD - 1;
  // TIM1/APB2 source frequency equals to SystemCoreClock in our configuration,
  // we want 1 MHz
  TIM1_Handle.Init.Prescaler = SystemCoreClock / 1000000 - 1;
  TIM1_Handle.Init.ClockDivision = TIM_CLOCKDIVISION_DIV1;
  TIM1_Handle.Init.CounterMode = TIM_COUNTERMODE_UP;
  TIM1_Handle.Init.RepetitionCounter = 0;
  HAL_TIM_PWM_Init(&TIM1_Handle);
  pwm_period = LED_PWM_TIM_PERIOD;

  TIM_OC_InitTypeDef TIM_OC_InitStructure;
  TIM_OC_InitStructure.Pulse = 0;
  TIM_OC_InitStructure.OCMode = TIM_OCMODE_PWM2;
  TIM_OC_InitStructure.OCPolarity = TIM_OCPOLARITY_HIGH;
  TIM_OC_InitStructure.OCFastMode = TIM_OCFAST_DISABLE;
  TIM_OC_InitStructure.OCNPolarity = TIM_OCNPOLARITY_HIGH;
  TIM_OC_InitStructure.OCIdleState = TIM_OCIDLESTATE_SET;
  TIM_OC_InitStructure.OCNIdleState = TIM_OCNIDLESTATE_SET;
  HAL_TIM_PWM_ConfigChannel(&TIM1_Handle, &TIM_OC_InitStructure, TIM_CHANNEL_1);

  display_backlight(0);

  HAL_TIM_PWM_Start(&TIM1_Handle, TIM_CHANNEL_1);
  HAL_TIMEx_PWMN_Start(&TIM1_Handle, TIM_CHANNEL_1);

  // LCD_RST/PC14
  GPIO_InitStructure.Mode = GPIO_MODE_OUTPUT_PP;
  GPIO_InitStructure.Pull = GPIO_NOPULL;
  GPIO_InitStructure.Speed = GPIO_SPEED_FREQ_LOW;
  GPIO_InitStructure.Alternate = 0;
  GPIO_InitStructure.Pin = GPIO_PIN_14;
  // default to keeping display in reset
  HAL_GPIO_WritePin(GPIOC, GPIO_PIN_14, GPIO_PIN_RESET);
  HAL_GPIO_Init(GPIOC, &GPIO_InitStructure);

  // LCD_FMARK/PD12 (tearing effect)
  GPIO_InitStructure.Mode = GPIO_MODE_INPUT;
  GPIO_InitStructure.Pull = GPIO_NOPULL;
  GPIO_InitStructure.Speed = GPIO_SPEED_FREQ_VERY_HIGH;
  GPIO_InitStructure.Alternate = 0;
  GPIO_InitStructure.Pin = GPIO_PIN_12;
  HAL_GPIO_Init(GPIOD, &GPIO_InitStructure);

  GPIO_InitStructure.Mode = GPIO_MODE_AF_PP;
  GPIO_InitStructure.Pull = GPIO_NOPULL;
  GPIO_InitStructure.Speed = GPIO_SPEED_FREQ_VERY_HIGH;
  GPIO_InitStructure.Alternate = GPIO_AF12_FMC;
  //                       LCD_CS/PD7   LCD_RS/PD11   LCD_RD/PD4   LCD_WR/PD5
  GPIO_InitStructure.Pin = GPIO_PIN_7 | GPIO_PIN_11 | GPIO_PIN_4 | GPIO_PIN_5;
  HAL_GPIO_Init(GPIOD, &GPIO_InitStructure);
  //                       LCD_D0/PD14   LCD_D1/PD15   LCD_D2/PD0   LCD_D3/PD1
  GPIO_InitStructure.Pin = GPIO_PIN_14 | GPIO_PIN_15 | GPIO_PIN_0 | GPIO_PIN_1;
  HAL_GPIO_Init(GPIOD, &GPIO_InitStructure);
  //                       LCD_D4/PE7   LCD_D5/PE8   LCD_D6/PE9   LCD_D7/PE10
  GPIO_InitStructure.Pin = GPIO_PIN_7 | GPIO_PIN_8 | GPIO_PIN_9 | GPIO_PIN_10;
  HAL_GPIO_Init(GPIOE, &GPIO_InitStructure);

  display_setup_fmc();

  display_init_seq();

  display_set_little_endian();
}

void display_reinit(void) {
  // reinitialize FMC to set correct timing, have to do this in reinit because
  // boardloader is fixed.
  display_setup_fmc();

  // important for model T as this is not set in boardloader
  display_set_little_endian();

  // enable PWM timer
  TIM_HandleTypeDef TIM1_Handle;
  TIM1_Handle.Instance = TIM1;
  TIM1_Handle.Init.Period = LED_PWM_TIM_PERIOD - 1;
  // TIM1/APB2 source frequency equals to SystemCoreClock in our configuration,
  // we want 1 MHz
  TIM1_Handle.Init.Prescaler = SystemCoreClock / 1000000 - 1;
  TIM1_Handle.Init.ClockDivision = TIM_CLOCKDIVISION_DIV1;
  TIM1_Handle.Init.CounterMode = TIM_COUNTERMODE_UP;
  TIM1_Handle.Init.RepetitionCounter = 0;
  HAL_TIM_PWM_Init(&TIM1_Handle);

  pwm_period = LED_PWM_TIM_PERIOD;
  display_backlight(DISPLAY_BACKLIGHT);
}

void display_refresh(void) {
  uint32_t id = display_identify();
  if (id && (id != DISPLAY_ID_GC9307)) {
    // synchronize with the panel synchronization signal
    // in order to avoid visual tearing effects
    while (GPIO_PIN_RESET == HAL_GPIO_ReadPin(GPIOD, GPIO_PIN_12)) {
    }
    while (GPIO_PIN_SET == HAL_GPIO_ReadPin(GPIOD, GPIO_PIN_12)) {
    }
  }
}

void display_set_slow_pwm(void) {
  // enable PWM timer
  TIM_HandleTypeDef TIM1_Handle;
  TIM1_Handle.Instance = TIM1;
  TIM1_Handle.Init.Period = LED_PWM_SLOW_TIM_PERIOD - 1;
  // TIM1/APB2 source frequency equals to SystemCoreClock in our configuration,
  // we want 1 MHz
  TIM1_Handle.Init.Prescaler = SystemCoreClock / 1000000 - 1;
  TIM1_Handle.Init.ClockDivision = TIM_CLOCKDIVISION_DIV1;
  TIM1_Handle.Init.CounterMode = TIM_COUNTERMODE_UP;
  TIM1_Handle.Init.RepetitionCounter = 0;
  HAL_TIM_PWM_Init(&TIM1_Handle);

  pwm_period = LED_PWM_SLOW_TIM_PERIOD;
  display_backlight(DISPLAY_BACKLIGHT);
}

void display_set_little_endian(void) {
  uint32_t id = display_identify();
  if (id == DISPLAY_ID_GC9307) {
    // CANNOT SET ENDIAN FOR GC9307
  } else if (id == DISPLAY_ID_ST7789V) {
    CMD(0xB0);
    DATA(0x00);
    DATA(0xF8);
  } else if (id == DISPLAY_ID_ILI9341V) {
    // Interface Control: XOR BGR as ST7789V does
    CMD(0xF6);
    DATA(0x09);
    DATA(0x30);
    DATA(0x20);
  }
}

void display_set_big_endian(void) {
  uint32_t id = display_identify();
  if (id == DISPLAY_ID_GC9307) {
    // CANNOT SET ENDIAN FOR GC9307
  } else if (id == DISPLAY_ID_ST7789V) {
    CMD(0xB0);
    DATA(0x00);
    DATA(0xF0);
  } else if (id == DISPLAY_ID_ILI9341V) {
    // Interface Control: XOR BGR as ST7789V does
    CMD(0xF6);
    DATA(0x09);
    DATA(0x30);
    DATA(0x00);
  }
}

const char *display_save(const char *prefix) { return NULL; }

void display_clear_save(void) {}
