section constants
    0 => "Hello World"
    

section aliases
    hello_world => const[0]

section imports
    printString

section exports
    main

section code
.print
    stack_push hello_world
    ext_call printString
    stack_pop
    ret

.main
    call print
    ret
