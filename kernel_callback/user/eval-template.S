
.global _start
.section .text
_start:
    # Call SYS_REG_LOAD = 2
    li      a7, 2
    la      a0, load_callback
    ecall
    li      a1, 0
    bne     a0, a1, exit

    # Call SYS_REG_LOAD = 2
    li      a7, 2
    la      a0, brk_callback
    ecall
    li      a1, 1
    bne     a0, a1, exit

    li      a1, 0xdeadbeef
    lw      a2, 0(a1)
    j exit

load_callback:
    # Call SYS_REG_LOAD = 1
    li      a7, 1
    la      a0, brk_callback
    ecall
    li      a1, 0
    bne     a0, a1, exit

    ebreak
    j exit

brk_callback:

    li     a7, 64             
    li     a0, 1               
    la     a1, message            
    la     a2, message_end
    sub    a2, a2, a1            
    ecall
    j exit

exit:
    li     a7, 93              
    li     a0, 0
    ecall

.section .rodata                  
message:
    .ascii  "Token:\n"
message_end: