
# SusVM

## About
SusVM is a virtual machine written in rust with x86_64 like instructions


## Manual

1. Registers
###rax: General purpose
###rcx: Counter register(Recomended to be used as counter in loops)
###rdx: General purpose
###rbx: General purpose

2. Instructions

2.1 Data movement
###mov: Moves a value from a register to another register (SYNTAX): ```mov <reg> <reg>```
###push: Moves a value or register onto the stack (SYNTAX): ```push <reg/value>```
###pop: Pops a value from the stack into a register (SYNTAX): ```pop <reg>```

2.2 Binary operations
###add: Add two registers and put the value into the first register (SYNTAX): ```add <reg> <reg>```
###sub: Subtract two registers and put the value into the first register (SYNTAX): ```sub <reg> <reg>```
###mul: Multiply two registers and put the value into the first register (SYNTAX): ```mul <reg> <reg>```

2.3 Jumping
###jmp: Set the intruction pointer to the instruction specified (SYNTAX): ```jmp <inst>```
###je: Set the instruction pointer to the instruction specified if 1[Code for equals] is on the stack (SYNTAX): ```je <inst>```
###jne: Set the instruction pointer to the instruction specified if 1[Code for equals] is not on the stack (SYNTAX): ```jne <inst>```
###js: Set the instruction pointer to the instruction specified if 3[Code for smaller] is on the stack (SYNTAX): ```js <inst>```
###jb: Set the instruction pointer to the instruction specified if 2[Code for bigger] is on the stack (SYNTAX): ```jb <inst>```

2.4 Comparing registers
###cmp: compare the two registers specified and push the status code into the stack (SYNTAX): ```cmp <reg> <reg>```


## Credits
Cheat sheet for x86_64 asm:  https://cs.brown.edu/courses/cs033/docs/guides/x64_cheatsheet.pdf


