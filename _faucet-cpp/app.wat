(module
  (type $t0 (func (param i32)))
  (type $t1 (func (param i32 i32)))
  (type $t2 (func))
  (type $t3 (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type $t4 (func (param i32 i32 i32)))
  (type $t5 (func (param i32 i64)))
  (type $t6 (func (param i32 i32 i32) (result i32)))
  (type $t7 (func (param i32 i32 i32 i32) (result i32)))
  (type $t8 (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)))
  (type $t9 (func (param i32 i32) (result i32)))
  (type $t10 (func (param i32 i32 i32 i32 i32)))
  (import "env" "DocAddGroup" (func $DocAddGroup (type $t0)))
  (import "env" "DocAddText" (func $DocAddText (type $t1)))
  (import "env" "DocCloseGroup" (func $DocCloseGroup (type $t2)))
  (import "env" "DocAddArray" (func $DocAddArray (type $t0)))
  (import "env" "Vars_MoveNext" (func $Vars_MoveNext (type $t3)))
  (import "env" "DocAddBlob" (func $DocAddBlob (type $t4)))
  (import "env" "DocAddNum32" (func $DocAddNum32 (type $t1)))
  (import "env" "DocAddNum64" (func $DocAddNum64 (type $t5)))
  (import "env" "DocCloseArray" (func $DocCloseArray (type $t2)))
  (import "env" "Memcpy" (func $Memcpy (type $t6)))
  (import "env" "Vars_Enum" (func $Vars_Enum (type $t7)))
  (import "env" "Vars_Close" (func $Vars_Close (type $t0)))
  (import "env" "GenerateKernel" (func $GenerateKernel (type $t8)))
  (import "env" "DerivePk" (func $DerivePk (type $t4)))
  (import "env" "DocGetText" (func $DocGetText (type $t6)))
  (import "env" "Strcmp" (func $Strcmp (type $t9)))
  (import "env" "DocGetNum64" (func $DocGetNum64 (type $t9)))
  (import "env" "DocGetBlob" (func $DocGetBlob (type $t6)))
  (import "env" "Memset" (func $Memset (type $t6)))
  (import "env" "DocGetNum32" (func $DocGetNum32 (type $t9)))
  (func $Method_0 (type $t2)
    i32.const 1024
    call $DocAddGroup
    i32.const 1025
    call $DocAddGroup
    i32.const 1031
    call $DocAddGroup
    i32.const 1039
    call $DocAddGroup
    i32.const 1046
    i32.const 1060
    call $DocAddText
    i32.const 1067
    i32.const 1081
    call $DocAddText
    call $DocCloseGroup
    i32.const 1088
    call $DocAddGroup
    i32.const 1096
    i32.const 1100
    call $DocAddText
    call $DocCloseGroup
    i32.const 1111
    call $DocAddGroup
    call $DocCloseGroup
    i32.const 1116
    call $DocAddGroup
    i32.const 1096
    i32.const 1100
    call $DocAddText
    call $DocCloseGroup
    i32.const 1128
    call $DocAddGroup
    i32.const 1096
    i32.const 1100
    call $DocAddText
    call $DocCloseGroup
    i32.const 1139
    call $DocAddGroup
    i32.const 1096
    i32.const 1100
    call $DocAddText
    call $DocCloseGroup
    i32.const 1153
    call $DocAddGroup
    i32.const 1096
    i32.const 1100
    call $DocAddText
    i32.const 1166
    i32.const 1173
    call $DocAddText
    call $DocCloseGroup
    call $DocCloseGroup
    i32.const 1180
    call $DocAddGroup
    i32.const 1111
    call $DocAddGroup
    i32.const 1096
    i32.const 1100
    call $DocAddText
    call $DocCloseGroup
    i32.const 1191
    call $DocAddGroup
    i32.const 1096
    i32.const 1100
    call $DocAddText
    i32.const 1199
    i32.const 1081
    call $DocAddText
    i32.const 1206
    i32.const 1210
    call $DocAddText
    call $DocCloseGroup
    i32.const 1218
    call $DocAddGroup
    i32.const 1096
    i32.const 1100
    call $DocAddText
    i32.const 1199
    i32.const 1081
    call $DocAddText
    i32.const 1206
    i32.const 1210
    call $DocAddText
    call $DocCloseGroup
    call $DocCloseGroup
    call $DocCloseGroup
    call $DocCloseGroup)
  (func $DumpAccounts_Env::VarReaderEx<false>&_ (type $t0) (param $p0 i32)
    (local $l1 i32) (local $l2 i32)
    global.get $g0
    i32.const 96
    i32.sub
    local.tee $l1
    global.set $g0
    i32.const 1233
    call $DocAddArray
    local.get $l1
    i32.const 16
    i32.add
    i32.const 33
    i32.add
    local.set $l2
    loop $L0
      local.get $l1
      i32.const 0
      i32.store8 offset=48
      block $B1
        loop $L2
          local.get $l1
          i32.const 70
          i32.store offset=92
          local.get $l1
          i32.const 16
          i32.store offset=88
          local.get $p0
          i32.load
          local.get $l1
          i32.const 16
          i32.add
          local.get $l1
          i32.const 92
          i32.add
          local.get $l1
          local.get $l1
          i32.const 88
          i32.add
          i32.const 0
          call $Vars_MoveNext
          i32.eqz
          br_if $B1
          local.get $l1
          i32.load offset=92
          i32.const 70
          i32.ne
          br_if $L2
          local.get $l1
          i32.load offset=88
          i32.const 16
          i32.ne
          br_if $L2
        end
        i32.const 1024
        call $DocAddGroup
        i32.const 1242
        local.get $l2
        i32.const 33
        call $DocAddBlob
        i32.const 1210
        local.get $l1
        i32.load offset=82 align=2
        call $DocAddNum32
        i32.const 1081
        local.get $l1
        i64.load offset=8
        call $DocAddNum64
        i32.const 1250
        local.get $l1
        i64.load
        call $DocAddNum64
        call $DocCloseGroup
        br $L0
      end
    end
    call $DocCloseArray
    local.get $l1
    i32.const 96
    i32.add
    global.set $g0)
  (func $DumpAccount_Secp_point_data_const&__Opaque<32u>_const&_ (type $t1) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    global.get $g0
    i32.const 160
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    i32.const 0
    i32.store8 offset=120
    local.get $l2
    i32.const 88
    i32.add
    i32.const 8
    i32.add
    local.get $p1
    i32.const 8
    i32.add
    i64.load align=1
    i64.store
    local.get $l2
    i32.const 88
    i32.add
    i32.const 16
    i32.add
    local.get $p1
    i32.const 16
    i32.add
    i64.load align=1
    i64.store
    local.get $l2
    i32.const 88
    i32.add
    i32.const 24
    i32.add
    local.get $p1
    i32.const 24
    i32.add
    i64.load align=1
    i64.store
    local.get $l2
    i32.const 129
    i32.add
    local.get $p0
    i32.const 8
    i32.add
    i64.load align=1
    i64.store align=1
    local.get $l2
    i32.const 137
    i32.add
    local.get $p0
    i32.const 16
    i32.add
    i64.load align=1
    i64.store align=1
    local.get $l2
    i32.const 145
    i32.add
    local.get $p0
    i32.const 24
    i32.add
    i64.load align=1
    i64.store align=1
    local.get $l2
    i32.const 153
    i32.add
    local.get $p0
    i32.const 32
    i32.add
    i32.load8_u
    i32.store8
    local.get $l2
    i32.const 0
    i32.store8 offset=48
    local.get $l2
    local.get $p1
    i64.load align=1
    i64.store offset=88
    local.get $l2
    local.get $p0
    i64.load align=1
    i64.store offset=121 align=1
    local.get $l2
    i32.const 88
    i32.add
    i32.const 66
    i32.add
    i32.const 0
    i32.store align=2
    local.get $l2
    i32.const 16
    i32.add
    local.get $l2
    i32.const 88
    i32.add
    i32.const 70
    call $Memcpy
    drop
    local.get $l2
    i32.const 16
    i32.add
    i32.const 66
    i32.add
    i32.const -1
    i32.store align=2
    local.get $l2
    local.get $l2
    i32.const 88
    i32.add
    i32.const 70
    local.get $l2
    i32.const 16
    i32.add
    i32.const 70
    call $Vars_Enum
    local.tee $p0
    i32.store offset=8
    local.get $l2
    i32.const 8
    i32.add
    call $DumpAccounts_Env::VarReaderEx<false>&_
    local.get $p0
    call $Vars_Close
    local.get $l2
    i32.const 160
    i32.add
    global.set $g0)
  (func $EnumAndDumpContracts_Opaque<32u>_const&_ (type $t0) (param $p0 i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i64)
    global.get $g0
    i32.const 128
    i32.sub
    local.tee $l1
    global.set $g0
    i32.const 1545
    call $DocAddArray
    local.get $l1
    i32.const 0
    i32.store offset=112
    local.get $l1
    i32.const 0
    i32.store8 offset=32
    local.get $l1
    local.get $p0
    call $WalkerContracts::Enum_Opaque<32u>_const*_
    local.get $l1
    i32.const 65
    i32.add
    local.set $l2
    local.get $l1
    i32.const 104
    i32.add
    local.set $p0
    block $B0
      loop $L1
        local.get $l1
        i32.const 97
        i32.store offset=124
        local.get $l1
        i32.const 8
        i32.store offset=120
        local.get $l1
        i32.load offset=112
        local.get $l1
        local.get $l1
        i32.const 124
        i32.add
        local.get $p0
        local.get $l1
        i32.const 120
        i32.add
        i32.const 0
        call $Vars_MoveNext
        i32.eqz
        br_if $B0
        local.get $l1
        i32.load offset=124
        i32.const 97
        i32.ne
        br_if $L1
        local.get $l1
        i32.load offset=120
        i32.const 8
        i32.ne
        br_if $L1
        local.get $l1
        local.get $l1
        i64.load offset=104
        local.tee $l3
        i64.const 56
        i64.shl
        local.get $l3
        i64.const 40
        i64.shl
        i64.const 71776119061217280
        i64.and
        i64.or
        local.get $l3
        i64.const 24
        i64.shl
        i64.const 280375465082880
        i64.and
        local.get $l3
        i64.const 8
        i64.shl
        i64.const 1095216660480
        i64.and
        i64.or
        i64.or
        local.get $l3
        i64.const 8
        i64.shr_u
        i64.const 4278190080
        i64.and
        local.get $l3
        i64.const 24
        i64.shr_u
        i64.const 16711680
        i64.and
        i64.or
        local.get $l3
        i64.const 40
        i64.shr_u
        i64.const 65280
        i64.and
        local.get $l3
        i64.const 56
        i64.shr_u
        i64.or
        i64.or
        i64.or
        i64.store offset=104
        i32.const 1024
        call $DocAddGroup
        i32.const 1096
        local.get $l2
        i32.const 32
        call $DocAddBlob
        i32.const 1060
        local.get $l1
        i64.load offset=104
        call $DocAddNum64
        call $DocCloseGroup
        br $L1
      end
    end
    block $B2
      local.get $l1
      i32.load offset=112
      local.tee $p0
      i32.eqz
      br_if $B2
      local.get $p0
      call $Vars_Close
    end
    call $DocCloseArray
    local.get $l1
    i32.const 128
    i32.add
    global.set $g0)
  (func $WalkerContracts::Enum_Opaque<32u>_const*_ (type $t1) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    global.get $g0
    i32.const 208
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    i32.const 0
    i32.store8 offset=136
    local.get $l2
    i32.const 0
    i32.store8 offset=32
    local.get $l2
    i32.const 104
    i32.add
    i32.const 0
    i32.const 97
    call $Memset
    drop
    local.get $l2
    i32.const 16
    i32.store8 offset=136
    block $B0
      block $B1
        local.get $p1
        i32.eqz
        br_if $B1
        local.get $l2
        i32.const 137
        i32.add
        local.get $p1
        i32.const 32
        call $Memcpy
        drop
        local.get $l2
        local.get $l2
        i32.const 104
        i32.add
        i32.const 97
        call $Memcpy
        drop
        local.get $l2
        i32.const 65
        i32.add
        local.set $p1
        br $B0
      end
      local.get $l2
      local.get $l2
      i32.const 104
      i32.add
      i32.const 97
      call $Memcpy
      drop
      local.get $l2
      i32.const 65
      i32.add
      i32.const 255
      i32.const 32
      call $Memset
      drop
      local.get $l2
      i32.const 33
      i32.add
      local.set $p1
    end
    local.get $p1
    i32.const 255
    i32.const 32
    call $Memset
    drop
    block $B2
      local.get $p0
      i32.load offset=112
      local.tee $p1
      i32.eqz
      br_if $B2
      local.get $p1
      call $Vars_Close
    end
    local.get $p0
    local.get $l2
    i32.const 104
    i32.add
    i32.const 97
    local.get $l2
    i32.const 97
    call $Vars_Enum
    i32.store offset=112
    local.get $l2
    i32.const 208
    i32.add
    global.set $g0)
  (func $On_manager_view_params_Opaque<32u>_const&__int_ (type $t1) (param $p0 i32) (param $p1 i32)
    (local $l2 i32)
    global.get $g0
    i32.const 64
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    i32.const 24
    i32.add
    i32.const 8
    i32.add
    local.get $p0
    i32.const 8
    i32.add
    i64.load align=1
    i64.store
    local.get $l2
    i32.const 24
    i32.add
    i32.const 16
    i32.add
    local.get $p0
    i32.const 16
    i32.add
    i64.load align=1
    i64.store
    local.get $l2
    i32.const 24
    i32.add
    i32.const 24
    i32.add
    local.get $p0
    i32.const 24
    i32.add
    i64.load align=1
    i64.store
    local.get $l2
    i32.const 0
    i32.store16 offset=56
    local.get $l2
    local.get $p0
    i64.load align=1
    i64.store offset=24
    block $B0
      block $B1
        local.get $l2
        i32.const 24
        i32.add
        local.get $l2
        i32.const 8
        i32.add
        call $bool_Env::VarReaderEx<false>::Read_T<Env::Key_T<unsigned_char>__Faucet::Params>_Env::Key_T<unsigned_char>_const&__Faucet::Params&_
        br_if $B1
        i32.const 1227
        i32.const 1373
        call $DocAddText
        br $B0
      end
      i32.const 1388
      call $DocAddGroup
      i32.const 1046
      local.get $l2
      i64.load offset=8
      call $DocAddNum64
      i32.const 1067
      local.get $l2
      i64.load offset=16
      call $DocAddNum64
      call $DocCloseGroup
    end
    local.get $l2
    i32.const 64
    i32.add
    global.set $g0)
  (func $bool_Env::VarReaderEx<false>::Read_T<Env::Key_T<unsigned_char>__Faucet::Params>_Env::Key_T<unsigned_char>_const&__Faucet::Params&_ (type $t9) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32) (local $l3 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $p0
    i32.const 34
    local.get $p0
    i32.const 34
    call $Vars_Enum
    local.set $p0
    local.get $l2
    i32.const 0
    i32.store offset=12
    local.get $l2
    i32.const 16
    i32.store offset=8
    local.get $p0
    i32.const 0
    local.get $l2
    i32.const 12
    i32.add
    local.get $p1
    local.get $l2
    i32.const 8
    i32.add
    i32.const 0
    call $Vars_MoveNext
    local.set $p1
    local.get $l2
    i32.load offset=8
    local.set $l3
    local.get $p0
    call $Vars_Close
    local.get $l2
    i32.const 16
    i32.add
    global.set $g0
    local.get $p1
    i32.const 0
    i32.ne
    local.get $l3
    i32.const 16
    i32.eq
    i32.and)
  (func $On_manager_view_funds_Opaque<32u>_const&__int_ (type $t1) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i64) (local $l4 i32)
    global.get $g0
    i32.const 96
    i32.sub
    local.tee $l2
    global.set $g0
    i32.const 1395
    call $DocAddArray
    local.get $l2
    i32.const 48
    i32.add
    i32.const 24
    i32.add
    local.get $p0
    i32.const 24
    i32.add
    i64.load align=1
    i64.store
    local.get $l2
    i32.const 48
    i32.add
    i32.const 16
    i32.add
    local.get $p0
    i32.const 16
    i32.add
    i64.load align=1
    i64.store
    local.get $l2
    i32.const 48
    i32.add
    i32.const 8
    i32.add
    local.get $p0
    i32.const 8
    i32.add
    i64.load align=1
    i64.store
    local.get $l2
    i32.const 1
    i32.store8 offset=80
    local.get $l2
    local.get $p0
    i64.load align=1
    i64.store offset=48
    local.get $l2
    i32.const 0
    i32.store8 offset=40
    local.get $l2
    i32.const 8
    i32.add
    local.get $l2
    i32.const 48
    i32.add
    i32.const 33
    call $Memcpy
    drop
    local.get $l2
    i32.const -1
    i32.store offset=41 align=1
    local.get $l2
    i32.const 0
    i32.store offset=81 align=1
    local.get $l2
    i32.const 48
    i32.add
    i32.const 37
    local.get $l2
    i32.const 8
    i32.add
    i32.const 37
    call $Vars_Enum
    local.set $p0
    loop $L0
      local.get $l2
      i32.const 0
      i32.store8 offset=80
      block $B1
        loop $L2
          local.get $l2
          i32.const 37
          i32.store offset=92
          local.get $l2
          i32.const 16
          i32.store offset=88
          local.get $p0
          local.get $l2
          i32.const 48
          i32.add
          local.get $l2
          i32.const 92
          i32.add
          local.get $l2
          i32.const 8
          i32.add
          local.get $l2
          i32.const 88
          i32.add
          i32.const 0
          call $Vars_MoveNext
          i32.eqz
          br_if $B1
          local.get $l2
          i32.load offset=92
          i32.const 37
          i32.ne
          br_if $L2
          local.get $l2
          i32.load offset=88
          i32.const 16
          i32.ne
          br_if $L2
        end
        local.get $l2
        i64.load offset=16
        local.set $l3
        local.get $l2
        i32.load offset=81 align=1
        local.set $l4
        i32.const 1024
        call $DocAddGroup
        i32.const 1401
        local.get $l4
        i32.const 24
        i32.shl
        local.get $l4
        i32.const 8
        i32.shl
        i32.const 16711680
        i32.and
        i32.or
        local.get $l4
        i32.const 8
        i32.shr_u
        i32.const 65280
        i32.and
        local.get $l4
        i32.const 24
        i32.shr_u
        i32.or
        i32.or
        call $DocAddNum32
        i32.const 1081
        local.get $l3
        i64.const 56
        i64.shl
        local.get $l3
        i64.const 40
        i64.shl
        i64.const 71776119061217280
        i64.and
        i64.or
        local.get $l3
        i64.const 24
        i64.shl
        i64.const 280375465082880
        i64.and
        local.get $l3
        i64.const 8
        i64.shl
        i64.const 1095216660480
        i64.and
        i64.or
        i64.or
        local.get $l3
        i64.const 8
        i64.shr_u
        i64.const 4278190080
        i64.and
        local.get $l3
        i64.const 24
        i64.shr_u
        i64.const 16711680
        i64.and
        i64.or
        local.get $l3
        i64.const 40
        i64.shr_u
        i64.const 65280
        i64.and
        local.get $l3
        i64.const 56
        i64.shr_u
        i64.or
        i64.or
        i64.or
        call $DocAddNum64
        call $DocCloseGroup
        br $L0
      end
    end
    block $B3
      local.get $p0
      i32.eqz
      br_if $B3
      local.get $p0
      call $Vars_Close
    end
    call $DocCloseArray
    local.get $l2
    i32.const 96
    i32.add
    global.set $g0)
  (func $On_manager_view_accounts_Opaque<32u>_const&__int_ (type $t1) (param $p0 i32) (param $p1 i32)
    (local $l2 i32) (local $l3 i64) (local $l4 i64) (local $l5 i64)
    global.get $g0
    i32.const 96
    i32.sub
    local.tee $l2
    global.set $g0
    local.get $l2
    i32.const 0
    i32.store8 offset=88
    local.get $l2
    i32.const 56
    i32.add
    i32.const 8
    i32.add
    local.get $p0
    i32.const 8
    i32.add
    i64.load align=1
    local.tee $l3
    i64.store
    local.get $l2
    i32.const 56
    i32.add
    i32.const 16
    i32.add
    local.get $p0
    i32.const 16
    i32.add
    i64.load align=1
    local.tee $l4
    i64.store
    local.get $l2
    i32.const 56
    i32.add
    i32.const 24
    i32.add
    local.get $p0
    i32.const 24
    i32.add
    i64.load align=1
    local.tee $l5
    i64.store
    local.get $l2
    i32.const 16
    i32.add
    i32.const 8
    i32.add
    local.get $l3
    i64.store
    local.get $l2
    i32.const 16
    i32.add
    i32.const 16
    i32.add
    local.get $l4
    i64.store
    local.get $l2
    i32.const 16
    i32.add
    i32.const 24
    i32.add
    local.get $l5
    i64.store
    local.get $l2
    i32.const 1
    i32.store8 offset=48
    local.get $l2
    local.get $p0
    i64.load align=1
    local.tee $l3
    i64.store offset=56
    local.get $l2
    local.get $l3
    i64.store offset=16
    local.get $l2
    local.get $l2
    i32.const 56
    i32.add
    i32.const 33
    local.get $l2
    i32.const 16
    i32.add
    i32.const 33
    call $Vars_Enum
    local.tee $p0
    i32.store offset=8
    local.get $l2
    i32.const 8
    i32.add
    call $DumpAccounts_Env::VarReaderEx<false>&_
    local.get $p0
    call $Vars_Close
    local.get $l2
    i32.const 96
    i32.add
    global.set $g0)
  (func $On_my_account_move_unsigned_char_const&__Opaque<32u>_const&__unsigned_long_long_const&__unsigned_int_const&__int_ (type $t10) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32) (param $p4 i32)
    (local $l5 i32) (local $l6 i64)
    global.get $g0
    i32.const 80
    i32.sub
    local.tee $l5
    global.set $g0
    block $B0
      block $B1
        local.get $p2
        i64.load
        local.tee $l6
        i64.const 0
        i64.ne
        br_if $B1
        i32.const 1227
        i32.const 1405
        call $DocAddText
        br $B0
      end
      local.get $l5
      local.get $l6
      i64.store offset=64
      local.get $l5
      local.get $p3
      i32.load
      local.tee $p2
      i32.store offset=72
      local.get $l5
      local.get $p0
      i32.load8_u
      local.tee $p0
      i32.store8 offset=76
      block $B2
        local.get $p0
        i32.eqz
        br_if $B2
        local.get $l5
        local.get $l6
        i64.store offset=20 align=4
        local.get $l5
        local.get $p2
        i32.store offset=16
        local.get $p1
        i32.const 2
        local.get $l5
        i32.const 16
        i32.add
        i32.const 12
        local.get $l5
        i32.const 64
        i32.add
        i32.const 1
        i32.const 0
        i32.const 0
        i32.const 1426
        i32.const 0
        call $GenerateKernel
        br $B0
      end
      local.get $l5
      local.get $p2
      i32.store offset=49 align=1
      local.get $l5
      local.get $l6
      i64.store offset=53 align=1
      local.get $l5
      i32.const 16
      i32.add
      local.get $p1
      i32.const 32
      call $DerivePk
      local.get $l5
      i32.const 32
      i32.store offset=12
      local.get $l5
      local.get $p1
      i32.store offset=8
      local.get $p1
      i32.const 3
      local.get $l5
      i32.const 16
      i32.add
      i32.const 45
      local.get $l5
      i32.const 64
      i32.add
      i32.const 1
      local.get $l5
      i32.const 8
      i32.add
      i32.const 1
      i32.const 1444
      i32.const 0
      call $GenerateKernel
    end
    local.get $l5
    i32.const 80
    i32.add
    global.set $g0)
  (func $Method_1 (type $t2)
    (local $l0 i32) (local $l1 i64) (local $l2 i64)
    global.get $g0
    i32.const 128
    i32.sub
    local.tee $l0
    global.set $g0
    i32.const 1024
    call $DocAddGroup
    block $B0
      block $B1
        i32.const 1465
        local.get $l0
        i32.const 64
        i32.add
        i32.const 16
        call $DocGetText
        br_if $B1
        i32.const 1227
        i32.const 1470
        call $DocAddText
        br $B0
      end
      block $B2
        i32.const 1489
        local.get $l0
        i32.const 48
        i32.add
        i32.const 16
        call $DocGetText
        br_if $B2
        i32.const 1227
        i32.const 1496
        call $DocAddText
        br $B0
      end
      block $B3
        local.get $l0
        i32.const 64
        i32.add
        i32.const 1031
        call $Strcmp
        br_if $B3
        block $B4
          local.get $l0
          i32.const 48
          i32.add
          i32.const 1039
          call $Strcmp
          br_if $B4
          local.get $l0
          i64.const 0
          i64.store offset=8
          i32.const 1046
          local.get $l0
          i32.const 8
          i32.add
          call $DocGetNum64
          drop
          local.get $l0
          i64.const 0
          i64.store offset=40
          i32.const 1067
          local.get $l0
          i32.const 40
          i32.add
          call $DocGetNum64
          drop
          block $B5
            block $B6
              local.get $l0
              i64.load offset=8
              local.tee $l1
              i64.eqz
              br_if $B6
              local.get $l0
              i64.load offset=40
              local.tee $l2
              i64.const 0
              i64.ne
              br_if $B5
            end
            i32.const 1227
            i32.const 1285
            call $DocAddText
            br $B0
          end
          local.get $l0
          local.get $l2
          i64.store offset=96
          local.get $l0
          local.get $l1
          i64.store offset=88
          i32.const 0
          i32.const 0
          local.get $l0
          i32.const 88
          i32.add
          i32.const 16
          i32.const 0
          i32.const 0
          i32.const 0
          i32.const 0
          i32.const 1326
          i32.const 0
          call $GenerateKernel
          br $B0
        end
        block $B7
          local.get $l0
          i32.const 48
          i32.add
          i32.const 1088
          call $Strcmp
          br_if $B7
          block $B8
            i32.const 1096
            local.get $l0
            i32.const 88
            i32.add
            i32.const 32
            call $DocGetBlob
            i32.const 32
            i32.eq
            br_if $B8
            local.get $l0
            i32.const 88
            i32.add
            i32.const 0
            i32.const 32
            call $Memset
            drop
          end
          local.get $l0
          i32.const 88
          i32.add
          i32.const 1
          i32.const 0
          i32.const 0
          i32.const 0
          i32.const 0
          i32.const 0
          i32.const 0
          i32.const 1349
          i32.const 0
          call $GenerateKernel
          br $B0
        end
        block $B9
          local.get $l0
          i32.const 48
          i32.add
          i32.const 1111
          call $Strcmp
          br_if $B9
          i32.const 1253
          call $EnumAndDumpContracts_Opaque<32u>_const&_
          br $B0
        end
        block $B10
          local.get $l0
          i32.const 48
          i32.add
          i32.const 1116
          call $Strcmp
          br_if $B10
          block $B11
            i32.const 1096
            local.get $l0
            i32.const 88
            i32.add
            i32.const 32
            call $DocGetBlob
            i32.const 32
            i32.eq
            br_if $B11
            local.get $l0
            i32.const 88
            i32.add
            i32.const 0
            i32.const 32
            call $Memset
            drop
          end
          local.get $l0
          i32.const 88
          i32.add
          local.get $l0
          call $On_manager_view_params_Opaque<32u>_const&__int_
          br $B0
        end
        block $B12
          local.get $l0
          i32.const 48
          i32.add
          i32.const 1128
          call $Strcmp
          br_if $B12
          block $B13
            i32.const 1096
            local.get $l0
            i32.const 88
            i32.add
            i32.const 32
            call $DocGetBlob
            i32.const 32
            i32.eq
            br_if $B13
            local.get $l0
            i32.const 88
            i32.add
            i32.const 0
            i32.const 32
            call $Memset
            drop
          end
          local.get $l0
          i32.const 88
          i32.add
          local.get $l0
          call $On_manager_view_funds_Opaque<32u>_const&__int_
          br $B0
        end
        block $B14
          local.get $l0
          i32.const 48
          i32.add
          i32.const 1139
          call $Strcmp
          br_if $B14
          block $B15
            i32.const 1096
            local.get $l0
            i32.const 88
            i32.add
            i32.const 32
            call $DocGetBlob
            i32.const 32
            i32.eq
            br_if $B15
            local.get $l0
            i32.const 88
            i32.add
            i32.const 0
            i32.const 32
            call $Memset
            drop
          end
          local.get $l0
          i32.const 88
          i32.add
          local.get $l0
          call $On_manager_view_accounts_Opaque<32u>_const&__int_
          br $B0
        end
        block $B16
          local.get $l0
          i32.const 48
          i32.add
          i32.const 1153
          call $Strcmp
          br_if $B16
          block $B17
            i32.const 1096
            local.get $l0
            i32.const 8
            i32.add
            i32.const 32
            call $DocGetBlob
            i32.const 32
            i32.eq
            br_if $B17
            local.get $l0
            i32.const 8
            i32.add
            i32.const 0
            i32.const 32
            call $Memset
            drop
          end
          block $B18
            i32.const 1166
            local.get $l0
            i32.const 88
            i32.add
            i32.const 33
            call $DocGetBlob
            i32.const 33
            i32.eq
            br_if $B18
            local.get $l0
            i32.const 88
            i32.add
            i32.const 0
            i32.const 33
            call $Memset
            drop
          end
          local.get $l0
          i32.const 88
          i32.add
          local.get $l0
          i32.const 8
          i32.add
          call $DumpAccount_Secp_point_data_const&__Opaque<32u>_const&_
          br $B0
        end
        i32.const 1227
        i32.const 1517
        call $DocAddText
        br $B0
      end
      block $B19
        local.get $l0
        i32.const 64
        i32.add
        i32.const 1180
        call $Strcmp
        br_if $B19
        block $B20
          local.get $l0
          i32.const 48
          i32.add
          i32.const 1111
          call $Strcmp
          br_if $B20
          block $B21
            i32.const 1096
            local.get $l0
            i32.const 8
            i32.add
            i32.const 32
            call $DocGetBlob
            i32.const 32
            i32.eq
            br_if $B21
            local.get $l0
            i32.const 8
            i32.add
            i32.const 0
            i32.const 32
            call $Memset
            drop
          end
          local.get $l0
          i32.const 88
          i32.add
          local.get $l0
          i32.const 8
          i32.add
          i32.const 32
          call $DerivePk
          local.get $l0
          i32.const 88
          i32.add
          local.get $l0
          i32.const 8
          i32.add
          call $DumpAccount_Secp_point_data_const&__Opaque<32u>_const&_
          br $B0
        end
        block $B22
          local.get $l0
          i32.const 48
          i32.add
          i32.const 1191
          call $Strcmp
          br_if $B22
          block $B23
            i32.const 1096
            local.get $l0
            i32.const 88
            i32.add
            i32.const 32
            call $DocGetBlob
            i32.const 32
            i32.eq
            br_if $B23
            local.get $l0
            i32.const 88
            i32.add
            i32.const 0
            i32.const 32
            call $Memset
            drop
          end
          local.get $l0
          i64.const 0
          i64.store offset=8
          i32.const 1199
          local.get $l0
          i32.const 8
          i32.add
          call $DocGetNum64
          drop
          local.get $l0
          i32.const 0
          i32.store offset=40
          i32.const 1206
          local.get $l0
          i32.const 40
          i32.add
          call $DocGetNum32
          drop
          local.get $l0
          i32.const 1
          i32.store8 offset=127
          local.get $l0
          i32.const 127
          i32.add
          local.get $l0
          i32.const 88
          i32.add
          local.get $l0
          i32.const 8
          i32.add
          local.get $l0
          i32.const 40
          i32.add
          local.get $l0
          call $On_my_account_move_unsigned_char_const&__Opaque<32u>_const&__unsigned_long_long_const&__unsigned_int_const&__int_
          br $B0
        end
        block $B24
          local.get $l0
          i32.const 48
          i32.add
          i32.const 1218
          call $Strcmp
          br_if $B24
          block $B25
            i32.const 1096
            local.get $l0
            i32.const 88
            i32.add
            i32.const 32
            call $DocGetBlob
            i32.const 32
            i32.eq
            br_if $B25
            local.get $l0
            i32.const 88
            i32.add
            i32.const 0
            i32.const 32
            call $Memset
            drop
          end
          local.get $l0
          i64.const 0
          i64.store offset=8
          i32.const 1199
          local.get $l0
          i32.const 8
          i32.add
          call $DocGetNum64
          drop
          local.get $l0
          i32.const 0
          i32.store offset=40
          i32.const 1206
          local.get $l0
          i32.const 40
          i32.add
          call $DocGetNum32
          drop
          local.get $l0
          i32.const 0
          i32.store8 offset=127
          local.get $l0
          i32.const 127
          i32.add
          local.get $l0
          i32.const 88
          i32.add
          local.get $l0
          i32.const 8
          i32.add
          local.get $l0
          i32.const 40
          i32.add
          local.get $l0
          call $On_my_account_move_unsigned_char_const&__Opaque<32u>_const&__unsigned_long_long_const&__unsigned_int_const&__int_
          br $B0
        end
        i32.const 1227
        i32.const 1517
        call $DocAddText
        br $B0
      end
      i32.const 1227
      i32.const 1532
      call $DocAddText
    end
    call $DocCloseGroup
    local.get $l0
    i32.const 128
    i32.add
    global.set $g0)
  (table $T0 1 1 funcref)
  (memory $memory 2)
  (global $g0 (mut i32) (i32.const 67104))
  (export "memory" (memory 0))
  (export "Method_0" (func $Method_0))
  (export "Method_1" (func $Method_1))
  (data $d0 (i32.const 1024) "\00roles\00manager\00create\00backlogPeriod\00Height\00withdrawLimit\00Amount\00destroy\00cid\00ContractID\00view\00view_params\00view_funds\00view_accounts\00view_account\00pubKey\00PubKey\00my_account\00deposit\00amount\00aid\00AssetID\00withdraw\00error\00accounts\00Account\00h0\00\e3!\90DB\00\0f\fd-/\f0F?\8aw!\1b\82\e4a\d3\97\e1\b2 \8c\c9;\b6\a1\04ebacklog and withdraw limit should be nnz\00create Faucet contract\00destroy Faucet contract\00failed to read\00params\00funds\00Aid\00amount should be nnz\00deposit to Faucet\00withdraw from Faucet\00role\00Role not specified\00action\00Action not specified\00invalid Action\00unknown Role\00contracts\00"))
