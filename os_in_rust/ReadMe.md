# LInk to Post
https://os.phil-opp.com/minimal-rust-kernel/

#Alternatively, we can compile it for the host system by passing additional linker arguments:
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"