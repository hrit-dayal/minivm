# MiniVM: Infix to Stack Code Translation

This document explains how ordinary infix expressions are translated into stack-machine instructions used by MiniVM.

---

# Stack Machine Rule

For a binary operation:

```
a op b
```

Push the left operand first, then the right operand, then perform the operation.

```
PUSH a
PUSH b
OP
```

The VM executes:

```
b = pop()
a = pop()
push(a op b)
```

---

# Basic Operations

## Addition

Infix:

```
a + b
```

Stack Code:

```
a b ADD
```

Assembly:

```asm
PUSH a
PUSH b
ADD
```

---

## Subtraction

Infix:

```
a - b
```

Stack Code:

```
a b SUB
```

Assembly:

```asm
PUSH a
PUSH b
SUB
```

---

## Multiplication

Infix:

```
a * b
```

Stack Code:

```
a b MUL
```

Assembly:

```asm
PUSH a
PUSH b
MUL
```

---

## Division

Infix:

```
a / b
```

Stack Code:

```
a b DIV
```

Assembly:

```asm
PUSH a
PUSH b
DIV
```

---

## Modulo

Infix:

```
a % b
```

Stack Code:

```
a b MOD
```

Assembly:

```asm
PUSH a
PUSH b
MOD
```

---

## Unary Minus

Infix:

```
-a
```

Stack Code:

```
a NEG
```

Assembly:

```asm
PUSH a
NEG
```

---

# Compound Expressions

## Example 1

Infix:

```
(7 + 3)
```

Stack Code:

```
7 3 ADD
```

Assembly:

```asm
PUSH 7
PUSH 3
ADD
```

---

## Example 2

Infix:

```
(7 + 3) * 5
```

Stack Code:

```
7 3 ADD 5 MUL
```

Assembly:

```asm
PUSH 7
PUSH 3
ADD

PUSH 5
MUL
```

---

## Example 3

Infix:

```
(7 + 3) * (9 - 4)
```

Stack Code:

```
7 3 ADD 9 4 SUB MUL
```

Assembly:

```asm
PUSH 7
PUSH 3
ADD

PUSH 9
PUSH 4
SUB

MUL
```

---

## Example 4

Infix:

```
((7 + 3) * (9 - 4)) / 5
```

Stack Code:

```
7 3 ADD 9 4 SUB MUL 5 DIV
```

Assembly:

```asm
PUSH 7
PUSH 3
ADD

PUSH 9
PUSH 4
SUB

MUL

PUSH 5
DIV
```

---

# Horner Polynomial Example

Polynomial:

```
3x³ + 2x² + 5x + 7
```

Horner Form:

```
((3x + 2)x + 5)x + 7
```

For x = 11

Stack Code:

```
3 x MUL
2 ADD
x MUL
5 ADD
x MUL
7 ADD
```

Assembly:

```asm
PUSH 11
STORE 0

PUSH 3

LOAD 0
MUL

PUSH 2
ADD

LOAD 0
MUL

PUSH 5
ADD

LOAD 0
MUL

PUSH 7
ADD

PRINT
```

---

# Celsius to Fahrenheit

Formula:

```
F = C × 9 / 5 + 32
```

For C = 100

Stack Code:

```
100 9 MUL 5 DIV 32 ADD
```

Assembly:

```asm
PUSH 100

PUSH 9
MUL

PUSH 5
DIV

PUSH 32
ADD

PRINT
```

---

# a² + b²

Expression:

```
a² + b²
```

For:

```
a = 12
b = 35
```

Stack Code:

```
a a MUL
b b MUL
ADD
```

Assembly:

```asm
PUSH 12
STORE 0

PUSH 35
STORE 1

LOAD 0
DUP
MUL

LOAD 1
DUP
MUL

ADD

PRINT
```

---

# Digits of 9274

Expression:

```
9274
```

Digit Extraction:

```
9274 / 1000
(9274 / 100) % 10
(9274 / 10) % 10
9274 % 10
```

Stack Code:

```
9274 1000 DIV
9274 100 DIV 10 MOD
9274 10 DIV 10 MOD
9274 10 MOD
```

Assembly:

```asm
PUSH 9274
STORE 0

LOAD 0
PUSH 1000
DIV
PRINT

LOAD 0
PUSH 100
DIV
PUSH 10
MOD
PRINT

LOAD 0
PUSH 10
DIV
PUSH 10
MOD
PRINT

LOAD 0
PUSH 10
MOD
PRINT
```

---

# General Translation Algorithm

Given an infix expression:

```
(A op B)
```

1. Translate A.
2. Translate B.
3. Emit the operator.

Example:

```
(A + B) * (C - D)
```

becomes:

```
A B ADD
C D SUB
MUL
```
