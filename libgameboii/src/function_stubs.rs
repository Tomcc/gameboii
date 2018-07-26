
use cpu::*;
use bit_field::BitField;

#[allow(dead_code)]
unsafe fn stubs(cpu: &mut CPU, dummy: &str) {
    match dummy {
		"ADC_z_h_c_u8_u8_out_u8" => {
			let imm0 = cpu.immediate_u8();
			let reg0 = cpu.AF.r8.first;
			let reg1 = imm0;
			//----------------
			let (added, of1) = reg1.overflowing_add(cpu.c() as u8);
			let (a, of2) = reg0.overflowing_add(added);
			let out = a;

			let h = ((reg0 & 0x0F) + (reg1 & 0x0F) + cpu.c() as u8) > 0x0F;

			cpu.set_c(of1 || of2);
			cpu.set_z(out == 0);
			cpu.set_h(h);
			//----------------
			cpu.AF.r8.first = out;
		}

		"ADD_h_c_u16_i8" => {
			let imm0 = cpu.immediate_i8();
			let reg0 = cpu.SP;
			let reg1 = imm0;
			//----------------
			let (out, _, _) = CPU::signed_offset(reg0, reg1);
			cpu.SP = out;

			// this function is weird you have to use the lowest byte of the 16-bit value,
			// and use the immediate signed 8-bit value as unsigned
			let (_, c, h) = CPU::add8(reg0 as u8, reg1 as u8);

			cpu.set_c(c);
			cpu.set_h(h);
			//----------------
		}

		"ADD_h_c_u16_u16_out_u16" => {
			let reg0 = cpu.HL.r16;
			let reg1 = cpu.SP;
			//----------------
			let (res, c, h) = CPU::add16(reg0, reg1);
			let out = res;
			cpu.set_c(c);
			cpu.set_h(h);
			//----------------
			cpu.HL.r16 = out;
		}

		"ADD_z_h_c_u8_u8" => {
			let imm0 = cpu.immediate_u8();
			let reg0 = cpu.AF.r8.first;
			let reg1 = imm0;
			//----------------
			let (a, c, h) = CPU::add8(reg0, reg1);
			cpu.AF.r8.first = a;

			cpu.set_z(a == 0);
			cpu.set_c(c);
			cpu.set_h(h);
			//----------------
		}

		"AND_z_u8" => {
			let imm0 = cpu.immediate_u8();
			let reg0 = imm0;
			//----------------
			let mut a = cpu.AF.r8.first;
			a &= reg0;
			cpu.set_z(a == 0);
			cpu.AF.r8.first = a;
			//----------------
		}

		"BIT_z_u8_u8" => {
			let reg0 = 7;
			let reg1 = cpu.AF.r8.first;
			//----------------
			cpu.set_z(!reg1.get_bit(reg0));
			//----------------
		}

		"CALL_bool_u16" => {
			let imm0 = cpu.immediate_u16();
			let reg0 = cpu.c();
			let reg1 = imm0;
			//----------------
			if reg0 {
				cpu.call(reg1);
			}
			//----------------
		}

		"CALL_u16" => {
			let imm0 = cpu.immediate_u16();
			let reg0 = imm0;
			//----------------
			cpu.call(reg0);
			//----------------
		}

		"CCF_c" => {
			//----------------
			let c = cpu.c();
			cpu.set_c(!c);
			//----------------
		}

		"CPL" => {
			//----------------
			cpu.AF.r8.first = !cpu.AF.r8.first;
			//----------------
		}

		"CP_z_h_c_u8" => {
			let imm0 = cpu.immediate_u8();
			let reg0 = imm0;
			//----------------
			let (a, c, h) = CPU::sub8(cpu.AF.r8.first, reg0);
			cpu.set_z(a == 0);
			cpu.set_c(c);
			cpu.set_h(h);
			//----------------
		}

		"DAA_z_c" => {
			//----------------
			// taken from
			// https://gammpei.github.io/blog/posts/2018-03-04/how-to-write-a-game-boy-emulator-part-8-blarggs-cpu-test-roms-1-3-4-5-7-8-9-10-11.html

			let mut a = cpu.AF.r8.first;
			if !cpu.n() {
				if cpu.c() || a > 0x99 {
					a = a.wrapping_add(0x60);
					cpu.set_c(true);
				}
				if cpu.h() || a & 0x0f > 0x9 {
					a = a.wrapping_add(0x06);
				}
			} else {
				if cpu.c() {
					a = a.wrapping_sub(0x60);
				}
				if cpu.h() {
					a = a.wrapping_sub(0x06);
				}
			}

			cpu.AF.r8.first = a;
			cpu.set_z(a == 0);

			//----------------
		}

		"DEC_u16_out_u16" => {
			let reg0 = cpu.SP;
			//----------------
			let out = reg0.wrapping_sub(1);
			//----------------
			cpu.SP = out;
		}

		"DEC_z_h_u8_out_u8" => {
			let reg0 = cpu.AF.r8.first;
			//----------------
			let (out, _, h) = CPU::sub8(reg0, 1);

			cpu.set_z(out == 0);
			cpu.set_h(h);
			//----------------
			cpu.AF.r8.first = out;
		}

		"DI" => {
			//----------------
			cpu.enable_interrupts(false);
			//----------------
		}

		"EI" => {
			//----------------
			cpu.enable_interrupts(true);
			//----------------
		}

		"HALT" => {
			//----------------
			cpu.halt();
			//----------------
		}

		"INC_u16_out_u16" => {
			let reg0 = cpu.SP;
			//----------------
			let out = reg0.wrapping_add(1);
			//----------------
			cpu.SP = out;
		}

		"INC_z_h_u8_out_u8" => {
			let reg0 = cpu.AF.r8.first;
			//----------------
			let (out, _, h) = CPU::add8(reg0, 1);

			cpu.set_z(out == 0);
			cpu.set_h(h);
			//----------------
			cpu.AF.r8.first = out;
		}

		"JP_bool_u16" => {
			let imm0 = cpu.immediate_u16();
			let reg0 = cpu.c();
			let reg1 = imm0;
			//----------------
			if reg0 {
				cpu.PC = reg1;
			}
			//----------------
		}

		"JP_u16" => {
			let reg0 = cpu.HL.r16;
			//----------------
			// despite all the material using the (HL) notation, this just loads HL itself
			// not the byte pointed by it
			cpu.PC = reg0;
			//----------------
		}

		"JR_bool_i8" => {
			let imm0 = cpu.immediate_i8();
			let reg0 = cpu.c();
			let reg1 = imm0;
			//----------------
			if reg0 {
				cpu.PC = CPU::signed_offset(cpu.PC, reg1).0;
			}
			//----------------
		}

		"JR_i8" => {
			let imm0 = cpu.immediate_i8();
			let reg0 = imm0;
			//----------------
			cpu.PC = CPU::signed_offset(cpu.PC, reg0).0;
			//----------------
		}

		"LDH_u8_out_u8" => {
			let imm0 = cpu.immediate_u8();
			let reg0 = cpu.address((imm0 as u16).wrapping_add(0xff00));
			//----------------
			let out = reg0;
			//----------------
			cpu.AF.r8.first = out;
		}

		"LD_h_c_i8_out_u16" => {
			let imm0 = cpu.immediate_i8();
			let reg0 = imm0;
			//----------------
			let (out, _, _) = CPU::signed_offset(cpu.SP, reg0);

			// this function is weird you have to use the lowest byte of the 16-bit value,
			// and use the immediate signed 8-bit value as unsigned
			let (_, c, h) = CPU::add8(cpu.SP as u8, reg0 as u8);

			cpu.set_c(c);
			cpu.set_h(h);
			//----------------
			cpu.HL.r16 = out;
		}

		"LD_u16_out_u16" => {
			let reg0 = cpu.HL.r16;
			//----------------
			let out = reg0;
			//----------------
			cpu.SP = out;
		}

		"LD_u16_out_u8" => {
			let imm0 = cpu.immediate_u16();
			let reg0 = cpu.SP;
			//----------------
			let out = reg0;
			//----------------
			let addr = imm0;
			cpu.set_address16(addr, out);
		}

		"LD_u8_out_u8" => {
			let imm0 = cpu.immediate_u16();
			let reg0 = cpu.address(imm0);
			//----------------
			let out = reg0;
			//----------------
			cpu.AF.r8.first = out;
		}

		"NOP" => {
			//----------------

			//----------------
		}

		"OR_z_u8" => {
			let imm0 = cpu.immediate_u8();
			let reg0 = imm0;
			//----------------
			let mut a = cpu.AF.r8.first;
			a |= reg0;
			cpu.set_z(a == 0);
			cpu.AF.r8.first = a;
			//----------------
		}

		"POP_out_u16" => {
			//----------------
			let out = cpu.pop16();
			//----------------
			cpu.HL.r16 = out;
		}

		"POP_z_n_h_c_out_u16" => {
			//----------------
			//mask out the first unusable nibble of AF
			let out = cpu.pop16() & 0xFFF0;
			//----------------
			cpu.AF.r16 = out;
		}

		"PUSH_u16" => {
			let reg0 = cpu.AF.r16;
			//----------------
			cpu.push16(reg0);
			//----------------
		}

		"RES_u8_u8_out_u8" => {
			let reg0 = 7;
			let reg1 = cpu.AF.r8.first;
			//----------------
			let mut out = reg1;
			out.set_bit(reg0, false);
			//----------------
			cpu.AF.r8.first = out;
		}

		"RET" => {
			//----------------
			cpu.PC = cpu.pop16();
			//----------------
		}

		"RETI" => {
			//----------------
			cpu.PC = cpu.pop16();
			//TODO do we want the delay here?
			cpu.enable_interrupts(true);
			//----------------
		}

		"RET_bool" => {
			let reg0 = cpu.c();
			//----------------
			if reg0 {
				cpu.PC = cpu.pop16();
			}
			//----------------
		}

		"RLA_c" => {
			//----------------
			let mut a = cpu.AF.r8.first;
			let old_c = cpu.c();
			cpu.set_c(a.get_bit(7));
			a = a << 1;
			a.set_bit(0, old_c);
			cpu.AF.r8.first = a;
			//----------------
		}

		"RLCA_c" => {
			//----------------
			let a = cpu.AF.r8.first;
			cpu.set_c(a.get_bit(7));
			cpu.AF.r8.first = a.rotate_left(1);
			//----------------
		}

		"RLC_z_c_u8_out_u8" => {
			let reg0 = cpu.AF.r8.first;
			//----------------

			cpu.set_c(reg0.get_bit(7));
			let out = reg0.rotate_left(1);
			cpu.set_z(out == 0);
			//----------------
			cpu.AF.r8.first = out;
		}

		"RL_z_c_u8_out_u8" => {
			let reg0 = cpu.AF.r8.first;
			//----------------
			let old_c = cpu.c();
			cpu.set_c(reg0.get_bit(7));
			let mut out = reg0 << 1;
			out.set_bit(0, old_c);
			cpu.set_z(out == 0);
			//----------------
			cpu.AF.r8.first = out;
		}

		"RRA_c" => {
			//----------------
			let mut a = cpu.AF.r8.first;
			let old_c = cpu.c();
			cpu.set_c(a.get_bit(0));
			a = a >> 1;
			a.set_bit(7, old_c);
			cpu.AF.r8.first = a;
			//----------------
		}

		"RRCA_c" => {
			//----------------
			let a = cpu.AF.r8.first;
			cpu.set_c(a.get_bit(0));
			cpu.AF.r8.first = a.rotate_right(1);
			//----------------
		}

		"RRC_z_c_u8_out_u8" => {
			let reg0 = cpu.AF.r8.first;
			//----------------
			cpu.set_c(reg0.get_bit(0));
			let out = reg0.rotate_right(1);
			cpu.set_z(out == 0);
			//----------------
			cpu.AF.r8.first = out;
		}

		"RR_z_c_u8_out_u8" => {
			let reg0 = cpu.AF.r8.first;
			//----------------
			let old_c = cpu.c();
			cpu.set_c(reg0.get_bit(0));
			let mut out = reg0 >> 1;
			out.set_bit(7, old_c);
			cpu.set_z(out == 0);
			//----------------
			cpu.AF.r8.first = out;
		}

		"RST_u16" => {
			let reg0 = 0x38;
			//----------------
			let pc = cpu.PC;
			cpu.push16(pc);
			cpu.PC = reg0;
			//----------------
		}

		"SBC_z_h_c_u8_u8_out_u8" => {
			let imm0 = cpu.immediate_u8();
			let reg0 = cpu.AF.r8.first;
			let reg1 = imm0;
			//----------------

			let (out, of2) = reg0.overflowing_sub(reg1);
			let (out, of1) = out.overflowing_sub(cpu.c() as u8);

			let a = (reg0 & 0x0F) as i32;
			let b = (reg1 & 0x0F) as i32;
			let c = cpu.c() as i32;

			cpu.set_h(a - b - c < 0);
			cpu.set_c(of1 || of2);
			cpu.set_z(out == 0);

			//----------------
			cpu.AF.r8.first = out;
		}

		"SCF" => {
			//----------------
			//no code, just set C
			//----------------
		}

		"SET_u8_u8_out_u8" => {
			let reg0 = 7;
			let reg1 = cpu.AF.r8.first;
			//----------------
			let mut out = reg1;
			out.set_bit(reg0, true);
			//----------------
			cpu.AF.r8.first = out;
		}

		"SLA_z_c_u8_out_u8" => {
			let reg0 = cpu.AF.r8.first;
			//----------------
			cpu.set_c(reg0.get_bit(7));
			let out = reg0 << 1;
			cpu.set_z(out == 0);
			//----------------
			cpu.AF.r8.first = out;
		}

		"SRA_z_c_u8_out_u8" => {
			let reg0 = cpu.AF.r8.first;
			//----------------
			//the MSB doesn't change, it's not zeroed
			let old_msb = reg0.get_bit(7);

			cpu.set_c(reg0.get_bit(0));
			let mut out = reg0 >> 1;
			out.set_bit(7, old_msb);
			cpu.set_z(out == 0);
			//----------------
			cpu.AF.r8.first = out;
		}

		"SRL_z_c_u8_out_u8" => {
			let reg0 = cpu.AF.r8.first;
			//----------------
			cpu.set_c(reg0.get_bit(0));
			let out = reg0 >> 1;
			cpu.set_z(out == 0);
			//----------------
			cpu.AF.r8.first = out;
		}

		"STOP_u8" => {
			let reg0 = 0;
			//----------------
			cpu.stop(reg0);
			//----------------
		}

		"SUB_z_h_c_u8" => {
			let imm0 = cpu.immediate_u8();
			let reg0 = imm0;
			//----------------
			let (a, c, h) = CPU::sub8(cpu.AF.r8.first, reg0);
			cpu.AF.r8.first = a;

			cpu.set_z(a == 0);
			cpu.set_c(c);
			cpu.set_h(h);
			//----------------
		}

		"SWAP_z_u8_out_u8" => {
			let reg0 = cpu.AF.r8.first;
			//----------------
			let out = ((reg0 & 0x0F) << 4) | ((reg0 & 0xF0) >> 4);
			cpu.set_z(out == 0);
			//----------------
			cpu.AF.r8.first = out;
		}

		"XOR_z_u8" => {
			let imm0 = cpu.immediate_u8();
			let reg0 = imm0;
			//----------------
			cpu.AF.r8.first = cpu.AF.r8.first ^ reg0;
			let z = cpu.AF.r8.first == 0;
			cpu.set_z(z);
			//----------------
		}


        _ => ()
    }
}
    
