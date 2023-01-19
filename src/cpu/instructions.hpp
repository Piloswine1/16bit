namespace Instructions {
const static auto MOV_LIT_REG     = 0x10;
const static auto MOV_REG_REG     = 0x11;
const static auto MOV_REG_MEM     = 0x12;
const static auto MOV_MEM_REG     = 0x13;
const static auto MOV_LIT_MEM     = 0x14;
const static auto MOV_REG_PTR_REG = 0x15;
const static auto MOV_LIT_OFF_REG = 0x16;

const static auto ADD_REG_REG     = 0x17;
const static auto ADD_LIT_REG     = 0x18;
const static auto SUB_LIT_REG     = 0x19;
const static auto SUB_REG_LIT     = 0x1a;
const static auto SUB_REG_REG     = 0x1b;
const static auto INC_REG         = 0x1c;
const static auto DEC_REG         = 0x1d;
const static auto MUL_LIT_REG     = 0x1e;
const static auto MUL_REG_REG     = 0x1f;

const static auto LSF_REG_LIT     = 0x20;
const static auto LSF_REG_REG     = 0x21;
const static auto RSF_REG_LIT     = 0x22;
const static auto RSF_REG_REG     = 0x23;
const static auto AND_REG_LIT     = 0x24;
const static auto AND_REG_REG     = 0x25;
const static auto OR_REG_LIT      = 0x26;
const static auto OR_REG_REG      = 0x27;
const static auto XOR_REG_LIT     = 0x28;
const static auto XOR_REG_REG     = 0x29;
const static auto NOT             = 0x2a;

const static auto JMP_NOT_EQ      = 0x2b;
const static auto JNE_REG         = 0x2c;
const static auto JEQ_REG         = 0x2d;
const static auto JEQ_LIT         = 0x2e;
const static auto JLT_REG         = 0x2f;
const static auto JLT_LIT         = 0x30;
const static auto JGT_REG         = 0x31;
const static auto JGT_LIT         = 0x32;
const static auto JLE_REG         = 0x33;
const static auto JLE_LIT         = 0x34;
const static auto JGE_REG         = 0x35;
const static auto JGE_LIT         = 0x36;

const static auto PSH_LIT         = 0x37;
const static auto PSH_REG         = 0x38;
const static auto POP             = 0x39;

const static auto CALL_LIT        = 0x3a;
const static auto CALL_REG        = 0x3b;
const static auto RET             = 0x3c;

const static auto HLT             = 0x3d;
}  // namespace Instructions
