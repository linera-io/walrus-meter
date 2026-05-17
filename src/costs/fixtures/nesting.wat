(module
  (func $f
    block ;; 2
      block ;; 1
        block ;; 0
          i32.const 1
          br_table 1 2 0 2
        end
      end
      (drop (i32.const 1))
    end
  )

  (start $f)
)
