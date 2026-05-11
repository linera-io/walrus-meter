(module $loop-39a729d35e71e94c.wasm
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i32)))
  (type (;4;) (func (param i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (param i64)))
  (import "host" "spend" (func (;0;) (type 6)))
  (table (;0;) 1 1 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048920)
  (global (;2;) i32 i32.const 1048928)
  (export "memory" (memory 0))
  (export "run" (func $run))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (func $_ZN4core3num21_$LT$impl$u20$u32$GT$13unchecked_add18precondition_check17h3e954603ce7f5d5eE (;1;) (type 5) (param i32 i32 i32)
    (local i32 i32 i32)
    i64.const 18
    call 0
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.set 3
    local.get 3
    global.set $__stack_pointer
    local.get 3
    local.get 0
    i32.store offset=44
    local.get 3
    local.get 1
    i32.store offset=48
    local.get 3
    i32.const 1048728
    i32.store offset=52
    local.get 3
    i32.const 184
    i32.store offset=56
    block ;; label = @1
      i64.const 8
      call 0
      local.get 0
      local.get 1
      i32.add
      local.get 0
      i32.lt_u
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      i64.const 5
      call 0
      local.get 3
      i32.const 64
      i32.add
      global.set $__stack_pointer
      return
    end
    i64.const 45
    call 0
    local.get 3
    i32.const 1048728
    i32.store offset=36
    local.get 3
    i32.const 184
    i32.store offset=40
    local.get 3
    local.get 3
    i32.const 36
    i32.add
    i32.store offset=60
    local.get 3
    local.get 3
    i32.const 36
    i32.add
    i32.store offset=12
    local.get 3
    i32.const 1
    i32.store offset=16
    i32.const 0
    i32.load offset=1048912
    local.set 4
    i32.const 0
    i32.load offset=1048916
    local.set 5
    local.get 3
    local.get 4
    i32.store offset=28
    local.get 3
    local.get 5
    i32.store offset=32
    local.get 3
    i32.const 4
    i32.store offset=20
    local.get 3
    i32.const 0
    i32.store offset=24
    local.get 3
    i32.const 12
    i32.add
    i32.const 0
    i32.const 1
    i32.and
    local.get 2
    call $_ZN4core9panicking18panic_nounwind_fmt17h6f1b873ef106ada8E
    unreachable
  )
  (func $run (;2;) (type 1) (param i32)
    (local i32 i32 i32 i32 i32)
    i64.const 27
    call 0
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.set 1
    local.get 1
    global.set $__stack_pointer
    local.get 1
    local.get 0
    i32.store offset=40
    i32.const 0
    local.set 2
    local.get 1
    i32.const 16
    i32.add
    local.get 2
    local.get 0
    call $_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h5d0848ce79c6b573E
    local.get 1
    i32.load offset=20
    local.set 3
    local.get 1
    local.get 1
    i32.load offset=16
    i32.store offset=24
    local.get 1
    local.get 3
    i32.store offset=28
    block ;; label = @1
      i64.const 0
      call 0
      loop ;; label = @2
        i64.const 23
        call 0
        local.get 1
        i32.const 8
        i32.add
        local.get 1
        i32.const 24
        i32.add
        call $_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h51be710b7e82c384E
        local.get 1
        i32.load offset=12
        local.set 4
        local.get 1
        local.get 1
        i32.load offset=8
        i32.store offset=32
        local.get 1
        local.get 4
        i32.store offset=36
        local.get 1
        i32.load offset=32
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        i64.const 9
        call 0
        local.get 1
        i32.load offset=36
        local.set 5
        local.get 1
        local.get 5
        i32.store offset=44
        local.get 5
        call $_ZN4core4hint9black_box17h76542dfadf2e1770E
        drop
        br 0 (;@2;)
      end
    end
    i64.const 5
    call 0
    local.get 1
    i32.const 48
    i32.add
    global.set $__stack_pointer
    return
  )
  (func $_ZN89_$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$9spec_next17h57552b6448674138E (;3;) (type 3) (param i32 i32)
    (local i32 i32 i32 i32)
    i64.const 19
    call 0
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.set 2
    local.get 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    i32.store offset=16
    local.get 1
    i32.const 4
    i32.add
    local.set 3
    local.get 2
    local.get 1
    i32.store offset=24
    local.get 2
    local.get 3
    i32.store offset=28
    block ;; label = @1
      i64.const 0
      call 0
      block ;; label = @2
        i64.const 8
        call 0
        local.get 1
        i32.load
        local.get 1
        i32.load offset=4
        i32.lt_u
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        i64.const 4
        call 0
        local.get 2
        i32.const 0
        i32.store offset=8
        br 1 (;@1;)
      end
      i64.const 17
      call 0
      local.get 1
      i32.load
      local.set 4
      local.get 2
      local.get 4
      i32.store offset=20
      local.get 1
      local.get 4
      i32.const 1
      call $_ZN47_$LT$u32$u20$as$u20$core..iter..range..Step$GT$17forward_unchecked17h5a0d44615613dbdaE
      i32.store
      local.get 2
      local.get 4
      i32.store offset=12
      local.get 2
      i32.const 1
      i32.store offset=8
    end
    i64.const 15
    call 0
    local.get 2
    i32.load offset=8
    local.set 5
    local.get 0
    local.get 2
    i32.load offset=12
    i32.store offset=4
    local.get 0
    local.get 5
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    return
  )
  (func $_ZN4core9panicking18panic_nounwind_fmt17h6f1b873ef106ada8E (;4;) (type 5) (param i32 i32 i32)
    (local i32)
    i64.const 47
    call 0
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 0
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 3
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 3
    local.get 0
    i64.load align=4
    i64.store offset=8
    local.get 3
    local.get 1
    i32.store8 offset=45
    local.get 3
    i32.const 0
    i32.store8 offset=44
    local.get 3
    local.get 2
    i32.store offset=40
    local.get 3
    local.get 3
    i32.const 8
    i32.add
    i32.store offset=36
    local.get 3
    i32.const 36
    i32.add
    call $_RNvCseDuWfX95ZFZ_7___rustc17rust_begin_unwind
    unreachable
  )
  (func $_ZN47_$LT$u32$u20$as$u20$core..iter..range..Step$GT$17forward_unchecked17h5a0d44615613dbdaE (;5;) (type 4) (param i32 i32) (result i32)
    (local i32 i32)
    i64.const 29
    call 0
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.set 2
    local.get 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    local.get 1
    i32.store offset=8
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 0
    local.get 1
    i32.const 1048712
    call $_ZN4core3num21_$LT$impl$u20$u32$GT$13unchecked_add18precondition_check17h3e954603ce7f5d5eE
    local.get 0
    local.get 1
    i32.add
    local.set 3
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3
    return
  )
  (func $_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h51be710b7e82c384E (;6;) (type 3) (param i32 i32)
    (local i32 i32)
    i64.const 27
    call 0
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.set 2
    local.get 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 2
    local.get 1
    call $_ZN89_$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$9spec_next17h57552b6448674138E
    local.get 2
    i32.load
    local.set 3
    local.get 0
    local.get 2
    i32.load offset=4
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    return
  )
  (func $_ZN4core4hint9black_box17h76542dfadf2e1770E (;7;) (type 2) (param i32) (result i32)
    (local i32 i32)
    i64.const 17
    call 0
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.set 1
    local.get 1
    local.get 0
    i32.store offset=8
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.const 12
    i32.add
    local.set 2
    local.get 1
    i32.load offset=12
    return
  )
  (func $_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h5d0848ce79c6b573E (;8;) (type 5) (param i32 i32 i32)
    (local i32)
    i64.const 17
    call 0
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.set 3
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 3
    local.get 2
    i32.store offset=12
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    return
  )
  (func $_RNvCseDuWfX95ZFZ_7___rustc17rust_begin_unwind (;9;) (type 1) (param i32)
    (local i32)
    i64.const 10
    call 0
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.set 1
    local.get 1
    global.set $__stack_pointer
    local.get 1
    local.get 0
    i32.store offset=12
    call $_ZN4core9core_arch6wasm3211unreachable17hba35bd823e1b3316E
    unreachable
  )
  (func $_ZN4core9core_arch6wasm3211unreachable17hba35bd823e1b3316E (;10;) (type 0)
    i64.const 0
    call 0
    unreachable
  )
  (data $.rodata (;0;) (i32.const 1048576) "/nix/store/yn1gs8qj4698s8kwzl13vad09jv95361-rust-minimal-1.92.0-nightly-2025-10-20/lib/rustlib/src/rust/library/core/src/iter/range.rs\00\00\00\00\10\00\86\00\00\00\b1\01\00\00\01\00\00\00unsafe precondition(s) violated: u32::unchecked_add cannot overflow\0a\0aThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.\00\00\00\00\00\00\00\00")
  (@producers
    (language "Rust" "")
    (processed-by "rustc" "1.92.0-nightly (f04e3dfc8 2025-10-19)")
    (processed-by "walrus" "0.26.1")
  )
  (@custom "target_features" (after data) "\08+\0bbulk-memory+\0fbulk-memory-opt+\16call-indirect-overlong+\0amultivalue+\0fmutable-globals+\13nontrapping-fptoint+\0freference-types+\08sign-ext")
)
