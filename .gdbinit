target remote :3333
load
monitor arm semihosting enable
break main
break minimal.rs:750
break minimal.rs:882
break minimal.rs:357
break minimal.rs:325
// break minimal.rs:38
// break minimal.rs:51s