# mo65x-rs
stands for My Own 65 eXpandable emulator written in Rust programming language. The project has educational motivation and is a genuine project based on technical specifications of 6502/6510 family microprocessors. See the screenshot of assembler mode running slightly modified program "demoscene.asm" from http://www.6502asm.com

![Alt text](https://github.com/rjaremczak/mo65x-rs/blob/master/img/mo65x-rs.png "Interactive console view")

Project is based on original version written in C++, available here https://github.com/rjaremczak/mo65x-rs. This version is a terminal application with extra window for showing frame-buffer's content. Functionality is almost identical with the original version. Several functions like assembler or disassembler have been implemented in command line interface. Please launch the app with --help to see what are the options.

## Console mode
When started in console mode (interactive mode) all options available through GUI in original version are now implemented in command line.
In short these are:

* `l <address> <file-path>` - load binary file at given address
* `d <address>` - set starting address of disassembly view
* `m <address>` - set starting address of memody dump view
* `reset` - trigger hardware reset
* `nmi` - trigger NMI request
* `irq` - trigger IRQ request
* `sb <address> <byte-value>` - set byte at address to given byte value
* `sw <address> <word-value>` - set word at address to given 16-bit value
* `pc=<word-value>` - set PC register to given 16-bit value
* `sp=<byte-value>` - set LSB of SP register to given 8-bit value
* `a=<byte-value>` - set Accumulator to given 8-bit value
* `x=<byte-value>` - set X register to given 8-bit value
* `y=<byte-value>` - set Y register to given 8-bit value
* `n=<bit-value>` - set Negative flag
* `v=<bit-value>` - set Overflog flag
* `d=<bit-value>` - set Decimal Mode flag
* `i=<bit-value>` - set Interrupt flag
* `z=<bit-value>` - set Zero flag
* `c=<bit-value>` - set Carry flag

All expected and displayed values are hexadecimal, except the binary values of the flags.
Press `ESC` to quit the emulator.

## License
All design and code so far is written entirely by Robert Jaremczak (robert@mindpart.com) and is licensed as GPL.

GPL in short means this: feel free to use as long as sources of your own work remain public. Don't forget to mention me, as an author of this code :-)
