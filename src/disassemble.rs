use crate::byteutil;

fn print_opcode(index: usize, first: u8, second: u8) {
    print!("{:04X} | ", index);
    match first {
        0x00 => {
            match second {
                0xE0 => {
                    print!("rsd")
                },

                0xEE => {
                    print!("ret");
                },

                _ => {
                    print!("nop");
                }
            }
        },

        0x10..=0x1F => {
            print!("jmp 0x{:03X}", byteutil::get_3nyb_addr(first, second))
        },

        0x20..=0x2F => {
            print!("call 0x{:03X}", byteutil::get_3nyb_addr(first, second))
        },

        0x30..=0x3F => {
            print!("se V{:01X} {}", byteutil::get_low_nyb(first), second)
        },

        0x40..=0x4F => {
            print!("sne V{:01X} {}", byteutil::get_low_nyb(first), second)
        },

        0x50..=0x5F => {
            print!("se V{:01X} V{:01X}", byteutil::get_low_nyb(first), byteutil::get_high_nyb(second))
        },

        0x60..=0x6F => {
            print!("set V{:01X} {}", byteutil::get_low_nyb(first), second)
        },

        0x70..=0x7F => {
            print!("add V{:01X} {}", byteutil::get_low_nyb(first), second)
        },

        0x80..=0x8F => {
            match byteutil::get_low_nyb(second){
                0x00 => {
                    print!("set V{:01X} V{:01X}", byteutil::get_low_nyb(first), byteutil::get_high_nyb(second))
                }

                0x01 => {
                    print!("or V{:01X} V{:01X}", byteutil::get_low_nyb(first), byteutil::get_high_nyb(second))
                }

                0x02 => {
                    print!("and V{:01X} V{:01X}", byteutil::get_low_nyb(first), byteutil::get_high_nyb(second))
                }

                0x03 => {
                    print!("xor V{:01X} V{:01X}", byteutil::get_low_nyb(first), byteutil::get_high_nyb(second))
                }

                0x04 => {
                    print!("adc V{:01X} V{:01X}", byteutil::get_low_nyb(first), byteutil::get_high_nyb(second))
                }

                0x05 => {
                    print!("sub V{:01X} V{:01X}", byteutil::get_low_nyb(first), byteutil::get_high_nyb(second))
                }

                0x06 => {
                    print!("sr V{:01X}", byteutil::get_low_nyb(first))
                }

                0x07 => {
                    print!("isb V{:01X} V{:01X}", byteutil::get_low_nyb(first), byteutil::get_high_nyb(second))
                }

                0x0E => {
                    print!("sl V{:01X}", byteutil::get_low_nyb(first))
                }

                _ => {
                    print!("nop ; unknown math operation on V{:01X}, V{:01X}", byteutil::get_low_nyb(first), byteutil::get_high_nyb(second))
                }
            }
        }

        0x90..=0x9F => {
            print!("sne V{:01X} V{:01X}", byteutil::get_low_nyb(first), byteutil::get_high_nyb(second))
        }

        0xA0..=0xAF => {
            print!("set I 0x{:03X} ; {} in decimal", byteutil::get_3nyb_addr(first, second), byteutil::get_3nyb_addr(first, second))
        }

        0xB0..=0xBF => {
            print!("jmp V0+{}", byteutil::get_3nyb_addr(first, second))
        }

        0xC0..=0xCF => {
            if second == 0x00 {
                print!("rnd V{:01X}", byteutil::get_low_nyb(first))
            } else {
                print!("rnd V{:01X} & 0b{:08b}", byteutil::get_low_nyb(first), second)
            }
        }

        0xD0..=0xDF => {
            print!("dsp V{:01X} V{:01X} {}", byteutil::get_low_nyb(first), byteutil::get_high_nyb(second), byteutil::get_low_nyb(second))
        }

        0xE0..=0xEF => {
            if second == 0x9E {
                print!("skd V{:01X}", byteutil::get_low_nyb(first))
            } else if second == 0xA1 {
                print!("sku V{:01X}", byteutil::get_low_nyb(first))
            } else {
                print!("nop ; unknown key operation affecting V{:01X}", byteutil::get_low_nyb(first))
            }
        }

        0xF0..=0xFF => {
            match second {
                0x07 => {
                    print!("rld {:01X}", byteutil::get_low_nyb(first))
                }

                0x0A => {
                    print!("rlk V{:01X}", byteutil::get_low_nyb(first))
                }

                0x15 => {
                    print!("rsd V{:01X}", byteutil::get_low_nyb(first))
                }

                0x18 => {
                    print!("rss V{:01X}", byteutil::get_low_nyb(first))
                }

                0x1E => {
                    print!("add I V{:01X}", byteutil::get_low_nyb(first))
                }

                0x29 => {
                    print!("get sprite_char V{:01X}", byteutil::get_low_nyb(first))
                }

                0x33 => {
                    print!("get bcd V{:01X}", byteutil::get_low_nyb(first))
                }

                0x55 => {
                    print!("mov V0/V{:01X} I/I+{}", byteutil::get_low_nyb(first), byteutil::get_low_nyb(first))
                }

                0x65 => {
                    print!("mov I/I+{} V0/V{:01X}", byteutil::get_low_nyb(first), byteutil::get_low_nyb(first))
                }

                _ => {
                    print!("nop ; unknown special operation affecting V{:01X}", byteutil::get_low_nyb(first))
                }
            }
        }

        _ => {
            print!("nop ; unknown bytes {:02X} {:02X}", first, second)
        }
    }
    print!("\n");
}

pub fn print_disassembly(bytes: Vec<u8>) {
    println!("BYTE | INSTRUCTION\n__________________");
    for i  in (0..bytes.len()-1).step_by(2) {
        print_opcode(i+0x200, bytes[i], bytes[i+1]);
    }
}