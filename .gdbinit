target remote :3333
load
monitor arm semihosting enable
b _start
break main
break minimal.rs:68
break minimal.rs:104
break minimal.rs:112
break minimal.rs:191
break minimal.rs:299
break checkpoint/mod.rs:475
// break minimal.rs:31
// break minimal.rs:38
// break minimal.rs:51s