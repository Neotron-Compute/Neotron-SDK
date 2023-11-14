.syntax unified
.thumb
.cpu cortex-m0plus
.global app_entry
.thumb_func

.text
.section ".text"

// r0 will contain a pointer to the syscall table
// the write function is the third entry (offset 8)
// and takes the arguments r0=handle, r1=pointer, r2=length
app_entry:
    // Save registers
    push {r0, r1, r2, lr}
    // Fetch write function address
    ldr r3, [r0, #8]
    // Set up data length
    movs r2, #13
    // Set up file handle
    movs r0, #1
    // Set up data pointer
    ldr r1, =message
    // Call write function
    blx r3
    // Set return value
    movs r0, #0
    // Exit
    pop {r1, r2, r3, pc}

.data
.section ".rodata"

message:
.ascii "Hello world!\n"
