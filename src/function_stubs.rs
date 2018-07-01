
use cpu::*;
use bit_field::BitField;

#[allow(unused, unreachable_code)]
unsafe fn stubs(cpu: &mut CPU) {
	{
	// NAME: ADC_z_h_c_u8_u8_out_u8
			let imm0 = cpu.immediate_u8();
			let reg0 = cpu.AF.r8.first;
			let reg1 = imm0;
			let mut out;
	//----------------
		let (added, of1) = reg1.overflowing_add(cpu.c() as u8);
		let (a, of2) = reg0.overflowing_add(added);
		out = a;

		let h = ((reg0 & 0x0F) + (reg1 & 0x0F) + cpu.c() as u8) > 0x0F;

		cpu.set_c(of1 || of2);
		cpu.set_z(out == 0);
		cpu.set_h(h);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: ADD_h_c_u16_i8
			let imm0 = cpu.immediate_i8();
			let reg0 = cpu.SP;
			let reg1 = imm0;
	//----------------
		let (a, c, h) = CPU::signed_offset(reg0, reg1);

		cpu.SP = a;

		cpu.set_c(c);
		cpu.set_h(h);
	//----------------
	}
	{
	// NAME: ADD_h_c_u16_u16_out_u16
			let reg0 = cpu.HL.r16;
			let reg1 = cpu.SP;
			let mut out;
	//----------------
		let (res, c, h) = CPU::add16(reg0, reg1);
		out = res;
		cpu.set_c(c);
		cpu.set_h(h);
	//----------------
			cpu.HL.r16 = out;
	}
	{
	// NAME: ADD_z_h_c_u8_u8
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
	{
	// NAME: AND_z_u8
			let imm0 = cpu.immediate_u8();
			let reg0 = imm0;
	//----------------
		let mut a = cpu.AF.r8.first;
		a &= reg0;
		cpu.set_z(a == 0);
		cpu.AF.r8.first = a;
	//----------------
	}
	{
	// NAME: BIT_z_u8_u8
			let reg0 = 7;
			let reg1 = cpu.AF.r8.first;
	//----------------
		cpu.set_z(!reg1.get_bit(reg0));
	//----------------
	}
	{
	// NAME: CALL_bool_u16
			let imm0 = cpu.immediate_u16();
			let reg0 = cpu.c();
			let reg1 = imm0;
	//----------------
		if reg0 {
			cpu.call(reg1);
		}
	//----------------
	}
	{
	// NAME: CALL_u16
			let imm0 = cpu.immediate_u16();
			let reg0 = imm0;
	//----------------
		cpu.call(reg0);
	//----------------
	}
	{
	// NAME: CCF_c
	//----------------
		let c = cpu.c();
		cpu.set_c(!c);
	//----------------
	}
	{
	// NAME: CPL
	//----------------
		cpu.AF.r8.first = !cpu.AF.r8.first;
	//----------------
	}
	{
	// NAME: CP_z_h_c_u8
			let imm0 = cpu.immediate_u8();
			let reg0 = imm0;
	//----------------
		let (a, c, h) = CPU::sub8(cpu.AF.r8.first, reg0);
		cpu.set_z(a == 0);
		cpu.set_c(c);
		cpu.set_h(h);
	//----------------
	}
	{
	// NAME: DAA_z_c
	//----------------
		//TODO pass blargg's test 01, it only fails on this instruction
		// this code doesn't seem to work
		let mut a = cpu.AF.r8.first;
		let mut correction = if cpu.c() { 0x60 } else { 0x00 };

		if (cpu.h()) {
			correction |= 0x06;
		}

		if (!cpu.n()) {
			if ((a & 0x0F) > 0x09) {
				correction |= 0x06;
			}
			if (a > 0x99) {
				correction |= 0x60;
			}

			a = a.wrapping_add(correction);
		} else {
			a = a.wrapping_sub(correction);
		}

		cpu.set_c(correction > 0x6 && correction < 0xfa);
		cpu.set_z(a == 0);

	//----------------
	}
	{
	// NAME: DEC_u16_out_u16
			let reg0 = cpu.SP;
			let mut out;
	//----------------
		out = reg0.wrapping_sub(1);
	//----------------
			cpu.SP = out;
	}
	{
	// NAME: DEC_z_h_u8_out_u8
			let reg0 = cpu.AF.r8.first;
			let mut out;
	//----------------
		let (a, c, h) = CPU::sub8(reg0, 1);
		out = a;

		cpu.set_z(a == 0);
		cpu.set_h(h);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: DI
	//----------------
		cpu.enable_interrupts(false);
	//----------------
	}
	{
	// NAME: EI
	//----------------
		cpu.enable_interrupts(true);
	//----------------
	}
	{
	// NAME: HALT
	//----------------
		panic!("HALT not implemented");
	//----------------
	}
	{
	// NAME: INC_u16_out_u16
			let reg0 = cpu.SP;
			let mut out;
	//----------------
		out = reg0.wrapping_add(1);
	//----------------
			cpu.SP = out;
	}
	{
	// NAME: INC_z_h_u8_out_u8
			let reg0 = cpu.AF.r8.first;
			let mut out;
	//----------------
		let (a, c, h) = CPU::add8(reg0, 1);
		out = a;

		cpu.set_z(a == 0);
		cpu.set_h(h);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: JP_bool_u16
			let imm0 = cpu.immediate_u16();
			let reg0 = cpu.c();
			let reg1 = imm0;
	//----------------
		if reg0 {
			cpu.PC = reg1;
		}
	//----------------
	}
	{
	// NAME: JP_u16
			let reg0 = cpu.HL.r16;
	//----------------
		// despite all the material using the (HL) notation, this just loads HL itself
		// not the byte pointed by it
		cpu.PC = reg0;
	//----------------
	}
	{
	// NAME: JR_bool_i8
			let imm0 = cpu.immediate_i8();
			let reg0 = cpu.c();
			let reg1 = imm0;
	//----------------
		if reg0 {
			cpu.PC = CPU::signed_offset(cpu.PC, reg1).0;
		}
	//----------------
	}
	{
	// NAME: JR_i8
			let imm0 = cpu.immediate_i8();
			let reg0 = imm0;
	//----------------
		cpu.PC = CPU::signed_offset(cpu.PC, reg0).0;
	//----------------
	}
	{
	// NAME: LDH_u8_out_u8
			let imm0 = cpu.immediate_u8();
			let reg0 = cpu.address((imm0 as u16).wrapping_add(0xff00));
			let mut out;
	//----------------
		out = reg0;
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: LD_h_c_i8_out_u16
			let imm0 = cpu.immediate_i8();
			let reg0 = imm0;
			let mut out;
	//----------------
		let (res, c, h) = CPU::signed_offset(cpu.SP, reg0);

		out = res;
		cpu.set_c(c);
		cpu.set_h(h);
	//----------------
			cpu.HL.r16 = out;
	}
	{
	// NAME: LD_u16_out_u16
			let reg0 = cpu.HL.r16;
			let mut out;
	//----------------
		out = reg0;
		if out == 0xDF7E {
			let lol = 1;
		}
	//----------------
			cpu.SP = out;
	}
	{
	// NAME: LD_u16_out_u8
			let imm0 = cpu.immediate_u16();
			let reg0 = cpu.SP;
			let mut out;
	//----------------
		out = reg0;
	//----------------
			let addr = imm0;
			cpu.set_address16(addr, out);
	}
	{
	// NAME: LD_u8_out_u8
			let imm0 = cpu.immediate_u16();
			let reg0 = cpu.address(imm0);
			let mut out;
	//----------------
		out = reg0;
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: NOP
	//----------------

	//----------------
	}
	{
	// NAME: OR_z_u8
			let imm0 = cpu.immediate_u8();
			let reg0 = imm0;
	//----------------
		let mut a = cpu.AF.r8.first;
		a |= reg0;
		cpu.set_z(a == 0);
		cpu.AF.r8.first = a;
	//----------------
	}
	{
	// NAME: POP_out_u16
			let mut out;
	//----------------
		out = cpu.pop16();
	//----------------
			cpu.HL.r16 = out;
	}
	{
	// NAME: POP_z_n_h_c_out_u16
			let mut out;
	//----------------
		//mask out the first unusable nibble of AF
		out = cpu.pop16() & 0xFFF0;
	//----------------
			cpu.AF.r16 = out;
	}
	{
	// NAME: PUSH_u16
			let reg0 = cpu.AF.r16;
	//----------------
		cpu.push16(reg0);
	//----------------
	}
	{
	// NAME: RES_u8_u8_out_u8
			let reg0 = 7;
			let reg1 = cpu.AF.r8.first;
			let mut out;
	//----------------
		out = reg1;
		out.set_bit(reg0, false);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: RET
	//----------------
		cpu.PC = cpu.pop16();
	//----------------
	}
	{
	// NAME: RETI
	//----------------
		cpu.PC = cpu.pop16();
		//TODO do we want the delay here?
		cpu.enable_interrupts(true);
	//----------------
	}
	{
	// NAME: RET_bool
			let reg0 = cpu.c();
	//----------------
		if reg0 {
			cpu.PC = cpu.pop16();
		}
	//----------------
	}
	{
	// NAME: RLA_c
	//----------------
		let mut a = cpu.AF.r8.first;
		let old_c = cpu.c();
		cpu.set_c(a.get_bit(7));
		a = a << 1;
		a.set_bit(0, old_c);
		cpu.AF.r8.first = a;
	//----------------
	}
	{
	// NAME: RLCA_c
	//----------------
		let mut a = cpu.AF.r8.first;
		cpu.set_c(a.get_bit(7));
		cpu.AF.r8.first = a.rotate_left(1);
	//----------------
	}
	{
	// NAME: RLC_z_c_u8_out_u8
			let reg0 = cpu.AF.r8.first;
			let mut out;
	//----------------

		cpu.set_c(reg0.get_bit(7));
		out = reg0.rotate_left(1);
		cpu.set_z(out == 0);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: RL_z_c_u8_out_u8
			let reg0 = cpu.AF.r8.first;
			let mut out;
	//----------------
		let old_c = cpu.c();
		cpu.set_c(reg0.get_bit(7));
		out = reg0 << 1;
		out.set_bit(0, old_c);
		cpu.set_z(out == 0);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: RRA_c
	//----------------
		let mut a = cpu.AF.r8.first;
		let old_c = cpu.c();
		cpu.set_c(a.get_bit(0));
		a = a >> 1;
		a.set_bit(7, old_c);
		cpu.AF.r8.first = a;
	//----------------
	}
	{
	// NAME: RRCA_c
	//----------------
		let mut a = cpu.AF.r8.first;
		cpu.set_c(a.get_bit(0));
		cpu.AF.r8.first = a.rotate_right(1);
	//----------------
	}
	{
	// NAME: RRC_z_c_u8_out_u8
			let reg0 = cpu.AF.r8.first;
			let mut out;
	//----------------
		cpu.set_c(reg0.get_bit(0));
		out = reg0.rotate_right(1);
		cpu.set_z(out == 0);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: RR_z_c_u8_out_u8
			let reg0 = cpu.AF.r8.first;
			let mut out;
	//----------------
		let old_c = cpu.c();
		cpu.set_c(reg0.get_bit(0));
		out = reg0 >> 1;
		out.set_bit(7, old_c);
		cpu.set_z(out == 0);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: RST_u16
			let reg0 = 0x38;
	//----------------
		let pc = cpu.PC;
		cpu.push16(pc);
		cpu.PC = reg0;
	//----------------
	}
	{
	// NAME: SBC_z_h_c_u8_u8_out_u8
			let imm0 = cpu.immediate_u8();
			let reg0 = cpu.AF.r8.first;
			let reg1 = imm0;
			let mut out;
	//----------------

		let (a, of2) = reg0.overflowing_sub(reg1);
		let (a, of1) = a.overflowing_sub(cpu.c() as u8);
		out = a;

		let a = (reg0 & 0x0F) as i32;
		let b = (reg1 & 0x0F) as i32;
		let c = cpu.c() as i32;

		cpu.set_h(a - b - c < 0);
		cpu.set_c(of1 || of2);
		cpu.set_z(out == 0);

	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: SCF
	//----------------
		//no code, just set C
	//----------------
	}
	{
	// NAME: SET_u8_u8_out_u8
			let reg0 = 7;
			let reg1 = cpu.AF.r8.first;
			let mut out;
	//----------------
		out = reg1;
		out.set_bit(reg0, true);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: SLA_z_c_u8_out_u8
			let reg0 = cpu.AF.r8.first;
			let mut out;
	//----------------
		cpu.set_c(reg0.get_bit(7));
		out = reg0 << 1;
		cpu.set_z(out == 0);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: SRA_z_c_u8_out_u8
			let reg0 = cpu.AF.r8.first;
			let mut out;
	//----------------
		//the MSB doesn't change, it's not zeroed
		let old_msb = reg0.get_bit(7);

		cpu.set_c(reg0.get_bit(0));
		out = reg0 >> 1;
		out.set_bit(7, old_msb);
		cpu.set_z(out == 0);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: SRL_z_c_u8_out_u8
			let reg0 = cpu.AF.r8.first;
			let mut out;
	//----------------
		cpu.set_c(reg0.get_bit(0));
		out = reg0 >> 1;
		cpu.set_z(out == 0);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: STOP_u8
			let reg0 = 0;
	//----------------
		cpu.stop();
	//----------------
	}
	{
	// NAME: SUB_z_h_c_u8
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
	{
	// NAME: SWAP_z_u8_out_u8
			let reg0 = cpu.AF.r8.first;
			let mut out;
	//----------------
		out = (((reg0 & 0x0F) << 4) | ((reg0 & 0xF0) >> 4));
		cpu.set_z(out == 0);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: XOR_z_u8
			let imm0 = cpu.immediate_u8();
			let reg0 = imm0;
	//----------------
		cpu.AF.r8.first = cpu.AF.r8.first ^ reg0;
		let z = cpu.AF.r8.first == 0;
		cpu.set_z(z);
	//----------------
	}

}
