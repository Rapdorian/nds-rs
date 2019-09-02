IO_SPACE = 0x04000000

IME = 0x208
IE  = 0x210
IF  = 0x214
DISPSTAT = 0x4

BIOS = 0x03003F00
EVECTOR = 0xFC
BIOS_FLAG = 0xF8


.section .text.startup
.global __start
.arm
__start:
	mov r11, r11
	@ Fetch DTCM Offset
	MRC p15, 0, r2, c9, c1, 0
	ldr r1, =0x3EF6
	ADD r2, r2, r1

	@ Enable CPU irq
	MRS r1, cpsr
	BIC r1, r1, #0x80
	MSR cpsr_c, r1

	ldr r0, =_irq_dispatch
	str r0, [r2, #EVECTOR]
		
	ldr r0, =1
	ldr r1, =IO_SPACE
	str r0, [r1, #IME]
	str r0, [r1, #IE]
	ldr r0, =0b1000
	str r0, [r1, #DISPSTAT]

	b _start

.section .text
.arm
_irq_dispatch:
	@ Fetch DTCM Offset
	MRC p15, 0, r2, c9, c1, 0
	ldr r1, =0x3EF6
	ADD r2, r2, r1

	ldr r1, =IO_SPACE
	ldr r0, =1
	str r0, [r1, #IF]
	str r0, [r2, #BIOS_FLAG]
	bx lr
