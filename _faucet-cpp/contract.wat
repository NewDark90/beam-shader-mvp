(module
  (type $t0 (func (param i32 i32 i32 i32 i32) (result i32)))
  (type $t1 (func (param i32 i64)))
  (type $t2 (func (result i64)))
  (type $t3 (func))
  (type $t4 (func (param i32)))
  (import "env" "SaveVar" (func $SaveVar (type $t0)))
  (import "env" "FundsLock" (func $FundsLock (type $t1)))
  (import "env" "get_Height" (func $get_Height (type $t2)))
  (import "env" "LoadVar" (func $LoadVar (type $t0)))
  (import "env" "Halt" (func $Halt (type $t3)))
  (import "env" "FundsUnlock" (func $FundsUnlock (type $t1)))
  (import "env" "AddSig" (func $AddSig (type $t4)))
  (func $Ctor (type $t4) (param $p0 i32)
    (local $l1 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l1
    global.set $g0
    local.get $l1
    i32.const 0
    i32.store8 offset=15
    local.get $l1
    i32.const 15
    i32.add
    i32.const 1
    local.get $p0
    i32.const 16
    i32.const 0
    call $SaveVar
    drop
    local.get $l1
    i32.const 16
    i32.add
    global.set $g0)
  (func $Dtor (type $t4) (param $p0 i32)
    (local $l1 i32)
    global.get $g0
    i32.const 16
    i32.sub
    local.tee $l1
    global.set $g0
    local.get $l1
    i32.const 0
    i32.store8 offset=15
    local.get $l1
    i32.const 15
    i32.add
    i32.const 1
    i32.const 0
    i32.const 0
    i32.const 0
    call $SaveVar
    drop
    local.get $l1
    i32.const 16
    i32.add
    global.set $g0)
  (func $Method_2 (type $t4) (param $p0 i32)
    local.get $p0
    i32.load align=1
    local.get $p0
    i64.load offset=4 align=1
    call $FundsLock)
  (func $Method_3 (type $t4) (param $p0 i32)
    (local $l1 i32) (local $l2 i64) (local $l3 i32) (local $l4 i64) (local $l5 i64) (local $l6 i64)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l1
    global.set $g0
    call $get_Height
    local.set $l2
    local.get $l1
    i32.const 0
    i32.store8
    local.get $l1
    i32.const 1
    local.get $l1
    i32.const 16
    i32.add
    i32.const 16
    i32.const 0
    call $LoadVar
    drop
    local.get $p0
    i32.const 37
    local.get $l1
    i32.const 16
    i32.const 0
    call $LoadVar
    local.set $l3
    local.get $l2
    local.get $l1
    i64.load
    i64.sub
    local.set $l4
    local.get $l1
    i64.load offset=16
    local.set $l5
    block $B0
      block $B1
        local.get $p0
        i64.load offset=37 align=1
        local.tee $l6
        i64.eqz
        br_if $B1
        block $B2
          block $B3
            block $B4
              local.get $l4
              local.get $l5
              i64.ge_u
              br_if $B4
              local.get $l3
              i32.const 16
              i32.eq
              br_if $B3
            end
            local.get $l1
            local.get $l6
            i64.store offset=8
            local.get $l1
            local.get $l2
            i64.store
            br $B2
          end
          local.get $l1
          local.get $l1
          i64.load offset=8
          local.tee $l2
          local.get $l6
          i64.add
          local.tee $l6
          i64.store offset=8
          local.get $l6
          local.get $l2
          i64.ge_u
          br_if $B2
          call $Halt
          local.get $l1
          i64.load offset=8
          local.set $l6
        end
        block $B5
          local.get $l6
          local.get $l1
          i64.load offset=24
          i64.le_u
          br_if $B5
          call $Halt
        end
        local.get $p0
        i32.const 37
        local.get $l1
        i32.const 16
        i32.const 0
        call $SaveVar
        drop
        local.get $p0
        i32.load offset=33 align=1
        local.get $p0
        i64.load offset=37 align=1
        call $FundsUnlock
        br $B0
      end
      local.get $l3
      i32.const 16
      i32.ne
      br_if $B0
      local.get $l4
      local.get $l5
      i64.lt_u
      br_if $B0
      local.get $p0
      i32.const 37
      i32.const 0
      i32.const 0
      i32.const 0
      call $SaveVar
      drop
    end
    local.get $p0
    call $AddSig
    local.get $l1
    i32.const 32
    i32.add
    global.set $g0)
  (table $T0 1 1 funcref)
  (memory $memory 2)
  (global $g0 (mut i32) (i32.const 66560))
  (export "memory" (memory 0))
  (export "Ctor" (func $Ctor))
  (export "Dtor" (func $Dtor))
  (export "Method_2" (func $Method_2))
  (export "Method_3" (func $Method_3)))
