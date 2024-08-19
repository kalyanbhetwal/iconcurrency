target extended-remote :3333
load
monitor arm semihosting enable
break main
break minimal.rs:750
break minimal.rs:886
break minimal.rs:357
break minimal.rs:900
// break minimal.rs:38
// break minimal.rs:51s