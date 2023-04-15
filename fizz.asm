# FizzBuzz
# This program implements the FizzBuzz challenge. It runs for 1 million
# iterations.
#
# Compile this using the "rasm" program included in the Resurgence SDK.

section constants [
    1,
    1000000,
    0,
    "Fizz",
    "Buzz",
    3,
    5,
]
    

section aliases
    one => const[0]
    zero => const[2]
    three => const[5]
    five => const[6]
    
    totalLoops => const[1]
    
    fizz => const[3]
    buzz => const[4]
    
    i => local[0]
    m => local[1]
    n => local[2]
    stackreg => local[3]

section imports
    printNumber
    printString

section exports
    main

section code
.printFizz
    stack_push fizz
    ext_call printString
    stack_pop
    ret
.printBuzz
    stack_push buzz
    ext_call printString
    stack_pop
    ret

.main
    alloc 4
    cpy i, one # dst, src

.loopstart
    # print the number
    # stackpush consumes the register, so we copy it to an otherwise unused register
    cpy stackreg, i
    stack_push stackreg
    ext_call printNumber
    stack_pop

    # do the math
    mod m, i, three # m = i % 3
    mod n, i, five # n = i % 5

    # Check fizz and buzz and print if necessary
    not_equal m, zero
    call printFizz
    not_equal n, zero
    call printBuzz
    
    # increment i and restart loop
    add i, i, one
    equal i, totalLoops
    jump loopstart # not equal, restart loop
    ret # exit program
