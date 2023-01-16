namespace Instructions {
const static auto MOV_LIT_REG = 0x10;
const static auto MOV_REG_REG = 0x11;
const static auto MOV_REG_MEM = 0x12;
const static auto MOV_MEM_REG = 0x13;
const static auto ADD_REG_REG = 0x14;
const static auto JMP_NOT_EQ  = 0x15;
const static auto PSH_LIT     = 0x16;
const static auto PSH_REG     = 0x17;
const static auto POP         = 0x18;
const static auto CALL_LIT    = 0x19;
const static auto CALL_REG    = 0x20;
const static auto RET         = 0x21;
const static auto HLT         = 0x22;

}  // namespace Instructions
