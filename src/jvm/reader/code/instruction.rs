use std::io::SeekFrom;

use crate::{io::BufferReadable, util::code_err::{CodeParseError, ClassParseError}};

#[repr(u8)]
#[derive(Debug, Clone)]


pub enum Instruction {
    Nop,
    AconstNull,
    IconstM1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Lconst0,
    Lconst1,
    Fconst0,
    Fconst1,
    Fconst2,
    Dconst0,
    Dconst1,
    Bipush(u8),
    Sipush(u16),
    Ldc(u8),
    LdcW(u16),
    Ldc2W(u16),
    Iload(u8),
    Lload(u8),
    Fload(u8),
    Dload(u8),
    Aload(u8),
    Iload0,
    Iload1,
    Iload2,
    Iload3,
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Fload0,
    Fload1,
    Fload2,
    Fload3,
    Dload0,
    Dload1,
    Dload2,
    Dload3,
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Iaload,
    Laload,
    Faload,
    Daload,
    Aaload,
    Baload,
    Caload,
    Saload,
    Istore(u8),
    Lstore(u8),
    Fstore(u8),
    Dstore(u8),
    Astore(u8),
    Istore0,
    Istore1,
    Istore2,
    Istore3,
    Lstore0,
    Lstore1,
    Lstore2,
    Lstore3,
    Fstore0,
    Fstore1,
    Fstore2,
    Fstore3,
    Dstore0,
    Dstore1,
    Dstore2,
    Dstore3,
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Iastore,
    Lastore,
    Fastore,
    Dastore,
    Aastore,
    Bastore,
    Castore,
    Sastore,
    Pop,
    Pop2,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    Swap,
    Iadd,
    Ladd,
    Fadd,
    Dadd,
    Isub,
    Lsub,
    Fsub,
    Dsub,
    Imul,
    Lmul,
    Fmul,
    Dmul,
    Idiv,
    Ldiv,
    Fdiv,
    Ddiv,
    Irem,
    Lrem,
    Frem,
    Drem,
    Ineg,
    Lneg,
    Fneg,
    Dneg,
    Ishl,
    Lshl,
    Ishr,
    Lshr,
    Iushr,
    Lushr,
    Iand,
    Land,
    Ior,
    Lor,
    Ixor,
    Lxor,
    Iinc(u8, i8),
    I2l,
    I2f,
    I2d,
    L2i,
    L2f,
    L2d,
    F2i,
    F2l,
    F2d,
    D2i,
    D2l,
    D2f,
    I2b,
    I2c,
    I2s,
    Lcmp,
    Fcmpl,
    Fcmpg,
    Dcmpl,
    Dcmpg,
    Ifeq(i16),
    Ifne(i16),
    Iflt(i16),
    Ifge(i16),
    Ifgt(i16),
    Ifle(i16),
    IfIcmpeq(i16),
    IfIcmpne(i16),
    IfIcmplt(i16),
    IfIcmpge(i16),
    IfIcmpgt(i16),
    IfIcmple(i16),
    IfAcmpeq(i16),
    IfAcmpne(i16),
    Goto(i16),
    Jsr(i16),
    Ret(u8),
    Tableswitch(TableSwitch),
    Lookupswitch(LookupSwitch),
    Ireturn,
    Lreturn,
    Freturn,
    Dreturn,
    Areturn,
    Return,
    Getstatic(u16),
    Putstatic(u16),
    Getfield(u16),
    Putfield(u16),
    Invokevirtual(u16),
    Invokespecial(u16),
    Invokestatic(u16),
    Invokeinterface(u16, u8, u8), // InvokeInterface(_, _, Y) => Y must always be 0
    Invokedynamic(u16, u16), // InvokeDynamic(_, X) => X must always be 0
    New(u16),
    Newarray(u8),
    Arraylength,
    ANewarray(u16),
    Athrow,
    Checkcast(u16),
    Instanceof(u16),
    Monitorenter,
    Monitorexit,
    Wide(u8, u16, u16), // Wide(inst, _, X) => If inst is Iinc, X must be a signed 16-bit integer, otherwise X must be 0 as it should not be read. 
    Multianewarray(u16, u8),
    Ifnull(i16),
    Ifnonnull(i16),
    GotoW(i32),
    JsrW(i32),
}

impl Instruction {
    pub fn load<R: BufferReadable>(buf: &mut R, start: u64) -> Result<Instruction, ClassParseError> {
        match buf
            .read_byte()?
        {
            0 => Ok(Instruction::Nop),
            1 => Ok(Instruction::AconstNull),
            2 => Ok(Instruction::IconstM1),
            3 => Ok(Instruction::Iconst0),
            4 => Ok(Instruction::Iconst1),
            5 => Ok(Instruction::Iconst2),
            6 => Ok(Instruction::Iconst3),
            7 => Ok(Instruction::Iconst4),
            8 => Ok(Instruction::Iconst5),
            9 => Ok(Instruction::Lconst0),
            10 => Ok(Instruction::Lconst1),
            11 => Ok(Instruction::Fconst0),
            12 => Ok(Instruction::Fconst1),
            13 => Ok(Instruction::Fconst2),
            14 => Ok(Instruction::Dconst0),
            15 => Ok(Instruction::Dconst1),
            16 => Ok(Instruction::Bipush(buf.read_byte()?)),
            17 => Ok(Instruction::Sipush(buf.read_u2()?)),
            18 => Ok(Instruction::Ldc(buf.read_byte()?)),
            19 => Ok(Instruction::LdcW(buf.read_u2()?)),
            20 => Ok(Instruction::Ldc2W(buf.read_u2()?)),
            21 => Ok(Instruction::Iload(buf.read_byte()?)),
            22 => Ok(Instruction::Lload(buf.read_byte()?)),
            23 => Ok(Instruction::Fload(buf.read_byte()?)),
            24 => Ok(Instruction::Dload(buf.read_byte()?)),
            25 => Ok(Instruction::Aload(buf.read_byte()?)),
            26 => Ok(Instruction::Iload0),
            27 => Ok(Instruction::Iload1),
            28 => Ok(Instruction::Iload2),
            29 => Ok(Instruction::Iload3),
            30 => Ok(Instruction::Lload0),
            31 => Ok(Instruction::Lload1),
            32 => Ok(Instruction::Lload2),
            33 => Ok(Instruction::Lload3),
            34 => Ok(Instruction::Fload0),
            35 => Ok(Instruction::Fload1),
            36 => Ok(Instruction::Fload2),
            37 => Ok(Instruction::Fload3),
            38 => Ok(Instruction::Dload0),
            39 => Ok(Instruction::Dload1),
            40 => Ok(Instruction::Dload2),
            41 => Ok(Instruction::Dload3),
            42 => Ok(Instruction::Aload0),
            43 => Ok(Instruction::Aload1),
            44 => Ok(Instruction::Aload2),
            45 => Ok(Instruction::Aload3),
            46 => Ok(Instruction::Iaload),
            47 => Ok(Instruction::Laload),
            48 => Ok(Instruction::Faload),
            49 => Ok(Instruction::Daload),
            50 => Ok(Instruction::Aaload),
            51 => Ok(Instruction::Baload),
            52 => Ok(Instruction::Caload),
            53 => Ok(Instruction::Saload),
            54 => Ok(Instruction::Istore(buf.read_byte()?)),
            55 => Ok(Instruction::Lstore(buf.read_byte()?)),
            56 => Ok(Instruction::Fstore(buf.read_byte()?)),
            57 => Ok(Instruction::Dstore(buf.read_byte()?)),
            58 => Ok(Instruction::Astore(buf.read_byte()?)),
            59 => Ok(Instruction::Istore0),
            60 => Ok(Instruction::Istore1),
            61 => Ok(Instruction::Istore2),
            62 => Ok(Instruction::Istore3),
            63 => Ok(Instruction::Lstore0),
            64 => Ok(Instruction::Lstore1),
            65 => Ok(Instruction::Lstore2),
            66 => Ok(Instruction::Lstore3),
            67 => Ok(Instruction::Fstore0),
            68 => Ok(Instruction::Fstore1),
            69 => Ok(Instruction::Fstore2),
            70 => Ok(Instruction::Fstore3),
            71 => Ok(Instruction::Dstore0),
            72 => Ok(Instruction::Dstore1),
            73 => Ok(Instruction::Dstore2),
            74 => Ok(Instruction::Dstore3),
            75 => Ok(Instruction::Astore0),
            76 => Ok(Instruction::Astore1),
            77 => Ok(Instruction::Astore2),
            78 => Ok(Instruction::Astore3),
            79 => Ok(Instruction::Iastore),
            80 => Ok(Instruction::Lastore),
            81 => Ok(Instruction::Fastore),
            82 => Ok(Instruction::Dastore),
            83 => Ok(Instruction::Aastore),
            84 => Ok(Instruction::Bastore),
            85 => Ok(Instruction::Castore),
            86 => Ok(Instruction::Sastore),
            87 => Ok(Instruction::Pop),
            88 => Ok(Instruction::Pop2),
            89 => Ok(Instruction::Dup),
            90 => Ok(Instruction::DupX1),
            91 => Ok(Instruction::DupX2),
            92 => Ok(Instruction::Dup2),
            93 => Ok(Instruction::Dup2X1),
            94 => Ok(Instruction::Dup2X2),
            95 => Ok(Instruction::Swap),
            96 => Ok(Instruction::Iadd),
            97 => Ok(Instruction::Ladd),
            98 => Ok(Instruction::Fadd),
            99 => Ok(Instruction::Dadd),
            100 => Ok(Instruction::Isub),
            101 => Ok(Instruction::Lsub),
            102 => Ok(Instruction::Fsub),
            103 => Ok(Instruction::Dsub),
            104 => Ok(Instruction::Imul),
            105 => Ok(Instruction::Lmul),
            106 => Ok(Instruction::Fmul),
            107 => Ok(Instruction::Dmul),
            108 => Ok(Instruction::Idiv),
            109 => Ok(Instruction::Ldiv),
            110 => Ok(Instruction::Fdiv),
            111 => Ok(Instruction::Ddiv),
            112 => Ok(Instruction::Irem),
            113 => Ok(Instruction::Lrem),
            114 => Ok(Instruction::Frem),
            115 => Ok(Instruction::Drem),
            116 => Ok(Instruction::Ineg),
            117 => Ok(Instruction::Lneg),
            118 => Ok(Instruction::Fneg),
            119 => Ok(Instruction::Dneg),
            120 => Ok(Instruction::Ishl),
            121 => Ok(Instruction::Lshl),
            122 => Ok(Instruction::Ishr),
            123 => Ok(Instruction::Lshr),
            124 => Ok(Instruction::Iushr),
            125 => Ok(Instruction::Lushr),
            126 => Ok(Instruction::Iand),
            127 => Ok(Instruction::Land),
            128 => Ok(Instruction::Ior),
            129 => Ok(Instruction::Lor),
            130 => Ok(Instruction::Ixor),
            131 => Ok(Instruction::Lxor),
            132 => Ok(Instruction::Iinc(buf.read_byte()?, buf.read_byte()? as i8)),
            133 => Ok(Instruction::I2l),
            134 => Ok(Instruction::I2f),
            135 => Ok(Instruction::I2d),
            136 => Ok(Instruction::L2i),
            137 => Ok(Instruction::L2f),
            138 => Ok(Instruction::L2d),
            139 => Ok(Instruction::F2i),
            140 => Ok(Instruction::F2l),
            141 => Ok(Instruction::F2d),
            142 => Ok(Instruction::D2i),
            143 => Ok(Instruction::D2l),
            144 => Ok(Instruction::D2f),
            145 => Ok(Instruction::I2b),
            146 => Ok(Instruction::I2c),
            147 => Ok(Instruction::I2s),
            148 => Ok(Instruction::Lcmp),
            149 => Ok(Instruction::Fcmpl),
            150 => Ok(Instruction::Fcmpg),
            151 => Ok(Instruction::Dcmpl),
            152 => Ok(Instruction::Dcmpg),
            153 => Ok(Instruction::Ifeq(buf.read_u2()? as i16)),
            154 => Ok(Instruction::Ifne(buf.read_u2()? as i16)),
            155 => Ok(Instruction::Iflt(buf.read_u2()? as i16)),
            156 => Ok(Instruction::Ifge(buf.read_u2()? as i16)),
            157 => Ok(Instruction::Ifgt(buf.read_u2()? as i16)),
            158 => Ok(Instruction::Ifle(buf.read_u2()? as i16)),
            159 => Ok(Instruction::IfIcmpeq(buf.read_u2()? as i16)),
            160 => Ok(Instruction::IfIcmpne(buf.read_u2()? as i16)),
            161 => Ok(Instruction::IfIcmplt(buf.read_u2()? as i16)),
            162 => Ok(Instruction::IfIcmpge(buf.read_u2()? as i16)),
            163 => Ok(Instruction::IfIcmpgt(buf.read_u2()? as i16)),
            164 => Ok(Instruction::IfIcmple(buf.read_u2()? as i16)),
            165 => Ok(Instruction::IfAcmpeq(buf.read_u2()? as i16)),
            166 => Ok(Instruction::IfAcmpne(buf.read_u2()? as i16)),
            167 => Ok(Instruction::Goto(buf.read_u2()? as i16)),
            168 => Ok(Instruction::Jsr(buf.read_u2()? as i16)),
            169 => Ok(Instruction::Ret(buf.read_byte()?)),
            170 => Ok(Instruction::Tableswitch(TableSwitch::load(buf, start)?)),
            171 => Ok(Instruction::Lookupswitch(LookupSwitch::load(buf, start)?)),
            172 => Ok(Instruction::Ireturn),
            173 => Ok(Instruction::Lreturn),
            174 => Ok(Instruction::Freturn),
            175 => Ok(Instruction::Dreturn),
            176 => Ok(Instruction::Areturn),
            177 => Ok(Instruction::Return),
            178 => Ok(Instruction::Getstatic(buf.read_u2()?)),
            179 => Ok(Instruction::Putstatic(buf.read_u2()?)),
            180 => Ok(Instruction::Getfield(buf.read_u2()?)),
            181 => Ok(Instruction::Putfield(buf.read_u2()?)),
            182 => Ok(Instruction::Invokevirtual(buf.read_u2()?)),
            183 => Ok(Instruction::Invokespecial(buf.read_u2()?)),
            184 => Ok(Instruction::Invokestatic(buf.read_u2()?)),
            185 => Ok(Instruction::Invokeinterface(buf.read_u2()?, buf.read_byte()?, buf.read_byte()?)),
            186 => Ok(Instruction::Invokedynamic(buf.read_u2()?, buf.read_u2()?)),
            187 => Ok(Instruction::New(buf.read_u2()?)),
            188 => Ok(Instruction::Newarray(buf.read_byte()?)),
            189 => Ok(Instruction::ANewarray(buf.read_u2()?)),
            190 => Ok(Instruction::Arraylength),
            191 => Ok(Instruction::Athrow),
            192 => Ok(Instruction::Checkcast(buf.read_u2()?)),
            193 => Ok(Instruction::Instanceof(buf.read_u2()?)),
            194 => Ok(Instruction::Monitorenter),
            195 => Ok(Instruction::Monitorexit),
            196 => {
                let opcode = buf.read_byte()?;
                if opcode != 132 {
                    return Ok(Instruction::Wide(opcode, buf.read_u2()?, 0));
                } else {
                    return Ok(Instruction::Wide(opcode, buf.read_u2()?, buf.read_u2()?));
                }
            },
            197 => Ok(Instruction::Multianewarray(buf.read_u2()?, buf.read_byte()?)),
            198 => Ok(Instruction::Ifnull(buf.read_u2()? as i16)),
            199 => Ok(Instruction::Ifnonnull(buf.read_u2()? as i16)),
            200 => Ok(Instruction::GotoW(buf.read_u4()? as i32)),
            201 => Ok(Instruction::JsrW(buf.read_u4()? as i32)),

            x => {
                return Err(ClassParseError::CodeParseError { internal: CodeParseError::InvalidBytecode {
                    at: "here".to_string(),
                    what: format!("{}", x).to_string(),
                }, classpath: None, signature: None})
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct LookupSwitch {
    default: i32,
    npairs: i32,
    matches: Vec<(i32, i32)>,
}
impl LookupSwitch {
    pub fn load<R: BufferReadable>(buf: &mut R, start: u64) -> Result<LookupSwitch, ClassParseError> {
        let bytes_bffr = (buf.seek(SeekFrom::Current(0)).unwrap() - start) % 4; 
        if bytes_bffr != 0 {
            buf.seek(SeekFrom::Current(4 - bytes_bffr as i64)).unwrap();
        }
        let default = buf.read_u4()? as i32;
        let npairs = buf.read_u4()? as i32;
        let mut matches = Vec::new();
        for _ in 0..npairs {
            let match_ = buf.read_u4()? as i32;
            let offset = buf.read_u4()? as i32;
            matches.push((match_, offset));
        }
        Ok(LookupSwitch { default, npairs, matches })
        
    }
}

#[derive(Debug, Clone)]
pub struct TableSwitch {
    default: i32,
    low: i32,
    high: i32,
    offsets: Vec<i32>,
}
impl TableSwitch {
    pub fn load<R: BufferReadable>(buf: &mut R, start: u64) -> Result<TableSwitch, ClassParseError> {
        let bytes_bffr = (buf.seek(SeekFrom::Current(0)).unwrap() - start) % 4; 
        if bytes_bffr != 0 {
            buf.seek(SeekFrom::Current(4 - bytes_bffr as i64)).unwrap();
        }
        let default = buf.read_u4()? as i32;
        let low = buf.read_u4()? as i32;
        let high = buf.read_u4()? as i32;
        let mut offsets = Vec::new();
        for _ in 0..(high - low + 1) {
            let offset = buf.read_u4()? as i32;
            offsets.push(offset);
        }
        Ok(TableSwitch { default, low, high, offsets })
    }
}