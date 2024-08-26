target extended-remote :3333
load
monitor arm semihosting enable
break main
break minimal.rs:750
break minimal.rs:930
break minimal.rs:423
break minimal.rs:982
// break minimal.rs:38
// break minimal.rs:51s