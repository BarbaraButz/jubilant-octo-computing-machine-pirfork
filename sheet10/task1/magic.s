magic:
    push    rbp
    mov     rbp, rsp
    mov     rcx, rdi
    mov     esi, 2
.LBB0_1:
    mov     al, 1
    cmp     rsi, rcx
    jae     .LBB0_4
    xor     edx, edx
    mov     rax, rcx
    div     rsi
    inc     rsi
    test    rdx, rdx
    jne     .LBB0_1
    xor     eax, eax
.LBB0_4:
    pop     rbp
    ret

Die Funktion magic ist ein Primzahltest für alle Zahlen >= 2. Für 0 und 1 gibt die Funktion allerdings fälschlicherweise true zurück.

l.2: letzten Basepointer auf den Stack pushen, der auf die Base der Funktion im Stack zeigt, die magic aufgerufen hat.

l.3: Nun zeigt der Stackpointer auf die Base von magic, diese Adresse wird also zum neuen Basepointer.

l.4: The first (and in this case only) argument, a 64-bit number (let's call it n), is stored in the register rdi. We copy this number to the rcx register.

l.5: Set rsi to 2. Even though the destination register only is esi, the value of the entire 64-bit register also will be 2 due to the so-called zero-extension
     that is applied when the lower 32 bits of a 64-bit register are set. This does not hold when using 8- or 16-bit registers as the destination, though.
     rsi stores the current divisor (d), starting with 2.

l.7: al, the least significant byte of rax is set to 1. Therefore the entire rax register will be non-zero. rax is the register used for the return value.
     Here, its exact value does not matter since zero signifies rax and all other values signify true.

l.8: Compare d to n to see whether we have already tested for all possible divisors.

l.9: Jump if above or equal. If d is already greater or is equal to n, we jump to to the end of the function and return true (since at this point rax is non-zero)

l.10: Clear the rdx register, which will later be used in the DIV instruction as the higher 64 bits of the dividend.

l.11: Copy n to the rax register, because the lower 64 bits of the dividend always come from rax.

l.12: divide n by d (rsi). The resulting quotient is stored in rax, the remainder in rdx.

l.13: increment d in preparation for the next loop cycle (that may not be reached).

l.14: Set the zero-flag if the remainder was 0, clear it if not.

l.15: Jump not equal (= jump if not zero). If the remainder was not zero, jump to the beginning of the loop.

l.16: Else, we know that n is not prime. Therefore we clear the return register rax (will return false).

l.18: restore the previous base pointer by popping it from the stack into the rbp register.

l.19: return from the function. First pops the return address from the stack into a register and then jumps to this address.
