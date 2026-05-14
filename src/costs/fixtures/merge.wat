 (module
   (func $f
     block ;; @1
       block ;; @2 - wasm-instrument merges this with @1
         i32.const 1
         drop
         i32.const 2
         drop
         br 1          ;; exit to end of @1
         i32.const 3   ;; never executed
         drop
       end
       i32.const 4     ;; never executed (br 1 jumps past this)
       drop
     end
     i32.const 5       ;; executed
     drop
   )
   (start $f)
 )
