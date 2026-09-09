MEMORY
{
  FLASH : ORIGIN = 0x10000000, LENGTH = 4M
  SRAM : ORIGIN = 0x20000000, LENGTH = 520K
}

ENTRY(main)

/* LONG write 4BYTE */
/* > FLASH mean put everything in the FLASH */
/* docs: https://sourceware.org/binutils/docs/ld/SECTIONS.html */
/*
**  vector table docs
**  1st element is Initial SP value
**  2nd element is Reset value
**  for the moment we only need this but we will need to add everything from 
**  the vector table in the future
**    -> https://support.arm.com/documentation/100235/0100/The-Cortex-M33-Processor/Exception-model/Vector-table
*/

PICOBIN_BLOCK_MARKER_START = 0xffffded3;
PICOBIN_BLOCK_MARKER_END = 0xab123579;
PICOBIN_BLOCK_ITEM_2BS_LAST = 0xff;

SECTIONS
{
  .vector_table ORIGIN(FLASH) : 
  {
    LONG(0x20082000);
    LONG(main);
  } > FLASH

  .text :
  {
    *(.text.main);
  } > FLASH

  /* IMAGE_DEFs */
  .image_def ALIGN(4) :
  {
    LONG(PICOBIN_BLOCK_MARKER_START);
    BYTE(0x42);
    BYTE(0x01);
    /* 0001000000100001 -> section 5.9.3.1. IMAGE_DEF item from the datasheet */
    SHORT(0x1021);
    /* lenght of block */
    BYTE(PICOBIN_BLOCK_ITEM_2BS_LAST);
    SHORT(0x0001);
    BYTE(0);
    /* ptr to next */
    LONG(0);
    LONG(PICOBIN_BLOCK_MARKER_END);
  } > FLASH
}