
use cpu::CPU;

unsafe fn stubs(cpu: &mut CPU) {
	{
	// NAME: ADC_z_h_c_u8_u8
			let reg0 = cpu.AF.r8.0;
			let reg1 = cpu.immediateU8();
	//----------------
		panic!("ADC_z_h_c_u8_u8 not implemented");
	//----------------
	}
	{
	// NAME: ADD_h_c_u16_i8
			let reg0 = cpu.SP;
			let reg1 = cpu.immediateI8();
	//----------------
		panic!("ADD_h_c_u16_i8 not implemented");
	//----------------
	}
	{
	// NAME: ADD_h_c_u16_u16
			let reg0 = cpu.HL.r16;
			let reg1 = cpu.SP;
	//----------------
		panic!("ADD_h_c_u16_u16 not implemented");
	//----------------
	}
	{
	// NAME: ADD_z_h_c_u8_u8
			let reg0 = cpu.AF.r8.0;
			let reg1 = cpu.immediateU8();
	//----------------
		panic!("ADD_z_h_c_u8_u8 not implemented");
	//----------------
	}
	{
	// NAME: AND_z_u8
			let reg0 = cpu.immediateU8();
	//----------------
		panic!("AND_z_u8 not implemented");
	//----------------
	}
	{
	// NAME: BIT_z_u8_u8
			let reg0 = 7;
			let reg1 = cpu.AF.r8.0;
	//----------------
		panic!("BIT_z_u8_u8 not implemented");
	//----------------
	}
	{
	// NAME: CALL_bool_u16
			let reg0 = !cpu.c();
			let reg1 = cpu.immediateU16();
	//----------------
		panic!("CALL_bool_u16 not implemented");
	//----------------
	}
	{
	// NAME: CALL_u16
			let reg0 = cpu.immediateU16();
	//----------------
		panic!("CALL_u16 not implemented");
	//----------------
	}
	{
	// NAME: CALL_u8_u16
			let reg0 = cpu.BC.r8.1;
			let reg1 = cpu.immediateU16();
	//----------------
		panic!("CALL_u8_u16 not implemented");
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
		panic!("CPL not implemented");
	//----------------
	}
	{
	// NAME: CP_z_h_c_u8
			let reg0 = cpu.immediateU8();
	//----------------
		panic!("CP_z_h_c_u8 not implemented");
	//----------------
	}
	{
	// NAME: DAA_z_c
	//----------------
		panic!("DAA_z_c not implemented");
	//----------------
	}
	{
	// NAME: DEC_u16
			let reg0 = cpu.SP;
	//----------------
		panic!("DEC_u16 not implemented");
	//----------------
	}
	{
	// NAME: DEC_z_h_u8
			let reg0 = cpu.AF.r8.0;
	//----------------
		panic!("DEC_z_h_u8 not implemented");
	//----------------
	}
	{
	// NAME: DI
	//----------------
		panic!("DI not implemented");
	//----------------
	}
	{
	// NAME: EI
	//----------------
		panic!("EI not implemented");
	//----------------
	}
	{
	// NAME: HALT
	//----------------
		panic!("HALT not implemented");
	//----------------
	}
	{
	// NAME: INC_u16
			let reg0 = cpu.SP;
	//----------------
		panic!("INC_u16 not implemented");
	//----------------
	}
	{
	// NAME: INC_z_h_u8
			let reg0 = cpu.AF.r8.0;
	//----------------
		panic!("INC_z_h_u8 not implemented");
	//----------------
	}
	{
	// NAME: JP_bool_u16
			let reg0 = !cpu.c();
			let reg1 = cpu.immediateU16();
	//----------------
		panic!("JP_bool_u16 not implemented");
	//----------------
	}
	{
	// NAME: JP_u16
			let reg0 = cpu.immediateU16();
	//----------------
		panic!("JP_u16 not implemented");
	//----------------
	}
	{
	// NAME: JP_u8
			let reg0 = cpu.address(cpu.HL.r16);
	//----------------
		panic!("JP_u8 not implemented");
	//----------------
	}
	{
	// NAME: JP_u8_u16
			let reg0 = cpu.BC.r8.1;
			let reg1 = cpu.immediateU16();
	//----------------
		panic!("JP_u8_u16 not implemented");
	//----------------
	}
	{
	// NAME: JR_bool_i8
			let reg0 = !cpu.c();
			let reg1 = cpu.immediateI8();
	//----------------
		panic!("JR_bool_i8 not implemented");
	//----------------
	}
	{
	// NAME: JR_i8
			let reg0 = cpu.immediateI8();
	//----------------
		panic!("JR_i8 not implemented");
	//----------------
	}
	{
	// NAME: JR_u8_i8
			let reg0 = cpu.BC.r8.1;
			let reg1 = cpu.immediateI8();
	//----------------
		panic!("JR_u8_i8 not implemented");
	//----------------
	}
	{
	// NAME: LDH_u8_u8
			let reg0 = cpu.AF.r8.0;
			let reg1 = cpu.address(cpu.immediateU8() as u16 + 0xff00);
	//----------------
		panic!("LDH_u8_u8 not implemented");
	//----------------
	}
	{
	// NAME: LD_h_c_u16_u16
			let reg0 = cpu.HL.r16;
			let reg1 = cpu.offset_sp(cpu.immediateI8());
	//----------------
		panic!("LD_h_c_u16_u16 not implemented");
	//----------------
	}
	{
	// NAME: LD_u16_out_u16
			let reg0 = cpu.immediateU16();
			let out;
	//----------------
		out = reg0;
	//----------------
			cpu.SP = out;
	}
	{
	// NAME: LD_u16_u16
			let reg0 = cpu.SP;
			let reg1 = cpu.HL.r16;
	//----------------
		panic!("LD_u16_u16 not implemented");
	//----------------
	}
	{
	// NAME: LD_u8_u16
			let reg0 = cpu.address(cpu.immediateU16());
			let reg1 = cpu.SP;
	//----------------
		panic!("LD_u8_u16 not implemented");
	//----------------
	}
	{
	// NAME: LD_u8_u8
			let reg0 = cpu.AF.r8.0;
			let reg1 = cpu.address(cpu.immediateU16());
	//----------------
		panic!("LD_u8_u8 not implemented");
	//----------------
	}
	{
	// NAME: NOP
	//----------------
	
	//----------------
	}
	{
	// NAME: OR_z_u8
			let reg0 = cpu.immediateU8();
	//----------------
		panic!("OR_z_u8 not implemented");
	//----------------
	}
	{
	// NAME: POP_u16
			let reg0 = cpu.HL.r16;
	//----------------
		panic!("POP_u16 not implemented");
	//----------------
	}
	{
	// NAME: POP_z_n_h_c_u16
			let reg0 = cpu.AF.r16;
	//----------------
		panic!("POP_z_n_h_c_u16 not implemented");
	//----------------
	}
	{
	// NAME: PREFIX
	//----------------
		panic!("PREFIX not implemented");
	//----------------
	}
	{
	// NAME: PUSH_u16
			let reg0 = cpu.AF.r16;
	//----------------
		panic!("PUSH_u16 not implemented");
	//----------------
	}
	{
	// NAME: RES_u8_u8
			let reg0 = 7;
			let reg1 = cpu.AF.r8.0;
	//----------------
		panic!("RES_u8_u8 not implemented");
	//----------------
	}
	{
	// NAME: RET
	//----------------
		panic!("RET not implemented");
	//----------------
	}
	{
	// NAME: RETI
	//----------------
		panic!("RETI not implemented");
	//----------------
	}
	{
	// NAME: RET_bool
			let reg0 = !cpu.c();
	//----------------
		panic!("RET_bool not implemented");
	//----------------
	}
	{
	// NAME: RET_u8
			let reg0 = cpu.BC.r8.1;
	//----------------
		panic!("RET_u8 not implemented");
	//----------------
	}
	{
	// NAME: RLA_c
	//----------------
		panic!("RLA_c not implemented");
	//----------------
	}
	{
	// NAME: RLCA_c
	//----------------
		panic!("RLCA_c not implemented");
	//----------------
	}
	{
	// NAME: RLC_z_c_u8
			let reg0 = cpu.AF.r8.0;
	//----------------
		panic!("RLC_z_c_u8 not implemented");
	//----------------
	}
	{
	// NAME: RL_z_c_u8
			let reg0 = cpu.AF.r8.0;
	//----------------
		panic!("RL_z_c_u8 not implemented");
	//----------------
	}
	{
	// NAME: RRA_c
	//----------------
		panic!("RRA_c not implemented");
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
			let reg0 = cpu.AF.r8.0;
	//----------------
		panic!("RRC_z_c_u8 not implemented");
	//----------------
	}
	{
	// NAME: RR_z_c_u8
			let reg0 = cpu.AF.r8.0;
	//----------------
		panic!("RR_z_c_u8 not implemented");
	//----------------
	}
	{
	// NAME: RST_u16
			let reg0 = 0x38;
	//----------------
		panic!("RST_u16 not implemented");
	//----------------
	}
	{
	// NAME: SBC_z_h_c_u8_u8
			let reg0 = cpu.AF.r8.0;
			let reg1 = cpu.immediateU8();
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
	// NAME: SET_u8_u8
			let reg0 = 7;
			let reg1 = cpu.AF.r8.0;
	//----------------
		panic!("SET_u8_u8 not implemented");
	//----------------
	}
	{
	// NAME: SLA_z_c_u8
			let reg0 = cpu.AF.r8.0;
	//----------------
		panic!("SLA_z_c_u8 not implemented");
	//----------------
	}
	{
	// NAME: SRA_z_u8
			let reg0 = cpu.AF.r8.0;
	//----------------
		panic!("SRA_z_u8 not implemented");
	//----------------
	}
	{
	// NAME: SRL_z_c_u8
			let reg0 = cpu.AF.r8.0;
	//----------------
		panic!("SRL_z_c_u8 not implemented");
	//----------------
	}
	{
	// NAME: STOP_u8
			let reg0 = 0;
	//----------------
		panic!("STOP_u8 not implemented");
	//----------------
	}
	{
	// NAME: SUB_z_h_c_u8
			let reg0 = cpu.immediateU8();
	//----------------
		panic!("SUB_z_h_c_u8 not implemented");
	//----------------
	}
	{
	// NAME: SWAP_z_u8
			let reg0 = cpu.AF.r8.0;
	//----------------
		panic!("SWAP_z_u8 not implemented");
	//----------------
	}
	{
	// NAME: XOR_z_u8
			let reg0 = cpu.immediateU8();
	//----------------
		cpu.AF.r8.0 = cpu.AF.r8.0 ^ reg0;
		let z = cpu.AF.r8.0 == 0;
		cpu.set_z(z);
	//----------------
	}

}
