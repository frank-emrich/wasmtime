;;! target = "x86_64"

(module
    (func (result f64)
        (f64.const -1.1)
        (f64.const 2.2)
        (f64.copysign)
    )
)
;;      	 55                   	push	rbp
;;      	 4889e5               	mov	rbp, rsp
;;      	 4d8b5e08             	mov	r11, qword ptr [r14 + 8]
;;      	 4d8b1b               	mov	r11, qword ptr [r11]
;;      	 4981c308000000       	add	r11, 8
;;      	 4939e3               	cmp	r11, rsp
;;      	 0f8744000000         	ja	0x5f
;;   1b:	 4883ec08             	sub	rsp, 8
;;      	 4c893424             	mov	qword ptr [rsp], r14
;;      	 f20f10053d000000     	movsd	xmm0, qword ptr [rip + 0x3d]
;;      	 f20f100d3d000000     	movsd	xmm1, qword ptr [rip + 0x3d]
;;      	 49bb0000000000000080 	
;; 				movabs	r11, 0x8000000000000000
;;      	 664d0f6efb           	movq	xmm15, r11
;;      	 66410f54c7           	andpd	xmm0, xmm15
;;      	 66440f55f9           	andnpd	xmm15, xmm1
;;      	 66410f28cf           	movapd	xmm1, xmm15
;;      	 660f56c8             	orpd	xmm1, xmm0
;;      	 660f28c1             	movapd	xmm0, xmm1
;;      	 4883c408             	add	rsp, 8
;;      	 5d                   	pop	rbp
;;      	 c3                   	ret	
;;   5f:	 0f0b                 	ud2	
;;   61:	 0000                 	add	byte ptr [rax], al
;;   63:	 0000                 	add	byte ptr [rax], al
;;   65:	 0000                 	add	byte ptr [rax], al
;;   67:	 009a99999999         	add	byte ptr [rdx - 0x66666667], bl
;;   6d:	 99                   	cdq	
;;   6e:	 01409a               	add	dword ptr [rax - 0x66], eax
;;   71:	 99                   	cdq	
;;   72:	 99                   	cdq	
;;   73:	 99                   	cdq	
;;   74:	 99                   	cdq	
;;   75:	 99                   	cdq	
;;   76:	 f1                   	int1	
