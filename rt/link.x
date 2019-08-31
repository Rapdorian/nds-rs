/* Memory layout of the NDS-arm9 */
MEMORY
{
	RAM : ORIGIN =  0x02000000, LENGTH = 4M
	VRAM : ORIGIN = 0x06000000, LENGTH = 1M
	OAM : ORIGIN =  0x07000000, LENGTH = 2K
	VECTORS : ORIGIN = 0xFFFF0000, LENGTH = 256
}

ENTRY(_start);

EXTERN(EXCEPTION_VECTORS);

SECTIONS
{
	.text.startup 0x02000000 :
	{
		*(.text.startup)
	} > RAM

	.vector_table : 
	{
		*(.vector_table);	
	} > VECTORS

	.text : 
	{
	  *(.text);
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
