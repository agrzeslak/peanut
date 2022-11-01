# Ideas

```text
Input:        MOV EAX   EBX
Check valid:  MOV r1/m, r2/m/#n
```

1. Match instruction, if none match it's invalid.
2. There may be multiple matching instructions, but with different op codes and
   operand types.
3. For each matching instruction, check whether the provided operands are valid.
4. If any of them are, success. If not, failure.
