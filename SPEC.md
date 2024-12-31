# RV64I
This is a Spec-document intended to make it easy to understand the implementation in this repository, it generally explains the inner workings of the relevant this implementation of the Risc-V specification.

## Defintions

| Term | Meaning |
| ---- | ------- |
| XLEN | Size of the general registers (in the case for this implementation it's basically an alias for 64) |
| quadword (of memory) | 128 bits (of memory) |
| doubleword (of memory) | 64 bits (of memory) |
| word (of memory) | 32 bits (of memory) |
| halfword (of memory) | 16 bits (of memory) |
| hart | hardware thread |

## Base RV32I
Instruction format types
![image](https://github.com/user-attachments/assets/84959e68-486c-4e30-b662-011dba15cf40)

![image](https://github.com/user-attachments/assets/dd3f1743-16d4-4995-85a2-0c31aa02e0b7)
![image](https://github.com/user-attachments/assets/646b9a7b-2404-428b-b49c-8aabda9d681b)

![image](https://github.com/user-attachments/assets/8aa173d4-d4f0-4d32-b5ed-3aa4096f882e)


## M Extention (Multiplication & Division)
![image](https://github.com/user-attachments/assets/4febe4bf-791b-43ee-bfb1-1cf0efbc8810)
![image](https://github.com/user-attachments/assets/3d8e2204-9005-4d6e-9a30-316b967bf131)


## A Extention (Atomic Instructions)
![image](https://github.com/user-attachments/assets/9310543e-baff-4aae-9b64-82c475e09716)
