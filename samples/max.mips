00400000: <main> ; <input:0> main:
00400000: 20020005 ; <input:1> li $v0, 5
00400004: 0000000c ; <input:2> syscall
00400008: 00022020 ; <input:3> move $a0, $v0
0040000c: 20020005 ; <input:4> li $v0, 5
00400010: 0000000c ; <input:5> syscall
00400014: 0044402a ; <input:6> slt $t0, $v0, $a0
00400018: 15000001 ; <input:7> bne $t0, $zero, fir
0040001c: 00022020 ; <input:8> move $a0, $v0
00400020: <fir> ; <input:9> fir:
00400020: 20020005 ; <input:10> li $v0, 5
00400024: 0000000c ; <input:11> syscall
00400028: 0044402a ; <input:12> slt $t0, $v0, $a0
0040002c: 15000001 ; <input:13> bne $t0, $zero, sec
00400030: 00022020 ; <input:14> move $a0, $v0
00400034: <sec> ; <input:15> sec:
00400034: 00044020 ; <input:16> move $t0, $a0
00400038: 20020004 ; <input:17> li $v0, 4
0040003c: 3c041001 ; lui $4, 4097 # lui $a0, 4097
00400040: 3c051001 ; lui $5, 4097 #
00400044: 34a50010 ; ori $5, $5, 16 #
00400048: 0000000c ; <input:21> syscall
0040004c: 20020001 ; <input:22> li $v0, 1
00400050: 00082020 ; <input:23> move $a0, $t0
00400054: 0000000c ; <input:24> syscall
00400058: 2002000a ; <input:25> li $v0, 10
0040005c: 0000000c ; <input:26> syscall
;
; DATA IN MEMORY
; res
10010000: 0a477265 ; \nGre
10010004: 61746573 ; ates
10010008: 74204e75 ; t Nu
1001000c: 6d626572 ; mber
10010010: 3a200000 ; : 


; Created by frankensteining https://assembler.kwalsh.org/mips/ and https://alanhogan.com/asu/assembler.php, then correcting the memory model.