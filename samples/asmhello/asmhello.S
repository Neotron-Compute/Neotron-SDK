/**
 * Basic ARM Unified Assembly Language sample which runs on Neotron OS.
 */

.syntax unified

//
// Functions (.text)
//

.text

// Entry point for our application.
//
// * r0 will contain a pointer to the syscall table.
// * The write function is the third entry (offset 8)
//   and takes the arguments r0=handle, r1=pointer, r2=length
.thumb_func
.global app_entry
.func app_entry
app_entry:
    // "A subroutine must preserve the contents of the registers r4-r11
    // and SP" - and we only use R4
    push {r4, lr}
    // Fetch write function address
    ldr r3, [r0, #8]
    // Set up file handle
    movs r0, #1
    // Set up data pointer
    ldr r1, =message
    // Set up data length
    ldr r4, =message_len
    ldr r2, [r4]
    // Call write function
    blx r3
    // Set return value
    movs r0, #0
    // Exit
    pop {r4, pc}
.endfunc

//
// Read Only Data (.rodata)
//

.section .rodata

// The message we want to print
.type message,%object
message:
.ascii "Hello world!\n"

// The length of the string with the label `message`
//
// Must come immediately after `message:`
.type message_len,%object
message_len:
.long . - message  

// End of file
