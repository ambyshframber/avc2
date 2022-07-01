# AVC2 formal specification

The following is the version 1 specification for any emulator of the AVC2 VCPU. It is NOT the documentation for the reference implementation, also contained within this repository.

## 1: Preface

AVC2 is a virtual CPU, inspired by the Uxn/Varvara platform. It is intended to be complex enough to allow high-level work and support indefinite extensions, but still simple enough to not hamper its educational or enternainment value. It is presented as a fun toy or a set of problems to solve, rather than a serious development platform.
 
The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and "OPTIONAL" in this document are to be interpreted as described in [IETF RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

## 2: Processor and memory structure

AVC2 is a stack-based system. It possesses 4 registers, although none are available through the usual method. The registers are as follows:

- `wsp`: Working Stack Pointer
- `rsp`: Return Stack Pointer
- `st`: STatus
- `pc`: Program Counter

The two stack pointers correspond to two stacks in memory. They both point to the next unused slot, ie. pushing a value will write to the location they point to and then decrement them. The working stack MUST be located in the 0x0100 - 0x01ff page, and the return stack MUST be located in the 0x0200 - 0x02ff page. Both stacks MUST grow downwards. The 0x0000 - 0x00ff page is the zero page. The 0xff00 - 0xffff page is dedicated to memory-mapped device I/O. The CPU MUST jump to the program start point (0x0300) upon starting.

The status of uninitialised memory is undefined.

The status register is currently only used for the carry/inverse borrow flag (bit 0). Future versions of the specification may define other flags.

AVC2 is big-endian, which is to say the most significant byte of a multi-byte number (such as an address) is stored in the lower memory location. When pushing a 16-bit value to the stack, the low byte is pushed first, such that the value is oriented correctly in memory.

Values are unsigned unless otherwise mentioned. Signed values use 2's complement representation.

## 3: Instructions

AVC2 instructions consist of a 5-bit opcode and 3 mode bits. The modes are k, r, and 2, which mean "keep", "return" and "double width" respectively. The layout in memory is as follows:

```
kr2ooooo
```

Keep does not consume values from the stack. It uses the values on the stack, but does not move the stack pointer upwards. For example, ADD would turn `01 02` into `03` but ADDk would turn it into `01 02 03`

Return operates on the return stack directly, or if an opcode already operates on both stacks, it switches their positions.

Double width uses 16-bit values from the stack (in most cases). If the size of a value is not given, it is defined by the presence of this mode.

See also the opcode table contained in this repository.

### 3.1: Stack primitives

If the keep mode bit is set on a stack primitive, the behaviour is undefined. The only current exception is that the byte that would correspond to `POPk` (0x83) is instead used for `RTI` (see section 3.6).

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

### 3.2: Logic and jumps

- `EQU`: Compare the top two values of the stack for equality. If they are equal, push true (0xff). If they are not, push false (0x00).
- `GTH`: Pop b and a. If a is greater than b, push true. Otherwise, push false. This opcode MUST compare signed values.  
    `01 02 -- 00` (this runs `01 > 02`)
- `JMP`: Pop an address from the stack. In double width mode, the address is a 16-bit absolute address. Jump to it. Otherwise, it is a single signed byte representing an offset from the current program counter. Change the program counter by that amount.
- `JNZ`: Identical to JMP, except that after the address is popped before the jump, another value will be popped. This value MUST always be 8 bits. If the value is not zero, jump. Otherwise, resume execution.
- `JSR`: Identical to JMP, except that before jumping, the value of the program counter + 1 is pushed to the return stack.
- `STH`: Move a value from the working stack to the return stack. Does the inverse in return mode.

### 3.3: Memory accesses

- `LDZ`: Pop an 8-bit value from the stack. Get the value in the zero page at that address and push it to the stack.
- `STZ`: Pop an 8-bit value from the stack. Pop another value from the stack and store it at that address in the zero page.
- `LDR`: Pop an 8-bit signed value from the stack. Offset the program counter by that amount. Get the value at the offset address and push it to the stack.
- `STR`: Pop an 8-bit signed value from the stack. Offset the program counter by that amount. Pop another value from the stack and store it at the offset address.
- `LDA`: Pop a 16-bit value from the stack. Get the value at that address and push it to the stack.
- `STA`: Pop a 16-bit value from the stack. Pop another value from the stack and store it at that address.
- `PIC`: Pop an 8-bit value from the stack. Add it to the stack pointer, get the value at that address, and push it to the stack.
- `PUT`: Pop an 8 bit value from the stack. Add it to the stack pointer to get an address. Pop another value from the stack. Store that value at the address.

### 3.4: Arithmetic

- `ADC`: Pop two values and add them, then add 1 if the carry flag is set. If the calculation overflows, set the carry flag. Otherwise unset it. Push the result to the stack.
- `SBC`: Pop a and b. Subtract a from b. If the carry flag is NOT set, subtract 1. If the calculation underflows, unset the carry flag. Otherwise set it. Push the result to the stack.
- `MUL`: Pop a and b. Push a * b.
- `DVM`: Pop a and b. Push b / a, then push b % a (b modulo a).
- `AND`: Pop a and b. Push a & b.
- `IOR`: Pop a and b. Push a | b.
- `XOR`: Pop a and b. Push a ^ b.
- `SFT`: Pop an 8-bit value from the stack. The upper nybble is the shift left, and the lower nybble is the shift right. Pop another value and bitshift it by those amounts, shifting left first.

### 3.5: Literals

The `LIT` opcode occupies the keep mode of the null byte (if that makes any sense). It takes the next byte (or 16-bit word in 16-bit mode) relative to the program counter and pushes it to the stack. It then increments the program counter such that the bytes it just pushed will not be executed.

### 3.6: Odds and ends

`SEC` and `CLC` are used to set and clear the carry flag, respectively. `RTI` pops a value from the stack and places it in the status register, then pops a 16-bit word from the return stack and jumps to it. This is used to return from device interrupts. `EXT` pushes 0 to the stack. This will later be used to determine what processor instruction set extensions are enabled. None of these have any modes available.  
A value of 0 MUST be a no-op. All other values are undefined.

## 4: Device I/O

Memory-mapped I/O takes place in the top page of memory. Each device has 16 bytes of I/O space. The first byte (the DEVID port) MUST return the device ID, a value in the range [1, 240] that corresponds to the device type, when read. The first device (in the range 0xff00 - 0xff0f) MUST be the system device, defined below.

Devices MAY use multi-byte ports. Devices MAY modify their internal state on read. If a cell in the device page is read, but no device is using it for a port, the return value MUST be 0 if it would be a DEVID port, and is undefined otherwise.

As of revision 1.1, devices MAY perform Direct Memory Access (DMA).

### 4.1: The system device

This device is used to allow the processor to control itself, and to perform a few functions impossible with pure CPU instructions. The device MUST buffer terminal input such that reads of STDIN are non-blocking. The device SHOULD NOT have local terminal echo.

|Port|Function|
|---|---|
|0 DEVID|Returns 1 when read|
|1 WAIT|When written to, suspend the CPU for that number of ms|
|2 RANDOM|When read, returns a random number in the range [0, 256). The generator does not need to be cryptographically secure, but it SHOULD be sufficiently non-deterministic to make a dice program (say)|
|8 STDIN|When read, returns a byte from the terminal input, or 0x00 if no bytes are present in the buffer|
|9 STDOUT|When written to, send a byte to the standard output terminal|
|a STDERR|When written to, send a byte to the standard error terminal. This MAY be the same as the standard output|
|b BUFLEN|When read, returns the size of the input buffer, or 0xff if the length of the buffer is greater than 0xff|
|f HALT|When written to, immediately halt the CPU|

### 4.2: The drive device 

This device is an emulation of a 16mb block-based drive. Blocks are 256 bytes long (the same as a page of memory). There are 65536 blocks.

When a block is read, it is placed in an entire page. When written, an entire page is placed into the block.

|Port|Function|
|---|---|
|0 DEVID|Returns 2|
|2 BLKHB|When written to, set the hibyte of the block address|
|3 BLKLB|When written to, set the lobyte of the block address|
|4 PAGE|When written to, set the page of memory to use|
|8 READ|When written to, move the specified block of the drive into memory, at the specified page|
|9 WRITE|When written to, move the specified page of memory into the drive, at the specified block|

The drive SHOULD be saved to a file on the host machine when the emulator is shut down. The drive archive format starts with `41 56 44 00` (hex), and proceeds in blocks of 258 bytes. The first 2 bytes of a block are the block number within the drive (in big-endian representation), and the remaining 256 are the contents of the block. Any blocks not given in the archive are assumed to be empty. This choice was made to keep the size of drive archives down and make it easier to compress the data at runtime and reduce memory usage.

## 5: Conventions and caveats

### 5.1: Assembled ROM format

The first 4 bytes of the rom are the magic number. Version 1 ROMs have the magic number `41 56 43 00` (hex). Any file without this header is an invalid Version 1 ROM. The remainder of the rom is the program data, and MUST be loaded in with the first byte at the program start point.

### 5.2: Jumping

When jumping to an address, the next byte to be executed must be the byte at that address, not the next address.

### 5.3: Data structures

Boolean values are zero for false and any non-zero value for true. The CPU's EQU and GTH instructions return 0xff for true. Strings should be stored on the heap and referred to with a fat pointer on the stack. Null-terminated strings SHOULD NOT be used.
