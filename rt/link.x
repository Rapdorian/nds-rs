/* Memory layout of the NDS-arm9 */
MEMORY
{
	DTCM : ORIGIN =  0x027C0000, LENGTH = 16K
	ITCM : ORIGIN =  0x00000000, LENGTH = 32K
	RAM : ORIGIN =  0x02000000, LENGTH = 4M
	VRAM : ORIGIN = 0x06000000, LENGTH = 1M
	OAM : ORIGIN =  0x07000000, LENGTH = 2K
}

ENTRY(__start);

SECTIONS
{
	.dtcm.irq ORIGIN(DTCM) + 0x3FFC :
	{
		*(.dtcm.irq)
	} > DTCM

	.dtcm :
	{
		*(.dtcm .dtcm.*)
	} > DTCM

	.itcm :
	{
		*(.itcm, .itcm.*)
	} > ITCM

	.text.startup ORIGIN(RAM) :
	{
		*(.text.startup)
	} > RAM

	.text : 
	{
	  *(.text .text.*);
	} > RAM

	.rodata :
	{
	  *(.rodata .rodata.*);
	} > RAM

	/DISCARD/ :
	{
		*(.ARM.exidx);
	}
}
