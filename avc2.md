# AVC2 formal specification

The following is the specification for any emulator of the AVC2 VCPU. It is NOT the documentation 

## Preface

AVC2 is a virtual CPU, inspired by the Uxn/Varvara platform. It is intended to be complex enough to allow high-level work and support indefinite extensions, but still simple enough to not hamper its educational or enternainment value. It is presented as a fun toy or a set of problems to solve, rather than a serious development platform.
 
The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and "OPTIONAL" in this document are to be interpreted as described in [IETF RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

## Processor and memory structure

AVC2 is a stack-based system. It possesses 4 registers, although none are available through the usual method. The registers are as follows:

- `wsp`: Working Stack Pointer
- `rsp`: Return Stack Pointer
- `st`: STatus
- `pc`: Program Counter

The two stack pointers correspond to two stacks in memory. The working stack SHOULD be located in the 0x0100 - 0x01ff page, and the return stack SHOULD be located in the 0x0200 - 0x02ff page. Both stacks MUST grow downwards. The 0x0000 - 0x00ff page is the zero page. The 0xff00 - 0xffff page is dedicated to memory-mapped device I/O. The CPU MUST jump to the program start point (which SHOULD be 0x0300) upon starting.

AVC2 is big-endian, which is to say the most significant byte of a multi-byte number (such as an address) is stored in the lower memory location. When pushing a 16-bit value to the stack, the low byte is pushed first, such that the value is oriented correctly in memory.

Values are unsigned unless otherwise mentioned. Signed values use 2's complement representation.

## Instructions

AVC2 instructions consist of a 5-bit opcode and 3 mode bits. The modes are k, r, and 2, which mean "keep", "return" and "double width" respectively.

Keep does not consume values from the stack. It uses the values on the stack, but does not move the stack pointer upwards. For example, ADD would turn `01 02` into `03` but ADDk would turn it into `01 02 03`

Return operates on the return stack directly, or if an opcode already operates on both stacks, it switches their positions.

Double width uses 16-bit values from the stack (in most cases).

### Stack primitives

If the keep mode bit is set on a stack primitive, it becomes a NOP. The byte that would correspond to `POPk` (0x83) is instead used for `RTI` (see "Odds and ends" section).

- `POP`: Remove the top value from the stack.
    `a b c -- a b`
- `SWP`: Swap the top two values of the stack.
    `a b c -- a c b`
- `ROT`: Swap the second and third items of the stack.
    `a b c -- b a c`
- `DUP`: Duplicate the top item of the stack.
    `a b c -- a b c c`
- `OVR`: Take the second item of the stack and push it to the top.
    `a b c -- a b c b`

### Logic and jumps

- `EQU`: Compare the top two values of the stack for equality. If they are equal, push true (0xff). If they are not, push false (0x00).
- `GTH`: Pop b and a. If a is greater than b, push true. Otherwise, push false. This opcode MUST compare signed values.
    `01 02 -- 00` (this runs `01 > 02`)
- `JMP`: Pop an address from the stack. In double width mode, the address is a 16-bit absolute address. Jump to it. Otherwise, it is a single signed byte representing an offset from the current program counter. Change the program counter by that amount.
- `JNZ`: Identical to JMP, except that after the address is popped before the jump, another value will be popped. This value MUST always be 8 bits. If the value is not zero, jump. Otherwise, resume execution.
- `JSR`: Identical to JMP, except that before jumping, the` value of the program counter is pushed to the return stack.
- `STH`: Move a value from the working stack to the return stack. Does the inverse in return mode.

### Memory accesses

- `LDZ`: Pop an 8-bit value from the stack. Get the value in the zero page at that address and push it to the stack.
- `STZ`: Pop an 8-bit value from the stack. Pop another value from the stack and store it at that address in the zero page.
- `LDR`: Pop an 8-bit signed value from the stack. Offset the program counter by that amount. Get the value at the offset address and push it to the stack.
- `STR`: Pop an 8-bit signed value from the stack. Offset the program counter by that amount. Pop another value from the stack and store it at the offset address.
- `LDA`: Pop a 16-bit value from the stack. Get the value at that address and push it to the stack.
- `STA`: Pop a 16-bit value from the stack. Pop another value from the stack and store it at that address.
- `PIC`: Pop an 8-bit value from the stack. Add it to the stack pointer, get the value at that address, and push it to the stack.
- `PUT`: Pop an 8 bit value from the stack. Add it to the stack pointer to get an address. Pop another value from the stack. Store that value at the address.

### Arithmetic

- `ADC`: 

### Literals

The `LIT` opcode occupies the keep mode of the null byte (if that makes any sense). It takes the next byte (or 16-bit word in 16-bit mode) and pushes it to the stack.

### Odds and ends

`SEC` and `CLC` are used to set and clear the carry flag, respectively. `RTI` pops a value from the stack and places it in the status register, then pops a 16-bit word from the return stack and jumps to it. This is used to return from device interrupts. `EXT` pushes 0 to the stack. This will later be used to determine what processor instruction set extensions are enabled. None of these have any modes available. 

## Device I/O



## Conventions


