.global main
.text
main:
sub $16, %rsp
movl $10, -8(%rsp)
cmpl $5, -8(%rsp)
jle main_0
movl $5, -8(%rsp)
main_0:
mov $format, %rdi
mov -8(%rsp), %rsi
xor %rax, %rax
call printf
mov $0, %rdi
call exit
format: .asciz "Value: %d\n"
