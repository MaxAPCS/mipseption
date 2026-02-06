.data
prompt: .asciiz "How many fibonacci numbers should be generated? "

.text
main:
  li $v0, 4
  la $a0, prompt
  la $a1, prompt
  addiu $a1, $a1, 48 # pointer addition like 48(prompt) is not supported by the assembler
  syscall
  li $v0, 5
  syscall
  blez $v0, main
  move $t2, $v0

  li $t0, 0
  li $t1, 1
  li $v0, 1

next:
  add $a0, $t0, $t1
  li $v0, 1
  syscall
  move $t0, $t1
  move $t1, $a0
  addi $t2, $t2, -1
  bgtz $t2, next

end:
  li $v0, 10
  syscall
