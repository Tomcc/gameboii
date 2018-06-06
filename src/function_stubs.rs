
use cpu::CPU;

unsafe fn stubs(cpu: &mut CPU) {
	{
	// NAME: ADC_z_h_c_u8_u8
			let reg0 = cpu.AF.r8.0;
			let reg1 = cpu.immediateU8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: ADD_h_c_u16_i8
			let reg0 = cpu.SP;
			let reg1 = cpu.immediateI8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: ADD_h_c_u16_u16
			let reg0 = cpu.HL.r16;
			let reg1 = cpu.SP;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: ADD_z_h_c_u8_u8
			let reg0 = cpu.AF.r8.0;
			let reg1 = cpu.immediateU8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: AND_z_u8
			let reg0 = cpu.immediateU8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: BIT_z_u8_u8
			let reg0 = 7;
			let reg1 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: CALL_bool_u16
			let reg0 = !cpu.c();
			let reg1 = cpu.immediateU16();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: CALL_u16
			let reg0 = cpu.immediateU16();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: CALL_u8_u16
			let reg0 = cpu.BC.r8.1;
			let reg1 = cpu.immediateU16();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: CCF_c
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: CPL
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: CP_z_h_c_u8
			let reg0 = cpu.immediateU8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: DAA_z_c
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: DEC_u16
			let reg0 = cpu.SP;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: DEC_z_h_u8
			let reg0 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: DI
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: EI
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: HALT
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: INC_u16
			let reg0 = cpu.SP;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: INC_z_h_u8
			let reg0 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: JP_bool_u16
			let reg0 = !cpu.c();
			let reg1 = cpu.immediateU16();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: JP_u16
			let reg0 = cpu.immediateU16();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: JP_u8
			let reg0 = cpu.address(cpu.HL.r16);
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: JP_u8_u16
			let reg0 = cpu.BC.r8.1;
			let reg1 = cpu.immediateU16();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: JR_bool_i8
			let reg0 = !cpu.c();
			let reg1 = cpu.immediateI8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: JR_i8
			let reg0 = cpu.immediateI8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: JR_u8_i8
			let reg0 = cpu.BC.r8.1;
			let reg1 = cpu.immediateI8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: LDH_u8_u8
			let reg0 = cpu.AF.r8.0;
			let reg1 = cpu.address(cpu.immediateU8() as u16 + 0xff00);
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: LD_h_c_u16_u16
			let reg0 = cpu.HL.r16;
			let reg1 = cpu.offset_sp(cpu.immediateI8());
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: LD_u16_out_u16
			let reg0 = cpu.immediateU16();
			let out;
	//----------------

		panic!("instruction not implemented");

	//----------------
			cpu.SP = out;
	}
	{
	// NAME: LD_u16_u16
			let reg0 = cpu.SP;
			let reg1 = cpu.HL.r16;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: LD_u8_u16
			let reg0 = cpu.address(cpu.immediateU16());
			let reg1 = cpu.SP;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: LD_u8_u8
			let reg0 = cpu.AF.r8.0;
			let reg1 = cpu.address(cpu.immediateU16());
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: NOP
	//----------------

		NOPLOL

	//----------------
	}
	{
	// NAME: OR_z_u8
			let reg0 = cpu.immediateU8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: POP_u16
			let reg0 = cpu.HL.r16;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: POP_z_n_h_c_u16
			let reg0 = cpu.AF.r16;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: PREFIX
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: PUSH_u16
			let reg0 = cpu.AF.r16;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RES_u8_u8
			let reg0 = 7;
			let reg1 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RET
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RETI
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RET_bool
			let reg0 = !cpu.c();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RET_u8
			let reg0 = cpu.BC.r8.1;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RLA_c
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RLCA_c
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RLC_z_c_u8
			let reg0 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RL_z_c_u8
			let reg0 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RRA_c
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RRCA_c
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RRC_z_c_u8
			let reg0 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RR_z_c_u8
			let reg0 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: RST_u16
			let reg0 = 0x38;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: SBC_z_h_c_u8_u8
			let reg0 = cpu.AF.r8.0;
			let reg1 = cpu.immediateU8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: SCF
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: SET_u8_u8
			let reg0 = 7;
			let reg1 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: SLA_z_c_u8
			let reg0 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: SRA_z_u8
			let reg0 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: SRL_z_c_u8
			let reg0 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: STOP_u8
			let reg0 = 0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: SUB_z_h_c_u8
			let reg0 = cpu.immediateU8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: SWAP_z_u8
			let reg0 = cpu.AF.r8.0;
	//----------------

		panic!("instruction not implemented");

	//----------------
	}
	{
	// NAME: XOR_z_u8
			let reg0 = cpu.immediateU8();
	//----------------

		panic!("instruction not implemented");

	//----------------
	}

}
