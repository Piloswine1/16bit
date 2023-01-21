namespace Instructions {
const static auto MOV_LIT_REG     = 0x10;
const static auto MOV_REG_REG     = 0x11;
const static auto MOV_REG_MEM     = 0x12;
const static auto MOV_MEM_REG     = 0x13;
const static auto MOV_LIT_MEM     = 0x1B;
const static auto MOV_REG_PTR_REG = 0x1C;
const static auto MOV_LIT_OFF_REG = 0x1D;

const static auto ADD_REG_REG     = 0x14;
const static auto ADD_LIT_REG     = 0x3F;
const static auto SUB_LIT_REG     = 0x16;
const static auto SUB_REG_LIT     = 0x1E;
const static auto SUB_REG_REG     = 0x1F;
const static auto INC_REG         = 0x35;
const static auto DEC_REG         = 0x36;
const static auto MUL_LIT_REG     = 0x20;
const static auto MUL_REG_REG     = 0x21;

const static auto LSF_REG_LIT     = 0x26;
const static auto LSF_REG_REG     = 0x27;
const static auto RSF_REG_LIT     = 0x2A;
const static auto RSF_REG_REG     = 0x2B;
const static auto AND_REG_LIT     = 0x2E;
const static auto AND_REG_REG     = 0x2F;
const static auto OR_REG_LIT      = 0x30;
const static auto OR_REG_REG      = 0x31;
const static auto XOR_REG_LIT     = 0x32;
const static auto XOR_REG_REG     = 0x33;
const static auto NOT             = 0x34;

const static auto JMP_NOT_EQ      = 0x15;
const static auto JNE_REG         = 0x40;
const static auto JEQ_REG         = 0x3E;
const static auto JEQ_LIT         = 0x41;
const static auto JLT_REG         = 0x42;
const static auto JLT_LIT         = 0x43;
const static auto JGT_REG         = 0x44;
const static auto JGT_LIT         = 0x45;
const static auto JLE_REG         = 0x46;
const static auto JLE_LIT         = 0x47;
const static auto JGE_REG         = 0x48;
const static auto JGE_LIT         = 0x49;

const static auto PSH_LIT         = 0x17;
const static auto PSH_REG         = 0x18;
const static auto POP             = 0x1A;

const static auto CALL_LIT        = 0x5E;
const static auto CALL_REG        = 0x5F;
const static auto RET             = 0x60;

const static auto HLT             = 0xFF;
}  // namespace Instructions
