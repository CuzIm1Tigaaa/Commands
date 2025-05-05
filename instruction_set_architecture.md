# Cheap-16: Instruction Set Architecture (ISA)

Welcome to the **Cheap-16 Instruction Set Architecture (ISA)** — a lightweight, fictional ISA designed with simplicity and fun in mind.

> **Disclaimer:** This ISA is neither comprehensive nor intended to serve as a model of best practices in computer architecture design. It is a conceptual and educational model, crafted in more and less two days over the 2025 Easter holdiay to be easy to understand and work with. Use it as a playground for learning, experimentation, or pure creative exploration.

## Program Execution And Generation Model

![program execution and generation model](/docs/diagrams/program-generation-and-execution-flow.svg)

1. *Cheap-16 source code* (`.cheaps`) is written by the user and can be loaded as a file or in-memory (as string).
2. The code is compiled and linked by a *combined assembler and linker*, producing a *Cheap-16 executable* (`.cheapx`) which also can either be stored as a file or kept in memory.
3. An *interpreter* then runs the `.cheapx` executable, supporting both execution and debugging modes.

## Adress Space

![address space](/docs/diagrams/address-space.svg)


**1** Byte can be considered as *the smallest addressable unit* in our architecture although we do not have any instruction to manipulate a single Byte directly. Endianess is *little endian*.

We support various sections defined as follows:

1. *Code section* will be used to store machine instructions has a size of **2<sup>12</sup> = 4096** Bits or **512** Bytes.
2. *Reserved sections* are there for later or unplanned modifications and should not be used as an address. Any operation on these addresses will result into an error.
3. *Registers sections* will be used for registers as discussed later in this documentation.
4. *Data section* is used for values that can be used in operations engaging our instructions and has a size of **2<sup>15</sup> = 32768** Bits or **4** Kilobytes.
5. *Stack* is used as temporary storage and has a size of **2<sup>10</sup> = 1024** Bits or **128** Bytes resulting into a storage for **8** values each **16** Bits. It is important to note that our stack is a fully-ascending one which means that each `PUSH` will increase `SP` by **4** and each `POP` will reduce it by **4**.

## Registers

We support **13** general-purpose registers and **4** special-purpose registers, each **16**-bit wide.

![registers overview](/docs/diagrams/registers.svg)

### General-Purpose And Special-Purpose Registers

These registers can *partially* be used for holding both data and addresses. They are specified as follows:

- `R0` - `R12`: General-purpose registers  
- `R13` or `SP`<sup>1</sup>: Stack Pointer (cannot be used with `LET`, `WRITE`, or `READ`)  
- `R14` or `RP`<sup>2</sup>: Return Pointer (cannot be used with `LET` or `WRITE`)  
- `R15` or `IP`<sup>3</sup>: Instruction Pointer (cannot be used with `LET` or `WRITE`)  

<sup>1</sup>: Holds the address of the last memory cell used in the stack and is *readonly*.
<sup>2</sup>: Holds the return address and is *readonly*.  
<sup>3</sup>: Holds the address of the next instruction to be executed and is *readonly*.

### Status Register

We support **1** status register, containing information about following **4** flags:

- `N`: **Negative Flag** – Set when a result is negative  
- `Z`: **Zero Flag** – Set when a result is zero  
- `C`: **Carry Flag** – Set when a carry-out occurred in arithmetic operations  
- `O`: **Overflow Flag** – Set when an arithmetic overflow occurs.

**Status Register Binary Encoded:**

This register is encoded as binary with following considerations:
- First **1** Bit is used for `N` and is defined as follows:
  - `0` is used to signalize the positivity of result. 
  - `1` is used to signalize the negativity of result. 
- Next **1** Bit is used for `Z` and is defined as follows:
  - `0` is used to signalize that result is zero. 
  - `1` is used to signalize that result is non-zero.
- Upcomming **1** Bit is used for `C` that refers to carry.
- Following **1** Bit is used for `O` and is defined as follows:
  - `0` is used to signalize non-existence of overflow in result. 
  - `1` is used to signalize occurence of overflow in result.  
- Rest of available Bits are used for `<dont-care>`.

Concretely defined as follows:
```
nzco xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care), `n` (for `N`), and `z` (for `Z`), `c` (for `C`), and `o` (for `O`) all refer to binary digits (`0`/`1`).

<hr>

## Instructions

| Index | Opcode | Name | Reference|
|---|---|--- | ---|
| 0 | 0b00000 | NOPE | [Nope](#nope) |
| 1 | 0b00001 | LET | [LET](#let) |
| 2 | 0b00010 | COPY | [COPY](#copy) |
| 3 | 0b00011 | WRITE | [WRITE](#write) |
| 4 | 0b00100 | READ | [READ](#read) |
| 5 | 0b00101 | PUSH | [PUSH](#push) |
| 6 | 0b00110 | POP | [POP](#pop) |
| 7 | 0b00111 | PEEK | [PEEK](#peek) |
| 8 | 0b01000 | JUMP | [JUMP](#jump) |
| 9 | 0b01001 | EVAL | [EVAL](#eval) |
| 10 | 0b01010 | COMP | [COMP](#comp) |
| 11 | 0b01011 | ADD | [ADD](#add) |
| 12 | 0b01100 | SUB | [SUB](#sub) |
| 13 | 0b01101 | MUL | [MUL](#mul) |
| 14 | 0b01110 | DIV | [DIV](#div) |
| 15 | 0b01111 | NEG | [NEG](#neg) |
| 16 | 0b10000 | OR | [OR](#or) |
| 17 | 0b10001 | AND | [AND](#and) |
| 18 | 0b10010 | XOR | [XOR](#xor) |
| 19 | 0b10011 | SHIFT | [SHIFT](#shift) |
| 20 | 0b10100 | LOG | [LOG](#log) |
| 21 | 0b10101 | CLEAR | [CLEAR](#clear) |
| 22 | 0b10110 | SETPX | [SETPX](#setpx) |

### NOPE

**Opcode**: 0b00000

**Abbreviation Resolution:** No-operation

**Explanation:** Does nothing.

**Syntax:**
```
NOPE
```
**Example:**
```
NOPE
```

**Instruction Binary Encoded:**
```
<opcode><dont-care>
```

With following considerations:
- First **5** Bits are used for `<opcode>`. 
- Rest of available Bits are used for `<dont-care>`.

Concretely defined as follows:
```
0000 0xxx xxxx xxxx xxxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care) refers to a binary digit (`0`/`1`).

<hr>

### LET

**Opcode**: 0b00001

**Abbreviation Resolution:** -

**Explanation:** Assigns a value to a register.

**Syntax:**
```
LET <register> BE <value>
```

**Important:** `<value>` can be both an address and a constant value.

**Examples:**
```
LET R0 BE 0b10
```
```
LET R0 BE 10
```
```
LET R0 BE 010
```
```
LET R0 BE 0x10
```

**Instruction Binary Encoded:**
```
opcode | register-index | value | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used to encode `register-index` given as `<register>`.
- Following **16** Bits will be used to encode `value` given as `<value>`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0000 1rrr rvvv vvvv vvvv vvvv vxxx
```

**Important:**  `x` (for Don't-Care), `v` (for value), and `r` (for register index) all refer to binary digits (`0`/`1`).

<hr>

### COPY

**Opcode**: 0b00010

**Abbreviation Resolution:** -

**Explanation:** Copies content of a register into another one.

**Syntax:**
```
COPY <register> INTO <register>
```

**Examples:**
```
COPY R1 INTO R0
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used to encode `register-index` given as `<register>`.
- Following **4** Bits will be used to encode `register-index` given as `<register>`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0001 0rrr rrrr rrrr rrrr rxxx xxxx
```

**Important:**  `x` (for Don't-Care) and `r` (for register index) both refer to binary digits (`0`/`1`).

<hr>


### WRITE

**Opcode**: 0b00011

**Abbreviation Resolution:** -

**Explanation:** Writes a value (content of a register) at a memory address (content of a register).

**Syntax:**
```
WRITE <register> TO <register>
```

**Example:**
```
WRITE R0 TO R1
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used to encode `register-index` given as `<register>`.
- Following **4** Bits will be used to encode `register-index` given as `<register>`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0001 1rrr rrrr rrrr rxxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care) and `r` (for register index) both refer to binary digits (`0`/`1`).

<hr>

### READ

**Opcode**: 0b00100

**Abbreviation Resolution:** -

**Explanation:** Reads a value stored at a memory address (stored in a register) into a register.

**Syntax:**
```
READ FROM <register> INTO <register>
```
**Examples:**
```
READ FROM R0 INTO R1
```
```
READ FROM 0x1234 INTO R1
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used to encode `register-index` given as `<register>`.
- Following **4** Bits will be used to encode `register-index` given as `<register>`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0010 0rrr rrrr rrrx xxxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care) and `r` (for register index) both refer to binary digits (`0`/`1`).

<hr>

### PUSH

**Opcode**: 0b00101

**Abbreviation Resolution:** -

**Explanation:** Pushes a value (content of a register) on the stack.

**Syntax:**
```
PUSH <register>
```

**Example:**
```
PUSH R0
```

**Instruction Binary Encoded:**
```
opcode | register-index | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0010 1rrr rxxx xxxx xxxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care) and `r` (for register index) both refer to binary digits (`0`/`1`).


<hr>

### POP

**Opcode**: 0b00110

**Abbreviation Resolution:** -

**Explanation:** Pops the value pushed last on the stack into a register.

**Syntax:**
```
POP INTO <register>
```

**Example:**
```
POP INTO R0
```

**Instruction Binary Encoded:**
```
opcode | register-index | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0011 0rrr rxxx xxxx xxxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care) and `r` (for register index) both refer to binary digits (`0`/`1`).


<hr>

### PEEK

**Opcode**: 0b00111

**Abbreviation Resolution:** -

**Explanation:** Peeks the value pushed last on the stack into a register without removing it.

**Syntax:**
```
PEEK INTO <register>
```

**Example:**
```
PEEK INTO R0
```

**Instruction Binary Encoded:**
```
opcode | register-index | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0011 1rrr rxxx xxxx xxxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care) and `r` (for register index) both refer to binary digits (`0`/`1`).

<hr>

### JUMP

**Opcode**: 0b01000

**Abbreviation Resolution:** -

**Explanation:** Jumps to a label (un-) conditionally.

**Important:** Momentary values of status registers will be used to check the condition specified.

**Syntax:**
```
JUMP TO <label> [IF <condition>]
```

With `<label>` defined as follows:
```
<label> = "BACK" | <identifier>
```

**Important:** `BACK` is used to signalize returning back from callee to caller.

And `<condition>` defined as follows:
```
<conditions> = "TRUE" | "EQUAL" | "NOT-EQUAL" | "LESS" | "LESS-EQUAL" | "GREATER" | "GREATER EQUAL"
```

**Important:** `TRUE` is used to signalize an unconditional jump and can be omitted.


**Examples:**
```
JUMP TO end
```
```
JUMP TO label IF LESS
```
```
JUMP TO label IF LESS-EQUAL
```
```
JUMP TO label IF EQUAL
```

**Instruction Binary Encoded:**
```
opcode | address | condition | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **16** Bits will be used To encode `address` given as `<function-label>`.
- Following **3** Bits will be used to encode `condition` as follows:
  - `0b000` for `TRUE` or unconditional jump.
  - `0b001` for `EQUAL`.
  - `0b010` for `NOT-EQUAL`.
  - `0b011` for `LESS`.
  - `0b100` for `LESS-EQUAL`.
  - `0b101` for `GREATER`.
  - `0b110` for `GREATER-EQUAL`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0100 0aaa aaaa aaaa aaaa appp xxxx xxxx
```

**Important:**  `x` (for Don't-Care), `a` (for address), and  `p` (for pre-condition) all refer to binary digits (`0`/`1`).

<hr>

### EVAL

**Opcode**: 0b01001

**Abbreviation Resolution:** Evaluate

**Explanation:** Evaluates (i. e. executes, invokes, applies, or simply calls) a function.

**Syntax:**
```
EVAL <function-label>
```

**Example:**
```
EVAL factorial
```

**Instruction Binary Encoded:**
```
opcode | address | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **16** Bits will be used To encode `address` given as `<function-label>`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0100 1aaa aaaa aaaa aaaa axxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care) and `a` (for label address) both refer to binary digits (`0`/`1`).


<hr>


### COMP

**Opcode**: 0b01010

**Abbreviation Resolution:** Compare

**Explanation:** Compares two values (contents of registers) and updates status registers.

**Syntax:**
```
COMP <register> WITH <register>
```

**Example:**
```
COMP R0 WITH R1
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Following **4** Bits will be used To encode `register-index` given as `<register>`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0101 0r rrrr rrrx xxxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care) and `r` (for register index) both refer to binary digits (`0`/`1`).

<hr>

### ADD

**Opcode**: 0b01011

**Abbreviation Resolution:** Addition

**Explanation:** Adds two values (contents of registers) with following considerations:
1. Status registers will not get updated by default. You can still use `WITH UPDATE` to update the status registers.
2. Carry is not considered by default. To consider it, please use `WITH CARRY`.

**Syntax:**
```
ADD <register> TO <register> INTO <register> [WITH CARRY] [WITH UPDATE]
```

**Examples:**
```
ADD R0 TO R1 INTO R2
```
```
ADD R0 TO R1 INTO R2 WITH UPDATE
```
```
ADD R0 TO R1 INTO R2 WITH CARRY WITH UPDATE 
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | register-index | mode | update | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Following **4** Bits will be used To encode `register-index` given as `<register>`.
- Upcoming **4** Bits will be used To encode `register-index` given as `<register>`.
- Another **1** Bit is used for `update` defined as follows:
  - `0` is used to signalize non-existence of `WITH UPDATE`.
  - `1` is used to signalize occurence of `WITH UPDATE`.
- Another **1** Bit is used for `mode` defined as follows:
  - `0` is used to signalize non-existence of `WITH CARRY`.
  - `1` is used to signalize occurence of `WITH CARRY`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0101 1rrr rrrr rrrr rmux xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care), `r` (for register index), `m` (for addition mode), and `u` (for status flags update) all refer to binary digits (`0`/`1`).

<hr>

### SUB

**Opcode**: 0b01100

**Abbreviation Resolution:** Subtraction

**Explanation:** Subtracts two values (contents of registers). Status registers will not get updated by default (use `WITH UPDATE` to update the status registers).

**Syntax:**
```
SUB <register> FROM <register> INTO <register> [WITH UPDATE]
```

**Examples:**
```
SUB R0 FROM R1 INTO R2
```
```
SUB R0 FROM R1 INTO R2 WITH UPDATE
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | register-index | update | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Following **4** Bits will be used To encode `register-index` given as `<register>`.
- Upcoming **4** Bits will be used To encode `register-index` given as `<register>`.
- Another **1** Bit is used for `update` defined as follows:
  - `0` is used to signalize non-existence of `WITH UPDATE`.
  - `1` is used to signalize occurence of `WITH UPDATE`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0110 0rrr rrrr rrrr ruxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care), `r` (for register index), and `u` (for status flags update) all refer to binary digits (`0`/`1`).

<hr>

### MUL

**Opcode**: 0b01101

**Abbreviation Resolution:** Multiplication

**Explanation:** Multiplies two values with following considerations:
1. Status registers will not get updated by default (use `WITH UPDATE` to update the status registers).
2. If registers are used as values, then content of the registers will be interpreted as values.

**Syntax:**
```
MUL <register> WITH <register> INTO <register> [WITH UPDATE]
```

**Examples:**
```
MUL 15 WITH 10 INTO R0
```
```
MUL R0 WITH 0b10 INTO R1 
```
```
MUL R0 WITH R1 INTO R2
```
```
MUL R0 WITH R1 INTO R2 WITH UPDATE
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | register-index | update | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Following **4** Bits will be used To encode `register-index` given as `<register>`.
- Upcoming **4** Bits will be used To encode `register-index` given as `<register>`.
- Another **1** Bit is used for `update` defined as follows:
  - `0` is used to signalize non-existence of `WITH UPDATE`.
  - `1` is used to signalize occurence of `WITH UPDATE`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0110 1rrr rrrr rrrr ruxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care), `r` (for register index), and `u` (for status flags update) all refer to binary digits (`0`/`1`).

<hr>

### DIV

**Opcode**: 0b01110

**Abbreviation Resolution:** Division

**Explanation:** Divides two values (contents of registers) with following considerations:
1. Status registers will not get updated by default (use `WITH UPDATE` to update the status registers).
2. There is currently to no support for floating point numbers. Hence, the result will be interpreted as an integer.

**Syntax:**
```
DIV <register> BY <register> INTO <register> [WITH UPDATE]
```

**Examples:**
```
DIV R0 BY R1 INTO R2
```
```
DIV R0 BY R1 INTO R2 WITH UPDATE
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | register-index | update | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Following **4** Bits will be used To encode `register-index` given as `<register>`.
- Upcoming **4** Bits will be used To encode `register-index` given as `<register>`.
- Another **1** Bit is used for `update` defined as follows:
  - `0` is used to signalize non-existence of `WITH UPDATE`.
  - `1` is used to signalize occurence of `WITH UPDATE`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0111 0rrr rrrr rrrr ruxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care), `r` (for register index), and `u` (for status flags update) all refer to binary digits (`0`/`1`).

<hr>

### NEG

**Opcode**: 0b01111

**Abbreviation Resolution:** Negation

**Explanation:** Negates a value (content of a register). Status registers will not get updated by default (use `WITH UPDATE` to update the status registers).

**Syntax:**
```
NEG <register> INTO <register> [WITH UPDATE]
```

**Examples:**
```
NEG R0 INTO R1
```
```
NEG R0 INTO R1 WITH UPDATE
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Following **4** Bits will be used To encode `register-index` given as `<register>`.
- Upcoming **1** Bit is used for `update` defined as follows:
  - `0` is used to signalize non-existence of `WITH UPDATE`.
  - `1` is used to signalize occurence of `WITH UPDATE`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
0111 1rrr rrrr ruxx xxxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care), `r` (for register index), and `u` (for status flags update) all refer to binary digits (`0`/`1`).

<hr>

### OR

**Opcode**: 0b10000

**Abbreviation Resolution:** -

**Explanation:** Performs bitwise `OR` between two values (contents of registers). Status registers will not get updated by default (use `WITH UPDATE` to update the status registers).

**Syntax:**
```
OR <register> WITH <register> INTO <register> [WITH UPDATE]
```

**Examples:**
```
OR R0 WITH R1 INTO R2
```
```
OR R0 WITH R1 INTO R2 WITH UPDATE
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | register-index | update | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Following **4** Bits will be used To encode `register-index` given as `<register>`.
- Upcoming **4** Bits will be used To encode `register-index` given as `<register>`.
- Another **1** Bit is used for `update` defined as follows:
  - `0` is used to signalize non-existence of `WITH UPDATE`.
  - `1` is used to signalize occurence of `WITH UPDATE`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
1000 0rrr rrrr rrrr ruxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care), `r` (for register index), and `u` (for status flags update) all refer to binary digits (`0`/`1`).

<hr>

### AND

**Opcode**: 0b10001

**Abbreviation Resolution:** -

**Explanation:** Performs bitwise `AND` between two values (contents of registers). Status registers will not get updated by default (use `WITH UPDATE` to update the status registers).

**Syntax:**
```
AND <register> WITH <register> INTO <register> [WITH UPDATE]
```

**Examples:**
```
AND R0 WITH R1 INTO R2
```
```
AND R0 WITH R1 INTO R2 WITH UPDATE
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | register-index | update | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Following **4** Bits will be used To encode `register-index` given as `<register>`.
- Upcoming **4** Bits will be used To encode `register-index` given as `<register>`.
- Another **1** Bit is used for `update` defined as follows:
  - `0` is used to signalize non-existence of `WITH UPDATE`.
  - `1` is used to signalize occurence of `WITH UPDATE`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
1000 1rrr rrrr rrrr ruxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care), `r` (for register index), and `u` (for status flags update) all refer to binary digits (`0`/`1`).

<hr>

### XOR

**Opcode**: 0b10010

**Abbreviation Resolution:** Exclusive Or

**Explanation:** Performs bitwise `XOR` between two values (contents of registers). Status registers will not get updated by default (use `WITH UPDATE` to update the status registers).

**Syntax:**
```
XOR <register> WITH <register> INTO <register> [WITH UPDATE]
```

**Examples:**
```
XOR R0 WITH R1 INTO R2
```
```
XOR R0 WITH R1 INTO R2 WITH UPDATE
```

**Instruction Binary Encoded:**
```
opcode | register-index | register-index | register-index | update | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode `register-index` given as `<register>`.
- Following **4** Bits will be used To encode `register-index` given as `<register>`.
- Upcoming **4** Bits will be used To encode `register-index` given as `<register>`.
- Another **1** Bit is used for `update` defined as follows:
  - `0` is used to signalize non-existence of `WITH UPDATE`.
  - `1` is used to signalize occurence of `WITH UPDATE`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
1001 0rrr rrrr rrrr ruxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care), `r` (for register index), and `u` (for status flags update) all refer to binary digits (`0`/`1`).


<hr>

### SHIFT

**Opcode**: 0b10011

**Abbreviation Resolution:** -

**Explanation:** Performs arithmetic/logical left/right shift on a value (content of the registers) by another value with following considerations:
1. Status registers will not get updated by default (use `WITH UPDATE` to update the status registers).
2. There is no difference between `SHIFT ARITHEMTIC LEFT` and `SHIFT LOGICAL LEFT`.

**Syntax:**
```
SHIFT <shift-type> <shift-direction> <register> BY <value> INTO <register> [WITH UPDATE]
```

with `<shift-type>` being defined as follows:
```
<shift-type> = ARITHMETICALLY | LOGICALLY
```

And `<shift-direction>` specified as follows:
```
<shift-type> = LEFT | RIGHT
```

**Examples:**
```
SHIFT LOGICAL LEFT R0 BY 10 INTO R0
```
```
SHIFT ARITHMETIC RIGHT R0 BY R1 INTO R2
```
```
SHIFT ARITHMETIC RIGHT R1 BY R2 INTO R0 WITH UPDATE
```

**Instruction Binary Encoded:**
```
opcode | shift-type | shift-direction | register-index | offset |update | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **1** Bits will be used for `shift-type` defined as follows:
  - `0` is used to signalize `ARITHMETICALLY`.
  - `1` is used to signalize `LOGICALLY`.
- Following **1** Bits will be used for `shift-direction` defined as follows:
  - `0` is used to signalize `LEFT`.
  - `1` is used to signalize `RIGHT`.
- Upcoming **4** Bits will be used To encode `register-index` given as `<register>`.
- Subsequent **5** Bits will be used To encode `offset` given as `<value>`.
- Another **4** Bits will be used To encode `register-index` given as `<register>`.
- Also **1** Bit is used for `update` defined as follows:
  - `0` is used to signalize non-existence of `WITH UPDATE`.
  - `1` is used to signalize occurence of `WITH UPDATE`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
1001 1tdr rrrc cccc rrrr uxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care), `c` (for constant value as offset), `r` (for register index), `t` (for shift type), `d` (for shift direction), and `u` (for status flags update) all refer to binary digits (`0`/`1`).

<hr>

### LOG

**Opcode**: 0b10100

**Abbreviation Resolution:** -

**Explanation:** Writes/logs a value (content of the register provided) to the console.

**Syntax:**
```
LOG <register>
```

**Example:**
```
LOG R0
```

**Instruction Binary Encoded:**
```
opcode | register-index | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`.
- Next **4** Bits will be used To encode a register index given as `<register>`.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
1010 0rrr rxxx xxxx xxxx xxxx xxxx xxxx
```

**Important:**  `x` (for Don't-Care) and `r` (for register index) both refer to binary digits (`0`/`1`).

<hr>

### CLEAR

**Opcode**: 0b10101

**Abbreviation Resolution:** -

**Explanation:** Clears the screen.

**Syntax:**
```
CLEAR
```

**Example:**
```
CLEAR
```

**Instruction Binary Encoded:**
```
opcode | dont-care
```

Concretely defined as follows:
```
1010 1xxx xxxx xxxx xxxx xxxx xxxx xxxx
```
**Important:** `x` (for Don't-Care) refers to binary digits (`0`/`1`).

<hr>

### SETPX

**Opcode**: 0b10110

**Abbreviation Resolution:** Set Pixel

**Explanation:** Sets a pixel on the screen with following considerations:
1. First coordinate represents the x-coordinate.
2. Second coordinate represents the y-coordinate.

**Syntax:**
```
SETPX <coordinate> <coordinate>
```

**Example:**
```
SETPX 4 9
```

**Instruction Binary Encoded:**
```
opcode | constant-value | constant-value | dont-care
```

With following considerations:
- First **5** Bits are used for `opcode`. 
- Following **8** Bits are used for `constant-value` representing the x-coordinate and ranging between **0** and **255**.
- Following **8** Bits are used for the `constant-value` representing the y-coordinate and ranging between **0** and **255**.
- Rest of available Bits are used for `dont-care`.

Concretely defined as follows:
```
1011 0ccc cccc cccc cccc cxxx xxxx xxxx
```
**Important:**  `x` (for Don't-Care) and `c` (for constant value) both refer to binary digits (`0`/`1`).

<hr>