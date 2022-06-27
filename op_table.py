#!/bin/env python3

ops = [
    # stack/misc
    "",    ""   , ""   , "POP", "SWP", "ROT", "DUP", "OVR",
    # logic/jumps
    "EQU", "GTH", "JMP", "JNZ", "JSR", "STH", "", "",
    # mem
    "LDZ", "STZ", "LDR", "STR", "LDA", "STA", "PIC", "PUT",
    # maths
    "ADC", "SBC", "MUL", "DVM", "AND", "IOR", "XOR", "SFT"
]

table = []

for i in range(0x100):
    o = i & 0x1f
    k = i & 0x80
    r = i & 0x40
    d = i & 0x20

    op_text = ""
    do_modes = True
    if o < 8:
        if o == 0:
            if k != 0:
                op_text = "LIT"
                k = False
            else:
                #if i == 0: op_text = "NOP"
                if i == 0x20: op_text = "SEC"
                if i == 0x40: op_text = "CLC"
                if i == 0x60: op_text = "EXT"
                do_modes = False
        elif o == 1 and False:
            if d == 0 and k == 0:
                op_text = "GWS"
            elif d != 0:
                op_text = "SWS"
                d = False
        elif o == 2 and False:
            if d == 0 and k == 0:
                op_text = "GRS"
            elif d != 0:
                op_text = "SRS"
                d = False

        elif o > 2:
            k = False
            if i == 0x83:
                op_text = "RTI"
        else: op_text = ""
    
    if do_modes:
        if op_text == "":
            op_text = ops[o]
        if op_text == "":
            table.append(" " * 8)
            continue

        if k: op_text += "k"
        if r: op_text += "r"
        if d: op_text += "2"

    pad = 8 - len(op_text)
    op_text += " " * pad
    if not op_text in table:
        table.append(op_text)
    else:
        table.append(" " * 8)

if False:
    print("  ", end="")
    for i in range(16):
        print(f"{i:x}" + " " * 7, end="")

    print("")

    for i in range(16):
        print(f"{i:x} ", end="")
        high_nyb = i << 4
        for j in range(16):
            idx = high_nyb | j
            op = table[idx]
            print(op, end="")
        print("")
else:
    table2 = []
    for i in table:
        table2.append(i.strip())
    
    print(table2)
