section imports
    printString

section exports
    main

section code
.print
    stack_push "Hello World" 
    ext_call printString
    stack_pop
    ret

.main
    call print
    ret
