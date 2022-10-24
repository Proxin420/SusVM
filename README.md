
# SusVM

## About
SusVM is a virtual machine written in rust with x86_64 like instructions


## Manual

1. Registers<br>
rax: General purpose<br>
rcx: Counter register(Recomended to be used as counter in loops)<br>
rdx: General purpose<br>
rbx: General purpose<br>
<br>
2. Instructions<br>
<br>
2.1 Data movement<br>
mov: Moves a value from a register to another register (SYNTAX): mov reg reg<br>
push: Moves a value or register onto the stack (SYNTAX): push reg/value<br>
pop: Pops a value from the stack into a register (SYNTAX): pop reg<br>
<br>
2.2 Binary operations<br>
add: Add two registers and put the value into the first register (SYNTAX): add reg reg<br>
sub: Subtract two registers and put the value into the first register (SYNTAX): sub reg reg<br>
mul: Multiply two registers and put the value into the first register (SYNTAX): mul reg reg<br>
<br>
2.3 Jumping<br>
jmp: Set the intruction pointer to the instruction specified (SYNTAX): jmp inst<br>
je: Set the instruction pointer to the instruction specified if 1[Code for equals] is on the stack (SYNTAX): je inst<br>
jne: Set the instruction pointer to the instruction specified if 1[Code for equals] is not on the stack (SYNTAX): jne inst<br>
js: Set the instruction pointer to the instruction specified if 3[Code for smaller] is on the stack (SYNTAX): js inst<br>
jb: Set the instruction pointer to the instruction specified if 2[Code for bigger] is on the stack (SYNTAX): jb inst<br>
<br>
2.4 Comparing registers
cmp: compare the two registers specified and push the status code into the stack (SYNTAX): cmp reg reg<br>

## Example
Fibonacci number: <br>
```
// starting values
push 0
push 1

// how to many times to loop
push 11
pop rax

// loop counter
push 0
pop rcx

// loop
pop rdx
pop rbx

// copy rdx into rsi
push rdx
pop rsi

add rbx rdx
push rbx
push rsi
push rdx

// add 1 to the loop counter
push 1
pop rsi
add rsi rcx

// compare counter to how many times to loop
cmp rcx rax

// jump if counter is smaller than how many times to loop
js 5
```


## Credits
Cheat sheet for x86_64 asm:  https://cs.brown.edu/courses/cs033/docs/guides/x64_cheatsheet.pdf


