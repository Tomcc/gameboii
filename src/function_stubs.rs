
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
		//TODO H
		let (added, of1) = reg1.overflowing_add(cpu.c() as u8);
		let (a, of2) = reg0.overflowing_add(added);
		out = a;
		cpu.set_c(of1 || of2);
		cpu.set_z(out == 0);
	//----------------
			cpu.AF.r8.first = out;
	}
	{
	// NAME: ADD_h_c_u16_i8
			let imm0 = cpu.immediate_i8();
			let reg0 = cpu.SP;
			let reg1 = imm0;
	//----------------
		panic!("ADD_h_c_u16_i8 not implemented");
	//----------------
	}
	{
	// NAME: ADD_h_c_u16_u16_out_u16
			let reg0 = cpu.HL.r16;
			let reg1 = cpu.SP;
			let mut out;
	//----------------
		//TODO H
		let (res, of) = reg0.overflowing_add(reg1);
		out = res;
		cpu.set_c(of);
	//----------------
			cpu.HL.r16 = out;
	}
	{
	// NAME: ADD_z_h_c_u8_u8
			let imm0 = cpu.immediate_u8();
			let reg0 = cpu.AF.r8.first;
			let reg1 = imm0;
	//----------------
		//TODO H
		let (a, overflow) = reg0.overflowing_add(reg0);
		cpu.AF.r8.first = a;
		cpu.set_z(a == 0);
		cpu.set_c(overflow);
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
		panic!("CCF_c not implemented");
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
		//TODO H
		let a = cpu.AF.r8.first;
		cpu.set_z(reg0 == a);
		cpu.set_c(reg0 < a);
	//----------------
	}
	{
	// NAME: DAA_z_c
	//----------------
		panic!("DAA_z_c not implemented");
	//----------------
	}
	{
	// NAME: DEC_u16_out_u16
			let reg0 = cpu.SP;
			let mut out;
	//----------------
		out = reg0.wrapping_sub(1);
		cpu.set_z(out == 0);
	//----------------
			cpu.SP = out;
	}
	{
	// NAME: DEC_z_h_u8_out_u8
			let reg0 = cpu.AF.r8.first;
			let mut out;
	//----------------
		//TODO H
		out = reg0.wrapping_sub(1);
		cpu.set_z(out == 0);
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
		out = reg0.wrapping_add(1);
		cpu.set_z(out == 0);

		// TODO H - Set if carry from bit 3.

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
		let (res, of) = CPU::signed_offset(cpu.SP, reg0);

		out = res;
		//TODO H
		cpu.set_c(of);

	//----------------
			cpu.HL.r16 = out;
	}
	{
	// NAME: LD_u16_out_u16
			let reg0 = cpu.HL.r16;
			let mut out;
	//----------------
		out = reg0;
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
		out = cpu.pop16();
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
		cpu.set_z(a == 0);
		cpu.AF.r8.first = a;
	//----------------
	}
	{
	// NAME: RLCA_c
	//----------------
		let mut a = cpu.AF.r8.first;
		let old_c = cpu.c();
		cpu.set_c(a.get_bit(7));
		a = a.rotate_left(1);
		cpu.set_z(a == 0);
		cpu.AF.r8.first = a;
	//----------------
	}
	{
	// NAME: RLC_z_c_u8
			let reg0 = cpu.AF.r8.first;
	//----------------
		panic!("RLC_z_c_u8 not implemented");
	//----------------
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
		cpu.set_z(a == 0);
		cpu.AF.r8.first = a;
	//----------------
	}
	{
	// NAME: RRCA_c
	//----------------
		panic!("RRCA_c not implemented");
	//----------------
	}
	{
	// NAME: RRC_z_c_u8
			let reg0 = cpu.AF.r8.first;
	//----------------
		panic!("RRC_z_c_u8 not implemented");
	//----------------
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
	// NAME: SBC_z_h_c_u8_u8
			let imm0 = cpu.immediate_u8();
			let reg0 = cpu.AF.r8.first;
			let reg1 = imm0;
	//----------------
		panic!("SBC_z_h_c_u8_u8 not implemented");
	//----------------
	}
	{
	// NAME: SCF
	//----------------
		panic!("SCF not implemented");
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
	// NAME: SLA_z_c_u8
			let reg0 = cpu.AF.r8.first;
	//----------------
		panic!("SLA_z_c_u8 not implemented");
	//----------------
	}
	{
	// NAME: SRA_z_u8
			let reg0 = cpu.AF.r8.first;
	//----------------
		panic!("SRA_z_u8 not implemented");
	//----------------
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
		//TODO H
		let (a, overflow) = cpu.AF.r8.first.overflowing_sub(reg0);
		cpu.AF.r8.first = a;
		cpu.set_z(a == 0);
		cpu.set_c(overflow);
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
