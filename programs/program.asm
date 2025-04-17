
is_valid:
    mov64 r0, 0
    ldxw r2, [r1+0x0]
    jne r2, 0, lbb_5
    add64 r1, 8
    call raydium_is_valid
lbb_5:
    exit

get_quote:
    ldxw r4, [r1+0x0]
    jeq r4, 1, lbb_13
    mov64 r0, 0
    jne r4, 0, lbb_15
    add64 r1, 8
    call raydium_get_quote
    ja lbb_15
lbb_13:
    add64 r1, 8
    call pump_fun_get_quote
lbb_15:
    exit

get_liquidity:
    ldxw r5, [r1+0x0]
    jeq r5, 1, lbb_25
    jne r5, 0, lbb_30
    mov64 r5, 0
    stxw [r4+0x0], r5
    add64 r1, 8
    add64 r4, 8
    call raydium_get_liquidity
    ja lbb_30
lbb_25:
    mov64 r5, 1
    stxw [r4+0x0], r5
    add64 r1, 8
    add64 r4, 8
    call pump_fun_get_liquidity
lbb_30:
    exit

get_quote_and_liquidity:
    ldxw r5, [r1+0x0]
    jeq r5, 1, lbb_41
    mov64 r0, 0
    jne r5, 0, lbb_46
    mov64 r5, 0
    stxw [r4+0x0], r5
    add64 r1, 8
    add64 r4, 8
    call raydium_get_quote_and_liquidity
    ja lbb_46
lbb_41:
    mov64 r5, 1
    stxw [r4+0x0], r5
    add64 r1, 8
    add64 r4, 8
    call pump_fun_get_quote_and_liquidity
lbb_46:
    exit

calculate_profit_optimised:
    mov64 r7, r4
    stxdw [r10-0x20], r2
    mov64 r6, r1
    mov64 r9, r10
    add64 r9, -24
    mov64 r1, r3
    mov64 r2, r6
    mov64 r3, r7
    mov64 r4, r9
    call get_quote_and_liquidity
    mov64 r8, r0
    mov64 r1, r9
    ldxdw r2, [r10-0x20]
    mov64 r3, r7
    mov64 r4, r9
    call get_liquidity
    xor64 r7, 1
    mov64 r1, r9
    mov64 r2, r8
    mov64 r3, r7
    call get_quote
    sub64 r0, r6
    exit

calculate_hinted_max_amount_optimised:
    mov64 r0, 0
    jgt r2, r1, lbb_94
    sub64 r1, r2
    lddw r2, 0x68db8bac710cc
    jgt r2, r1, lbb_81
    mov64 r2, 10000
    sub64 r2, r3
    div64 r1, r2
    mul64 r1, 10000
    ja lbb_85
lbb_81:
    mov64 r2, 10000
    sub64 r2, r3
    mul64 r1, 10000
    div64 r1, r2
lbb_85:
    lddw r2, 0x68db8bac710cc
    jgt r2, r1, lbb_91
    div64 r1, 10000
    mul64 r1, r4
    ja lbb_93
lbb_91:
    mul64 r1, r4
    div64 r1, 10000
lbb_93:
    mov64 r0, r1
lbb_94:
    exit

calculate_upper_bound_optimised:
    mov64 r5, 9975
    mov64 r0, 0
    ldxw r6, [r2+0x0]
    jeq r6, 0, lbb_101
    jne r6, 1, lbb_121
    mov64 r5, 9900
lbb_101:
    ldxdw r6, [r2+0x8]
    jne r4, 0, lbb_104
    ldxdw r6, [r2+0x10]
lbb_104:
    jgt r6, r1, lbb_121
    sub64 r1, r6
    lddw r2, 0x68db8bac710cc
    jgt r2, r1, lbb_112
    div64 r1, r5
    mul64 r1, 10000
    ja lbb_114
lbb_112:
    mul64 r1, 10000
    div64 r1, r5
lbb_114:
    jgt r2, r1, lbb_118
    div64 r1, 10000
    mul64 r1, r3
    ja lbb_120
lbb_118:
    mul64 r1, r3
    div64 r1, 10000
lbb_120:
    mov64 r0, r1
lbb_121:
    exit

calculate_optimal_strategy_optimised:
    mov64 r8, r4
    stxdw [r10-0x30], r2
    mov64 r4, 9975
    mov64 r9, 0
    ldxdw r0, [r5-0xfe8]
    ldxdw r2, [r5-0xff0]
    ldxdw r7, [r5-0xff8]
    ldxdw r3, [r5-0x1000]
    ldxw r5, [r8+0x0]
    jeq r5, 0, lbb_134
    jne r5, 1, lbb_162
    mov64 r4, 9900
lbb_134:
    ldxdw r5, [r8+0x8]
    jne r7, 0, lbb_137
    ldxdw r5, [r8+0x10]
lbb_137:
    jgt r5, r1, lbb_162
    sub64 r1, r5
    lddw r5, 0x68db8bac710cc
    jgt r5, r1, lbb_145
    div64 r1, r4
    mul64 r1, 10000
    ja lbb_147
lbb_145:
    mul64 r1, 10000
    div64 r1, r4
lbb_147:
    mov64 r9, r1
    lddw r1, 0x68db8bac710cc
    jgt r1, r9, lbb_159
    div64 r9, 10000
    mul64 r9, r3
    jgt r9, 999, lbb_155
    ja lbb_162
lbb_155:
    jeq r2, 0, lbb_171
    stxdw [r0+0x0], r9
lbb_157:
    mov64 r6, 1
    ja lbb_169
lbb_159:
    mul64 r9, r3
    div64 r9, 10000
    jgt r9, 999, lbb_155
lbb_162:
    mov64 r6, 0
    mov64 r1, 1000
    mov64 r2, r9
    mov64 r3, 0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
lbb_169:
    mov64 r0, r6
    exit
lbb_171:
    stxdw [r10-0x78], r0
    mov64 r1, 1000
    stxdw [r10-0x20], r1
    mov64 r6, r10
    add64 r6, -24
    mov64 r1, r8
    mov64 r2, 1000
    mov64 r3, r7
    mov64 r4, r6
    call get_quote_and_liquidity
    stxdw [r10-0x28], r0
    mov64 r1, r6
    stxdw [r10-0x38], r9
    ldxdw r2, [r10-0x30]
    mov64 r3, r7
    mov64 r9, r7
    mov64 r4, r6
    call get_liquidity
    xor64 r7, 1
    mov64 r1, r6
    ldxdw r2, [r10-0x28]
    mov64 r3, r7
    call get_quote
    mov64 r6, r10
    add64 r6, -24
    stxdw [r10-0x68], r8
    mov64 r1, r8
    ldxdw r2, [r10-0x38]
    mov64 r3, r9
    mov64 r8, r9
    mov64 r4, r6
    call get_quote_and_liquidity
    stxdw [r10-0x28], r0
    mov64 r1, r6
    ldxdw r2, [r10-0x30]
    ldxdw r9, [r10-0x38]
    stxdw [r10-0x70], r8
    mov64 r3, r8
    mov64 r4, r6
    call get_liquidity
    mov64 r1, r6
    ldxdw r2, [r10-0x28]
    stxdw [r10-0x40], r7
    mov64 r3, r7
    call get_quote
    mov64 r6, r0
    sub64 r6, r9
    mov64 r1, r9
    add64 r1, -1000
    mov64 r2, 10001
    jgt r2, r1, lbb_238
    mov64 r2, 0
    stxdw [r10-0x58], r9
    mov64 r4, r9
    mov64 r3, r6
    mov64 r7, 1000
    ja lbb_247
lbb_228:
    mov64 r1, r2
    and64 r1, 255
    jgt r1, 14, lbb_238
    add64 r2, 1
    ldxdw r1, [r10-0x58]
    sub64 r1, r7
    stxdw [r10-0x20], r7
    mov64 r4, r9
    mov64 r3, r6
    jgt r1, 10000, lbb_247
lbb_238:
    syscall sol_log_compute_units_
    lddw r1, 0x100018e3e
    mov64 r2, 16
    syscall sol_log_
    ldxdw r1, [r10-0x78]
    stxdw [r1+0x8], r6
    stxdw [r1+0x0], r9
    ja lbb_157
lbb_247:
    stxdw [r10-0x28], r3
    stxdw [r10-0x60], r4
    stxdw [r10-0x48], r2
    call function_12023
    mov64 r6, r0
    mov64 r1, r6
    lddw r2, 0x3fd8722191a029e3
    call function_11552
    mov64 r1, r0
    call function_9815
    add64 r7, r0
    stxdw [r10-0x50], r7
    mov64 r8, r10
    add64 r8, -24
    ldxdw r9, [r10-0x68]
    mov64 r1, r9
    mov64 r2, r7
    ldxdw r7, [r10-0x70]
    mov64 r3, r7
    mov64 r4, r8
    call get_quote_and_liquidity
    stxdw [r10-0x38], r0
    mov64 r1, r8
    ldxdw r2, [r10-0x30]
    mov64 r3, r7
    mov64 r4, r8
    call get_liquidity
    mov64 r1, r6
    lddw r2, 0x3fe3c6ef372fe7e6
    call function_11552
    mov64 r1, r0
    call function_9815
    mov64 r6, r0
    mov64 r1, r8
    ldxdw r2, [r10-0x38]
    ldxdw r3, [r10-0x40]
    call get_quote
    mov64 r8, r0
    ldxdw r2, [r10-0x20]
    stxdw [r10-0x20], r2
    add64 r2, r6
    mov64 r6, r10
    add64 r6, -24
    mov64 r1, r9
    stxdw [r10-0x38], r2
    mov64 r3, r7
    mov64 r4, r6
    call get_quote_and_liquidity
    mov64 r9, r0
    mov64 r1, r6
    ldxdw r2, [r10-0x30]
    mov64 r3, r7
    ldxdw r7, [r10-0x50]
    mov64 r4, r6
    call get_liquidity
    sub64 r8, r7
    mov64 r2, r7
    ldxdw r1, [r10-0x28]
    jsgt r8, r1, lbb_309
    ldxdw r2, [r10-0x60]
lbb_309:
    stxdw [r10-0x60], r2
    mov64 r1, r6
    mov64 r2, r9
    ldxdw r3, [r10-0x40]
    call get_quote
    mov64 r6, r0
    ldxdw r9, [r10-0x38]
    sub64 r6, r9
    jsgt r6, r8, lbb_319
    ldxdw r7, [r10-0x20]
lbb_319:
    mov64 r1, r8
    ldxdw r2, [r10-0x48]
    ldxdw r3, [r10-0x28]
    jsgt r8, r3, lbb_324
    mov64 r1, r3
lbb_324:
    jsgt r6, r8, lbb_326
    stxdw [r10-0x58], r9
lbb_326:
    jsgt r6, r1, lbb_328
    ldxdw r9, [r10-0x60]
lbb_328:
    jsgt r6, r1, lbb_228
    mov64 r6, r1
    ja lbb_228

raydium_v4_parse_liquidity:
    mov64 r0, 0
    ldxdw r2, [r2+0x18]
    ldxdw r2, [r2+0x40]
    jeq r2, 0, lbb_350
    ldxdw r3, [r3+0x18]
    ldxdw r3, [r3+0x40]
    jeq r3, 0, lbb_350
    ldxdw r5, [r1+0x10]
    mov64 r6, 208
    jgt r6, r5, lbb_350
    ldxdw r1, [r1+0x18]
    ldxdw r5, [r1+0xc0]
    ldxdw r1, [r1+0xc8]
    stxw [r4+0x0], r0
    sub64 r3, r1
    stxdw [r4+0x10], r3
    sub64 r2, r5
    stxdw [r4+0x8], r2
    mov64 r0, 1
lbb_350:
    exit

raydium_cp_parse_liquidity:
    mov64 r6, r4
    mov64 r8, r3
    mov64 r9, r2
    mov64 r7, r1
    ldxdw r1, [r7+0x0]
    syscall sol_log_pubkey
    ldxdw r1, [r9+0x0]
    syscall sol_log_pubkey
    ldxdw r1, [r8+0x0]
    syscall sol_log_pubkey
    mov64 r0, 0
    ldxdw r1, [r9+0x18]
    ldxdw r1, [r1+0x40]
    jeq r1, 0, lbb_385
    ldxdw r2, [r8+0x18]
    ldxdw r2, [r2+0x40]
    jeq r2, 0, lbb_385
    ldxdw r3, [r7+0x10]
    mov64 r4, 408
    jgt r4, r3, lbb_385
    ldxdw r3, [r7+0x18]
    ldxdw r4, [r3+0x16d]
    ldxdw r5, [r3+0x15d]
    add64 r5, r4
    sub64 r2, r5
    ldxdw r4, [r3+0x165]
    ldxdw r3, [r3+0x155]
    mov64 r5, 0
    stxw [r6+0x0], r5
    stxdw [r6+0x10], r2
    add64 r3, r4
    sub64 r1, r3
    stxdw [r6+0x8], r1
    mov64 r0, 1
lbb_385:
    exit

raydium_is_valid:
    ldxdw r3, [r1+0x8]
    mov64 r0, 1
    mov64 r2, 1
    jgt r3, 1000, lbb_391
    mov64 r2, 0
lbb_391:
    ldxdw r1, [r1+0x0]
    jgt r1, 1000, lbb_394
    mov64 r0, 0
lbb_394:
    and64 r0, r2
    exit

raydium_get_quote:
    mov64 r8, r2
    div64 r8, 10000
    mul64 r8, -25
    jeq r3, 0, lbb_436
    ldxdw r3, [r1+0x0]
    mov64 r6, r3
    lsh64 r6, 32
    rsh64 r6, 32
    ldxdw r0, [r1+0x8]
    mov64 r5, r0
    lsh64 r5, 32
    rsh64 r5, 32
    mov64 r1, r3
    rsh64 r1, 32
    mov64 r4, r5
    mul64 r4, r1
    mul64 r5, r6
    rsh64 r5, 32
    add64 r5, r4
    add64 r8, r2
    mov64 r2, r0
    rsh64 r2, 32
    mov64 r4, r2
    mul64 r4, r1
    mov64 r9, r5
    rsh64 r9, 32
    add64 r9, r4
    mul64 r2, r6
    lsh64 r5, 32
    rsh64 r5, 32
    add64 r5, r2
    rsh64 r5, 32
    add64 r9, r5
    add64 r8, r3
    mov64 r1, r0
    mul64 r1, r3
    jne r9, 0, lbb_470
lbb_433:
    div64 r1, r8
    mov64 r3, r1
    ja lbb_670
lbb_436:
    ldxdw r0, [r1+0x0]
    mov64 r5, r0
    lsh64 r5, 32
    rsh64 r5, 32
    ldxdw r3, [r1+0x8]
    mov64 r1, r3
    lsh64 r1, 32
    rsh64 r1, 32
    mov64 r7, r0
    rsh64 r7, 32
    mov64 r6, r7
    mul64 r6, r1
    mov64 r4, r5
    mul64 r4, r1
    rsh64 r4, 32
    add64 r4, r6
    add64 r8, r2
    mov64 r2, r3
    rsh64 r2, 32
    mul64 r7, r2
    mov64 r9, r4
    rsh64 r9, 32
    add64 r9, r7
    mul64 r5, r2
    lsh64 r4, 32
    rsh64 r4, 32
    add64 r4, r5
    rsh64 r4, 32
    add64 r9, r4
    add64 r8, r3
    mov64 r1, r0
    mul64 r1, r3
    jne r9, 0, lbb_569
    ja lbb_433
lbb_470:
    mov64 r3, 1
    lddw r5, 0xffffffff
    jgt r8, r5, lbb_475
    mov64 r3, 0
lbb_475:
    mov64 r4, r8
    rsh64 r4, 32
    jgt r8, r5, lbb_479
    mov64 r4, r8
lbb_479:
    lsh64 r3, 5
    mov64 r5, r3
    or64 r5, 16
    jgt r4, 65535, lbb_484
    mov64 r5, r3
lbb_484:
    mov64 r6, r4
    rsh64 r6, 16
    jgt r4, 65535, lbb_488
    mov64 r6, r4
lbb_488:
    mov64 r3, r5
    or64 r3, 8
    jgt r6, 255, lbb_492
    mov64 r3, r5
lbb_492:
    stxdw [r10-0x8], r0
    mov64 r4, r6
    rsh64 r4, 8
    jgt r6, 255, lbb_497
    mov64 r4, r6
lbb_497:
    lddw r5, 0x10001903b
    add64 r5, r4
    ldxb r4, [r5+0x0]
    add64 r3, r4
    mov64 r4, 64
    sub64 r4, r3
    mov64 r2, r1
    mov64 r5, r2
    rsh64 r5, r3
    lsh64 r9, r4
    or64 r9, r5
    lsh64 r8, r4
    lsh64 r2, r4
    mov64 r3, r2
    rsh64 r3, 32
    mov64 r6, r8
    rsh64 r6, 32
    mov64 r5, r9
    div64 r5, r6
    mov64 r4, r6
    mul64 r4, r5
    mov64 r1, r9
    sub64 r1, r4
    lsh64 r2, 32
    rsh64 r2, 32
    stxdw [r10-0x18], r2
    stxdw [r10-0x10], r8
    mov64 r7, r8
    lsh64 r7, 32
    rsh64 r7, 32
    lddw r2, 0xffffffff
    lddw r4, 0x100000000
lbb_532:
    jgt r5, r2, lbb_539
    mov64 r0, r5
    mul64 r0, r7
    mov64 r8, r1
    lsh64 r8, 32
    or64 r8, r3
    jge r8, r0, lbb_542
lbb_539:
    add64 r1, r6
    add64 r5, -1
    jgt r4, r1, lbb_532
lbb_542:
    mov64 r1, r5
    ldxdw r2, [r10-0x10]
    mul64 r1, r2
    lsh64 r9, 32
    or64 r9, r3
    sub64 r9, r1
    mov64 r3, r9
    div64 r3, r6
    mov64 r1, r3
    mul64 r1, r6
    sub64 r9, r1
    lddw r4, 0xffffffff
    lddw r0, 0x100000000
    ldxdw r8, [r10-0x18]
lbb_558:
    jgt r3, r4, lbb_565
    mov64 r1, r3
    mul64 r1, r7
    mov64 r2, r9
    lsh64 r2, 32
    or64 r2, r8
    jge r2, r1, lbb_568
lbb_565:
    add64 r9, r6
    add64 r3, -1
    jgt r0, r9, lbb_558
lbb_568:
    ja lbb_667
lbb_569:
    mov64 r3, 1
    lddw r5, 0xffffffff
    jgt r8, r5, lbb_574
    mov64 r3, 0
lbb_574:
    mov64 r4, r8
    rsh64 r4, 32
    jgt r8, r5, lbb_578
    mov64 r4, r8
lbb_578:
    lsh64 r3, 5
    mov64 r5, r3
    or64 r5, 16
    jgt r4, 65535, lbb_583
    mov64 r5, r3
lbb_583:
    mov64 r6, r4
    rsh64 r6, 16
    jgt r4, 65535, lbb_587
    mov64 r6, r4
lbb_587:
    mov64 r3, r5
    or64 r3, 8
    jgt r6, 255, lbb_591
    mov64 r3, r5
lbb_591:
    stxdw [r10-0x8], r0
    mov64 r4, r6
    rsh64 r4, 8
    jgt r6, 255, lbb_596
    mov64 r4, r6
lbb_596:
    lddw r5, 0x10001903b
    add64 r5, r4
    ldxb r4, [r5+0x0]
    add64 r3, r4
    mov64 r4, 64
    sub64 r4, r3
    mov64 r2, r1
    mov64 r5, r2
    rsh64 r5, r3
    lsh64 r9, r4
    or64 r9, r5
    lsh64 r8, r4
    lsh64 r2, r4
    mov64 r3, r2
    rsh64 r3, 32
    mov64 r6, r8
    rsh64 r6, 32
    mov64 r5, r9
    div64 r5, r6
    mov64 r4, r6
    mul64 r4, r5
    mov64 r1, r9
    sub64 r1, r4
    lsh64 r2, 32
    rsh64 r2, 32
    stxdw [r10-0x18], r2
    stxdw [r10-0x10], r8
    mov64 r7, r8
    lsh64 r7, 32
    rsh64 r7, 32
    lddw r2, 0xffffffff
    lddw r4, 0x100000000
lbb_631:
    jgt r5, r2, lbb_638
    mov64 r0, r5
    mul64 r0, r7
    mov64 r8, r1
    lsh64 r8, 32
    or64 r8, r3
    jge r8, r0, lbb_641
lbb_638:
    add64 r1, r6
    add64 r5, -1
    jgt r4, r1, lbb_631
lbb_641:
    mov64 r1, r5
    ldxdw r2, [r10-0x10]
    mul64 r1, r2
    lsh64 r9, 32
    or64 r9, r3
    sub64 r9, r1
    mov64 r3, r9
    div64 r3, r6
    mov64 r1, r3
    mul64 r1, r6
    sub64 r9, r1
    lddw r4, 0xffffffff
    lddw r0, 0x100000000
    ldxdw r8, [r10-0x18]
lbb_657:
    jgt r3, r4, lbb_664
    mov64 r1, r3
    mul64 r1, r7
    mov64 r2, r9
    lsh64 r2, 32
    or64 r2, r8
    jge r2, r1, lbb_667
lbb_664:
    add64 r9, r6
    add64 r3, -1
    jgt r0, r9, lbb_657
lbb_667:
    lsh64 r5, 32
    add64 r3, r5
    ldxdw r0, [r10-0x8]
lbb_670:
    xor64 r3, -1
    add64 r0, r3
    exit

raydium_get_liquidity:
    mov64 r8, r2
    div64 r8, 10000
    mul64 r8, -25
    jeq r3, 0, lbb_712
    ldxdw r6, [r1+0x0]
    mov64 r0, r6
    lsh64 r0, 32
    rsh64 r0, 32
    ldxdw r9, [r1+0x8]
    mov64 r3, r9
    lsh64 r3, 32
    rsh64 r3, 32
    mov64 r1, r6
    rsh64 r1, 32
    mov64 r7, r3
    mul64 r7, r1
    mul64 r3, r0
    rsh64 r3, 32
    add64 r3, r7
    add64 r8, r2
    mov64 r7, r9
    rsh64 r7, 32
    mov64 r2, r7
    mul64 r2, r1
    mov64 r1, r3
    rsh64 r1, 32
    add64 r1, r2
    mul64 r9, r6
    add64 r8, r6
    mul64 r7, r0
    lsh64 r3, 32
    rsh64 r3, 32
    add64 r3, r7
    rsh64 r3, 32
    add64 r1, r3
    jne r1, 0, lbb_749
    div64 r9, r8
    mov64 r5, r9
    ja lbb_959
lbb_712:
    ldxdw r6, [r1+0x0]
    mov64 r0, r6
    lsh64 r0, 32
    rsh64 r0, 32
    ldxdw r9, [r1+0x8]
    mov64 r1, r9
    lsh64 r1, 32
    rsh64 r1, 32
    mov64 r5, r6
    rsh64 r5, 32
    mov64 r7, r5
    mul64 r7, r1
    mov64 r3, r0
    mul64 r3, r1
    rsh64 r3, 32
    add64 r3, r7
    mov64 r7, r6
    add64 r8, r2
    mov64 r1, r9
    rsh64 r1, 32
    mul64 r5, r1
    mov64 r6, r3
    rsh64 r6, 32
    add64 r6, r5
    mul64 r7, r9
    mov64 r5, r8
    add64 r5, r9
    mul64 r0, r1
    lsh64 r3, 32
    rsh64 r3, 32
    add64 r3, r0
    rsh64 r3, 32
    add64 r6, r3
    jne r6, 0, lbb_851
    div64 r7, r5
    mov64 r8, r7
    ja lbb_959
lbb_749:
    mov64 r2, 1
    lddw r0, 0xffffffff
    jgt r8, r0, lbb_754
    mov64 r2, 0
lbb_754:
    mov64 r3, r8
    rsh64 r3, 32
    jgt r8, r0, lbb_758
    mov64 r3, r8
lbb_758:
    lsh64 r2, 5
    mov64 r0, r2
    or64 r0, 16
    jgt r3, 65535, lbb_763
    mov64 r0, r2
lbb_763:
    mov64 r6, r3
    rsh64 r6, 16
    jgt r3, 65535, lbb_767
    mov64 r6, r3
lbb_767:
    mov64 r2, r0
    or64 r2, 8
    jgt r6, 255, lbb_771
    mov64 r2, r0
lbb_771:
    mov64 r3, r6
    rsh64 r3, 8
    jgt r6, 255, lbb_775
    mov64 r3, r6
lbb_775:
    stxdw [r10-0x18], r4
    lddw r0, 0x10001903b
    add64 r0, r3
    ldxb r4, [r0+0x0]
    add64 r2, r4
    mov64 r4, 64
    sub64 r4, r2
    mov64 r0, r9
    rsh64 r0, r2
    lsh64 r1, r4
    or64 r1, r0
    mov64 r7, r8
    lsh64 r7, r4
    lsh64 r9, r4
    mov64 r5, r9
    rsh64 r5, 32
    mov64 r6, r7
    rsh64 r6, 32
    mov64 r0, r1
    div64 r0, r6
    mov64 r3, r6
    mul64 r3, r0
    mov64 r2, r1
    sub64 r2, r3
    lsh64 r9, 32
    rsh64 r9, 32
    stxdw [r10-0x8], r9
    stxdw [r10-0x10], r7
    lsh64 r7, 32
    rsh64 r7, 32
    lddw r4, 0x100000000
lbb_808:
    lddw r3, 0xffffffff
    jgt r0, r3, lbb_817
    mov64 r3, r0
    mul64 r3, r7
    mov64 r9, r2
    lsh64 r9, 32
    or64 r9, r5
    jge r9, r3, lbb_820
lbb_817:
    add64 r2, r6
    add64 r0, -1
    jgt r4, r2, lbb_808
lbb_820:
    mov64 r2, r0
    ldxdw r3, [r10-0x10]
    mul64 r2, r3
    lsh64 r1, 32
    or64 r1, r5
    sub64 r1, r2
    mov64 r5, r1
    div64 r5, r6
    mov64 r2, r5
    mul64 r2, r6
    sub64 r1, r2
    lddw r4, 0xffffffff
    lddw r2, 0x100000000
lbb_835:
    jgt r5, r4, lbb_845
    mov64 r3, r5
    mul64 r3, r7
    mov64 r9, r1
    lsh64 r9, 32
    ldxdw r4, [r10-0x8]
    or64 r9, r4
    lddw r4, 0xffffffff
    jge r9, r3, lbb_848
lbb_845:
    add64 r1, r6
    add64 r5, -1
    jgt r2, r1, lbb_835
lbb_848:
    lsh64 r0, 32
    add64 r5, r0
    ja lbb_958
lbb_851:
    mov64 r2, 1
    lddw r0, 0xffffffff
    jgt r5, r0, lbb_856
    mov64 r2, 0
lbb_856:
    mov64 r1, r5
    rsh64 r1, 32
    jgt r5, r0, lbb_860
    mov64 r1, r5
lbb_860:
    lsh64 r2, 5
    mov64 r0, r2
    or64 r0, 16
    jgt r1, 65535, lbb_865
    mov64 r0, r2
lbb_865:
    mov64 r2, r1
    rsh64 r2, 16
    jgt r1, 65535, lbb_869
    mov64 r2, r1
lbb_869:
    mov64 r3, r0
    or64 r3, 8
    jgt r2, 255, lbb_873
    mov64 r3, r0
lbb_873:
    mov64 r1, r2
    rsh64 r1, 8
    jgt r2, 255, lbb_877
    mov64 r1, r2
lbb_877:
    stxdw [r10-0x18], r4
    lddw r0, 0x10001903b
    add64 r0, r1
    ldxb r1, [r0+0x0]
    add64 r3, r1
    mov64 r1, 64
    sub64 r1, r3
    mov64 r0, r7
    rsh64 r0, r3
    lsh64 r6, r1
    or64 r6, r0
    mov64 r4, r5
    lsh64 r4, r1
    lsh64 r7, r1
    mov64 r1, r7
    rsh64 r1, 32
    stxdw [r10-0x8], r1
    mov64 r1, r4
    rsh64 r1, 32
    mov64 r0, r6
    div64 r0, r1
    mov64 r3, r1
    mul64 r3, r0
    mov64 r2, r6
    sub64 r2, r3
    lsh64 r7, 32
    rsh64 r7, 32
    stxdw [r10-0x10], r7
    stxdw [r10-0x20], r4
    mov64 r7, r4
    lsh64 r7, 32
    rsh64 r7, 32
    lddw r8, 0xffffffff
    lddw r4, 0x100000000
lbb_914:
    jgt r0, r8, lbb_924
    mov64 r3, r0
    mul64 r3, r7
    mov64 r9, r2
    lsh64 r9, 32
    ldxdw r8, [r10-0x8]
    or64 r9, r8
    lddw r8, 0xffffffff
    jge r9, r3, lbb_927
lbb_924:
    add64 r2, r1
    add64 r0, -1
    jgt r4, r2, lbb_914
lbb_927:
    mov64 r2, r0
    ldxdw r3, [r10-0x20]
    mul64 r2, r3
    lsh64 r6, 32
    ldxdw r3, [r10-0x8]
    or64 r6, r3
    sub64 r6, r2
    mov64 r8, r6
    div64 r8, r1
    mov64 r2, r8
    mul64 r2, r1
    sub64 r6, r2
    lddw r4, 0xffffffff
    lddw r2, 0x100000000
lbb_943:
    jgt r8, r4, lbb_953
    mov64 r3, r8
    mul64 r3, r7
    mov64 r9, r6
    lsh64 r9, 32
    ldxdw r4, [r10-0x10]
    or64 r9, r4
    lddw r4, 0xffffffff
    jge r9, r3, lbb_956
lbb_953:
    add64 r6, r1
    add64 r8, -1
    jgt r2, r6, lbb_943
lbb_956:
    lsh64 r0, 32
    add64 r8, r0
lbb_958:
    ldxdw r4, [r10-0x18]
lbb_959:
    stxdw [r4+0x8], r5
    stxdw [r4+0x0], r8
    exit

raydium_get_quote_and_liquidity:
    mov64 r9, r2
    div64 r9, 10000
    mul64 r9, -25
    jeq r3, 0, lbb_1002
    ldxdw r3, [r1+0x0]
    mov64 r6, r3
    lsh64 r6, 32
    rsh64 r6, 32
    ldxdw r0, [r1+0x8]
    mov64 r8, r0
    lsh64 r8, 32
    rsh64 r8, 32
    mov64 r1, r3
    rsh64 r1, 32
    mov64 r5, r8
    mul64 r5, r1
    mul64 r8, r6
    rsh64 r8, 32
    add64 r8, r5
    add64 r9, r2
    mov64 r2, r0
    rsh64 r2, 32
    mov64 r7, r2
    mul64 r7, r1
    mov64 r5, r8
    rsh64 r5, 32
    add64 r5, r7
    mul64 r2, r6
    lsh64 r8, 32
    rsh64 r8, 32
    add64 r8, r2
    rsh64 r8, 32
    add64 r5, r8
    add64 r9, r3
    mov64 r7, r0
    mul64 r7, r3
    jne r5, 0, lbb_1039
    div64 r7, r9
    mov64 r3, r7
    ja lbb_1252
lbb_1002:
    ldxdw r0, [r1+0x0]
    mov64 r8, r0
    lsh64 r8, 32
    rsh64 r8, 32
    ldxdw r3, [r1+0x8]
    mov64 r7, r3
    lsh64 r7, 32
    rsh64 r7, 32
    mov64 r5, r0
    rsh64 r5, 32
    mov64 r6, r5
    mul64 r6, r7
    mov64 r1, r8
    mul64 r1, r7
    rsh64 r1, 32
    add64 r1, r6
    add64 r9, r2
    mov64 r6, r3
    rsh64 r6, 32
    mul64 r5, r6
    mov64 r2, r1
    rsh64 r2, 32
    add64 r2, r5
    mul64 r8, r6
    lsh64 r1, 32
    rsh64 r1, 32
    add64 r1, r8
    rsh64 r1, 32
    add64 r2, r1
    mov64 r7, r9
    add64 r7, r3
    mov64 r9, r0
    mul64 r9, r3
    jne r2, 0, lbb_1146
    div64 r9, r7
    mov64 r3, r9
    ja lbb_1252
lbb_1039:
    mov64 r2, 1
    lddw r1, 0xffffffff
    jgt r9, r1, lbb_1044
    mov64 r2, 0
lbb_1044:
    mov64 r3, r9
    rsh64 r3, 32
    jgt r9, r1, lbb_1048
    mov64 r3, r9
lbb_1048:
    lsh64 r2, 5
    mov64 r1, r2
    or64 r1, 16
    jgt r3, 65535, lbb_1053
    mov64 r1, r2
lbb_1053:
    mov64 r6, r3
    rsh64 r6, 16
    jgt r3, 65535, lbb_1057
    mov64 r6, r3
lbb_1057:
    mov64 r2, r1
    or64 r2, 8
    jgt r6, 255, lbb_1061
    mov64 r2, r1
lbb_1061:
    stxdw [r10-0x18], r0
    stxdw [r10-0x10], r4
    mov64 r3, r6
    rsh64 r3, 8
    jgt r6, 255, lbb_1067
    mov64 r3, r6
lbb_1067:
    lddw r4, 0x10001903b
    add64 r4, r3
    ldxb r3, [r4+0x0]
    add64 r2, r3
    mov64 r3, 64
    sub64 r3, r2
    mov64 r0, r7
    mov64 r4, r0
    rsh64 r4, r2
    lsh64 r5, r3
    or64 r5, r4
    stxdw [r10-0x20], r9
    mov64 r8, r9
    lsh64 r8, r3
    lsh64 r0, r3
    mov64 r9, r0
    rsh64 r9, 32
    mov64 r7, r8
    rsh64 r7, 32
    mov64 r6, r5
    div64 r6, r7
    mov64 r3, r7
    mul64 r3, r6
    mov64 r1, r5
    sub64 r1, r3
    lsh64 r0, 32
    rsh64 r0, 32
    stxdw [r10-0x8], r0
    stxdw [r10-0x28], r8
    lsh64 r8, 32
    rsh64 r8, 32
    lddw r4, 0xffffffff
    lddw r3, 0x100000000
lbb_1103:
    jgt r6, r4, lbb_1110
    mov64 r0, r6
    mul64 r0, r8
    mov64 r2, r1
    lsh64 r2, 32
    or64 r2, r9
    jge r2, r0, lbb_1113
lbb_1110:
    add64 r1, r7
    add64 r6, -1
    jgt r3, r1, lbb_1103
lbb_1113:
    mov64 r1, r6
    ldxdw r2, [r10-0x28]
    mul64 r1, r2
    lsh64 r5, 32
    or64 r5, r9
    sub64 r5, r1
    mov64 r3, r5
    div64 r3, r7
    mov64 r1, r3
    mul64 r1, r7
    sub64 r5, r1
    lddw r2, 0xffffffff
    lddw r4, 0x100000000
    ldxdw r9, [r10-0x8]
lbb_1129:
    jgt r3, r2, lbb_1136
    mov64 r1, r3
    mul64 r1, r8
    mov64 r0, r5
    lsh64 r0, 32
    or64 r0, r9
    jge r0, r1, lbb_1139
lbb_1136:
    add64 r5, r7
    add64 r3, -1
    jgt r4, r5, lbb_1129
lbb_1139:
    lsh64 r6, 32
    add64 r3, r6
    mov64 r7, r3
    ldxdw r4, [r10-0x10]
    ldxdw r0, [r10-0x18]
    ldxdw r9, [r10-0x20]
    ja lbb_1252
lbb_1146:
    mov64 r1, 1
    lddw r5, 0xffffffff
    stxdw [r10-0x8], r7
    jgt r7, r5, lbb_1152
    mov64 r1, 0
lbb_1152:
    ldxdw r6, [r10-0x8]
    mov64 r3, r6
    rsh64 r3, 32
    jgt r6, r5, lbb_1157
    ldxdw r3, [r10-0x8]
lbb_1157:
    lsh64 r1, 5
    mov64 r5, r1
    or64 r5, 16
    jgt r3, 65535, lbb_1162
    mov64 r5, r1
lbb_1162:
    mov64 r6, r3
    rsh64 r6, 16
    jgt r3, 65535, lbb_1166
    mov64 r6, r3
lbb_1166:
    mov64 r1, r5
    or64 r1, 8
    jgt r6, 255, lbb_1170
    mov64 r1, r5
lbb_1170:
    stxdw [r10-0x18], r0
    stxdw [r10-0x10], r4
    mov64 r3, r6
    rsh64 r3, 8
    jgt r6, 255, lbb_1176
    mov64 r3, r6
lbb_1176:
    lddw r4, 0x10001903b
    add64 r4, r3
    ldxb r3, [r4+0x0]
    add64 r1, r3
    mov64 r3, 64
    sub64 r3, r1
    mov64 r4, r9
    rsh64 r4, r1
    lsh64 r2, r3
    or64 r2, r4
    ldxdw r8, [r10-0x8]
    lsh64 r8, r3
    lsh64 r9, r3
    mov64 r4, r9
    rsh64 r9, 32
    mov64 r7, r8
    rsh64 r7, 32
    mov64 r6, r2
    div64 r6, r7
    mov64 r3, r7
    mul64 r3, r6
    mov64 r1, r2
    sub64 r1, r3
    lsh64 r4, 32
    rsh64 r4, 32
    stxdw [r10-0x20], r4
    stxdw [r10-0x28], r8
    lsh64 r8, 32
    rsh64 r8, 32
    lddw r4, 0xffffffff
    lddw r3, 0x100000000
lbb_1210:
    jgt r6, r4, lbb_1217
    mov64 r5, r6
    mul64 r5, r8
    mov64 r0, r1
    lsh64 r0, 32
    or64 r0, r9
    jge r0, r5, lbb_1220
lbb_1217:
    add64 r1, r7
    add64 r6, -1
    jgt r3, r1, lbb_1210
lbb_1220:
    mov64 r1, r6
    ldxdw r3, [r10-0x28]
    mul64 r1, r3
    lsh64 r2, 32
    or64 r2, r9
    sub64 r2, r1
    mov64 r3, r2
    div64 r3, r7
    mov64 r1, r3
    mul64 r1, r7
    sub64 r2, r1
    lddw r1, 0xffffffff
    lddw r4, 0x100000000
    ldxdw r9, [r10-0x20]
lbb_1236:
    jgt r3, r1, lbb_1243
    mov64 r5, r3
    mul64 r5, r8
    mov64 r0, r2
    lsh64 r0, 32
    or64 r0, r9
    jge r0, r5, lbb_1246
lbb_1243:
    add64 r2, r7
    add64 r3, -1
    jgt r4, r2, lbb_1236
lbb_1246:
    lsh64 r6, 32
    add64 r3, r6
    mov64 r9, r3
    ldxdw r4, [r10-0x10]
    ldxdw r0, [r10-0x18]
    ldxdw r7, [r10-0x8]
lbb_1252:
    stxdw [r4+0x8], r7
    stxdw [r4+0x0], r9
    xor64 r3, -1
    add64 r0, r3
    exit

pump_fun_parse_liquidity:
    ldxdw r3, [r1+0x10]
    mov64 r4, 24
    jgt r4, r3, lbb_1267
    ldxdw r1, [r1+0x18]
    ldxdw r4, [r1+0x8]
    ldxdw r1, [r1+0x10]
    stxdw [r2+0x10], r1
    stxdw [r2+0x8], r4
    mov64 r1, 1
    stxw [r2+0x0], r1
lbb_1267:
    mov64 r0, 1
    jgt r3, 23, lbb_1270
    mov64 r0, 0
lbb_1270:
    exit

pump_fun_k:
    mov64 r6, r1
    ldxdw r3, [r2+0x8]
    ldxdw r4, [r2+0x0]
    mov64 r1, r10
    add64 r1, -16
    mov64 r2, r3
    mov64 r3, 0
    mov64 r5, 0
    call function_9839
    ldxdw r1, [r10-0x8]
    stxdw [r6+0x8], r1
    ldxdw r1, [r10-0x10]
    stxdw [r6+0x0], r1
    exit

pump_fun_price:
    mov64 r7, r2
    ldxdw r6, [r1+0x8]
    ldxdw r9, [r1+0x0]
    mov64 r1, r9
    jne r7, 0, lbb_1291
    mov64 r1, r6
lbb_1291:
    call function_12023
    mov64 r8, r0
    jne r7, 0, lbb_1295
    mov64 r6, r9
lbb_1295:
    mov64 r1, r6
    call function_12023
    mov64 r1, r8
    mov64 r2, r0
    call function_12129
    exit

pump_fun_is_valid:
    mov64 r0, 0
    ldxdw r6, [r1+0x0]
    mov64 r2, 1001
    jgt r2, r6, lbb_1327
    ldxdw r1, [r1+0x8]
    jgt r2, r1, lbb_1327
    call function_12023
    mov64 r7, r0
    mov64 r1, r6
    call function_12023
    mov64 r1, r7
    mov64 r2, r0
    call function_12129
    mov64 r1, r0
    lddw r2, 0x42d6bcc41e900000
    call function_11552
    mov64 r1, r0
    lddw r2, 0x4253ca6512000000
    call function_11519
    mov64 r1, r0
    mov64 r0, 1
    mov64 r2, 0
    jsgt r2, r1, lbb_1327
    mov64 r0, 0
lbb_1327:
    and64 r0, 1
    exit

pump_fun_get_quote:
    mov64 r6, r1
    jeq r3, 0, lbb_1365
    ldxdw r8, [r6+0x0]
    mov64 r5, r8
    lsh64 r5, 32
    rsh64 r5, 32
    ldxdw r0, [r6+0x8]
    mov64 r4, r0
    lsh64 r4, 32
    rsh64 r4, 32
    mov64 r1, r8
    rsh64 r1, 32
    mov64 r3, r4
    mul64 r3, r1
    mul64 r4, r5
    rsh64 r4, 32
    add64 r4, r3
    mov64 r7, r0
    rsh64 r7, 32
    mov64 r6, r7
    mul64 r6, r1
    mov64 r3, r4
    rsh64 r3, 32
    add64 r3, r6
    mul64 r7, r5
    lsh64 r4, 32
    rsh64 r4, 32
    add64 r4, r7
    rsh64 r4, 32
    add64 r3, r4
    mov64 r4, r0
    mul64 r4, r8
    add64 r8, r2
    jne r3, 0, lbb_1414
    div64 r4, r8
    ja lbb_1514
lbb_1365:
    mov64 r1, r10
    add64 r1, -16
    mov64 r3, 0
    mov64 r4, 100
    mov64 r5, 0
    call function_9839
    mov64 r1, r10
    add64 r1, -32
    ldxdw r2, [r10-0x10]
    ldxdw r3, [r10-0x8]
    mov64 r4, 101
    mov64 r5, 0
    call function_9883
    ldxdw r0, [r6+0x0]
    mov64 r2, r0
    lsh64 r2, 32
    rsh64 r2, 32
    ldxdw r9, [r6+0x8]
    mov64 r1, r9
    lsh64 r1, 32
    rsh64 r1, 32
    mov64 r4, r0
    rsh64 r4, 32
    mov64 r5, r4
    mul64 r5, r1
    mov64 r3, r2
    mul64 r3, r1
    rsh64 r3, 32
    add64 r3, r5
    mov64 r5, r9
    rsh64 r5, 32
    mul64 r4, r5
    mov64 r7, r3
    rsh64 r7, 32
    add64 r7, r4
    mul64 r2, r5
    lsh64 r3, 32
    rsh64 r3, 32
    add64 r3, r2
    rsh64 r3, 32
    add64 r7, r3
    ldxdw r3, [r10-0x20]
    mov64 r8, r0
    mul64 r8, r9
    add64 r9, r3
    jne r7, 0, lbb_1520
    div64 r8, r9
    mov64 r3, r8
    ja lbb_1620
lbb_1414:
    mov64 r2, 1
    lddw r1, 0xffffffff
    jgt r8, r1, lbb_1419
    mov64 r2, 0
lbb_1419:
    mov64 r5, r8
    rsh64 r5, 32
    jgt r8, r1, lbb_1423
    mov64 r5, r8
lbb_1423:
    lsh64 r2, 5
    mov64 r1, r2
    or64 r1, 16
    jgt r5, 65535, lbb_1428
    mov64 r1, r2
lbb_1428:
    mov64 r6, r5
    rsh64 r6, 16
    jgt r5, 65535, lbb_1432
    mov64 r6, r5
lbb_1432:
    mov64 r2, r1
    or64 r2, 8
    jgt r6, 255, lbb_1436
    mov64 r2, r1
lbb_1436:
    mov64 r5, r6
    rsh64 r5, 8
    jgt r6, 255, lbb_1440
    mov64 r5, r6
lbb_1440:
    stxdw [r10-0x28], r0
    lddw r0, 0x10001913b
    add64 r0, r5
    ldxb r5, [r0+0x0]
    add64 r2, r5
    mov64 r5, 64
    sub64 r5, r2
    mov64 r0, r4
    rsh64 r0, r2
    lsh64 r3, r5
    or64 r3, r0
    lsh64 r8, r5
    lsh64 r4, r5
    mov64 r7, r4
    rsh64 r7, 32
    mov64 r5, r8
    rsh64 r5, 32
    mov64 r2, r3
    div64 r2, r5
    mov64 r0, r5
    mul64 r0, r2
    mov64 r1, r3
    sub64 r1, r0
    lsh64 r4, 32
    rsh64 r4, 32
    stxdw [r10-0x30], r4
    stxdw [r10-0x38], r8
    mov64 r6, r8
    lsh64 r6, 32
    rsh64 r6, 32
    lddw r9, 0xffffffff
    lddw r0, 0x100000000
lbb_1475:
    jgt r2, r9, lbb_1482
    mov64 r4, r2
    mul64 r4, r6
    mov64 r8, r1
    lsh64 r8, 32
    or64 r8, r7
    jge r8, r4, lbb_1485
lbb_1482:
    add64 r1, r5
    add64 r2, -1
    jgt r0, r1, lbb_1475
lbb_1485:
    mov64 r1, r2
    ldxdw r4, [r10-0x38]
    mul64 r1, r4
    lsh64 r3, 32
    or64 r3, r7
    sub64 r3, r1
    mov64 r4, r3
    div64 r4, r5
    mov64 r1, r4
    mul64 r1, r5
    sub64 r3, r1
    lddw r7, 0xffffffff
    lddw r0, 0x100000000
    ldxdw r9, [r10-0x30]
lbb_1501:
    jgt r4, r7, lbb_1508
    mov64 r1, r4
    mul64 r1, r6
    mov64 r8, r3
    lsh64 r8, 32
    or64 r8, r9
    jge r8, r1, lbb_1511
lbb_1508:
    add64 r3, r5
    add64 r4, -1
    jgt r0, r3, lbb_1501
lbb_1511:
    lsh64 r2, 32
    add64 r4, r2
    ldxdw r0, [r10-0x28]
lbb_1514:
    xor64 r4, -1
    add64 r0, r4
    mov64 r1, r0
    div64 r1, 100
    sub64 r0, r1
    ja lbb_1622
lbb_1520:
    mov64 r3, 1
    lddw r5, 0xffffffff
    jgt r9, r5, lbb_1525
    mov64 r3, 0
lbb_1525:
    mov64 r4, r9
    rsh64 r4, 32
    jgt r9, r5, lbb_1529
    mov64 r4, r9
lbb_1529:
    lsh64 r3, 5
    mov64 r5, r3
    or64 r5, 16
    jgt r4, 65535, lbb_1534
    mov64 r5, r3
lbb_1534:
    mov64 r6, r4
    rsh64 r6, 16
    jgt r4, 65535, lbb_1538
    mov64 r6, r4
lbb_1538:
    mov64 r3, r5
    or64 r3, 8
    jgt r6, 255, lbb_1542
    mov64 r3, r5
lbb_1542:
    mov64 r4, r6
    rsh64 r4, 8
    jgt r6, 255, lbb_1546
    mov64 r4, r6
lbb_1546:
    stxdw [r10-0x28], r0
    lddw r5, 0x10001913b
    add64 r5, r4
    ldxb r4, [r5+0x0]
    add64 r3, r4
    mov64 r4, 64
    sub64 r4, r3
    mov64 r5, r8
    rsh64 r5, r3
    lsh64 r7, r4
    or64 r7, r5
    lsh64 r9, r4
    lsh64 r8, r4
    mov64 r2, r8
    rsh64 r2, 32
    mov64 r5, r9
    rsh64 r5, 32
    mov64 r4, r7
    div64 r4, r5
    mov64 r3, r5
    mul64 r3, r4
    mov64 r1, r7
    sub64 r1, r3
    lsh64 r8, 32
    rsh64 r8, 32
    stxdw [r10-0x30], r8
    stxdw [r10-0x38], r9
    mov64 r6, r9
    lsh64 r6, 32
    rsh64 r6, 32
    lddw r9, 0xffffffff
    lddw r3, 0x100000000
lbb_1581:
    jgt r4, r9, lbb_1588
    mov64 r0, r4
    mul64 r0, r6
    mov64 r8, r1
    lsh64 r8, 32
    or64 r8, r2
    jge r8, r0, lbb_1591
lbb_1588:
    add64 r1, r5
    add64 r4, -1
    jgt r3, r1, lbb_1581
lbb_1591:
    mov64 r1, r4
    ldxdw r3, [r10-0x38]
    mul64 r1, r3
    lsh64 r7, 32
    or64 r7, r2
    sub64 r7, r1
    mov64 r3, r7
    div64 r3, r5
    mov64 r1, r3
    mul64 r1, r5
    sub64 r7, r1
    lddw r1, 0xffffffff
    lddw r0, 0x100000000
    ldxdw r9, [r10-0x30]
lbb_1607:
    jgt r3, r1, lbb_1614
    mov64 r2, r3
    mul64 r2, r6
    mov64 r8, r7
    lsh64 r8, 32
    or64 r8, r9
    jge r8, r2, lbb_1617
lbb_1614:
    add64 r7, r5
    add64 r3, -1
    jgt r0, r7, lbb_1607
lbb_1617:
    lsh64 r4, 32
    add64 r3, r4
    ldxdw r0, [r10-0x28]
lbb_1620:
    xor64 r3, -1
    add64 r0, r3
lbb_1622:
    exit

pump_fun_get_liquidity:
    mov64 r9, r4
    mov64 r8, r2
    mov64 r7, r1
    jeq r3, 0, lbb_1662
    ldxdw r1, [r7+0x0]
    mov64 r0, r1
    lsh64 r0, 32
    rsh64 r0, 32
    ldxdw r3, [r7+0x8]
    mov64 r5, r3
    lsh64 r5, 32
    rsh64 r5, 32
    mov64 r4, r1
    rsh64 r4, 32
    mov64 r6, r5
    mul64 r6, r4
    mul64 r5, r0
    rsh64 r5, 32
    add64 r5, r6
    mov64 r6, r3
    rsh64 r6, 32
    mov64 r7, r6
    mul64 r7, r4
    mov64 r2, r5
    rsh64 r2, 32
    add64 r2, r7
    mov64 r7, r3
    mul64 r7, r1
    add64 r1, r8
    mul64 r6, r0
    lsh64 r5, 32
    rsh64 r5, 32
    add64 r5, r6
    rsh64 r5, 32
    add64 r2, r5
    jne r2, 0, lbb_1711
    div64 r7, r1
    mov64 r3, r7
    ja lbb_1926
lbb_1662:
    mov64 r1, r10
    add64 r1, -16
    mov64 r2, r8
    mov64 r3, 0
    mov64 r4, 100
    mov64 r5, 0
    call function_9839
    mov64 r1, r10
    add64 r1, -32
    ldxdw r2, [r10-0x10]
    ldxdw r3, [r10-0x8]
    mov64 r4, 101
    mov64 r5, 0
    call function_9883
    ldxdw r2, [r7+0x0]
    mov64 r1, r2
    lsh64 r1, 32
    rsh64 r1, 32
    ldxdw r3, [r7+0x8]
    mov64 r4, r3
    lsh64 r4, 32
    rsh64 r4, 32
    mov64 r0, r2
    rsh64 r0, 32
    mov64 r6, r0
    mul64 r6, r4
    mov64 r5, r1
    mul64 r5, r4
    rsh64 r5, 32
    add64 r5, r6
    mov64 r6, r3
    rsh64 r6, 32
    mul64 r0, r6
    mov64 r8, r5
    rsh64 r8, 32
    add64 r8, r0
    mul64 r1, r6
    lsh64 r5, 32
    rsh64 r5, 32
    add64 r5, r1
    rsh64 r5, 32
    add64 r8, r5
    mul64 r2, r3
    ldxdw r1, [r10-0x20]
    add64 r3, r1
    jne r8, 0, lbb_1819
    div64 r2, r3
    mov64 r1, r2
    ja lbb_1926
lbb_1711:
    mov64 r3, 1
    lddw r0, 0xffffffff
    jgt r1, r0, lbb_1716
    mov64 r3, 0
lbb_1716:
    mov64 r5, r1
    rsh64 r5, 32
    jgt r1, r0, lbb_1720
    mov64 r5, r1
lbb_1720:
    lsh64 r3, 5
    mov64 r0, r3
    or64 r0, 16
    jgt r5, 65535, lbb_1725
    mov64 r0, r3
lbb_1725:
    mov64 r6, r5
    rsh64 r6, 16
    jgt r5, 65535, lbb_1729
    mov64 r6, r5
lbb_1729:
    mov64 r3, r0
    or64 r3, 8
    jgt r6, 255, lbb_1733
    mov64 r3, r0
lbb_1733:
    mov64 r5, r6
    rsh64 r5, 8
    jgt r6, 255, lbb_1737
    mov64 r5, r6
lbb_1737:
    stxdw [r10-0x38], r9
    lddw r0, 0x10001913b
    add64 r0, r5
    ldxb r5, [r0+0x0]
    add64 r3, r5
    mov64 r5, 64
    sub64 r5, r3
    mov64 r0, r7
    rsh64 r0, r3
    lsh64 r2, r5
    or64 r2, r0
    mov64 r6, r1
    lsh64 r6, r5
    lsh64 r7, r5
    mov64 r3, r7
    rsh64 r3, 32
    stxdw [r10-0x28], r3
    mov64 r0, r6
    rsh64 r0, 32
    mov64 r5, r2
    div64 r5, r0
    mov64 r3, r0
    mul64 r3, r5
    mov64 r4, r2
    sub64 r4, r3
    lsh64 r7, 32
    rsh64 r7, 32
    stxdw [r10-0x30], r7
    stxdw [r10-0x40], r6
    mov64 r7, r6
    lsh64 r7, 32
    rsh64 r7, 32
    lddw r6, 0xffffffff
    lddw r8, 0x100000000
lbb_1774:
    jgt r5, r6, lbb_1784
    mov64 r3, r5
    mul64 r3, r7
    mov64 r9, r4
    lsh64 r9, 32
    ldxdw r6, [r10-0x28]
    or64 r9, r6
    lddw r6, 0xffffffff
    jge r9, r3, lbb_1787
lbb_1784:
    add64 r4, r0
    add64 r5, -1
    jgt r8, r4, lbb_1774
lbb_1787:
    mov64 r3, r5
    ldxdw r4, [r10-0x40]
    mul64 r3, r4
    lsh64 r2, 32
    ldxdw r4, [r10-0x28]
    or64 r2, r4
    sub64 r2, r3
    mov64 r3, r2
    div64 r3, r0
    mov64 r4, r3
    mul64 r4, r0
    sub64 r2, r4
    lddw r6, 0xffffffff
    lddw r8, 0x100000000
lbb_1803:
    jgt r3, r6, lbb_1813
    mov64 r4, r3
    mul64 r4, r7
    mov64 r9, r2
    lsh64 r9, 32
    ldxdw r6, [r10-0x30]
    or64 r9, r6
    lddw r6, 0xffffffff
    jge r9, r4, lbb_1816
lbb_1813:
    add64 r2, r0
    add64 r3, -1
    jgt r8, r2, lbb_1803
lbb_1816:
    lsh64 r5, 32
    add64 r3, r5
    ja lbb_1925
lbb_1819:
    mov64 r1, 1
    lddw r0, 0xffffffff
    jgt r3, r0, lbb_1824
    mov64 r1, 0
lbb_1824:
    mov64 r5, r3
    rsh64 r5, 32
    jgt r3, r0, lbb_1828
    mov64 r5, r3
lbb_1828:
    lsh64 r1, 5
    mov64 r0, r1
    or64 r0, 16
    jgt r5, 65535, lbb_1833
    mov64 r0, r1
lbb_1833:
    mov64 r6, r5
    rsh64 r6, 16
    jgt r5, 65535, lbb_1837
    mov64 r6, r5
lbb_1837:
    mov64 r1, r0
    or64 r1, 8
    jgt r6, 255, lbb_1841
    mov64 r1, r0
lbb_1841:
    mov64 r5, r6
    rsh64 r5, 8
    jgt r6, 255, lbb_1845
    mov64 r5, r6
lbb_1845:
    stxdw [r10-0x38], r9
    lddw r0, 0x10001913b
    add64 r0, r5
    ldxb r5, [r0+0x0]
    add64 r1, r5
    mov64 r5, 64
    sub64 r5, r1
    mov64 r0, r2
    rsh64 r0, r1
    lsh64 r8, r5
    or64 r8, r0
    mov64 r7, r3
    lsh64 r7, r5
    lsh64 r2, r5
    mov64 r1, r2
    rsh64 r1, 32
    stxdw [r10-0x28], r1
    mov64 r0, r7
    rsh64 r0, 32
    mov64 r5, r8
    div64 r5, r0
    mov64 r1, r0
    mul64 r1, r5
    mov64 r4, r8
    sub64 r4, r1
    lsh64 r2, 32
    rsh64 r2, 32
    stxdw [r10-0x30], r2
    stxdw [r10-0x40], r7
    lsh64 r7, 32
    rsh64 r7, 32
    lddw r6, 0xffffffff
    lddw r1, 0x100000000
lbb_1881:
    jgt r5, r6, lbb_1891
    mov64 r2, r5
    mul64 r2, r7
    mov64 r9, r4
    lsh64 r9, 32
    ldxdw r6, [r10-0x28]
    or64 r9, r6
    lddw r6, 0xffffffff
    jge r9, r2, lbb_1894
lbb_1891:
    add64 r4, r0
    add64 r5, -1
    jgt r1, r4, lbb_1881
lbb_1894:
    mov64 r1, r5
    ldxdw r2, [r10-0x40]
    mul64 r1, r2
    lsh64 r8, 32
    ldxdw r2, [r10-0x28]
    or64 r8, r2
    sub64 r8, r1
    mov64 r1, r8
    div64 r1, r0
    mov64 r2, r1
    mul64 r2, r0
    sub64 r8, r2
    lddw r6, 0xffffffff
    lddw r2, 0x100000000
lbb_1910:
    jgt r1, r6, lbb_1920
    mov64 r4, r1
    mul64 r4, r7
    mov64 r9, r8
    lsh64 r9, 32
    ldxdw r6, [r10-0x30]
    or64 r9, r6
    lddw r6, 0xffffffff
    jge r9, r4, lbb_1923
lbb_1920:
    add64 r8, r0
    add64 r1, -1
    jgt r2, r8, lbb_1910
lbb_1923:
    lsh64 r5, 32
    add64 r1, r5
lbb_1925:
    ldxdw r9, [r10-0x38]
lbb_1926:
    stxdw [r9+0x8], r3
    stxdw [r9+0x0], r1
    exit

pump_fun_get_quote_and_liquidity:
    mov64 r9, r4
    mov64 r5, r2
    mov64 r7, r1
    jeq r3, 0, lbb_1967
    ldxdw r8, [r7+0x0]
    mov64 r4, r8
    lsh64 r4, 32
    rsh64 r4, 32
    ldxdw r0, [r7+0x8]
    mov64 r3, r0
    lsh64 r3, 32
    rsh64 r3, 32
    mov64 r2, r8
    rsh64 r2, 32
    mov64 r1, r3
    mul64 r1, r2
    mul64 r3, r4
    rsh64 r3, 32
    add64 r3, r1
    mov64 r1, r0
    rsh64 r1, 32
    mov64 r7, r1
    mul64 r7, r2
    mov64 r6, r3
    rsh64 r6, 32
    add64 r6, r7
    mul64 r1, r4
    lsh64 r3, 32
    rsh64 r3, 32
    add64 r3, r1
    rsh64 r3, 32
    add64 r6, r3
    mov64 r4, r0
    mul64 r4, r8
    add64 r8, r5
    jne r6, 0, lbb_2018
    div64 r4, r8
    ja lbb_2127
lbb_1967:
    mov64 r1, r10
    add64 r1, -16
    mov64 r2, r5
    mov64 r3, 0
    mov64 r4, 100
    mov64 r5, 0
    call function_9839
    mov64 r1, r10
    add64 r1, -32
    ldxdw r2, [r10-0x10]
    ldxdw r3, [r10-0x8]
    mov64 r4, 101
    mov64 r5, 0
    call function_9883
    ldxdw r8, [r7+0x0]
    mov64 r6, r8
    lsh64 r6, 32
    rsh64 r6, 32
    ldxdw r4, [r7+0x8]
    mov64 r3, r4
    lsh64 r3, 32
    rsh64 r3, 32
    mov64 r5, r8
    rsh64 r5, 32
    mov64 r0, r5
    mul64 r0, r3
    mov64 r2, r6
    mul64 r2, r3
    rsh64 r2, 32
    add64 r2, r0
    mov64 r0, r4
    rsh64 r0, 32
    mul64 r5, r0
    mov64 r1, r2
    rsh64 r1, 32
    add64 r1, r5
    mul64 r6, r0
    lsh64 r2, 32
    rsh64 r2, 32
    add64 r2, r6
    rsh64 r2, 32
    add64 r1, r2
    ldxdw r2, [r10-0x20]
    stxdw [r10-0x38], r8
    mov64 r6, r8
    mul64 r6, r4
    add64 r4, r2
    jne r1, 0, lbb_2134
    div64 r6, r4
    mov64 r8, r6
    ja lbb_2236
lbb_2018:
    mov64 r3, 1
    lddw r1, 0xffffffff
    jgt r8, r1, lbb_2023
    mov64 r3, 0
lbb_2023:
    mov64 r5, r8
    rsh64 r5, 32
    jgt r8, r1, lbb_2027
    mov64 r5, r8
lbb_2027:
    lsh64 r3, 5
    mov64 r1, r3
    or64 r1, 16
    jgt r5, 65535, lbb_2032
    mov64 r1, r3
lbb_2032:
    mov64 r2, r5
    rsh64 r2, 16
    jgt r5, 65535, lbb_2036
    mov64 r2, r5
lbb_2036:
    mov64 r3, r1
    or64 r3, 8
    jgt r2, 255, lbb_2040
    mov64 r3, r1
lbb_2040:
    stxdw [r10-0x38], r0
    stxdw [r10-0x40], r9
    mov64 r5, r2
    rsh64 r5, 8
    jgt r2, 255, lbb_2046
    mov64 r5, r2
lbb_2046:
    lddw r0, 0x10001913b
    add64 r0, r5
    ldxb r5, [r0+0x0]
    add64 r3, r5
    mov64 r5, 64
    sub64 r5, r3
    mov64 r0, r4
    rsh64 r0, r3
    lsh64 r6, r5
    or64 r6, r0
    mov64 r1, r8
    lsh64 r1, r5
    lsh64 r4, r5
    mov64 r2, r4
    rsh64 r2, 32
    stxdw [r10-0x28], r2
    mov64 r7, r1
    rsh64 r7, 32
    mov64 r5, r6
    div64 r5, r7
    mov64 r0, r7
    mul64 r0, r5
    mov64 r2, r6
    sub64 r2, r0
    lsh64 r4, 32
    rsh64 r4, 32
    stxdw [r10-0x30], r4
    stxdw [r10-0x48], r1
    lsh64 r1, 32
    rsh64 r1, 32
    lddw r0, 0xffffffff
    lddw r9, 0x100000000
lbb_2081:
    jgt r5, r0, lbb_2091
    mov64 r4, r5
    mul64 r4, r1
    mov64 r3, r2
    lsh64 r3, 32
    ldxdw r0, [r10-0x28]
    or64 r3, r0
    lddw r0, 0xffffffff
    jge r3, r4, lbb_2094
lbb_2091:
    add64 r2, r7
    add64 r5, -1
    jgt r9, r2, lbb_2081
lbb_2094:
    mov64 r2, r5
    ldxdw r3, [r10-0x48]
    mul64 r2, r3
    lsh64 r6, 32
    ldxdw r3, [r10-0x28]
    or64 r6, r3
    sub64 r6, r2
    mov64 r4, r6
    div64 r4, r7
    mov64 r2, r4
    mul64 r2, r7
    sub64 r6, r2
    lddw r3, 0xffffffff
    lddw r0, 0x100000000
lbb_2110:
    jgt r4, r3, lbb_2120
    mov64 r2, r4
    mul64 r2, r1
    mov64 r9, r6
    lsh64 r9, 32
    ldxdw r3, [r10-0x30]
    or64 r9, r3
    lddw r3, 0xffffffff
    jge r9, r2, lbb_2123
lbb_2120:
    add64 r6, r7
    add64 r4, -1
    jgt r0, r6, lbb_2110
lbb_2123:
    lsh64 r5, 32
    add64 r4, r5
    ldxdw r9, [r10-0x40]
    ldxdw r0, [r10-0x38]
lbb_2127:
    mov64 r2, r4
    xor64 r2, -1
    add64 r0, r2
    mov64 r2, r0
    div64 r2, 100
    sub64 r0, r2
    ja lbb_2240
lbb_2134:
    mov64 r3, 1
    lddw r5, 0xffffffff
    jgt r4, r5, lbb_2139
    mov64 r3, 0
lbb_2139:
    mov64 r2, r4
    rsh64 r2, 32
    jgt r4, r5, lbb_2143
    mov64 r2, r4
lbb_2143:
    lsh64 r3, 5
    mov64 r5, r3
    or64 r5, 16
    jgt r2, 65535, lbb_2148
    mov64 r5, r3
lbb_2148:
    mov64 r0, r2
    rsh64 r0, 16
    jgt r2, 65535, lbb_2152
    mov64 r0, r2
lbb_2152:
    mov64 r3, r5
    or64 r3, 8
    jgt r0, 255, lbb_2156
    mov64 r3, r5
lbb_2156:
    stxdw [r10-0x40], r9
    mov64 r2, r0
    rsh64 r2, 8
    jgt r0, 255, lbb_2161
    mov64 r2, r0
lbb_2161:
    lddw r5, 0x10001913b
    add64 r5, r2
    ldxb r2, [r5+0x0]
    add64 r3, r2
    mov64 r2, 64
    sub64 r2, r3
    mov64 r5, r6
    rsh64 r5, r3
    lsh64 r1, r2
    or64 r1, r5
    mov64 r9, r4
    lsh64 r9, r2
    lsh64 r6, r2
    mov64 r8, r6
    rsh64 r8, 32
    mov64 r7, r9
    rsh64 r7, 32
    mov64 r5, r1
    div64 r5, r7
    mov64 r2, r7
    mul64 r2, r5
    mov64 r3, r1
    sub64 r3, r2
    lsh64 r6, 32
    rsh64 r6, 32
    stxdw [r10-0x28], r6
    stxdw [r10-0x30], r9
    lsh64 r9, 32
    rsh64 r9, 32
    lddw r0, 0x100000000
lbb_2193:
    lddw r2, 0xffffffff
    jgt r5, r2, lbb_2202
    mov64 r6, r5
    mul64 r6, r9
    mov64 r2, r3
    lsh64 r2, 32
    or64 r2, r8
    jge r2, r6, lbb_2205
lbb_2202:
    add64 r3, r7
    add64 r5, -1
    jgt r0, r3, lbb_2193
lbb_2205:
    mov64 r2, r5
    ldxdw r3, [r10-0x30]
    mul64 r2, r3
    lsh64 r1, 32
    or64 r1, r8
    sub64 r1, r2
    mov64 r8, r1
    div64 r8, r7
    mov64 r2, r8
    mul64 r2, r7
    sub64 r1, r2
    lddw r2, 0xffffffff
    lddw r0, 0x100000000
lbb_2220:
    jgt r8, r2, lbb_2230
    mov64 r3, r8
    mul64 r3, r9
    mov64 r6, r1
    lsh64 r6, 32
    ldxdw r2, [r10-0x28]
    or64 r6, r2
    lddw r2, 0xffffffff
    jge r6, r3, lbb_2233
lbb_2230:
    add64 r1, r7
    add64 r8, -1
    jgt r0, r1, lbb_2220
lbb_2233:
    lsh64 r5, 32
    add64 r8, r5
    ldxdw r9, [r10-0x40]
lbb_2236:
    mov64 r1, r8
    xor64 r1, -1
    ldxdw r0, [r10-0x38]
    add64 r0, r1
lbb_2240:
    stxdw [r9+0x8], r4
    stxdw [r9+0x0], r8
    exit

calculate_profit:
    mov64 r7, r4
    stxdw [r10-0x20], r2
    mov64 r6, r1
    mov64 r9, r10
    add64 r9, -24
    mov64 r1, r3
    mov64 r2, r6
    mov64 r3, r7
    mov64 r4, r9
    call get_quote_and_liquidity
    mov64 r8, r0
    mov64 r1, r9
    ldxdw r2, [r10-0x20]
    mov64 r3, r7
    mov64 r4, r9
    call get_liquidity
    xor64 r7, 1
    mov64 r1, r9
    mov64 r2, r8
    mov64 r3, r7
    call get_quote
    sub64 r0, r6
    exit

is_buy_amount_too_big:
    mov64 r6, r5
    mov64 r7, r3
    mov64 r8, r2
    mov64 r2, r1
    mov64 r9, r10
    add64 r9, -24
    mov64 r1, r4
    mov64 r3, r6
    mov64 r4, r9
    call get_liquidity
    mov64 r1, r9
    mov64 r2, r8
    mov64 r3, r6
    call get_quote
    mov64 r1, r0
    mov64 r0, 1
    jgt r7, r1, lbb_2287
    mov64 r1, r10
    add64 r1, -24
    call is_valid
    xor64 r0, 1
lbb_2287:
    exit

calculate_hinted_max_amount:
    mov64 r0, 0
    jgt r2, r1, lbb_2312
    sub64 r1, r2
    lddw r2, 0x68db8bac710cc
    jgt r2, r1, lbb_2299
    mov64 r2, 10000
    sub64 r2, r3
    div64 r1, r2
    mul64 r1, 10000
    ja lbb_2303
lbb_2299:
    mov64 r2, 10000
    sub64 r2, r3
    mul64 r1, 10000
    div64 r1, r2
lbb_2303:
    lddw r2, 0x68db8bac710cc
    jgt r2, r1, lbb_2309
    div64 r1, 10000
    mul64 r1, r4
    ja lbb_2311
lbb_2309:
    mul64 r1, r4
    div64 r1, 10000
lbb_2311:
    mov64 r0, r1
lbb_2312:
    exit

calculate_upper_bound:
    mov64 r0, 0
    ldxdw r3, [r5-0xff0]
    ldxdw r2, [r5-0xff8]
    ldxw r5, [r4+0x0]
    jeq r5, 1, lbb_2329
    jne r5, 0, lbb_2378
    jeq r3, 0, lbb_2339
    ldxdw r3, [r4+0x8]
    jgt r3, r1, lbb_2378
    sub64 r1, r3
    lddw r3, 0x68db8bac710cc
    jgt r3, r1, lbb_2357
    div64 r1, 9975
    mul64 r1, 10000
    ja lbb_2359
lbb_2329:
    jeq r3, 0, lbb_2348
    ldxdw r3, [r4+0x8]
    jgt r3, r1, lbb_2378
    sub64 r1, r3
    lddw r3, 0x68db8bac710cc
    jgt r3, r1, lbb_2363
    div64 r1, 9900
    mul64 r1, 10000
    ja lbb_2365
lbb_2339:
    ldxdw r3, [r4+0x10]
    jgt r3, r1, lbb_2378
    sub64 r1, r3
    lddw r3, 0x68db8bac710cc
    jgt r3, r1, lbb_2367
    div64 r1, 9975
    mul64 r1, 10000
    ja lbb_2369
lbb_2348:
    ldxdw r3, [r4+0x10]
    jgt r3, r1, lbb_2378
    sub64 r1, r3
    lddw r3, 0x68db8bac710cc
    jgt r3, r1, lbb_2371
    div64 r1, 9900
    mul64 r1, 10000
    ja lbb_2373
lbb_2357:
    mul64 r1, 10000
    div64 r1, 9975
lbb_2359:
    jgt r3, r1, lbb_2375
lbb_2360:
    div64 r1, 10000
    mul64 r1, r2
    ja lbb_2377
lbb_2363:
    mul64 r1, 10000
    div64 r1, 9900
lbb_2365:
    jgt r3, r1, lbb_2375
    ja lbb_2360
lbb_2367:
    mul64 r1, 10000
    div64 r1, 9975
lbb_2369:
    jgt r3, r1, lbb_2375
    ja lbb_2360
lbb_2371:
    mul64 r1, 10000
    div64 r1, 9900
lbb_2373:
    jgt r3, r1, lbb_2375
    ja lbb_2360
lbb_2375:
    mul64 r1, r2
    div64 r1, 10000
lbb_2377:
    mov64 r0, r1
lbb_2378:
    exit

calculate_optimal_strategy:
    mov64 r9, r4
    stxdw [r10-0x28], r2
    stxdw [r10-0x20], r5
    ldxdw r2, [r5-0xff8]
    stxdw [r10-0xff8], r2
    ldxdw r6, [r5-0xff0]
    stxdw [r10-0xff0], r6
    mov64 r5, r10
    call calculate_upper_bound
    mov64 r7, r0
    mov64 r8, r10
    add64 r8, -24
    stxdw [r10-0x30], r9
    mov64 r1, r9
    mov64 r2, r7
    mov64 r3, r6
    mov64 r4, r8
    call get_quote_and_liquidity
    mov64 r9, r0
    mov64 r1, r8
    ldxdw r2, [r10-0x28]
    mov64 r3, r6
    mov64 r4, r8
    call get_liquidity
    stxdw [r10-0x78], r6
    xor64 r6, 1
    mov64 r1, r8
    mov64 r2, r9
    stxdw [r10-0x38], r6
    mov64 r3, r6
    call get_quote
    mov64 r9, r0
    ldxdw r1, [r10-0x20]
    ldxdw r2, [r1-0xfe0]
    sub64 r9, r7
    mov64 r6, r7
    ldxdw r1, [r1-0xfe8]
    jne r1, 0, lbb_2561
    mov64 r0, 0
    mov64 r1, 1000
    jgt r1, r6, lbb_2564
    stxdw [r10-0x80], r2
    stxdw [r10-0x20], r1
    stxdw [r10-0x40], r6
    mov64 r6, r10
    add64 r6, -24
    ldxdw r1, [r10-0x30]
    mov64 r2, 1000
    ldxdw r8, [r10-0x78]
    mov64 r3, r8
    mov64 r4, r6
    call get_quote_and_liquidity
    stxdw [r10-0x48], r0
    mov64 r1, r6
    ldxdw r2, [r10-0x28]
    mov64 r3, r8
    mov64 r4, r6
    call get_liquidity
    mov64 r1, r6
    ldxdw r6, [r10-0x40]
    ldxdw r2, [r10-0x48]
    ldxdw r3, [r10-0x38]
    call get_quote
    mov64 r4, r6
    add64 r7, -1000
    mov64 r1, 10001
    jgt r1, r7, lbb_2462
    mov64 r2, 0
    stxdw [r10-0x68], r4
    mov64 r1, r4
    mov64 r3, r9
    mov64 r8, 1000
    ja lbb_2476
lbb_2452:
    mov64 r1, r2
    and64 r1, 255
    jgt r1, 14, lbb_2462
    add64 r2, 1
    ldxdw r7, [r10-0x68]
    sub64 r7, r8
    mov64 r1, r4
    mov64 r3, r9
    stxdw [r10-0x20], r8
    jgt r7, 10000, lbb_2476
lbb_2462:
    mov64 r6, r4
    syscall sol_log_compute_units_
    lddw r1, 0x100018e3e
    mov64 r2, 16
    syscall sol_log_
    mov64 r1, r6
    mov64 r2, r9
    mov64 r3, 0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    ldxdw r2, [r10-0x80]
    ja lbb_2561
lbb_2476:
    stxdw [r10-0x50], r3
    stxdw [r10-0x70], r1
    stxdw [r10-0x48], r2
    mov64 r1, r7
    call function_12023
    mov64 r7, r0
    mov64 r1, r7
    lddw r2, 0x3fd8722191a029e3
    call function_11552
    mov64 r1, r0
    call function_9815
    add64 r8, r0
    mov64 r2, r8
    stxdw [r10-0x58], r2
    mov64 r8, r10
    add64 r8, -24
    ldxdw r1, [r10-0x30]
    ldxdw r6, [r10-0x78]
    mov64 r3, r6
    mov64 r4, r8
    call get_quote_and_liquidity
    stxdw [r10-0x40], r0
    mov64 r1, r8
    ldxdw r2, [r10-0x28]
    mov64 r3, r6
    mov64 r4, r8
    call get_liquidity
    mov64 r1, r7
    lddw r2, 0x3fe3c6ef372fe7e6
    call function_11552
    mov64 r1, r0
    call function_9815
    mov64 r9, r0
    mov64 r1, r8
    ldxdw r2, [r10-0x40]
    ldxdw r3, [r10-0x38]
    call get_quote
    mov64 r7, r0
    ldxdw r2, [r10-0x20]
    stxdw [r10-0x20], r2
    add64 r2, r9
    mov64 r9, r10
    add64 r9, -24
    ldxdw r1, [r10-0x30]
    stxdw [r10-0x40], r2
    mov64 r3, r6
    mov64 r4, r9
    call get_quote_and_liquidity
    stxdw [r10-0x60], r0
    mov64 r1, r9
    ldxdw r2, [r10-0x28]
    mov64 r3, r6
    mov64 r4, r9
    call get_liquidity
    ldxdw r1, [r10-0x58]
    sub64 r7, r1
    mov64 r8, r1
    mov64 r2, r1
    ldxdw r1, [r10-0x50]
    mov64 r6, r1
    jsgt r7, r1, lbb_2540
    ldxdw r2, [r10-0x70]
lbb_2540:
    stxdw [r10-0x70], r2
    mov64 r1, r9
    ldxdw r2, [r10-0x60]
    ldxdw r3, [r10-0x38]
    call get_quote
    mov64 r9, r0
    ldxdw r4, [r10-0x40]
    sub64 r9, r4
    jsgt r9, r7, lbb_2550
    ldxdw r8, [r10-0x20]
lbb_2550:
    mov64 r1, r7
    ldxdw r2, [r10-0x48]
    jsgt r7, r6, lbb_2554
    mov64 r1, r6
lbb_2554:
    jsgt r9, r7, lbb_2556
    stxdw [r10-0x68], r4
lbb_2556:
    jsgt r9, r1, lbb_2558
    ldxdw r4, [r10-0x70]
lbb_2558:
    jsgt r9, r1, lbb_2452
    mov64 r9, r1
    ja lbb_2452
lbb_2561:
    stxdw [r2+0x8], r9
    stxdw [r2+0x0], r6
    mov64 r0, 1
lbb_2564:
    exit

calculate_optimal_strategy_deprecated:
    mov64 r9, r4
    stxdw [r10-0x20], r2
    ldxdw r2, [r5-0xff8]
    stxdw [r10-0xff8], r2
    stxdw [r10-0x38], r5
    ldxdw r6, [r5-0xff0]
    stxdw [r10-0xff0], r6
    mov64 r5, r10
    call calculate_upper_bound
    mov64 r7, r0
    mov64 r8, r10
    add64 r8, -24
    stxdw [r10-0x28], r9
    mov64 r1, r9
    mov64 r2, r7
    mov64 r3, r6
    mov64 r4, r8
    call get_quote_and_liquidity
    mov64 r9, r0
    mov64 r1, r8
    ldxdw r2, [r10-0x20]
    mov64 r3, r6
    mov64 r4, r8
    call get_liquidity
    stxdw [r10-0x68], r6
    xor64 r6, 1
    mov64 r1, r8
    mov64 r2, r9
    stxdw [r10-0x30], r6
    mov64 r3, r6
    call get_quote
    mov64 r3, r7
    mov64 r1, 1000
    jgt r1, r7, lbb_2714
    ldxdw r1, [r10-0x38]
    ldxdw r4, [r1-0xfe0]
    ldxdw r1, [r1-0xfe8]
    sub64 r0, r7
    jne r1, 0, lbb_2701
    add64 r7, -1000
    mov64 r1, 101
    jgt r1, r7, lbb_2697
    stxdw [r10-0x78], r0
    stxdw [r10-0x88], r4
    mov64 r4, 0
    mov64 r0, 1000
    stxdw [r10-0x80], r3
    mov64 r5, r3
    mov64 r2, 0
    mov64 r1, 0
    stxdw [r10-0x70], r1
    ja lbb_2637
lbb_2617:
    mov64 r2, r4
    and64 r2, 255
    jgt r2, 30, lbb_2626
    add64 r4, 1
    mov64 r7, r5
    sub64 r7, r6
    mov64 r0, r6
    mov64 r2, r1
    jgt r7, 100, lbb_2637
lbb_2626:
    ldxdw r0, [r10-0x78]
    ldxdw r7, [r10-0x70]
    jsge r7, r1, lbb_2704
    jsge r0, r1, lbb_2704
    mov64 r2, 2
    jsgt r2, r1, lbb_2704
    ldxdw r2, [r10-0x88]
    stxdw [r2+0x8], r1
    stxdw [r2+0x0], r6
    ldxdw r3, [r10-0x80]
    ja lbb_2714
lbb_2637:
    stxdw [r10-0x58], r2
    stxdw [r10-0x40], r4
    div64 r7, 3
    mov64 r2, r7
    stxdw [r10-0x60], r0
    add64 r2, r0
    stxdw [r10-0x48], r2
    mov64 r8, r10
    add64 r8, -24
    ldxdw r1, [r10-0x28]
    ldxdw r6, [r10-0x68]
    mov64 r3, r6
    mov64 r4, r8
    mov64 r9, r5
    call get_quote_and_liquidity
    stxdw [r10-0x38], r0
    mov64 r1, r8
    ldxdw r2, [r10-0x20]
    mov64 r3, r6
    mov64 r4, r8
    call get_liquidity
    mov64 r1, r8
    ldxdw r2, [r10-0x38]
    ldxdw r3, [r10-0x30]
    call get_quote
    mov64 r8, r0
    stxdw [r10-0x38], r9
    sub64 r9, r7
    mov64 r7, r10
    add64 r7, -24
    ldxdw r1, [r10-0x28]
    mov64 r2, r9
    mov64 r3, r6
    mov64 r4, r7
    call get_quote_and_liquidity
    stxdw [r10-0x50], r0
    mov64 r1, r7
    ldxdw r2, [r10-0x20]
    mov64 r3, r6
    ldxdw r6, [r10-0x48]
    mov64 r4, r7
    call get_liquidity
    sub64 r8, r6
    mov64 r1, r7
    ldxdw r2, [r10-0x50]
    ldxdw r3, [r10-0x30]
    call get_quote
    sub64 r0, r9
    jsgt r0, r8, lbb_2687
    ldxdw r6, [r10-0x60]
lbb_2687:
    ldxdw r5, [r10-0x38]
    jsgt r0, r8, lbb_2690
    mov64 r5, r9
lbb_2690:
    mov64 r1, r8
    ldxdw r4, [r10-0x40]
    jsgt r0, r8, lbb_2694
    ldxdw r1, [r10-0x58]
lbb_2694:
    jsgt r0, r8, lbb_2617
    stxdw [r10-0x70], r0
    ja lbb_2617
lbb_2697:
    mov64 r6, 1000
    mov64 r1, 0
lbb_2699:
    mov64 r2, 2
    jsgt r2, r0, lbb_2712
lbb_2701:
    stxdw [r4+0x8], r0
    stxdw [r4+0x0], r3
    ja lbb_2714
lbb_2704:
    ldxdw r3, [r10-0x80]
    ldxdw r4, [r10-0x88]
    jsge r0, r7, lbb_2699
    mov64 r2, 2
    jsgt r2, r7, lbb_2699
    stxdw [r4+0x8], r7
    stxdw [r4+0x0], r5
    ja lbb_2714
lbb_2712:
    stxdw [r4+0x8], r1
    stxdw [r4+0x0], r6
lbb_2714:
    mov64 r0, 1
    jgt r3, 999, lbb_2717
    mov64 r0, 0
lbb_2717:
    exit

entrypoint:
    mov64 r6, r1
    call fast_path_entrypoint
    jne r0, -1, lbb_3115
    mov64 r1, 0
    stxdw [r10-0x708], r1
    stxdw [r10-0x710], r1
    stxdw [r10-0x718], r1
    mov64 r1, r10
    add64 r1, -1792
    stxdw [r10-0x728], r1
    lddw r0, 0x200000000
    jeq r6, 0, lbb_3115
    ldxdw r1, [r6+0x0]
    stxdw [r10-0x720], r1
    jeq r1, 0, lbb_2805
    mov64 r0, r6
    add64 r0, 8
    mov64 r2, 0
    mov64 r3, r10
    add64 r3, -1742
    mov64 r4, 32
    ja lbb_2842
lbb_2741:
    jne r7, 255, lbb_2777
    ldxb r7, [r5+0x9]
    mov64 r0, 1
    mov64 r6, 1
    jne r7, 0, lbb_2747
    mov64 r6, 0
lbb_2747:
    stxb [r3-0x2], r6
    ldxb r7, [r5+0xa]
    mov64 r6, 1
    jne r7, 0, lbb_2752
    mov64 r6, 0
lbb_2752:
    stxb [r3-0x1], r6
    ldxb r6, [r5+0xb]
    jne r6, 0, lbb_2756
    mov64 r0, 0
lbb_2756:
    mov64 r6, r5
    add64 r6, 48
    stxdw [r3-0x12], r6
    mov64 r6, r5
    add64 r6, 16
    stxdw [r3-0x32], r6
    mov64 r6, r5
    add64 r6, 80
    stxdw [r3-0x2a], r6
    stxb [r3+0x0], r0
    ldxdw r0, [r5+0x58]
    add64 r5, 96
    stxdw [r3-0x1a], r5
    stxdw [r3-0x22], r0
    add64 r5, r0
    add64 r5, 10247
    and64 r5, -8
    ldxdw r0, [r5+0x0]
    stxdw [r3-0xa], r0
    mov64 r6, r5
    ja lbb_2800
lbb_2777:
    mul64 r7, 56
    mov64 r5, r10
    add64 r5, -1792
    add64 r5, r7
    ldxb r6, [r5+0x30]
    stxb [r3-0x2], r6
    ldxb r6, [r5+0x31]
    stxb [r3-0x1], r6
    ldxb r6, [r5+0x32]
    stxb [r3+0x0], r6
    ldxdw r6, [r5+0x0]
    stxdw [r3-0x32], r6
    ldxdw r6, [r5+0x20]
    stxdw [r3-0x12], r6
    ldxdw r6, [r5+0x8]
    stxdw [r3-0x2a], r6
    ldxdw r6, [r5+0x10]
    stxdw [r3-0x22], r6
    ldxdw r6, [r5+0x18]
    stxdw [r3-0x1a], r6
    ldxdw r5, [r5+0x28]
    stxdw [r3-0xa], r5
    mov64 r6, r0
lbb_2800:
    add64 r3, 56
    mov64 r0, r6
    add64 r0, 8
    add64 r2, 1
    jgt r1, r2, lbb_2842
lbb_2805:
    ldxdw r1, [r6+0x8]
    add64 r6, 16
    stxdw [r10-0x718], r6
    stxdw [r10-0x710], r1
    add64 r6, r1
    stxdw [r10-0x708], r6
    mov64 r1, r10
    add64 r1, -1832
    call authenticate
    mov64 r1, r0
    mov64 r0, 6000
    jeq r1, 0, lbb_3115
    ldxdw r2, [r10-0x718]
    ldxdw r1, [r2+0x0]
    add64 r2, 8
    stxdw [r10-0x718], r2
    lddw r0, 0x300000000
    lddw r2, 0x292d9bc73b85bfbc
    jsgt r1, r2, lbb_2853
    lddw r2, 0xe5091e91aa8ad5e0
    jsgt r1, r2, lbb_2873
    lddw r2, 0xbc0450a289f098fc
    jsgt r1, r2, lbb_2907
    lddw r2, 0xa3884d89296a3354
    jeq r1, r2, lbb_3044
    lddw r2, 0xa563df65028b38bf
    jeq r1, r2, lbb_3048
    lddw r2, 0xa83379e58f1fd8d8
    jeq r1, r2, lbb_3052
    ja lbb_3115
lbb_2842:
    mov64 r5, r6
    ldxb r7, [r5+0x8]
    jgt r4, r2, lbb_2741
    mov64 r6, r0
    jne r7, 255, lbb_2800
    ldxdw r0, [r5+0x58]
    add64 r5, r0
    add64 r5, 10343
    and64 r5, -8
    mov64 r6, r5
    ja lbb_2800
lbb_2853:
    lddw r2, 0x45666d976bab316c
    jsgt r1, r2, lbb_2890
    lddw r2, 0x381ae4e73aa3e4e2
    jsgt r1, r2, lbb_2921
    lddw r2, 0x2befeb7c0041112f
    jsgt r1, r2, lbb_2967
    lddw r2, 0x292d9bc73b85bfbd
    jeq r1, r2, lbb_3056
    lddw r2, 0x2961478385d144cf
    jeq r1, r2, lbb_2869
    ja lbb_3115
lbb_2869:
    mov64 r1, r10
    add64 r1, -1832
    call close_sandwiches_and_topup_tipper
    ja lbb_3115
lbb_2873:
    lddw r2, 0x22e74b661970b7b
    jsgt r1, r2, lbb_2935
    lddw r2, 0xec3d3620dfb0e382
    jsgt r1, r2, lbb_2978
    lddw r2, 0xe5091e91aa8ad5e1
    jeq r1, r2, lbb_3060
    lddw r2, 0xe815bf00ba9ccfcf
    jeq r1, r2, lbb_2886
    ja lbb_3115
lbb_2886:
    mov64 r1, r10
    add64 r1, -1832
    call create_auth
    ja lbb_3115
lbb_2890:
    lddw r2, 0x700a9133664266ff
    jsgt r1, r2, lbb_2949
    lddw r2, 0x50e664d2f3c2c444
    jsgt r1, r2, lbb_2989
    lddw r2, 0x45666d976bab316d
    jeq r1, r2, lbb_3064
    lddw r2, 0x47baf543786376c3
    jeq r1, r2, lbb_2903
    ja lbb_3115
lbb_2903:
    mov64 r1, r10
    add64 r1, -1832
    call auto_swap_out
    ja lbb_3115
lbb_2907:
    lddw r2, 0xc26f22db83dc51cd
    jsgt r1, r2, lbb_3000
    lddw r2, 0xbc0450a289f098fd
    jeq r1, r2, lbb_3068
    lddw r2, 0xbe9c2bf483e71996
    jeq r1, r2, lbb_2917
    ja lbb_3115
lbb_2917:
    mov64 r1, r10
    add64 r1, -1832
    call fast_path_create_pump_fun_auto_swap_out
    ja lbb_3115
lbb_2921:
    lddw r2, 0x41e6f8e0a8044ead
    jsgt r1, r2, lbb_3011
    lddw r2, 0x381ae4e73aa3e4e3
    jeq r1, r2, lbb_3072
    lddw r2, 0x3c9a358a0a7ea918
    jeq r1, r2, lbb_2931
    ja lbb_3115
lbb_2931:
    mov64 r1, r10
    add64 r1, -1832
    call fast_path_create_tip_static
    ja lbb_3115
lbb_2935:
    lddw r2, 0xbafe0e7d7ead715
    jsgt r1, r2, lbb_3022
    lddw r2, 0x22e74b661970b7c
    jeq r1, r2, lbb_3076
    lddw r2, 0x5eb6198694f4e0b
    jeq r1, r2, lbb_2945
    ja lbb_3115
lbb_2945:
    mov64 r1, r10
    add64 r1, -1832
    call extend_sandwich_tracker
    ja lbb_3115
lbb_2949:
    lddw r2, 0x748a5d14c9d78638
    jsgt r1, r2, lbb_3033
    lddw r2, 0x700a913366426700
    jeq r1, r2, lbb_3080
    lddw r2, 0x735469845667ab29
    jeq r1, r2, lbb_2959
    ja lbb_3115
lbb_2959:
    lddw r1, 0x100018e4f
    mov64 r2, 33
    syscall sol_log_
    mov64 r1, r10
    add64 r1, -1832
    call write_sandwich_tracker_identities
    ja lbb_3115
lbb_2967:
    lddw r2, 0x2befeb7c00411130
    jeq r1, r2, lbb_3084
    lddw r2, 0x3526fef56fc7239c
    jeq r1, r2, lbb_2974
    ja lbb_3115
lbb_2974:
    mov64 r1, r10
    add64 r1, -1832
    call tip_dynamic
    ja lbb_3115
lbb_2978:
    lddw r2, 0xec3d3620dfb0e383
    jeq r1, r2, lbb_3088
    lddw r2, 0xf0fc148216faf218
    jeq r1, r2, lbb_2985
    ja lbb_3115
lbb_2985:
    mov64 r1, r10
    add64 r1, -1832
    call exit_periodic
    ja lbb_3115
lbb_2989:
    lddw r2, 0x50e664d2f3c2c445
    jeq r1, r2, lbb_3092
    lddw r2, 0x6aee002ea7f26ca1
    jeq r1, r2, lbb_2996
    ja lbb_3115
lbb_2996:
    mov64 r1, r10
    add64 r1, -1832
    call create_tipper
    ja lbb_3115
lbb_3000:
    lddw r2, 0xc26f22db83dc51ce
    jeq r1, r2, lbb_3096
    lddw r2, 0xd99b033cc208e410
    jeq r1, r2, lbb_3007
    ja lbb_3115
lbb_3007:
    mov64 r1, r10
    add64 r1, -1832
    call create_global
    ja lbb_3115
lbb_3011:
    lddw r2, 0x41e6f8e0a8044eae
    jeq r1, r2, lbb_3100
    lddw r2, 0x43390bea1a302eeb
    jeq r1, r2, lbb_3018
    ja lbb_3115
lbb_3018:
    mov64 r1, r10
    add64 r1, -1832
    call close_account
    ja lbb_3115
lbb_3022:
    lddw r2, 0xbafe0e7d7ead716
    jeq r1, r2, lbb_3104
    lddw r2, 0x28e138e2b228f4a8
    jeq r1, r2, lbb_3029
    ja lbb_3115
lbb_3029:
    mov64 r1, r10
    add64 r1, -1832
    call fast_path_create_raydium_v4
    ja lbb_3115
lbb_3033:
    lddw r2, 0x748a5d14c9d78639
    jeq r1, r2, lbb_3112
    lddw r2, 0x7fb6e05470000286
    jeq r1, r2, lbb_3040
    ja lbb_3115
lbb_3040:
    mov64 r1, r10
    add64 r1, -1832
    call fast_path_create_pump_fun_auto_swap_in
    ja lbb_3115
lbb_3044:
    mov64 r1, r10
    add64 r1, -1832
    call initialize_token_account_3
    ja lbb_3115
lbb_3048:
    mov64 r1, r10
    add64 r1, -1832
    call fast_path_create_tip_dynamic
    ja lbb_3115
lbb_3052:
    mov64 r1, r10
    add64 r1, -1832
    call exit_price
    ja lbb_3115
lbb_3056:
    mov64 r1, r10
    add64 r1, -1832
    call auto_swap_in
    ja lbb_3115
lbb_3060:
    mov64 r1, r10
    add64 r1, -1832
    call close_sandwich
    ja lbb_3115
lbb_3064:
    mov64 r1, r10
    add64 r1, -1832
    call prepare
    ja lbb_3115
lbb_3068:
    mov64 r1, r10
    add64 r1, -1832
    call tip_static
    ja lbb_3115
lbb_3072:
    mov64 r1, r10
    add64 r1, -1832
    call create_creditor
    ja lbb_3115
lbb_3076:
    mov64 r1, r10
    add64 r1, -1832
    call prepare_full
    ja lbb_3115
lbb_3080:
    mov64 r1, r10
    add64 r1, -1832
    call topup_tipper
    ja lbb_3115
lbb_3084:
    mov64 r1, r10
    add64 r1, -1832
    call create_sandwich_tracker
    ja lbb_3115
lbb_3088:
    mov64 r1, r10
    add64 r1, -1832
    call topup_creditor
    ja lbb_3115
lbb_3092:
    mov64 r1, r10
    add64 r1, -1832
    call update_global
    ja lbb_3115
lbb_3096:
    mov64 r1, r10
    add64 r1, -1832
    call withdraw
    ja lbb_3115
lbb_3100:
    mov64 r1, r10
    add64 r1, -1832
    call exit_inactivity
    ja lbb_3115
lbb_3104:
    lddw r1, 0x100018e9a
    mov64 r2, 30
    syscall sol_log_
    mov64 r1, r10
    add64 r1, -1832
    call write_sandwich_tracker_leaders
    ja lbb_3115
lbb_3112:
    mov64 r1, r10
    add64 r1, -1832
    call exit_direct
lbb_3115:
    exit

fast_path_auto_swap_in_raydium_v4:
    lddw r1, 0x4000082e0
    lddw r2, 0x8f5c570f55dd7921
    stxdw [r1+0x0], r2
    lddw r1, 0x4000952f8
    ldxdw r1, [r1+0x0]
    lddw r2, 0x400008318
    stxdw [r2+0x0], r1
    mov64 r6, 6001
    lddw r1, 0x4000082d8
    ldxdw r1, [r1+0x0]
    jeq r1, 0, lbb_3292
    lddw r1, 0x40000837c
    ldxb r1, [r1+0x0]
    jne r1, 0, lbb_3292
    lddw r1, 0x4000833b0
    ldxdw r1, [r1+0x0]
    lddw r2, 0x40008afe0
    ldxdw r2, [r2+0x0]
    sub64 r2, r1
    lddw r1, 0x40009531a
    ldxb r1, [r1+0x0]
    jeq r1, 0, lbb_3154
    lddw r1, 0x4000833a8
    ldxdw r1, [r1+0x0]
    lddw r2, 0x4000886d8
    ldxdw r2, [r2+0x0]
    sub64 r2, r1
lbb_3154:
    lddw r1, 0x400095300
    ldxdw r1, [r1+0x0]
    jgt r2, r1, lbb_3292
    sub64 r1, r2
    lddw r3, 0x68db8bac710cc
    jgt r3, r1, lbb_3165
    div64 r1, 9975
    mul64 r1, 10000
    ja lbb_3167
lbb_3165:
    mul64 r1, 10000
    div64 r1, 9975
lbb_3167:
    jeq r1, 0, lbb_3292
    lddw r3, 0x400095318
    ldxh r3, [r3+0x0]
    lddw r4, 0x68db8bac710cc
    jgt r4, r1, lbb_3177
    div64 r1, 10000
    mul64 r1, r3
    ja lbb_3179
lbb_3177:
    mul64 r1, r3
    div64 r1, 10000
lbb_3179:
    lddw r3, 0x40009531c
    ldxb r3, [r3+0x0]
    jne r3, 0, lbb_3208
    mov64 r5, 0
    lddw r3, 0x400095310
    lddw r4, 0x400095308
    ldxdw r4, [r4+0x0]
    jgt r2, r4, lbb_3200
    sub64 r4, r2
    lddw r2, 0x68db8bac710cc
    jgt r2, r4, lbb_3197
    div64 r4, 9975
    mul64 r4, 10000
    ja lbb_3199
lbb_3197:
    mul64 r4, 10000
    div64 r4, 9975
lbb_3199:
    mov64 r5, r4
lbb_3200:
    ldxdw r2, [r3+0x0]
    mov64 r3, r1
    jgt r5, r1, lbb_3206
    add64 r5, r2
    rsh64 r5, 1
    mov64 r3, r5
lbb_3206:
    jgt r2, r1, lbb_3208
    mov64 r1, r3
lbb_3208:
    lddw r2, 0x40009531b
    ldxb r7, [r2+0x0]
    lddw r2, 0x4000901f0
    ldxdw r8, [r2+0x0]
    lddw r2, 0x400092aa8
    ldxdw r2, [r2+0x0]
    stxdw [r10-0x20], r2
    lddw r2, 0x40008d8e8
    ldxdw r9, [r2+0x0]
    mov64 r2, 9
    stxb [r10-0x11], r2
    mov64 r2, r9
    div64 r2, 100
    mul64 r2, 95
    jgt r2, r1, lbb_3228
    mov64 r1, r2
lbb_3228:
    stxdw [r10-0x10], r1
    lddw r1, 0x400000980
    mov64 r2, r10
    add64 r2, -17
    stxdw [r1+0x0], r2
    lddw r1, 0x400000968
    lddw r2, 0x400000068
    mov64 r3, 17
    lddw r4, 0x10001a388
    mov64 r5, 1
    syscall sol_invoke_signed_c
    mov64 r6, r0
    jne r6, 0, lbb_3292
    lddw r1, 0x40009531b
    ldxb r4, [r1+0x0]
    lddw r1, 0x40009531a
    ldxb r2, [r1+0x0]
    lddw r1, 0x400092aa8
    ldxdw r1, [r1+0x0]
    lddw r3, 0x40008d8e8
    ldxdw r3, [r3+0x0]
    lddw r5, 0x4000901f0
    ldxdw r5, [r5+0x0]
    stxdw [r10-0xfd0], r5
    stxdw [r10-0xfd8], r3
    stxdw [r10-0xfe0], r1
    mov64 r3, r7
    mov64 r1, r9
    jeq r3, 0, lbb_3267
    mov64 r1, r8
lbb_3267:
    stxdw [r10-0xfe8], r8
    stxdw [r10-0xff0], r9
    stxdw [r10-0xff8], r1
    ldxdw r1, [r10-0x20]
    stxdw [r10-0x1000], r1
    lddw r1, 0x4000952d8
    stxdw [r10-0xfc8], r1
    lddw r1, 0x40005adb8
    stxdw [r10-0xfc0], r1
    mov64 r5, r10
    lddw r1, 0x4000082e0
    mov64 r3, 0
    call sandwich_update_frontrun
    lddw r1, 0x40000abe0
    lddw r2, 0x4000952d8
    lddw r3, 0x400083298
    lddw r4, 0x40005adb8
    call token_data_update_frontrun
lbb_3292:
    mov64 r0, r6
    exit

fast_path_auto_swap_out_raydium_v4:
    lddw r1, 0x4000952e3
    ldxb r8, [r1+0x0]
    lddw r1, 0x4000952e2
    ldxb r9, [r1+0x0]
    lddw r1, 0x4000952d8
    ldxdw r6, [r1+0x0]
    lddw r1, 0x4000952d0
    ldxdw r7, [r1+0x0]
    lddw r1, 0x4000952e0
    ldxh r3, [r1+0x0]
    jeq r3, 65535, lbb_3319
    lddw r1, 0x40005adb8
    ldxdw r2, [r1+0x0]
    lddw r1, 0x400010118
    call sandwich_tracker_is_in_validator_id
    mov64 r1, r0
    mov64 r0, 6000
    jeq r1, 0, lbb_3523
lbb_3319:
    stxdw [r10-0x110], r8
    stxdw [r10-0x120], r7
    stxdw [r10-0x118], r6
    lddw r1, 0x40005adb8
    ldxdw r2, [r1+0x0]
    lddw r1, 0x400010118
    call sandwich_tracker_register
    mov64 r1, 0
    stxw [r10-0x18], r1
    lddw r1, 0x4000833a8
    ldxdw r1, [r1+0x0]
    lddw r2, 0x4000886d8
    ldxdw r2, [r2+0x0]
    sub64 r2, r1
    stxdw [r10-0x10], r2
    lddw r1, 0x4000833b0
    ldxdw r1, [r1+0x0]
    lddw r2, 0x40008afe0
    ldxdw r2, [r2+0x0]
    sub64 r2, r1
    stxdw [r10-0x8], r2
    mov64 r1, r9
    mov64 r8, 1
    mov64 r3, 1
    jne r1, 0, lbb_3351
    mov64 r3, 0
lbb_3351:
    lddw r1, 0x400008338
    ldxdw r7, [r1+0x0]
    mov64 r1, r10
    add64 r1, -24
    mov64 r4, r10
    add64 r4, -48
    mov64 r2, r7
    call get_quote_and_liquidity
    lddw r1, 0x400008330
    ldxdw r1, [r1+0x0]
    mov64 r6, r0
    sub64 r6, r1
    mov64 r1, r6
    mov64 r2, r7
    mov64 r3, r0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    mov64 r0, 6004
    jsgt r8, r6, lbb_3523
    ldxdw r8, [r10-0x110]
    mov64 r1, r8
    mov64 r2, 1
    jne r1, 0, lbb_3378
    mov64 r2, 0
lbb_3378:
    stxdw [r10-0x38], r7
    lddw r1, 0x40000ac68
    call kpl_any_initialized
    jeq r0, 0, lbb_3388
    mov64 r1, r10
    add64 r1, -216
    lddw r2, 0x40000ac68
    ja lbb_3392
lbb_3388:
    mov64 r1, r10
    add64 r1, -216
    lddw r2, 0x40000d798
lbb_3392:
    mov64 r3, 160
    call memcpy
    lddw r1, 0x40000ac48
    ldxdw r1, [r1+0x0]
    stxdw [r10-0xff8], r1
    stxdw [r10-0xff0], r6
    mov64 r1, r10
    add64 r1, -48
    stxdw [r10-0x1000], r1
    mov64 r1, r9
    mov64 r3, 1
    jne r1, 0, lbb_3406
    mov64 r3, 0
lbb_3406:
    mov64 r1, r8
    mov64 r4, 1
    jne r1, 0, lbb_3410
    mov64 r4, 0
lbb_3410:
    mov64 r1, r10
    add64 r1, -216
    mov64 r2, r10
    add64 r2, -56
    mov64 r5, r10
    call kpl_update_in_amount
    lddw r1, 0x4000901f0
    ldxdw r9, [r1+0x0]
    lddw r1, 0x40008d8e8
    ldxdw r7, [r1+0x0]
    lddw r1, 0x400092aa8
    ldxdw r8, [r1+0x0]
    mov64 r1, 9
    stxb [r10-0xe9], r1
    ldxdw r1, [r10-0x38]
    stxdw [r10-0xe8], r1
    lddw r1, 0x400000980
    mov64 r2, r10
    add64 r2, -233
    stxdw [r1+0x0], r2
    lddw r1, 0x400000968
    lddw r2, 0x400000068
    mov64 r3, 17
    lddw r4, 0x10001a388
    mov64 r5, 1
    syscall sol_invoke_signed_c
    jne r0, 0, lbb_3523
    mov64 r0, r9
    mov64 r4, r7
    lddw r1, 0x400092aa8
    ldxdw r1, [r1+0x0]
    lddw r2, 0x4000901f0
    ldxdw r7, [r2+0x0]
    lddw r2, 0x40008d8e8
    ldxdw r9, [r2+0x0]
    mov64 r2, 0
    stxw [r10-0x108], r2
    lddw r2, 0x4000833a8
    ldxdw r2, [r2+0x0]
    lddw r5, 0x4000886d8
    ldxdw r5, [r5+0x0]
    sub64 r5, r2
    stxdw [r10-0x100], r5
    lddw r2, 0x4000833b0
    ldxdw r2, [r2+0x0]
    lddw r5, 0x40008afe0
    ldxdw r5, [r5+0x0]
    sub64 r5, r2
    stxdw [r10-0xf8], r5
    ldxdw r2, [r10-0x110]
    mov64 r5, r2
    mov64 r2, r9
    jne r5, 0, lbb_3478
    mov64 r2, r7
lbb_3478:
    stxdw [r10-0xff0], r2
    stxdw [r10-0xff8], r1
    stxdw [r10-0x1000], r0
    stxdw [r10-0xfe8], r9
    stxdw [r10-0xfe0], r7
    mov64 r5, r10
    lddw r1, 0x4000082e0
    mov64 r2, r6
    mov64 r3, r8
    call sandwich_update_backrun
    stxdw [r10-0x1000], r7
    lddw r1, 0x40005adb8
    stxdw [r10-0xff8], r1
    mov64 r3, r10
    add64 r3, -264
    mov64 r5, r10
    lddw r1, 0x40000abe0
    lddw r2, 0x4000082e0
    mov64 r4, r9
    call token_data_update_backrun
    mov64 r0, 6005
    lddw r1, 0x400008350
    ldxdw r1, [r1+0x0]
    ldxdw r4, [r10-0x118]
    ldxdw r2, [r10-0x120]
    jsgt r2, r1, lbb_3523
    mov64 r0, 0
    lddw r1, 0x4000031e8
    ldxdw r2, [r1+0x0]
    jge r2, r4, lbb_3523
    sub64 r4, r2
    lddw r2, 0x400058540
    ldxdw r3, [r2+0x0]
    sub64 r3, r4
    stxdw [r2+0x0], r3
    ldxdw r2, [r1+0x0]
    add64 r2, r4
    stxdw [r1+0x0], r2
lbb_3523:
    exit

fast_path_create_raydium_v4:
    ldxdw r3, [r1+0x10]
    ldxw r2, [r3+0x0]
    stxw [r10-0x4], r2
    ldxb r3, [r3+0x4]
    stxb [r10-0x5], r3
    ldxdw r6, [r1+0x0]
    ldxdw r3, [r6+0x48]
    jne r3, 0, lbb_3573
    mov64 r2, 104
    stxb [r10-0x8], r2
    lddw r2, 0x7461705f74736166
    stxdw [r10-0x10], r2
    mov64 r2, r10
    add64 r2, -5
    stxdw [r10-0x20], r2
    mov64 r2, 4
    stxdw [r10-0x28], r2
    mov64 r2, r10
    add64 r2, -4
    stxdw [r10-0x30], r2
    mov64 r2, 9
    stxdw [r10-0x38], r2
    mov64 r2, r10
    add64 r2, -16
    stxdw [r10-0x40], r2
    mov64 r2, 1
    stxdw [r10-0x18], r2
    mov64 r3, 3
    stxdw [r10-0x48], r3
    mov64 r3, r10
    add64 r3, -64
    stxdw [r10-0x50], r3
    ldxdw r1, [r1+0x20]
    mov64 r3, r10
    add64 r3, -80
    stxdw [r10-0xff8], r3
    stxdw [r10-0xff0], r2
    stxdw [r10-0x1000], r1
    mov64 r2, r6
    add64 r2, 56
    mov64 r3, r6
    add64 r3, 112
    mov64 r5, r10
    mov64 r1, r6
    mov64 r4, 2360
    call create_account_
    jne r0, 0, lbb_3627
    ldxw r2, [r10-0x4]
lbb_3573:
    ldxdw r1, [r6+0x50]
    stxw [r1+0x930], r2
    lddw r2, 0xc6440cf5fdda263f
    stxdw [r1+0x0], r2
    mov64 r3, 8
    mov64 r2, r1
    add64 r2, 8
    mov64 r4, 0
lbb_3582:
    mov64 r5, r1
    add64 r5, r3
    stxb [r5+0x0], r4
    add64 r3, 1
    jne r3, 1800, lbb_3582
    mov64 r3, 0
lbb_3588:
    mov64 r4, r2
    add64 r4, r3
    lddw r5, 0x100019250
    add64 r5, r3
    ldxb r5, [r5+0x0]
    stxb [r4+0x0], r5
    add64 r3, 1
    jne r3, 952, lbb_3588
    mov64 r3, 0
    mov64 r2, r1
    add64 r2, 1800
    mov64 r4, 0
lbb_3601:
    mov64 r5, r2
    add64 r5, r4
    stxb [r5+0x0], r3
    add64 r4, 1
    jne r4, 512, lbb_3601
    mov64 r3, 0
lbb_3607:
    mov64 r4, r2
    add64 r4, r3
    lddw r5, 0x100019608
    add64 r5, r3
    ldxb r5, [r5+0x0]
    stxb [r4+0x0], r5
    add64 r3, 1
    jne r3, 272, lbb_3607
    mov64 r2, 17
    stxdw [r1+0x928], r2
    stxdw [r1+0x918], r2
    lddw r2, 0x400000768
    stxdw [r1+0x910], r2
    lddw r2, 0x40005d5f0
    stxdw [r1+0x908], r2
    mov64 r0, 0
    stxdw [r1+0x920], r0
lbb_3627:
    exit

memcpy:
    mov64 r0, r1
    mov64 r4, 32
    jgt r4, r3, lbb_3663
    mov64 r4, r2
    or64 r4, r0
    and64 r4, 7
    mov64 r1, r0
    jne r4, 0, lbb_3663
    mov64 r1, r0
lbb_3637:
    ldxdw r4, [r2+0x0]
    stxdw [r1+0x0], r4
    ldxdw r4, [r2+0x8]
    stxdw [r1+0x8], r4
    ldxdw r4, [r2+0x10]
    stxdw [r1+0x10], r4
    ldxdw r4, [r2+0x18]
    stxdw [r1+0x18], r4
    add64 r1, 32
    add64 r2, 32
    add64 r3, -32
    jgt r3, 31, lbb_3637
    mov64 r4, 8
    jgt r4, r3, lbb_3663
    mov64 r4, 0
lbb_3652:
    mov64 r5, r1
    add64 r5, r4
    mov64 r6, r2
    add64 r6, r4
    ldxdw r6, [r6+0x0]
    stxdw [r5+0x0], r6
    add64 r4, 8
    add64 r3, -8
    jgt r3, 7, lbb_3652
    add64 r1, r4
    add64 r2, r4
lbb_3663:
    jeq r3, 0, lbb_3670
lbb_3664:
    ldxb r4, [r2+0x0]
    stxb [r1+0x0], r4
    add64 r1, 1
    add64 r2, 1
    add64 r3, -1
    jne r3, 0, lbb_3664
lbb_3670:
    exit

create_account_:
    mov64 r6, r5
    mov64 r9, r4
    stxdw [r10-0xf8], r3
    mov64 r7, r2
    mov64 r8, r1
    mov64 r1, r9
    call calculate_rent
    stxdw [r10-0x108], r0
    ldxdw r1, [r8+0x0]
    stxdw [r10-0x20], r1
    mov64 r1, 257
    stxh [r10-0x18], r1
    ldxdw r2, [r7+0x0]
    stxh [r10-0x8], r1
    stxdw [r10-0x10], r2
    mov64 r1, r10
    add64 r1, -144
    stxdw [r10-0x100], r1
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -88
    mov64 r2, r7
    mov64 r3, 56
    call memcpy
    stxdw [r10-0xb8], r9
    ldxdw r1, [r10-0x108]
    stxdw [r10-0xc0], r1
    mov64 r1, 0
    stxw [r10-0xc4], r1
    ldxdw r1, [r6-0x1000]
    ldxdw r2, [r1+0x0]
    stxdw [r10-0xb0], r2
    ldxdw r2, [r1+0x8]
    stxdw [r10-0xa8], r2
    ldxdw r2, [r1+0x10]
    stxdw [r10-0xa0], r2
    ldxdw r1, [r1+0x18]
    stxdw [r10-0x98], r1
    ldxdw r1, [r10-0xf8]
    ldxdw r1, [r1+0x0]
    mov64 r2, 52
    stxdw [r10-0xd0], r2
    mov64 r2, r10
    add64 r2, -196
    stxdw [r10-0xd8], r2
    mov64 r2, r10
    add64 r2, -32
    stxdw [r10-0xe8], r2
    stxdw [r10-0xf0], r1
    ldxdw r4, [r6-0xff8]
    ldxdw r5, [r6-0xff0]
    mov64 r1, 2
    stxdw [r10-0xe0], r1
    mov64 r1, r10
    add64 r1, -240
    ldxdw r2, [r10-0x100]
    mov64 r3, 2
    syscall sol_invoke_signed_c
    exit

assign_:
    stxdw [r10-0xa0], r5
    mov64 r7, r4
    mov64 r6, r3
    mov64 r9, r2
    mov64 r2, r1
    ldxdw r1, [r2+0x0]
    mov64 r3, 257
    stxh [r10-0x8], r3
    stxdw [r10-0x10], r1
    mov64 r8, r10
    add64 r8, -72
    mov64 r1, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, 1
    stxw [r10-0x6c], r1
    ldxdw r2, [r6+0x0]
    stxdw [r10-0x68], r2
    ldxdw r2, [r6+0x8]
    stxdw [r10-0x60], r2
    ldxdw r2, [r6+0x10]
    stxdw [r10-0x58], r2
    ldxdw r2, [r6+0x18]
    stxdw [r10-0x50], r2
    ldxdw r2, [r9+0x0]
    mov64 r3, 36
    stxdw [r10-0x78], r3
    mov64 r3, r10
    add64 r3, -108
    stxdw [r10-0x80], r3
    mov64 r3, r10
    add64 r3, -16
    stxdw [r10-0x90], r3
    stxdw [r10-0x98], r2
    stxdw [r10-0x88], r1
    mov64 r1, r10
    add64 r1, -152
    mov64 r2, r8
    mov64 r3, 1
    mov64 r4, r7
    ldxdw r5, [r10-0xa0]
    syscall sol_invoke_signed_c
    exit

transfer_:
    mov64 r6, r5
    mov64 r9, r4
    stxdw [r10-0xf8], r3
    mov64 r8, r2
    mov64 r2, r1
    ldxdw r1, [r2+0x0]
    mov64 r3, 257
    stxh [r10-0x18], r3
    stxdw [r10-0x20], r1
    ldxdw r1, [r8+0x0]
    mov64 r3, 1
    stxh [r10-0x8], r3
    stxdw [r10-0x10], r1
    mov64 r7, r10
    add64 r7, -144
    mov64 r1, r7
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -88
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    stxdw [r10-0xc0], r9
    mov64 r1, 2
    stxw [r10-0xc4], r1
    ldxdw r2, [r10-0xf8]
    ldxdw r2, [r2+0x0]
    mov64 r3, 52
    stxdw [r10-0xd0], r3
    mov64 r3, r10
    add64 r3, -196
    stxdw [r10-0xd8], r3
    mov64 r3, r10
    add64 r3, -32
    stxdw [r10-0xe8], r3
    stxdw [r10-0xf0], r2
    ldxdw r4, [r6-0x1000]
    ldxdw r5, [r6-0xff8]
    stxdw [r10-0xe0], r1
    mov64 r1, r10
    add64 r1, -240
    mov64 r2, r7
    mov64 r3, 2
    syscall sol_invoke_signed_c
    exit

withdraw_:
    mov64 r8, r5
    stxdw [r10-0x48], r4
    mov64 r4, r3
    mov64 r7, r2
    mov64 r9, r1
    mov64 r1, 254
    stxb [r10-0x1], r1
    mov64 r1, 1802396002
    stxw [r10-0x2c], r1
    mov64 r1, r10
    add64 r1, -1
    stxdw [r10-0x18], r1
    mov64 r1, 4
    stxdw [r10-0x20], r1
    mov64 r1, r10
    add64 r1, -44
    stxdw [r10-0x28], r1
    mov64 r6, 1
    stxdw [r10-0x10], r6
    mov64 r1, 2
    stxdw [r10-0x38], r1
    mov64 r1, r10
    add64 r1, -40
    stxdw [r10-0x40], r1
    ldxdw r1, [r9+0x8]
    mov64 r2, r10
    add64 r2, -64
    stxdw [r10-0xff8], r2
    stxdw [r10-0xff0], r6
    ldxdw r2, [r8-0xff8]
    stxdw [r10-0x1000], r2
    ldxdw r3, [r8-0x1000]
    mov64 r5, r10
    mov64 r2, r4
    mov64 r4, r7
    call token_transfer_
    jne r0, 0, lbb_3868
    ldxdw r3, [r8-0xff0]
    ldxdw r4, [r9+0x0]
    mov64 r1, r10
    add64 r1, -64
    stxdw [r10-0x1000], r1
    stxdw [r10-0xff8], r6
    mov64 r5, r10
    mov64 r1, r7
    ldxdw r2, [r10-0x48]
    call transfer_
lbb_3868:
    exit

withdraw:
    ldxdw r6, [r1+0x0]
    ldxdw r8, [r1+0x10]
    mov64 r1, 254
    stxb [r10-0x1], r1
    mov64 r1, 1802396002
    stxw [r10-0x2c], r1
    mov64 r1, r10
    add64 r1, -1
    stxdw [r10-0x18], r1
    mov64 r1, 4
    stxdw [r10-0x20], r1
    mov64 r1, r10
    add64 r1, -44
    stxdw [r10-0x28], r1
    mov64 r9, 1
    stxdw [r10-0x10], r9
    mov64 r1, 2
    stxdw [r10-0x38], r1
    mov64 r1, r10
    add64 r1, -40
    stxdw [r10-0x40], r1
    ldxdw r1, [r8+0x8]
    mov64 r2, r10
    add64 r2, -64
    stxdw [r10-0xff8], r2
    stxdw [r10-0xff0], r9
    mov64 r2, r6
    add64 r2, 280
    stxdw [r10-0x1000], r2
    mov64 r2, r6
    add64 r2, 112
    mov64 r3, r6
    add64 r3, 224
    mov64 r7, r6
    add64 r7, 56
    mov64 r5, r10
    mov64 r4, r7
    call token_transfer_
    jne r0, 0, lbb_3920
    ldxdw r4, [r8+0x0]
    mov64 r1, r10
    add64 r1, -64
    stxdw [r10-0x1000], r1
    stxdw [r10-0xff8], r9
    mov64 r2, r6
    add64 r2, 168
    add64 r6, 336
    mov64 r5, r10
    mov64 r1, r7
    mov64 r3, r6
    call transfer_
lbb_3920:
    exit

get_key_type_optimised:
    ldxdw r2, [r1+0x0]
    lddw r3, 0xcf5a6693f6e05601
    jeq r2, r3, lbb_3959
    lddw r3, 0x5259294f8b5a2aa9
    jeq r2, r3, lbb_3945
    lddw r3, 0x3fc30236c449d94b
    jne r2, r3, lbb_3967
    ldxdw r2, [r1+0x8]
    lddw r3, 0x4c52a316ed907720
    jne r2, r3, lbb_3967
    ldxdw r2, [r1+0x10]
    lddw r3, 0xa9a221f15c97b9a1
    jne r2, r3, lbb_3967
    mov64 r0, 0
    ldxdw r1, [r1+0x18]
    lddw r2, 0xcd8ab6f87decff0c
    jeq r1, r2, lbb_3968
    ja lbb_3967
lbb_3945:
    ldxdw r2, [r1+0x8]
    lddw r3, 0x955bfd93aa502584
    jne r2, r3, lbb_3967
    ldxdw r2, [r1+0x10]
    lddw r3, 0x930c92eba8e6acb5
    jne r2, r3, lbb_3967
    mov64 r0, 1
    ldxdw r1, [r1+0x18]
    lddw r2, 0x73ec200c69432e94
    jeq r1, r2, lbb_3968
    ja lbb_3967
lbb_3959:
    ldxdw r2, [r1+0x8]
    lddw r3, 0xaa5b17bf6815db44
    jne r2, r3, lbb_3967
    ldxdw r2, [r1+0x10]
    lddw r3, 0x3bffd2f597cb8951
    jeq r2, r3, lbb_3969
lbb_3967:
    mov64 r0, 3
lbb_3968:
    exit
lbb_3969:
    mov64 r0, 3
    ldxdw r1, [r1+0x18]
    lddw r2, 0xb0186dfdb62b5d65
    jne r1, r2, lbb_3968
    mov64 r0, 2
    ja lbb_3968

deserialize_swap_optimised:
    mov64 r3, r4
    mov64 r7, r2
    mov64 r6, 0
    ldxdw r1, [r1+0x0]
    ldxdw r1, [r1+0x0]
    lddw r2, 0x3fc30236c449d94b
    jne r1, r2, lbb_4057
    ldxdw r4, [r5-0xfd8]
    ldxdw r0, [r5-0xfe0]
    ldxdw r8, [r5-0xfe8]
    ldxdw r9, [r5-0xff0]
    ldxdw r1, [r5-0xff8]
    ldxdw r2, [r5-0x1000]
    mov64 r5, r7
    add64 r5, 784
    stxdw [r3+0x0], r5
    mov64 r3, r7
    add64 r3, 840
    stxdw [r2+0x0], r3
    mov64 r2, r7
    add64 r2, 896
    stxdw [r1+0x0], r2
    mov64 r3, r7
    add64 r3, 280
    mov64 r2, r7
    add64 r2, 224
    mov64 r1, r7
    add64 r1, 56
    jeq r9, 0, lbb_4008
    ldxdw r5, [r1+0x0]
    stxdw [r9+0x0], r5
lbb_4008:
    mov64 r5, 17
    stxb [r0+0x0], r5
    call raydium_v4_parse_liquidity
    jeq r0, 0, lbb_4057
    ldxdw r1, [r7+0x38]
    ldxdw r2, [r7+0x0]
    stxdw [r8+0x0], r2
    mov64 r6, 1
    stxh [r8+0x18], r6
    stxdw [r8+0x10], r1
    mov64 r2, 0
    stxh [r8+0x8], r2
    ldxdw r3, [r7+0x70]
    stxh [r8+0x28], r2
    stxdw [r8+0x20], r3
    stxh [r8+0x38], r6
    stxdw [r8+0x30], r1
    ldxdw r2, [r7+0xe0]
    stxdw [r8+0x40], r2
    stxh [r8+0x48], r6
    ldxdw r2, [r7+0x118]
    stxdw [r8+0xd0], r1
    stxdw [r8+0xc0], r1
    stxdw [r8+0xb0], r1
    stxdw [r8+0xa0], r1
    stxdw [r8+0x90], r1
    stxdw [r8+0x80], r1
    stxdw [r8+0x70], r1
    stxdw [r8+0x60], r1
    stxdw [r8+0x50], r2
    stxh [r8+0xd8], r6
    stxh [r8+0xc8], r6
    stxh [r8+0xb8], r6
    stxh [r8+0xa8], r6
    stxh [r8+0x98], r6
    stxh [r8+0x88], r6
    stxh [r8+0x78], r6
    stxh [r8+0x68], r6
    stxh [r8+0x58], r6
    ldxdw r1, [r7+0x310]
    stxdw [r8+0xe0], r1
    stxh [r8+0xe8], r6
    ldxdw r1, [r7+0x348]
    stxdw [r8+0xf0], r1
    stxh [r8+0xf8], r6
    ldxdw r1, [r7+0x380]
    mov64 r2, 257
    stxh [r8+0x108], r2
    stxdw [r8+0x100], r1
lbb_4057:
    mov64 r0, r6
    exit

get_swap_instruction_optimised:
    stxdw [r10-0x18], r4
    mov64 r6, r3
    stxdw [r10-0x10], r2
    mov64 r0, 0
    ldxdw r4, [r5-0xfd8]
    ldxdw r7, [r5-0xfe0]
    ldxdw r3, [r5-0xfe8]
    stxdw [r10-0x8], r3
    ldxdw r2, [r5-0xff0]
    ldxdw r8, [r5-0xff8]
    ldxdw r3, [r8+0x0]
    lddw r9, 0xcf5a6693f6e05601
    jeq r3, r9, lbb_4126
    lddw r1, 0x5259294f8b5a2aa9
    jeq r3, r1, lbb_4099
    lddw r1, 0x3fc30236c449d94b
    jne r3, r1, lbb_4182
    ldxdw r1, [r8+0x8]
    lddw r3, 0x4c52a316ed907720
    jne r1, r3, lbb_4182
    ldxdw r1, [r8+0x10]
    lddw r3, 0xa9a221f15c97b9a1
    jne r1, r3, lbb_4182
    ldxdw r1, [r8+0x18]
    lddw r3, 0xcd8ab6f87decff0c
    jeq r1, r3, lbb_4092
    ja lbb_4182
lbb_4092:
    mov64 r3, 9
    mov64 r0, r7
    stxb [r0+0x0], r3
    mov64 r1, 17
    stxdw [r10-0x20], r1
    mov64 r5, 1
    ja lbb_4120
lbb_4099:
    ldxdw r1, [r8+0x8]
    lddw r3, 0x955bfd93aa502584
    jne r1, r3, lbb_4182
    ldxdw r1, [r8+0x10]
    lddw r3, 0x930c92eba8e6acb5
    jne r1, r3, lbb_4182
    ldxdw r1, [r8+0x18]
    lddw r3, 0x73ec200c69432e94
    jeq r1, r3, lbb_4112
    ja lbb_4182
lbb_4112:
    lddw r1, 0xde331ec4da5abe8f
    mov64 r0, r7
    stxdw [r0+0x0], r1
    mov64 r1, 24
    stxdw [r10-0x20], r1
    mov64 r3, 16
    mov64 r5, 8
lbb_4120:
    mov64 r7, r2
    mov64 r9, r4
    ldxdw r6, [r10-0x8]
    ldxdw r2, [r10-0x10]
    ldxdw r4, [r10-0x18]
    ja lbb_4169
lbb_4126:
    ldxdw r3, [r8+0x8]
    lddw r9, 0xaa5b17bf6815db44
    jne r3, r9, lbb_4182
    ldxdw r3, [r8+0x10]
    lddw r9, 0x3bffd2f597cb8951
    jne r3, r9, lbb_4182
    ldxdw r3, [r8+0x18]
    lddw r9, 0xb0186dfdb62b5d65
    jeq r3, r9, lbb_4139
    ja lbb_4182
lbb_4139:
    stxdw [r10-0x28], r2
    mov64 r9, r4
    mov64 r0, r7
    ldxdw r2, [r10-0x10]
    jeq r1, 0, lbb_4154
    lddw r1, 0xad837f01a485e633
    stxdw [r0+0x0], r1
    mov64 r1, 24
    stxdw [r10-0x20], r1
    mov64 r3, 16
    mov64 r5, 8
    ldxdw r6, [r10-0x8]
    ldxdw r4, [r10-0x18]
    ja lbb_4168
lbb_4154:
    ldxdw r1, [r5-0x1000]
    lddw r3, 0xeaebda01123d0666
    stxdw [r0+0x0], r3
    mov64 r3, 0
    call get_quote
    mov64 r2, r0
    mov64 r0, r7
    mov64 r1, 24
    stxdw [r10-0x20], r1
    mov64 r3, 16
    mov64 r5, 8
    mov64 r4, r6
    ldxdw r6, [r10-0x8]
lbb_4168:
    ldxdw r7, [r10-0x28]
lbb_4169:
    mov64 r1, r0
    add64 r0, r5
    stxdw [r0+0x0], r2
    mov64 r2, r1
    add64 r2, r3
    stxdw [r2+0x0], r4
    ldxdw r2, [r10-0x20]
    stxdw [r9+0x20], r2
    stxdw [r9+0x18], r1
    stxdw [r9+0x10], r6
    stxdw [r9+0x8], r7
    stxdw [r9+0x0], r8
    mov64 r0, 1
lbb_4182:
    exit

invoke_bank_signed_optimised:
    mov64 r5, 1802396002
    stxw [r10-0x4], r5
    stxdw [r10-0x18], r4
    mov64 r4, 4
    stxdw [r10-0x20], r4
    mov64 r4, r10
    add64 r4, -4
    stxdw [r10-0x28], r4
    mov64 r4, 1
    stxdw [r10-0x10], r4
    mov64 r4, 2
    stxdw [r10-0x30], r4
    mov64 r4, r10
    add64 r4, -40
    stxdw [r10-0x38], r4
    mov64 r4, r10
    add64 r4, -56
    mov64 r5, 1
    syscall sol_invoke_signed_c
    exit

execute_swap_optimised:
    mov64 r8, r3
    lddw r0, 0x400000000
    ldxdw r3, [r5-0xfd8]
    stxdw [r10-0x80], r3
    ldxdw r3, [r5-0xfe0]
    stxdw [r10-0x90], r3
    ldxdw r3, [r5-0xfe8]
    stxdw [r10-0x88], r3
    ldxdw r7, [r5-0xff0]
    ldxdw r9, [r5-0xff8]
    ldxdw r3, [r9+0x0]
    lddw r6, 0xcf5a6693f6e05601
    jeq r3, r6, lbb_4258
    lddw r1, 0x5259294f8b5a2aa9
    jeq r3, r1, lbb_4242
    lddw r1, 0x3fc30236c449d94b
    jne r3, r1, lbb_4330
    ldxdw r1, [r9+0x8]
    lddw r3, 0x4c52a316ed907720
    jne r1, r3, lbb_4330
    ldxdw r1, [r9+0x10]
    lddw r3, 0xa9a221f15c97b9a1
    jne r1, r3, lbb_4330
    ldxdw r1, [r9+0x18]
    lddw r3, 0xcd8ab6f87decff0c
    jeq r1, r3, lbb_4237
    ja lbb_4330
lbb_4237:
    mov64 r1, 17
    mov64 r3, 9
    stxb [r10-0x50], r3
    mov64 r5, 1
    ja lbb_4290
lbb_4242:
    ldxdw r1, [r9+0x8]
    lddw r3, 0x955bfd93aa502584
    jne r1, r3, lbb_4330
    ldxdw r1, [r9+0x10]
    lddw r3, 0x930c92eba8e6acb5
    jne r1, r3, lbb_4330
    ldxdw r1, [r9+0x18]
    lddw r3, 0x73ec200c69432e94
    jeq r1, r3, lbb_4255
    ja lbb_4330
lbb_4255:
    lddw r1, 0xde331ec4da5abe8f
    ja lbb_4274
lbb_4258:
    ldxdw r3, [r9+0x8]
    lddw r6, 0xaa5b17bf6815db44
    jne r3, r6, lbb_4330
    ldxdw r3, [r9+0x10]
    lddw r6, 0x3bffd2f597cb8951
    jne r3, r6, lbb_4330
    ldxdw r3, [r9+0x18]
    lddw r6, 0xb0186dfdb62b5d65
    jeq r3, r6, lbb_4271
    ja lbb_4330
lbb_4271:
    jeq r1, 0, lbb_4279
    lddw r1, 0xad837f01a485e633
lbb_4274:
    stxdw [r10-0x50], r1
    mov64 r1, 24
    mov64 r3, 16
    mov64 r5, 8
    ja lbb_4290
lbb_4279:
    ldxdw r1, [r5-0x1000]
    lddw r3, 0xeaebda01123d0666
    stxdw [r10-0x50], r3
    mov64 r3, 0
    call get_quote
    mov64 r2, r0
    mov64 r1, 24
    mov64 r3, 16
    mov64 r5, 8
    mov64 r4, r8
lbb_4290:
    ldxdw r8, [r10-0x90]
    mov64 r0, r10
    add64 r0, -80
    mov64 r6, r0
    add64 r6, r5
    stxdw [r6+0x0], r2
    mov64 r2, r0
    add64 r2, r3
    stxdw [r2+0x0], r4
    stxdw [r10-0x58], r1
    stxdw [r10-0x60], r0
    ldxdw r1, [r10-0x88]
    stxdw [r10-0x70], r1
    stxdw [r10-0x78], r9
    stxdw [r10-0x68], r8
    syscall sol_log_compute_units_
    mov64 r1, 1802396002
    stxw [r10-0x4], r1
    ldxdw r1, [r10-0x80]
    stxdw [r10-0x18], r1
    mov64 r1, 4
    stxdw [r10-0x20], r1
    mov64 r1, r10
    add64 r1, -4
    stxdw [r10-0x28], r1
    mov64 r1, 1
    stxdw [r10-0x10], r1
    mov64 r1, 2
    stxdw [r10-0x30], r1
    mov64 r1, r10
    add64 r1, -40
    stxdw [r10-0x38], r1
    mov64 r1, r10
    add64 r1, -120
    mov64 r4, r10
    add64 r4, -56
    mov64 r2, r7
    mov64 r3, r8
    mov64 r5, 1
    syscall sol_invoke_signed_c
lbb_4330:
    exit

update_global_:
    mov64 r6, r1
    ldxdw r7, [r2+0x18]
    ldxb r1, [r6+0xb8]
    jeq r1, 0, lbb_4345
    lddw r1, 0x100018f4f
    mov64 r2, 26
    syscall sol_log_
    ldxdw r1, [r6+0x10]
    stxdw [r7+0x18], r1
    ldxdw r1, [r6+0x8]
    stxdw [r7+0x10], r1
    ldxdw r1, [r6+0x0]
    stxdw [r7+0x8], r1
lbb_4345:
    ldxb r1, [r6+0xb9]
    jeq r1, 0, lbb_4357
    lddw r1, 0x100019014
    mov64 r2, 27
    syscall sol_log_
    add64 r7, 32
    add64 r6, 24
    mov64 r1, r7
    mov64 r2, r6
    mov64 r3, 160
    call memcpy
lbb_4357:
    mov64 r0, 0
    exit

update_global:
    ldxdw r2, [r1+0x0]
    ldxdw r6, [r2+0x50]
    ldxdw r7, [r1+0x10]
    ldxb r1, [r7+0xb8]
    jeq r1, 0, lbb_4374
    lddw r1, 0x100018f4f
    mov64 r2, 26
    syscall sol_log_
    ldxdw r1, [r7+0x10]
    stxdw [r6+0x18], r1
    ldxdw r1, [r7+0x8]
    stxdw [r6+0x10], r1
    ldxdw r1, [r7+0x0]
    stxdw [r6+0x8], r1
lbb_4374:
    ldxb r1, [r7+0xb9]
    jeq r1, 0, lbb_4386
    lddw r1, 0x100019014
    mov64 r2, 27
    syscall sol_log_
    add64 r6, 32
    add64 r7, 24
    mov64 r1, r6
    mov64 r2, r7
    mov64 r3, 160
    call memcpy
lbb_4386:
    mov64 r0, 0
    exit

create_global_:
    mov64 r6, r5
    mov64 r7, r2
    mov64 r2, 27745
    stxh [r10-0x4], r2
    mov64 r2, 1651469415
    stxw [r10-0x8], r2
    mov64 r2, r6
    add64 r2, 184
    stxdw [r10-0x18], r2
    mov64 r2, 6
    stxdw [r10-0x20], r2
    mov64 r2, r10
    add64 r2, -8
    stxdw [r10-0x28], r2
    mov64 r2, 1
    stxdw [r10-0x10], r2
    mov64 r5, 2
    stxdw [r10-0x30], r5
    mov64 r5, r10
    add64 r5, -40
    stxdw [r10-0x38], r5
    mov64 r5, r10
    add64 r5, -56
    stxdw [r10-0xff8], r5
    stxdw [r10-0xff0], r2
    stxdw [r10-0x1000], r4
    mov64 r5, r10
    mov64 r2, r7
    mov64 r4, 320
    call create_account_
    mov64 r8, r0
    jne r8, 0, lbb_4435
    ldxdw r1, [r7+0x18]
    lddw r2, 0x8001c27439650c5c
    stxdw [r1+0x0], r2
    ldxdw r2, [r6+0x0]
    stxdw [r1+0x8], r2
    ldxdw r2, [r6+0x8]
    stxdw [r1+0x10], r2
    ldxdw r2, [r6+0x10]
    stxdw [r1+0x18], r2
    add64 r6, 24
    add64 r1, 32
    mov64 r2, r6
    mov64 r3, 160
    call memcpy
lbb_4435:
    mov64 r0, r8
    exit

create_global:
    ldxdw r7, [r1+0x0]
    ldxdw r2, [r1+0x20]
    ldxdw r6, [r1+0x10]
    mov64 r1, 27745
    stxh [r10-0x4], r1
    mov64 r1, 1651469415
    stxw [r10-0x8], r1
    mov64 r1, r6
    add64 r1, 184
    stxdw [r10-0x18], r1
    mov64 r1, 6
    stxdw [r10-0x20], r1
    mov64 r1, r10
    add64 r1, -8
    stxdw [r10-0x28], r1
    mov64 r1, 1
    stxdw [r10-0x10], r1
    mov64 r3, 2
    stxdw [r10-0x30], r3
    mov64 r3, r10
    add64 r3, -40
    stxdw [r10-0x38], r3
    mov64 r3, r10
    add64 r3, -56
    stxdw [r10-0xff8], r3
    stxdw [r10-0xff0], r1
    stxdw [r10-0x1000], r2
    mov64 r2, r7
    add64 r2, 56
    mov64 r3, r7
    add64 r3, 112
    mov64 r5, r10
    mov64 r1, r7
    mov64 r4, 320
    call create_account_
    mov64 r8, r0
    jne r8, 0, lbb_4489
    ldxdw r1, [r7+0x50]
    lddw r2, 0x8001c27439650c5c
    stxdw [r1+0x0], r2
    ldxdw r2, [r6+0x0]
    stxdw [r1+0x8], r2
    ldxdw r2, [r6+0x8]
    stxdw [r1+0x10], r2
    ldxdw r2, [r6+0x10]
    stxdw [r1+0x18], r2
    add64 r6, 24
    add64 r1, 32
    mov64 r2, r6
    mov64 r3, 160
    call memcpy
lbb_4489:
    mov64 r0, r8
    exit

fast_path_auto_swap_in_pump_fun:
    lddw r1, 0x4000082e0
    lddw r2, 0x8f5c570f55dd7921
    stxdw [r1+0x0], r2
    lddw r1, 0x4000b8d40
    ldxdw r1, [r1+0x0]
    lddw r2, 0x400008318
    stxdw [r2+0x0], r1
    mov64 r6, 6001
    lddw r1, 0x4000082d8
    ldxdw r1, [r1+0x0]
    jeq r1, 0, lbb_4643
    lddw r1, 0x40000837c
    ldxb r1, [r1+0x0]
    jne r1, 0, lbb_4643
    mov64 r1, 1
    stxw [r10-0x18], r1
    lddw r1, 0x400067a48
    ldxdw r1, [r1+0x0]
    stxdw [r10-0x10], r1
    lddw r1, 0x400067a50
    ldxdw r1, [r1+0x0]
    stxdw [r10-0x8], r1
    lddw r2, 0x4000b8d48
    ldxdw r2, [r2+0x0]
    jgt r1, r2, lbb_4643
    sub64 r2, r1
    lddw r3, 0x68db8bac710cc
    jgt r3, r2, lbb_4532
    div64 r2, 9975
    mul64 r2, 10000
    ja lbb_4534
lbb_4532:
    mul64 r2, 10000
    div64 r2, 9975
lbb_4534:
    jeq r2, 0, lbb_4643
    lddw r3, 0x4000b8d60
    ldxh r3, [r3+0x0]
    lddw r4, 0x68db8bac710cc
    jgt r4, r2, lbb_4544
    div64 r2, 10000
    mul64 r2, r3
    ja lbb_4546
lbb_4544:
    mul64 r2, r3
    div64 r2, 10000
lbb_4546:
    lddw r3, 0x4000b8d62
    ldxb r3, [r3+0x0]
    jne r3, 0, lbb_4575
    mov64 r5, 0
    lddw r3, 0x4000b8d58
    lddw r4, 0x4000b8d50
    ldxdw r4, [r4+0x0]
    jgt r1, r4, lbb_4567
    sub64 r4, r1
    lddw r1, 0x68db8bac710cc
    jgt r1, r4, lbb_4564
    div64 r4, 9975
    mul64 r4, 10000
    ja lbb_4566
lbb_4564:
    mul64 r4, 10000
    div64 r4, 9975
lbb_4566:
    mov64 r5, r4
lbb_4567:
    ldxdw r1, [r3+0x0]
    mov64 r3, r2
    jgt r5, r2, lbb_4573
    add64 r5, r1
    rsh64 r5, 1
    mov64 r3, r5
lbb_4573:
    jgt r1, r2, lbb_4575
    mov64 r2, r3
lbb_4575:
    lddw r1, 0x40006f4d8
    ldxdw r7, [r1+0x0]
    mov64 r6, r7
    div64 r6, 100
    mul64 r6, 95
    jgt r6, r2, lbb_4583
    mov64 r2, r6
lbb_4583:
    lddw r9, 0x40006cc20
    ldxdw r8, [r9+0x0]
    mov64 r1, r10
    add64 r1, -24
    mov64 r3, 0
    call get_quote
    stxdw [r10-0x20], r6
    stxdw [r10-0x28], r0
    lddw r1, 0xeaebda01123d0666
    stxdw [r10-0x30], r1
    lddw r1, 0x400000980
    mov64 r2, r10
    add64 r2, -48
    stxdw [r1+0x0], r2
    lddw r1, 0x400000968
    lddw r2, 0x400000068
    mov64 r3, 12
    lddw r4, 0x10001a3b8
    mov64 r5, 1
    syscall sol_invoke_signed_c
    mov64 r6, r0
    jne r6, 0, lbb_4643
    lddw r1, 0x40006f4d8
    ldxdw r1, [r1+0x0]
    ldxdw r2, [r9+0x0]
    stxdw [r10-0xfd0], r2
    stxdw [r10-0xfd8], r1
    stxdw [r10-0xfe0], r1
    stxdw [r10-0xfe8], r8
    stxdw [r10-0xff0], r7
    stxdw [r10-0xff8], r7
    stxdw [r10-0x1000], r7
    lddw r1, 0x4000b8d20
    stxdw [r10-0xfc8], r1
    lddw r1, 0x40005adb8
    stxdw [r10-0xfc0], r1
    mov64 r5, r10
    lddw r1, 0x4000082e0
    mov64 r2, 0
    mov64 r3, 1
    mov64 r4, 0
    call sandwich_update_frontrun
    lddw r1, 0x40000abe0
    lddw r2, 0x4000b8d20
    mov64 r3, 0
    lddw r4, 0x40005adb8
    call token_data_update_frontrun
lbb_4643:
    mov64 r0, r6
    exit

fast_path_auto_swap_out_pump_fun:
    lddw r1, 0x4000b8d20
    ldxdw r9, [r1+0x0]
    lddw r1, 0x4000b8d18
    ldxdw r6, [r1+0x0]
    lddw r1, 0x4000b8d28
    ldxh r3, [r1+0x0]
    jeq r3, 65535, lbb_4664
    lddw r1, 0x40005adb8
    ldxdw r2, [r1+0x0]
    lddw r1, 0x400010118
    call sandwich_tracker_is_in_validator_id
    mov64 r1, r0
    mov64 r0, 6000
    jeq r1, 0, lbb_4823
lbb_4664:
    stxdw [r10-0x110], r6
    lddw r1, 0x40005adb8
    ldxdw r2, [r1+0x0]
    lddw r1, 0x400010118
    call sandwich_tracker_register
    mov64 r8, 1
    stxw [r10-0x18], r8
    lddw r1, 0x400067a48
    ldxdw r1, [r1+0x0]
    stxdw [r10-0x10], r1
    lddw r1, 0x400067a50
    ldxdw r1, [r1+0x0]
    stxdw [r10-0x8], r1
    lddw r1, 0x400008338
    ldxdw r7, [r1+0x0]
    mov64 r1, r10
    add64 r1, -24
    mov64 r4, r10
    add64 r4, -48
    mov64 r2, r7
    mov64 r3, 1
    call get_quote_and_liquidity
    lddw r1, 0x400008330
    ldxdw r1, [r1+0x0]
    mov64 r6, r0
    sub64 r6, r1
    mov64 r1, r6
    mov64 r2, r7
    mov64 r3, r0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    mov64 r0, 6004
    jsgt r8, r6, lbb_4823
    stxdw [r10-0x38], r7
    lddw r1, 0x40000ac68
    mov64 r2, 0
    call kpl_any_initialized
    stxdw [r10-0x118], r9
    jeq r0, 0, lbb_4716
    mov64 r1, r10
    add64 r1, -216
    lddw r2, 0x40000ac68
    ja lbb_4720
lbb_4716:
    mov64 r1, r10
    add64 r1, -216
    lddw r2, 0x40000d798
lbb_4720:
    mov64 r3, 160
    call memcpy
    lddw r1, 0x40000ac48
    ldxdw r1, [r1+0x0]
    stxdw [r10-0xff8], r1
    stxdw [r10-0xff0], r6
    mov64 r1, r10
    add64 r1, -48
    stxdw [r10-0x1000], r1
    mov64 r1, r10
    add64 r1, -216
    mov64 r2, r10
    add64 r2, -56
    mov64 r5, r10
    mov64 r3, 1
    mov64 r4, 0
    call kpl_update_in_amount
    lddw r9, 0x40006cc20
    ldxdw r1, [r9+0x0]
    stxdw [r10-0x120], r1
    lddw r7, 0x40006f4d8
    ldxdw r8, [r7+0x0]
    lddw r1, 0xad837f01a485e633
    stxdw [r10-0xf0], r1
    ldxdw r1, [r10-0x38]
    stxdw [r10-0xe8], r1
    lddw r1, 0x400000980
    mov64 r2, r10
    add64 r2, -240
    stxdw [r1+0x0], r2
    lddw r1, 0x400000968
    lddw r2, 0x400000068
    mov64 r3, 12
    lddw r4, 0x10001a3b8
    mov64 r5, 1
    syscall sol_invoke_signed_c
    jne r0, 0, lbb_4823
    ldxdw r7, [r7+0x0]
    ldxdw r9, [r9+0x0]
    mov64 r1, 1
    stxw [r10-0x108], r1
    lddw r1, 0x400067a48
    ldxdw r1, [r1+0x0]
    stxdw [r10-0x100], r1
    lddw r1, 0x400067a50
    ldxdw r1, [r1+0x0]
    stxdw [r10-0xf8], r1
    stxdw [r10-0xfe8], r9
    stxdw [r10-0xfe0], r7
    stxdw [r10-0xff0], r7
    stxdw [r10-0xff8], r7
    stxdw [r10-0x1000], r8
    mov64 r5, r10
    lddw r1, 0x4000082e0
    mov64 r2, r6
    mov64 r3, r8
    ldxdw r4, [r10-0x120]
    call sandwich_update_backrun
    stxdw [r10-0x1000], r7
    lddw r1, 0x40005adb8
    stxdw [r10-0xff8], r1
    mov64 r3, r10
    add64 r3, -264
    mov64 r5, r10
    lddw r1, 0x40000abe0
    lddw r2, 0x4000082e0
    mov64 r4, r9
    call token_data_update_backrun
    mov64 r0, 6005
    lddw r1, 0x400008350
    ldxdw r1, [r1+0x0]
    ldxdw r4, [r10-0x118]
    ldxdw r2, [r10-0x110]
    jsgt r2, r1, lbb_4823
    mov64 r0, 0
    lddw r1, 0x4000031e8
    ldxdw r2, [r1+0x0]
    jge r2, r4, lbb_4823
    sub64 r4, r2
    lddw r2, 0x400058540
    ldxdw r3, [r2+0x0]
    sub64 r3, r4
    stxdw [r2+0x0], r3
    ldxdw r2, [r1+0x0]
    add64 r2, r4
    stxdw [r1+0x0], r2
lbb_4823:
    exit

fast_path_create_pump_fun_auto_swap_in:
    ldxdw r3, [r1+0x10]
    mov64 r2, 2
    stxw [r10-0x4], r2
    ldxb r3, [r3+0x0]
    stxb [r10-0x5], r3
    ldxdw r6, [r1+0x0]
    ldxdw r3, [r6+0x48]
    jne r3, 0, lbb_4873
    mov64 r2, 104
    stxb [r10-0x8], r2
    lddw r2, 0x7461705f74736166
    stxdw [r10-0x10], r2
    mov64 r2, r10
    add64 r2, -5
    stxdw [r10-0x20], r2
    mov64 r2, 4
    stxdw [r10-0x28], r2
    mov64 r2, r10
    add64 r2, -4
    stxdw [r10-0x30], r2
    mov64 r2, 9
    stxdw [r10-0x38], r2
    mov64 r2, r10
    add64 r2, -16
    stxdw [r10-0x40], r2
    mov64 r2, 1
    stxdw [r10-0x18], r2
    mov64 r3, 3
    stxdw [r10-0x48], r3
    mov64 r3, r10
    add64 r3, -64
    stxdw [r10-0x50], r3
    ldxdw r1, [r1+0x20]
    mov64 r3, r10
    add64 r3, -80
    stxdw [r10-0xff8], r3
    stxdw [r10-0xff0], r2
    stxdw [r10-0x1000], r1
    mov64 r2, r6
    add64 r2, 56
    mov64 r3, r6
    add64 r3, 112
    mov64 r5, r10
    mov64 r1, r6
    mov64 r4, 2360
    call create_account_
    jne r0, 0, lbb_4928
    ldxw r2, [r10-0x4]
lbb_4873:
    ldxdw r1, [r6+0x50]
    stxw [r1+0x930], r2
    lddw r2, 0xc6440cf5fdda263f
    stxdw [r1+0x0], r2
    mov64 r3, 8
    mov64 r2, r1
    add64 r2, 8
    mov64 r4, 0
lbb_4882:
    mov64 r5, r1
    add64 r5, r3
    stxb [r5+0x0], r4
    add64 r3, 1
    jne r3, 1800, lbb_4882
    mov64 r3, 0
lbb_4888:
    mov64 r4, r2
    add64 r4, r3
    lddw r5, 0x100019728
    add64 r5, r3
    ldxb r5, [r5+0x0]
    stxb [r4+0x0], r5
    add64 r3, 1
    jne r3, 672, lbb_4888
    mov64 r3, 0
    mov64 r2, r1
    add64 r2, 1800
    mov64 r4, 0
lbb_4901:
    mov64 r5, r2
    add64 r5, r4
    stxb [r5+0x0], r3
    add64 r4, 1
    jne r4, 512, lbb_4901
    mov64 r3, 0
lbb_4907:
    mov64 r4, r2
    add64 r4, r3
    lddw r5, 0x1000199c8
    add64 r5, r3
    ldxb r5, [r5+0x0]
    stxb [r4+0x0], r5
    add64 r3, 1
    jne r3, 192, lbb_4907
    mov64 r2, 24
    stxdw [r1+0x928], r2
    mov64 r2, 12
    stxdw [r1+0x918], r2
    lddw r2, 0x400000768
    stxdw [r1+0x910], r2
    lddw r2, 0x40005d5f0
    stxdw [r1+0x908], r2
    mov64 r0, 0
    stxdw [r1+0x920], r0
lbb_4928:
    exit

fast_path_create_pump_fun_auto_swap_out:
    ldxdw r3, [r1+0x10]
    mov64 r2, 3
    stxw [r10-0x4], r2
    ldxb r3, [r3+0x0]
    stxb [r10-0x5], r3
    ldxdw r6, [r1+0x0]
    ldxdw r3, [r6+0x48]
    jne r3, 0, lbb_4978
    mov64 r2, 104
    stxb [r10-0x8], r2
    lddw r2, 0x7461705f74736166
    stxdw [r10-0x10], r2
    mov64 r2, r10
    add64 r2, -5
    stxdw [r10-0x20], r2
    mov64 r2, 4
    stxdw [r10-0x28], r2
    mov64 r2, r10
    add64 r2, -4
    stxdw [r10-0x30], r2
    mov64 r2, 9
    stxdw [r10-0x38], r2
    mov64 r2, r10
    add64 r2, -16
    stxdw [r10-0x40], r2
    mov64 r2, 1
    stxdw [r10-0x18], r2
    mov64 r3, 3
    stxdw [r10-0x48], r3
    mov64 r3, r10
    add64 r3, -64
    stxdw [r10-0x50], r3
    ldxdw r1, [r1+0x20]
    mov64 r3, r10
    add64 r3, -80
    stxdw [r10-0xff8], r3
    stxdw [r10-0xff0], r2
    stxdw [r10-0x1000], r1
    mov64 r2, r6
    add64 r2, 56
    mov64 r3, r6
    add64 r3, 112
    mov64 r5, r10
    mov64 r1, r6
    mov64 r4, 2360
    call create_account_
    jne r0, 0, lbb_5033
    ldxw r2, [r10-0x4]
lbb_4978:
    ldxdw r1, [r6+0x50]
    stxw [r1+0x930], r2
    lddw r2, 0xc6440cf5fdda263f
    stxdw [r1+0x0], r2
    mov64 r3, 8
    mov64 r2, r1
    add64 r2, 8
    mov64 r4, 0
lbb_4987:
    mov64 r5, r1
    add64 r5, r3
    stxb [r5+0x0], r4
    add64 r3, 1
    jne r3, 1800, lbb_4987
    mov64 r3, 0
lbb_4993:
    mov64 r4, r2
    add64 r4, r3
    lddw r5, 0x100019a98
    add64 r5, r3
    ldxb r5, [r5+0x0]
    stxb [r4+0x0], r5
    add64 r3, 1
    jne r3, 672, lbb_4993
    mov64 r3, 0
    mov64 r2, r1
    add64 r2, 1800
    mov64 r4, 0
lbb_5006:
    mov64 r5, r2
    add64 r5, r4
    stxb [r5+0x0], r3
    add64 r4, 1
    jne r4, 512, lbb_5006
    mov64 r3, 0
lbb_5012:
    mov64 r4, r2
    add64 r4, r3
    lddw r5, 0x100019d38
    add64 r5, r3
    ldxb r5, [r5+0x0]
    stxb [r4+0x0], r5
    add64 r3, 1
    jne r3, 192, lbb_5012
    mov64 r2, 24
    stxdw [r1+0x928], r2
    mov64 r2, 12
    stxdw [r1+0x918], r2
    lddw r2, 0x400000768
    stxdw [r1+0x910], r2
    lddw r2, 0x40005d5f0
    stxdw [r1+0x908], r2
    mov64 r0, 0
    stxdw [r1+0x920], r0
lbb_5033:
    exit

migrate_token_data_:
    ldxdw r2, [r2+0x18]
    ldxb r3, [r2+0x8]
    jeq r3, 0, lbb_5042
    lddw r1, 0x100018df8
    mov64 r2, 26
    syscall sol_log_
    ja lbb_5286
lbb_5042:
    ldxdw r3, [r1+0x10]
    jne r3, 0, lbb_5049
    lddw r1, 0x100018ff6
    mov64 r2, 29
    syscall sol_log_
    ja lbb_5286
lbb_5049:
    ldxdw r1, [r1+0x18]
    mov64 r3, 1
    stxb [r2+0x8], r3
    ldxdw r4, [r1+0x21]
    stxdw [r2+0x28], r4
    ldxdw r4, [r1+0x19]
    stxdw [r2+0x20], r4
    ldxdw r4, [r1+0x11]
    stxdw [r2+0x18], r4
    ldxdw r4, [r1+0x9]
    stxdw [r2+0x10], r4
    ldxdw r4, [r1+0x29]
    stxdw [r2+0x30], r4
    ldxdw r4, [r1+0x31]
    stxdw [r2+0x38], r4
    ldxdw r4, [r1+0x39]
    stxdw [r2+0x40], r4
    ldxdw r4, [r1+0x41]
    stxdw [r2+0x48], r4
    ldxdw r4, [r1+0x49]
    stxdw [r2+0x50], r4
    ldxdw r4, [r1+0x51]
    stxdw [r2+0x58], r4
    ldxdw r4, [r1+0x59]
    stxdw [r2+0x60], r4
    ldxdw r4, [r1+0x61]
    stxdw [r2+0x68], r4
    ldxb r4, [r1+0x69]
    stxb [r2+0x92], r4
    ldxdw r4, [r1+0x6a]
    stxdw [r2+0x88], r4
    ldxh r4, [r1+0x72]
    stxh [r2+0x90], r4
    ldxb r4, [r1+0x74]
    stxb [r2+0xa2], r4
    ldxdw r4, [r1+0x75]
    stxdw [r2+0x98], r4
    ldxh r4, [r1+0x7d]
    stxh [r2+0xa0], r4
    ldxb r4, [r1+0x7f]
    stxb [r2+0xb2], r4
    ldxdw r4, [r1+0x80]
    stxdw [r2+0xa8], r4
    ldxh r4, [r1+0x88]
    stxh [r2+0xb0], r4
    ldxb r4, [r1+0x8a]
    stxb [r2+0xc2], r4
    ldxdw r4, [r1+0x8b]
    stxdw [r2+0xb8], r4
    ldxh r4, [r1+0x93]
    stxh [r2+0xc0], r4
    ldxb r4, [r1+0x95]
    stxb [r2+0xd2], r4
    ldxdw r4, [r1+0x96]
    stxdw [r2+0xc8], r4
    ldxh r4, [r1+0x9e]
    stxh [r2+0xd0], r4
    ldxb r4, [r1+0xa0]
    stxb [r2+0xe2], r4
    ldxdw r4, [r1+0xa1]
    stxdw [r2+0xd8], r4
    ldxh r4, [r1+0xa9]
    stxh [r2+0xe0], r4
    ldxb r4, [r1+0xab]
    stxb [r2+0xf2], r4
    ldxdw r4, [r1+0xac]
    stxdw [r2+0xe8], r4
    ldxh r4, [r1+0xb4]
    stxh [r2+0xf0], r4
    ldxb r4, [r1+0xb6]
    stxb [r2+0x102], r4
    ldxdw r4, [r1+0xb7]
    stxdw [r2+0xf8], r4
    ldxh r4, [r1+0xbf]
    stxh [r2+0x100], r4
    ldxb r4, [r1+0xc1]
    stxb [r2+0x112], r4
    ldxdw r4, [r1+0xc2]
    stxdw [r2+0x108], r4
    ldxh r4, [r1+0xca]
    stxh [r2+0x110], r4
    ldxb r4, [r1+0xcc]
    stxb [r2+0x122], r4
    ldxdw r4, [r1+0xcd]
    stxdw [r2+0x118], r4
    ldxh r4, [r1+0xd5]
    stxh [r2+0x120], r4
    ldxdw r4, [r1+0xd7]
    stxdw [r2+0x128], r4
    ldxdw r4, [r1+0xdf]
    stxdw [r2+0x130], r4
    ldxdw r4, [r1+0xe7]
    stxdw [r2+0x138], r4
    ldxdw r4, [r1+0xef]
    stxdw [r2+0x140], r4
    ldxdw r4, [r1+0xf7]
    stxdw [r2+0x148], r4
    ldxdw r4, [r1+0xff]
    stxdw [r2+0x150], r4
    ldxdw r4, [r1+0x107]
    stxdw [r2+0x158], r4
    ldxdw r4, [r1+0x10f]
    stxdw [r2+0x160], r4
    ldxdw r4, [r1+0x117]
    stxdw [r2+0x168], r4
    ldxdw r4, [r1+0x11f]
    stxdw [r2+0x170], r4
    ldxdw r4, [r1+0x127]
    stxdw [r2+0x178], r4
    ldxdw r4, [r1+0x12f]
    stxdw [r2+0x180], r4
    ldxb r4, [r1+0x137]
    jne r4, 0, lbb_5163
    mov64 r3, 0
lbb_5163:
    stxb [r2+0x188], r3
    ldxb r3, [r1+0x138]
    stxb [r2+0x189], r3
    ldxb r3, [r1+0x139]
    stxb [r2+0x18a], r3
    ldxb r3, [r1+0x13a]
    stxb [r2+0x18b], r3
    ldxb r3, [r1+0x13b]
    stxb [r2+0x18c], r3
    ldxb r3, [r1+0x13c]
    stxb [r2+0x18d], r3
    ldxb r3, [r1+0x13d]
    stxb [r2+0x18e], r3
    ldxb r3, [r1+0x13e]
    stxb [r2+0x18f], r3
    ldxdw r3, [r1+0x13f]
    stxdw [r2+0x190], r3
    ldxdw r3, [r1+0x147]
    stxdw [r2+0x198], r3
    ldxdw r3, [r1+0x14f]
    stxdw [r2+0x1a0], r3
    ldxdw r3, [r1+0x157]
    stxdw [r2+0x1a8], r3
    ldxdw r3, [r1+0x15f]
    stxdw [r2+0x1b0], r3
    ldxdw r3, [r1+0x167]
    stxdw [r2+0x1b8], r3
    ldxdw r3, [r1+0x16f]
    stxdw [r2+0x1c0], r3
    ldxdw r3, [r1+0x177]
    stxdw [r2+0x1c8], r3
    ldxdw r3, [r1+0x17f]
    stxdw [r2+0x1d0], r3
    ldxdw r3, [r1+0x187]
    stxdw [r2+0x1d8], r3
    ldxdw r3, [r1+0x18f]
    stxdw [r2+0x1e0], r3
    ldxdw r3, [r1+0x197]
    stxdw [r2+0x1e8], r3
    ldxdw r3, [r1+0x19f]
    stxdw [r2+0x1f0], r3
    ldxdw r3, [r1+0x1a7]
    stxdw [r2+0x1f8], r3
    ldxdw r3, [r1+0x1af]
    stxdw [r2+0x200], r3
    ldxdw r3, [r1+0x1b7]
    stxdw [r2+0x208], r3
    ldxdw r3, [r1+0x1bf]
    stxdw [r2+0x210], r3
    ldxdw r3, [r1+0x1c7]
    stxdw [r2+0x218], r3
    ldxdw r3, [r1+0x1cf]
    stxdw [r2+0x220], r3
    ldxdw r3, [r1+0x1d7]
    stxdw [r2+0x228], r3
    ldxdw r3, [r1+0x1df]
    stxdw [r2+0x230], r3
    ldxdw r3, [r1+0x1e7]
    stxdw [r2+0x238], r3
    ldxdw r3, [r1+0x1ef]
    stxdw [r2+0x240], r3
    ldxdw r3, [r1+0x1f7]
    stxdw [r2+0x248], r3
    ldxdw r3, [r1+0x1ff]
    stxdw [r2+0x250], r3
    ldxdw r3, [r1+0x207]
    stxdw [r2+0x258], r3
    ldxdw r3, [r1+0x20f]
    stxdw [r2+0x260], r3
    ldxdw r3, [r1+0x217]
    stxdw [r2+0x268], r3
    ldxdw r3, [r1+0x21f]
    stxdw [r2+0x270], r3
    ldxdw r3, [r1+0x227]
    stxdw [r2+0x278], r3
    ldxdw r3, [r1+0x22f]
    stxdw [r2+0x280], r3
    ldxdw r3, [r1+0x237]
    stxdw [r2+0x288], r3
    ldxdw r3, [r1+0x23f]
    stxdw [r2+0x290], r3
    ldxdw r3, [r1+0x247]
    stxdw [r2+0x298], r3
    ldxdw r3, [r1+0x24f]
    stxdw [r2+0x2a0], r3
    ldxdw r3, [r1+0x257]
    stxdw [r2+0x2a8], r3
    ldxdw r3, [r1+0x25f]
    stxdw [r2+0x2b0], r3
    ldxb r3, [r1+0x267]
    stxb [r2+0x2b8], r3
    ldxdw r3, [r1+0x268]
    stxdw [r2+0x2c0], r3
    ldxdw r3, [r1+0x270]
    stxdw [r2+0x2c8], r3
    ldxdw r3, [r1+0x278]
    stxdw [r2+0x2d0], r3
    ldxdw r3, [r1+0x280]
    stxdw [r2+0x2d8], r3
    ldxdw r3, [r1+0x288]
    stxdw [r2+0x2e0], r3
    ldxdw r3, [r1+0x290]
    stxdw [r2+0x2e8], r3
    ldxdw r3, [r1+0x298]
    stxdw [r2+0x2f0], r3
    ldxdw r3, [r1+0x2a0]
    stxdw [r2+0x2f8], r3
    ldxdw r3, [r1+0x2a8]
    stxdw [r2+0x300], r3
    ldxdw r3, [r1+0x2b0]
    stxdw [r2+0x308], r3
    ldxdw r3, [r1+0x2b8]
    stxdw [r2+0x310], r3
    ldxdw r3, [r1+0x2c0]
    stxdw [r2+0x318], r3
    ldxdw r3, [r1+0x2c8]
    stxdw [r2+0x320], r3
    ldxdw r3, [r1+0x2d0]
    stxdw [r2+0x328], r3
    ldxdw r3, [r1+0x2d8]
    stxdw [r2+0x330], r3
    ldxdw r1, [r1+0x2e0]
    stxdw [r2+0x338], r1
lbb_5286:
    mov64 r0, 0
    exit

migrate_token_data:
    ldxdw r2, [r1+0x0]
    mov64 r1, r2
    add64 r1, 112
    add64 r2, 168
    call migrate_token_data_
    mov64 r0, 0
    exit

auto_swap_in:
    mov64 r8, r1
    ldxdw r1, [r8+0x10]
    ldxh r2, [r1+0x28]
    stxdw [r10-0x568], r2
    ldxdw r2, [r1+0x20]
    stxdw [r10-0x550], r2
    ldxdw r2, [r1+0x18]
    stxdw [r10-0x558], r2
    ldxdw r2, [r1+0x10]
    stxdw [r10-0x560], r2
    ldxdw r2, [r1+0x8]
    ldxb r3, [r1+0x2a]
    stxb [r10-0x1], r3
    ldxb r3, [r1+0x2e]
    stxdw [r10-0x570], r3
    ldxb r5, [r1+0x2d]
    ldxb r7, [r1+0x2c]
    ldxb r0, [r1+0x2b]
    ldxdw r9, [r8+0x0]
    ldxdw r3, [r9+0x1a0]
    ldxdw r4, [r9+0x168]
    stxdw [r4+0x38], r2
    lddw r1, 0x8f5c570f55dd7921
    stxdw [r4+0x0], r1
    mov64 r6, 6001
    ldxdw r1, [r9+0x160]
    jeq r1, 0, lbb_5412
    ldxb r1, [r4+0x9c]
    jne r1, 0, lbb_5412
    stxdw [r10-0x5b0], r7
    stxdw [r10-0x5c8], r5
    stxdw [r10-0x5c0], r4
    stxdw [r10-0x5b8], r3
    mov64 r1, r9
    add64 r1, 728
    stxdw [r10-0x580], r1
    mov64 r1, r9
    add64 r1, 672
    stxdw [r10-0x578], r1
    mov64 r1, r9
    add64 r1, 616
    stxdw [r10-0x590], r1
    mov64 r6, r9
    add64 r6, 560
    mov64 r1, r9
    add64 r1, 224
    stxdw [r10-0x588], r1
    mov64 r7, r0
    mov64 r1, r9
    add64 r1, 168
    stxdw [r10-0x5a0], r1
    mov64 r1, r9
    add64 r1, 112
    stxdw [r10-0x598], r1
    mov64 r1, r10
    add64 r1, -48
    syscall sol_get_clock_sysvar
    stxdw [r10-0x5a8], r7
    mov64 r1, r7
    mov64 r7, 1
    jne r1, 0, lbb_5358
    mov64 r7, 0
lbb_5358:
    syscall sol_log_compute_units_
    ldxdw r1, [r8+0x0]
    mov64 r2, r10
    add64 r2, -1273
    stxdw [r10-0xfc0], r2
    mov64 r2, r10
    add64 r2, -1304
    stxdw [r10-0xfb8], r2
    mov64 r2, r10
    add64 r2, -1272
    stxdw [r10-0xfc8], r2
    mov64 r2, r10
    add64 r2, -320
    stxdw [r10-0xfd0], r2
    mov64 r2, r10
    add64 r2, -1336
    stxdw [r10-0xfd8], r2
    add64 r1, 784
    stxdw [r10-0xfe0], r1
    ldxdw r1, [r10-0x580]
    stxdw [r10-0xfe8], r1
    stxdw [r10-0xff0], r6
    ldxdw r1, [r10-0x590]
    stxdw [r10-0xff8], r1
    ldxdw r1, [r10-0x578]
    stxdw [r10-0x1000], r1
    mov64 r5, r10
    mov64 r1, r7
    ldxdw r2, [r10-0x5a0]
    ldxdw r3, [r10-0x588]
    ldxdw r4, [r10-0x598]
    call deserialize_swap
    lddw r6, 0x400000000
    jeq r0, 0, lbb_5412
    ldxdw r7, [r10-0x5a8]
    mov64 r1, r7
    mov64 r6, 1
    jne r1, 0, lbb_5398
    mov64 r6, 0
lbb_5398:
    syscall sol_log_compute_units_
    mov64 r1, r10
    add64 r1, -1304
    ldxdw r8, [r10-0x558]
    mov64 r2, r8
    mov64 r3, r6
    call get_quote
    ldxdw r3, [r10-0x550]
    jgt r0, r3, lbb_5414
    lddw r1, 0x100018e13
    mov64 r2, 42
    syscall sol_log_
    mov64 r6, 6001
lbb_5412:
    mov64 r0, r6
    exit
lbb_5414:
    ldxdw r1, [r10-0x5b0]
    jeq r1, 0, lbb_5426
    ldxdw r1, [r9+0x78]
    ldxdw r0, [r1+0x0]
    div64 r0, 100
    mul64 r0, 80
    lddw r1, 0xba43b7400
    jgt r1, r0, lbb_5430
    lddw r0, 0xba43b7400
    ja lbb_5430
lbb_5426:
    ldxdw r1, [r9+0xc0]
    ldxdw r0, [r1+0x40]
    div64 r0, 100
    mul64 r0, 95
lbb_5430:
    ldxdw r1, [r10-0x570]
    mov64 r5, r1
    mov64 r4, r7
    mov64 r2, 1
    mov64 r1, 1
    jne r4, 0, lbb_5437
    mov64 r1, 0
lbb_5437:
    jne r5, 0, lbb_5439
    mov64 r2, 0
lbb_5439:
    stxdw [r10-0xfe8], r2
    mov64 r2, r10
    add64 r2, -1352
    stxdw [r10-0xfe0], r2
    stxdw [r10-0xff0], r1
    ldxdw r1, [r10-0x568]
    stxdw [r10-0xff8], r1
    mov64 r7, r0
    stxdw [r10-0x1000], r0
    mov64 r4, r10
    add64 r4, -1304
    mov64 r5, r10
    ldxdw r1, [r10-0x560]
    mov64 r2, r8
    call calculate_optimal_strategy
    mov64 r6, 6002
    jeq r0, 0, lbb_5412
    mov64 r6, 6004
    ldxdw r1, [r10-0x540]
    mov64 r8, 1
    jsgt r8, r1, lbb_5412
    ldxdw r1, [r10-0x5a8]
    jne r1, 0, lbb_5463
    mov64 r8, 0
lbb_5463:
    syscall sol_log_compute_units_
    ldxdw r6, [r10-0x548]
    jgt r7, r6, lbb_5467
    mov64 r6, r7
lbb_5467:
    mov64 r1, r10
    add64 r1, -1304
    mov64 r2, r6
    mov64 r3, r8
    call get_quote
    ldxdw r1, [r9+0x78]
    ldxdw r1, [r1+0x0]
    stxdw [r10-0x550], r1
    ldxdw r1, [r9+0x130]
    ldxdw r1, [r1+0x40]
    stxdw [r10-0x558], r1
    ldxdw r1, [r9+0xc0]
    ldxdw r1, [r1+0x40]
    stxdw [r10-0x560], r1
    ldxdw r1, [r9+0xf8]
    ldxdw r1, [r1+0x40]
    stxdw [r10-0x568], r1
    ldxdw r1, [r9+0x2d8]
    ldxb r2, [r10-0x4f9]
    stxdw [r10-0xfe0], r2
    mov64 r2, r10
    add64 r2, -1
    stxdw [r10-0xfd8], r2
    mov64 r2, r10
    add64 r2, -320
    stxdw [r10-0xfe8], r2
    mov64 r2, r10
    add64 r2, -1272
    stxdw [r10-0xff0], r2
    stxdw [r10-0xff8], r1
    mov64 r1, 0
    stxdw [r10-0x1000], r1
    mov64 r5, r10
    mov64 r1, r8
    mov64 r2, r6
    mov64 r3, r7
    mov64 r4, r0
    call execute_swap
    mov64 r6, r0
    jne r6, 0, lbb_5412
    ldxdw r1, [r9+0x78]
    ldxdw r7, [r1+0x0]
    ldxdw r1, [r9+0xc0]
    ldxdw r8, [r1+0x40]
    ldxdw r1, [r9+0xf8]
    ldxdw r6, [r1+0x40]
    syscall sol_log_compute_units_
    ldxdw r1, [r9+0x1c0]
    stxdw [r10-0xfc8], r1
    stxdw [r10-0xfd0], r6
    stxdw [r10-0xfd8], r8
    stxdw [r10-0xfe0], r7
    ldxdw r1, [r10-0x568]
    stxdw [r10-0xfe8], r1
    ldxdw r1, [r10-0x560]
    stxdw [r10-0xff0], r1
    ldxdw r1, [r10-0x558]
    stxdw [r10-0xff8], r1
    ldxdw r1, [r10-0x550]
    stxdw [r10-0x1000], r1
    ldxdw r1, [r10-0x5b0]
    ldxdw r2, [r10-0x5a8]
    mov64 r3, r2
    mov64 r6, 0
    mov64 r4, 1
    mov64 r2, 1
    jne r3, 0, lbb_5535
    mov64 r2, 0
lbb_5535:
    mov64 r3, 1
    ldxdw r5, [r10-0x5c8]
    jne r1, 0, lbb_5539
    mov64 r3, 0
lbb_5539:
    mov64 r1, r5
    jne r1, 0, lbb_5542
    mov64 r4, 0
lbb_5542:
    mov64 r7, r10
    add64 r7, -48
    stxdw [r10-0xfc0], r7
    mov64 r5, r10
    ldxdw r1, [r10-0x5c0]
    call sandwich_update_frontrun
    ldxdw r2, [r9+0x1c0]
    mov64 r3, r10
    add64 r3, -1336
    ldxdw r1, [r10-0x5b8]
    mov64 r4, r7
    call token_data_update_frontrun
    syscall sol_log_compute_units_
    ja lbb_5412

create_tipper_:
    mov64 r6, r2
    stxb [r10-0x1], r5
    mov64 r2, 29285
    stxh [r10-0x4], r2
    mov64 r2, 1886415220
    stxw [r10-0x8], r2
    mov64 r2, r10
    add64 r2, -1
    stxdw [r10-0x18], r2
    mov64 r2, 6
    stxdw [r10-0x20], r2
    mov64 r2, r10
    add64 r2, -8
    stxdw [r10-0x28], r2
    mov64 r2, 1
    stxdw [r10-0x10], r2
    mov64 r5, 2
    stxdw [r10-0x30], r5
    mov64 r5, r10
    add64 r5, -40
    stxdw [r10-0x38], r5
    mov64 r5, r10
    add64 r5, -56
    stxdw [r10-0xff8], r5
    stxdw [r10-0xff0], r2
    stxdw [r10-0x1000], r4
    mov64 r5, r10
    mov64 r2, r6
    mov64 r4, 8
    call create_account_
    jne r0, 0, lbb_5591
    ldxdw r1, [r6+0x18]
    lddw r2, 0xfc74a147ce401273
    stxdw [r1+0x0], r2
lbb_5591:
    exit

create_tipper:
    ldxdw r6, [r1+0x0]
    ldxdw r2, [r1+0x20]
    ldxdw r1, [r1+0x10]
    ldxb r1, [r1+0x0]
    stxb [r10-0x1], r1
    mov64 r1, 29285
    stxh [r10-0x4], r1
    mov64 r1, 1886415220
    stxw [r10-0x8], r1
    mov64 r1, r10
    add64 r1, -1
    stxdw [r10-0x18], r1
    mov64 r1, 6
    stxdw [r10-0x20], r1
    mov64 r1, r10
    add64 r1, -8
    stxdw [r10-0x28], r1
    mov64 r1, 1
    stxdw [r10-0x10], r1
    mov64 r3, 2
    stxdw [r10-0x30], r3
    mov64 r3, r10
    add64 r3, -40
    stxdw [r10-0x38], r3
    mov64 r3, r10
    add64 r3, -56
    stxdw [r10-0xff8], r3
    stxdw [r10-0xff0], r1
    stxdw [r10-0x1000], r2
    mov64 r2, r6
    add64 r2, 56
    mov64 r3, r6
    add64 r3, 112
    mov64 r5, r10
    mov64 r1, r6
    mov64 r4, 8
    call create_account_
    jne r0, 0, lbb_5634
    ldxdw r1, [r6+0x50]
    lddw r2, 0xfc74a147ce401273
    stxdw [r1+0x0], r2
lbb_5634:
    exit

fast_path_authenticate:
    mov64 r0, 0
    lddw r1, 0x4000031a0
    ldxb r1, [r1+0x0]
    jne r1, 255, lbb_5710
    lddw r1, 0x4000031a1
    ldxb r1, [r1+0x0]
    jeq r1, 0, lbb_5710
    lddw r1, 0x4000031f0
    ldxdw r1, [r1+0x0]
    jne r1, 0, lbb_5710
    lddw r1, 0x400005a00
    ldxb r1, [r1+0x0]
    jne r1, 255, lbb_5710
    lddw r1, 0x400005a50
    ldxdw r1, [r1+0x0]
    jne r1, 40, lbb_5710
    lddw r1, 0x400005a28
    ldxdw r1, [r1+0x0]
    lddw r2, 0x47872dc075ca93c2
    jne r1, r2, lbb_5710
    lddw r1, 0x400005a30
    ldxdw r1, [r1+0x0]
    lddw r2, 0x2ec56c9e7020425
    jne r1, r2, lbb_5710
    lddw r1, 0x400005a38
    ldxdw r1, [r1+0x0]
    lddw r2, 0x82930eec82511b93
    jne r1, r2, lbb_5710
    lddw r1, 0x400005a40
    ldxdw r1, [r1+0x0]
    lddw r2, 0x9f5bb38b82546b1c
    jne r1, r2, lbb_5710
    lddw r1, 0x400005a60
    ldxdw r1, [r1+0x0]
    lddw r2, 0x4000031a8
    ldxdw r2, [r2+0x0]
    jne r2, r1, lbb_5710
    lddw r1, 0x400005a68
    ldxdw r1, [r1+0x0]
    lddw r2, 0x4000031b0
    ldxdw r2, [r2+0x0]
    jne r2, r1, lbb_5710
    lddw r1, 0x400005a70
    ldxdw r1, [r1+0x0]
    lddw r2, 0x4000031b8
    ldxdw r2, [r2+0x0]
    jne r2, r1, lbb_5710
    lddw r1, 0x400005a78
    ldxdw r1, [r1+0x0]
    lddw r2, 0x4000031c0
    ldxdw r2, [r2+0x0]
    mov64 r0, 1
    jeq r2, r1, lbb_5710
    mov64 r0, 0
lbb_5710:
    and64 r0, 1
    exit

fast_path_entrypoint:
    mov64 r0, -1
    jeq r1, 0, lbb_5824
    ldxdw r2, [r1+0x0]
    jeq r2, 0, lbb_5824
    lddw r2, 0x400000058
    ldxdw r2, [r2+0x0]
    jne r2, 2360, lbb_5824
    lddw r2, 0x400000060
    ldxdw r2, [r2+0x0]
    lddw r3, 0xc6440cf5fdda263f
    jne r2, r3, lbb_5824
    mov64 r0, 6000
    lddw r2, 0x4000031a0
    ldxb r2, [r2+0x0]
    jne r2, 255, lbb_5824
    lddw r2, 0x4000031a1
    ldxb r2, [r2+0x0]
    jeq r2, 0, lbb_5824
    lddw r2, 0x4000031f0
    ldxdw r2, [r2+0x0]
    jne r2, 0, lbb_5824
    lddw r2, 0x400005a00
    ldxb r2, [r2+0x0]
    jne r2, 255, lbb_5824
    lddw r2, 0x400005a50
    ldxdw r2, [r2+0x0]
    jne r2, 40, lbb_5824
    lddw r2, 0x400005a28
    ldxdw r2, [r2+0x0]
    lddw r3, 0x47872dc075ca93c2
    jne r2, r3, lbb_5824
    lddw r2, 0x400005a30
    ldxdw r2, [r2+0x0]
    lddw r3, 0x2ec56c9e7020425
    jne r2, r3, lbb_5824
    lddw r2, 0x400005a38
    ldxdw r2, [r2+0x0]
    lddw r3, 0x82930eec82511b93
    jne r2, r3, lbb_5824
    lddw r2, 0x400005a40
    ldxdw r2, [r2+0x0]
    lddw r3, 0x9f5bb38b82546b1c
    jne r2, r3, lbb_5824
    lddw r2, 0x400005a60
    ldxdw r2, [r2+0x0]
    lddw r3, 0x4000031a8
    ldxdw r3, [r3+0x0]
    jne r3, r2, lbb_5824
    lddw r2, 0x400005a68
    ldxdw r2, [r2+0x0]
    lddw r3, 0x4000031b0
    ldxdw r3, [r3+0x0]
    jne r3, r2, lbb_5824
    lddw r2, 0x400005a70
    ldxdw r2, [r2+0x0]
    lddw r3, 0x4000031b8
    ldxdw r3, [r3+0x0]
    jne r3, r2, lbb_5824
    lddw r2, 0x400005a78
    ldxdw r2, [r2+0x0]
    lddw r3, 0x4000031c0
    ldxdw r3, [r3+0x0]
    jne r3, r2, lbb_5824
    lddw r2, 0x400000990
    ldxw r2, [r2+0x0]
    jsgt r2, 2, lbb_5810
    jeq r2, 0, lbb_5817
    jeq r2, 1, lbb_5819
    mov64 r0, -1
    jeq r2, 2, lbb_5808
    ja lbb_5824
lbb_5808:
    call fast_path_auto_swap_in_pump_fun
    ja lbb_5824
lbb_5810:
    jeq r2, 3, lbb_5821
    jeq r2, 256, lbb_5823
    mov64 r0, -1
    jeq r2, 257, lbb_5815
    ja lbb_5824
lbb_5815:
    call fast_path_tip_dynamic
    ja lbb_5824
lbb_5817:
    call fast_path_auto_swap_in_raydium_v4
    ja lbb_5824
lbb_5819:
    call fast_path_auto_swap_out_raydium_v4
    ja lbb_5824
lbb_5821:
    call fast_path_auto_swap_out_pump_fun
    ja lbb_5824
lbb_5823:
    call fast_path_tip_static
lbb_5824:
    exit

get_key_type:
    ldxdw r2, [r1+0x0]
    lddw r3, 0xcf5a6693f6e05601
    jeq r2, r3, lbb_5863
    lddw r3, 0x5259294f8b5a2aa9
    jeq r2, r3, lbb_5849
    lddw r3, 0x3fc30236c449d94b
    jne r2, r3, lbb_5871
    ldxdw r2, [r1+0x8]
    lddw r3, 0x4c52a316ed907720
    jne r2, r3, lbb_5871
    ldxdw r2, [r1+0x10]
    lddw r3, 0xa9a221f15c97b9a1
    jne r2, r3, lbb_5871
    mov64 r0, 0
    ldxdw r1, [r1+0x18]
    lddw r2, 0xcd8ab6f87decff0c
    jeq r1, r2, lbb_5872
    ja lbb_5871
lbb_5849:
    ldxdw r2, [r1+0x8]
    lddw r3, 0x955bfd93aa502584
    jne r2, r3, lbb_5871
    ldxdw r2, [r1+0x10]
    lddw r3, 0x930c92eba8e6acb5
    jne r2, r3, lbb_5871
    mov64 r0, 1
    ldxdw r1, [r1+0x18]
    lddw r2, 0x73ec200c69432e94
    jeq r1, r2, lbb_5872
    ja lbb_5871
lbb_5863:
    ldxdw r2, [r1+0x8]
    lddw r3, 0xaa5b17bf6815db44
    jne r2, r3, lbb_5871
    ldxdw r2, [r1+0x10]
    lddw r3, 0x3bffd2f597cb8951
    jeq r2, r3, lbb_5873
lbb_5871:
    mov64 r0, 3
lbb_5872:
    exit
lbb_5873:
    mov64 r0, 3
    ldxdw r1, [r1+0x18]
    lddw r2, 0xb0186dfdb62b5d65
    jne r1, r2, lbb_5872
    mov64 r0, 2
    ja lbb_5872

deserialize_swap:
    stxdw [r10-0x18], r4
    mov64 r7, r2
    stxdw [r10-0x8], r1
    ldxdw r4, [r5-0xfb8]
    ldxdw r1, [r5-0xfc0]
    ldxdw r2, [r5-0xfc8]
    stxdw [r10-0x38], r2
    ldxdw r6, [r5-0xfd0]
    ldxdw r8, [r5-0xfe0]
    ldxdw r2, [r5-0xff0]
    stxdw [r10-0x30], r2
    ldxdw r9, [r5-0xfe8]
    ldxdw r2, [r9+0x0]
    ldxdw r0, [r2+0x0]
    lddw r2, 0xcf5a6693f6e05601
    stxdw [r10-0x20], r3
    stxdw [r10-0x28], r7
    jeq r0, r2, lbb_6275
    ldxdw r2, [r5-0xfd8]
    lddw r5, 0x5259294f8b5a2aa9
    jeq r0, r5, lbb_6064
    mov64 r5, 0
    stxdw [r10-0x10], r5
    lddw r5, 0x3fc30236c449d94b
    jne r0, r5, lbb_6429
    jeq r2, 0, lbb_5918
    ldxdw r3, [r8+0x0]
    ldxdw r5, [r3+0x18]
    stxdw [r2+0x18], r5
    ldxdw r5, [r3+0x10]
    stxdw [r2+0x10], r5
    ldxdw r5, [r3+0x8]
    stxdw [r2+0x8], r5
    ldxdw r3, [r3+0x0]
    stxdw [r2+0x0], r3
lbb_5918:
    mov64 r2, 17
    stxb [r1+0x0], r2
    mov64 r7, r8
    add64 r7, 112
    mov64 r9, r8
    add64 r9, 168
    mov64 r1, r8
    mov64 r2, r7
    mov64 r3, r9
    call raydium_v4_parse_liquidity
    jeq r0, 0, lbb_6429
    ldxdw r1, [r8+0x0]
    ldxdw r2, [r10-0x30]
    ldxdw r3, [r2+0x0]
    stxdw [r6+0x0], r3
    mov64 r4, 1
    stxh [r6+0x18], r4
    stxdw [r6+0x10], r1
    mov64 r5, 0
    stxh [r6+0x8], r5
    ldxdw r3, [r8+0x38]
    stxh [r6+0x28], r5
    stxdw [r6+0x20], r3
    stxh [r6+0x38], r4
    stxdw [r6+0x30], r1
    ldxdw r3, [r8+0x70]
    stxdw [r6+0x40], r3
    stxh [r6+0x48], r4
    ldxdw r3, [r8+0xa8]
    stxdw [r6+0xd0], r1
    stxdw [r6+0xc0], r1
    stxdw [r6+0xb0], r1
    stxdw [r6+0xa0], r1
    stxdw [r6+0x90], r1
    stxdw [r6+0x80], r1
    stxdw [r6+0x70], r1
    stxdw [r6+0x60], r1
    stxdw [r6+0x50], r3
    stxh [r6+0xd8], r4
    stxh [r6+0xc8], r4
    stxh [r6+0xb8], r4
    stxh [r6+0xa8], r4
    stxh [r6+0x98], r4
    stxh [r6+0x88], r4
    stxh [r6+0x78], r4
    stxh [r6+0x68], r4
    stxh [r6+0x58], r4
    ldxdw r1, [r10-0x28]
    ldxdw r1, [r1+0x0]
    stxdw [r6+0xe0], r1
    stxh [r6+0xe8], r4
    ldxdw r1, [r10-0x20]
    ldxdw r1, [r1+0x0]
    stxdw [r6+0xf0], r1
    mov64 r1, 1
    stxdw [r10-0x10], r1
    stxh [r6+0xf8], r4
    mov64 r1, 257
    ldxdw r3, [r10-0x18]
    ldxdw r3, [r3+0x0]
    stxh [r6+0x108], r1
    stxdw [r6+0x100], r3
    ldxdw r6, [r10-0x38]
    mov64 r1, r6
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 56
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 112
    mov64 r2, r8
    add64 r2, 56
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 168
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 224
    mov64 r2, r7
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 280
    mov64 r2, r9
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 336
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 392
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 448
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 504
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 560
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 616
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 672
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 728
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 784
    ldxdw r2, [r10-0x28]
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 840
    ldxdw r2, [r10-0x20]
    mov64 r3, 56
    call memcpy
    add64 r6, 896
    mov64 r1, r6
    ldxdw r2, [r10-0x18]
    ja lbb_6427
lbb_6064:
    mov64 r3, 0
    stxdw [r10-0x10], r3
    mov64 r9, r8
    add64 r9, 112
    jeq r2, 0, lbb_6078
    ldxdw r3, [r9+0x0]
    ldxdw r5, [r3+0x18]
    stxdw [r2+0x18], r5
    ldxdw r5, [r3+0x10]
    stxdw [r2+0x10], r5
    ldxdw r5, [r3+0x8]
    stxdw [r2+0x8], r5
    ldxdw r3, [r3+0x0]
    stxdw [r2+0x0], r3
lbb_6078:
    mov64 r2, 13
    stxb [r1+0x0], r2
    mov64 r2, r8
    add64 r2, 168
    mov64 r7, r8
    add64 r7, 224
    mov64 r1, r9
    stxdw [r10-0x30], r2
    mov64 r3, r7
    call raydium_cp_parse_liquidity
    jeq r0, 0, lbb_6429
    ldxdw r1, [r10-0x18]
    ldxdw r1, [r1+0x0]
    mov64 r2, 256
    stxh [r6+0x8], r2
    stxdw [r6+0x0], r1
    ldxdw r1, [r8+0x0]
    stxdw [r6+0x10], r1
    mov64 r4, 0
    stxh [r6+0x18], r4
    ldxdw r2, [r8+0x38]
    stxdw [r6+0x20], r2
    mov64 r3, 392
    mov64 r2, 448
    ldxdw r1, [r10-0x8]
    jne r1, 0, lbb_6105
    mov64 r2, 392
lbb_6105:
    stxdw [r10-0x10], r2
    ldxdw r1, [r10-0x8]
    jne r1, 0, lbb_6109
    mov64 r3, 448
lbb_6109:
    mov64 r5, 280
    mov64 r1, 336
    ldxdw r2, [r10-0x8]
    jne r2, 0, lbb_6114
    mov64 r1, 280
lbb_6114:
    stxdw [r10-0x48], r3
    ldxdw r2, [r10-0x8]
    jne r2, 0, lbb_6118
    mov64 r5, 336
lbb_6118:
    stxdw [r10-0x40], r7
    mov64 r2, 168
    mov64 r0, 224
    ldxdw r3, [r10-0x8]
    jne r3, 0, lbb_6124
    mov64 r0, 168
lbb_6124:
    stxdw [r10-0x50], r1
    mov64 r7, r8
    add64 r7, 56
    ldxdw r3, [r10-0x8]
    jne r3, 0, lbb_6130
    mov64 r2, 224
lbb_6130:
    stxh [r6+0x28], r4
    ldxdw r3, [r8+0x70]
    stxdw [r6+0x30], r3
    mov64 r1, 0
    mov64 r4, 1
    stxh [r6+0x38], r4
    ldxdw r3, [r10-0x28]
    ldxdw r3, [r3+0x0]
    stxdw [r6+0x40], r3
    stxh [r6+0x48], r4
    ldxdw r3, [r10-0x20]
    ldxdw r3, [r3+0x0]
    stxdw [r6+0x50], r3
    mov64 r3, r8
    add64 r3, r2
    stxh [r6+0x58], r4
    ldxdw r2, [r3+0x0]
    stxdw [r6+0x60], r2
    mov64 r2, r8
    add64 r2, r0
    stxh [r6+0x68], r4
    ldxdw r2, [r2+0x0]
    stxdw [r6+0x70], r2
    mov64 r2, r8
    add64 r2, r5
    stxh [r6+0x78], r4
    ldxdw r2, [r2+0x0]
    stxdw [r6+0x80], r2
    mov64 r2, r8
    ldxdw r3, [r10-0x50]
    add64 r2, r3
    stxh [r6+0x88], r1
    ldxdw r2, [r2+0x0]
    stxdw [r6+0x90], r2
    mov64 r2, r8
    ldxdw r3, [r10-0x48]
    add64 r2, r3
    stxh [r6+0x98], r1
    ldxdw r2, [r2+0x0]
    stxdw [r6+0xa0], r2
    mov64 r2, r8
    ldxdw r3, [r10-0x10]
    add64 r2, r3
    stxh [r6+0xa8], r1
    ldxdw r2, [r2+0x0]
    stxh [r6+0xb8], r1
    stxdw [r6+0xb0], r2
    ldxdw r1, [r8+0x1f8]
    stxdw [r6+0xc0], r1
    mov64 r1, 1
    stxdw [r10-0x10], r1
    stxh [r6+0xc8], r4
    ldxdw r6, [r10-0x38]
    mov64 r1, r6
    ldxdw r2, [r10-0x18]
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 56
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 112
    mov64 r2, r7
    mov64 r3, 56
    call memcpy
    mov64 r1, r6
    add64 r1, 168
    mov64 r2, r9
    mov64 r9, r6
    mov64 r3, 56
    call memcpy
    mov64 r1, r9
    add64 r1, 224
    ldxdw r2, [r10-0x28]
    mov64 r3, 56
    call memcpy
    mov64 r1, r9
    add64 r1, 280
    ldxdw r2, [r10-0x20]
    mov64 r3, 56
    call memcpy
    ldxdw r7, [r10-0x30]
    mov64 r2, r7
    ldxdw r1, [r10-0x8]
    ldxdw r6, [r10-0x40]
    jne r1, 0, lbb_6219
    mov64 r2, r6
lbb_6219:
    mov64 r1, r9
    add64 r1, 336
    mov64 r3, 56
    call memcpy
    ldxdw r1, [r10-0x8]
    jne r1, 0, lbb_6226
    mov64 r6, r7
lbb_6226:
    mov64 r1, r9
    add64 r1, 392
    mov64 r2, r6
    mov64 r3, 56
    call memcpy
    mov64 r7, r8
    add64 r7, 336
    mov64 r6, r8
    add64 r6, 280
    mov64 r2, r6
    ldxdw r1, [r10-0x8]
    jne r1, 0, lbb_6239
    mov64 r2, r7
lbb_6239:
    mov64 r1, r9
    add64 r1, 448
    mov64 r3, 56
    call memcpy
    ldxdw r1, [r10-0x8]
    jne r1, 0, lbb_6246
    mov64 r7, r6
lbb_6246:
    mov64 r1, r9
    add64 r1, 504
    mov64 r2, r7
    mov64 r3, 56
    call memcpy
    mov64 r7, r8
    add64 r7, 448
    mov64 r6, r8
    add64 r6, 392
    mov64 r2, r6
    ldxdw r1, [r10-0x8]
    jne r1, 0, lbb_6259
    mov64 r2, r7
lbb_6259:
    mov64 r1, r9
    add64 r1, 560
    mov64 r3, 56
    call memcpy
    ldxdw r1, [r10-0x8]
    jne r1, 0, lbb_6266
    mov64 r7, r6
lbb_6266:
    mov64 r1, r9
    add64 r1, 616
    mov64 r2, r7
    mov64 r3, 56
    call memcpy
    add64 r9, 672
    add64 r8, 504
    mov64 r1, r9
    ja lbb_6426
lbb_6275:
    mov64 r2, 0
    stxdw [r10-0x10], r2
    ldxdw r7, [r5-0xff8]
    ldxdw r2, [r5-0x1000]
    stxdw [r10-0x40], r2
    mov64 r2, 12
    stxb [r1+0x0], r2
    mov64 r1, r8
    add64 r1, 168
    stxdw [r10-0x48], r1
    mov64 r2, r4
    call pump_fun_parse_liquidity
    jeq r0, 0, lbb_6429
    stxdw [r10-0x50], r7
    ldxdw r1, [r8+0x0]
    stxdw [r6+0x0], r1
    mov64 r3, 0
    stxh [r6+0x8], r3
    ldxdw r1, [r8+0x38]
    stxdw [r6+0x10], r1
    mov64 r2, 1
    stxh [r6+0x18], r2
    ldxdw r1, [r8+0x70]
    stxdw [r6+0x20], r1
    stxh [r6+0x28], r3
    ldxdw r1, [r8+0xa8]
    stxdw [r6+0x30], r1
    stxh [r6+0x38], r2
    ldxdw r1, [r8+0xe0]
    stxh [r6+0x48], r2
    stxdw [r6+0x40], r1
    mov64 r1, 257
    ldxdw r2, [r10-0x18]
    ldxdw r2, [r2+0x0]
    stxh [r6+0x68], r1
    stxdw [r6+0x60], r2
    ldxdw r1, [r10-0x40]
    ldxdw r1, [r1+0x0]
    stxdw [r6+0x70], r1
    stxh [r6+0x78], r3
    ldxdw r1, [r8+0x150]
    stxdw [r6+0xa0], r1
    stxh [r6+0xa8], r3
    ldxdw r1, [r9+0x0]
    stxh [r6+0xb8], r3
    stxdw [r6+0xb0], r1
    ldxdw r7, [r10-0x38]
    mov64 r1, r7
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r7
    add64 r1, 56
    mov64 r2, r8
    add64 r2, 56
    mov64 r3, 56
    call memcpy
    mov64 r1, r7
    add64 r1, 112
    mov64 r2, r8
    add64 r2, 112
    mov64 r3, 56
    call memcpy
    mov64 r1, r7
    add64 r1, 168
    ldxdw r2, [r10-0x48]
    mov64 r3, 56
    call memcpy
    mov64 r1, r7
    add64 r1, 224
    mov64 r2, r8
    add64 r2, 224
    mov64 r3, 56
    call memcpy
    mov64 r1, r7
    add64 r1, 336
    ldxdw r2, [r10-0x18]
    mov64 r3, 56
    call memcpy
    mov64 r1, r7
    add64 r1, 392
    ldxdw r2, [r10-0x40]
    mov64 r3, 56
    call memcpy
    mov64 r2, r8
    add64 r2, 336
    mov64 r1, r7
    add64 r1, 560
    mov64 r3, 56
    call memcpy
    mov64 r1, r7
    add64 r1, 616
    mov64 r2, r9
    mov64 r3, 56
    call memcpy
    mov64 r4, 0
    mov64 r1, r7
    add64 r1, 504
    stxdw [r10-0x18], r1
    mov64 r9, r7
    add64 r9, 448
    add64 r7, 280
    ldxdw r1, [r10-0x8]
    jeq r1, 0, lbb_6404
    ldxdw r2, [r10-0x28]
    ldxdw r1, [r2+0x0]
    stxdw [r6+0x50], r1
    mov64 r3, 1
    mov64 r1, 1
    stxdw [r10-0x10], r1
    stxh [r6+0x58], r3
    ldxdw r8, [r10-0x50]
    ldxdw r1, [r8+0x0]
    stxdw [r6+0x80], r1
    stxh [r6+0x88], r4
    ldxdw r1, [r10-0x30]
    ldxdw r1, [r1+0x0]
    stxh [r6+0x98], r4
    stxdw [r6+0x90], r1
    mov64 r1, r7
    mov64 r3, 56
    call memcpy
    mov64 r1, r9
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    ldxdw r1, [r10-0x18]
    ldxdw r2, [r10-0x30]
    ja lbb_6427
lbb_6404:
    add64 r8, 280
    ldxdw r2, [r10-0x20]
    ldxdw r1, [r2+0x0]
    stxdw [r6+0x50], r1
    mov64 r1, 1
    stxdw [r10-0x10], r1
    stxh [r6+0x58], r1
    ldxdw r1, [r10-0x30]
    ldxdw r1, [r1+0x0]
    stxdw [r6+0x80], r1
    stxh [r6+0x88], r4
    ldxdw r1, [r8+0x0]
    stxh [r6+0x98], r4
    stxdw [r6+0x90], r1
    mov64 r1, r7
    mov64 r3, 56
    call memcpy
    mov64 r1, r9
    ldxdw r2, [r10-0x30]
    mov64 r3, 56
    call memcpy
    ldxdw r1, [r10-0x18]
lbb_6426:
    mov64 r2, r8
lbb_6427:
    mov64 r3, 56
    call memcpy
lbb_6429:
    ldxdw r0, [r10-0x10]
    exit

get_swap_instruction:
    stxdw [r10-0x18], r2
    mov64 r0, 0
    ldxdw r6, [r5-0xfd8]
    ldxdw r2, [r5-0xfe0]
    ldxdw r7, [r5-0xfe8]
    stxdw [r10-0x10], r7
    ldxdw r7, [r5-0xff0]
    stxdw [r10-0x8], r7
    ldxdw r7, [r5-0x1000]
    ldxdw r5, [r5-0xff8]
    ldxdw r8, [r5+0x0]
    lddw r9, 0xcf5a6693f6e05601
    jeq r8, r9, lbb_6494
    lddw r1, 0x5259294f8b5a2aa9
    jeq r8, r1, lbb_6471
    lddw r1, 0x3fc30236c449d94b
    jne r8, r1, lbb_6528
    ldxdw r1, [r5+0x8]
    lddw r3, 0x4c52a316ed907720
    jne r1, r3, lbb_6528
    ldxdw r1, [r5+0x10]
    lddw r3, 0xa9a221f15c97b9a1
    jne r1, r3, lbb_6528
    ldxdw r1, [r5+0x18]
    lddw r3, 0xcd8ab6f87decff0c
    jeq r1, r3, lbb_6464
    ja lbb_6528
lbb_6464:
    stxdw [r2+0x9], r7
    ldxdw r1, [r10-0x18]
    stxdw [r2+0x1], r1
    mov64 r1, 9
    stxb [r2+0x0], r1
    mov64 r1, 17
    ja lbb_6491
lbb_6471:
    ldxdw r1, [r5+0x8]
    lddw r3, 0x955bfd93aa502584
    jne r1, r3, lbb_6528
    ldxdw r1, [r5+0x10]
    lddw r3, 0x930c92eba8e6acb5
    jne r1, r3, lbb_6528
    ldxdw r1, [r5+0x18]
    lddw r3, 0x73ec200c69432e94
    jeq r1, r3, lbb_6484
    ja lbb_6528
lbb_6484:
    stxdw [r2+0x10], r7
    ldxdw r1, [r10-0x18]
    stxdw [r2+0x8], r1
    lddw r1, 0xde331ec4da5abe8f
    stxdw [r2+0x0], r1
    mov64 r1, 24
lbb_6491:
    ldxdw r0, [r10-0x8]
    ldxdw r8, [r10-0x10]
    ja lbb_6522
lbb_6494:
    ldxdw r8, [r5+0x8]
    lddw r9, 0xaa5b17bf6815db44
    jne r8, r9, lbb_6528
    ldxdw r8, [r5+0x10]
    lddw r9, 0x3bffd2f597cb8951
    jne r8, r9, lbb_6528
    ldxdw r8, [r5+0x18]
    lddw r9, 0xb0186dfdb62b5d65
    jeq r8, r9, lbb_6507
    ja lbb_6528
lbb_6507:
    ldxdw r0, [r10-0x8]
    ldxdw r8, [r10-0x10]
    jeq r1, 0, lbb_6516
    stxdw [r2+0x10], r7
    ldxdw r1, [r10-0x18]
    stxdw [r2+0x8], r1
    lddw r1, 0xad837f01a485e633
    ja lbb_6520
lbb_6516:
    stxdw [r2+0x10], r3
    stxdw [r2+0x8], r4
    lddw r1, 0xeaebda01123d0666
lbb_6520:
    stxdw [r2+0x0], r1
    mov64 r1, 24
lbb_6522:
    stxdw [r6+0x20], r1
    stxdw [r6+0x18], r2
    stxdw [r6+0x10], r8
    stxdw [r6+0x8], r0
    stxdw [r6+0x0], r5
    mov64 r0, 1
lbb_6528:
    exit

invoke_bank_signed:
    mov64 r5, 1802396002
    stxw [r10-0x4], r5
    stxdw [r10-0x18], r4
    mov64 r4, 4
    stxdw [r10-0x20], r4
    mov64 r4, r10
    add64 r4, -4
    stxdw [r10-0x28], r4
    mov64 r4, 1
    stxdw [r10-0x10], r4
    mov64 r4, 2
    stxdw [r10-0x30], r4
    mov64 r4, r10
    add64 r4, -40
    stxdw [r10-0x38], r4
    mov64 r4, r10
    add64 r4, -56
    mov64 r5, 1
    syscall sol_invoke_signed_c
    exit

execute_swap:
    mov64 r6, r3
    stxdw [r10-0x90], r2
    lddw r0, 0x400000000
    ldxdw r2, [r5-0xfd8]
    stxdw [r10-0x80], r2
    ldxdw r9, [r5-0xfe0]
    ldxdw r2, [r5-0xfe8]
    stxdw [r10-0x88], r2
    ldxdw r7, [r5-0xff0]
    ldxdw r8, [r5-0x1000]
    ldxdw r5, [r5-0xff8]
    ldxdw r2, [r5+0x0]
    lddw r3, 0xcf5a6693f6e05601
    jeq r2, r3, lbb_6616
    lddw r1, 0x5259294f8b5a2aa9
    jeq r2, r1, lbb_6591
    lddw r1, 0x3fc30236c449d94b
    jne r2, r1, lbb_6674
    ldxdw r1, [r5+0x8]
    lddw r2, 0x4c52a316ed907720
    jne r1, r2, lbb_6674
    ldxdw r1, [r5+0x10]
    lddw r2, 0xa9a221f15c97b9a1
    jne r1, r2, lbb_6674
    ldxdw r1, [r5+0x18]
    lddw r2, 0xcd8ab6f87decff0c
    jeq r1, r2, lbb_6584
    ja lbb_6674
lbb_6584:
    stxdw [r10-0x47], r8
    ldxdw r1, [r10-0x90]
    stxdw [r10-0x4f], r1
    mov64 r1, 9
    stxb [r10-0x50], r1
    mov64 r1, 17
    ja lbb_6611
lbb_6591:
    ldxdw r1, [r5+0x8]
    lddw r2, 0x955bfd93aa502584
    jne r1, r2, lbb_6674
    ldxdw r1, [r5+0x10]
    lddw r2, 0x930c92eba8e6acb5
    jne r1, r2, lbb_6674
    ldxdw r1, [r5+0x18]
    lddw r2, 0x73ec200c69432e94
    jeq r1, r2, lbb_6604
    ja lbb_6674
lbb_6604:
    stxdw [r10-0x40], r8
    ldxdw r1, [r10-0x90]
    stxdw [r10-0x48], r1
    lddw r1, 0xde331ec4da5abe8f
    stxdw [r10-0x50], r1
    mov64 r1, 24
lbb_6611:
    mov64 r3, r9
    mov64 r2, r7
    ldxdw r0, [r10-0x80]
    ldxdw r7, [r10-0x88]
    ja lbb_6646
lbb_6616:
    ldxdw r2, [r5+0x8]
    lddw r3, 0xaa5b17bf6815db44
    jne r2, r3, lbb_6674
    ldxdw r2, [r5+0x10]
    lddw r3, 0x3bffd2f597cb8951
    jne r2, r3, lbb_6674
    ldxdw r2, [r5+0x18]
    lddw r3, 0xb0186dfdb62b5d65
    jeq r2, r3, lbb_6629
    ja lbb_6674
lbb_6629:
    mov64 r3, r9
    mov64 r2, r7
    ldxdw r0, [r10-0x80]
    ldxdw r7, [r10-0x88]
    jeq r1, 0, lbb_6640
    stxdw [r10-0x40], r8
    ldxdw r1, [r10-0x90]
    stxdw [r10-0x48], r1
    lddw r1, 0xad837f01a485e633
    ja lbb_6644
lbb_6640:
    stxdw [r10-0x40], r6
    stxdw [r10-0x48], r4
    lddw r1, 0xeaebda01123d0666
lbb_6644:
    stxdw [r10-0x50], r1
    mov64 r1, 24
lbb_6646:
    stxdw [r10-0x58], r1
    mov64 r1, r10
    add64 r1, -80
    stxdw [r10-0x60], r1
    stxdw [r10-0x70], r7
    stxdw [r10-0x78], r5
    stxdw [r10-0x68], r3
    mov64 r1, 1802396002
    stxw [r10-0x4], r1
    stxdw [r10-0x18], r0
    mov64 r1, 4
    stxdw [r10-0x20], r1
    mov64 r1, r10
    add64 r1, -4
    stxdw [r10-0x28], r1
    mov64 r1, 1
    stxdw [r10-0x10], r1
    mov64 r1, 2
    stxdw [r10-0x30], r1
    mov64 r1, r10
    add64 r1, -40
    stxdw [r10-0x38], r1
    mov64 r1, r10
    add64 r1, -120
    mov64 r4, r10
    add64 r4, -56
    mov64 r5, 1
    syscall sol_invoke_signed_c
lbb_6674:
    exit

exit_:
    mov64 r6, r2
    ldxdw r2, [r1+0x10]
    stxdw [r10-0x550], r2
    ldxdw r2, [r1+0x8]
    stxdw [r10-0x548], r2
    ldxdw r2, [r1+0x0]
    stxdw [r10-0x560], r2
    ldxb r8, [r1+0x18]
    ldxb r1, [r1+0x19]
    stxb [r10-0x1], r1
    ldxdw r1, [r6+0x130]
    stxdw [r10-0x558], r1
    mov64 r1, r10
    add64 r1, -1233
    stxdw [r10-0xfc0], r1
    mov64 r1, r10
    add64 r1, -1264
    stxdw [r10-0xfb8], r1
    mov64 r1, r10
    add64 r1, -1232
    stxdw [r10-0xfc8], r1
    mov64 r1, r10
    add64 r1, -280
    stxdw [r10-0xfd0], r1
    mov64 r1, 0
    stxdw [r10-0xfd8], r1
    mov64 r1, r6
    add64 r1, 560
    stxdw [r10-0xfe0], r1
    mov64 r1, r6
    add64 r1, 504
    stxdw [r10-0xfe8], r1
    mov64 r1, r6
    add64 r1, 336
    stxdw [r10-0xff0], r1
    mov64 r1, r6
    add64 r1, 392
    stxdw [r10-0xff8], r1
    mov64 r2, r6
    add64 r2, 168
    mov64 r3, r6
    add64 r3, 224
    mov64 r4, r6
    add64 r4, 112
    mov64 r9, r6
    add64 r9, 448
    stxdw [r10-0x1000], r9
    mov64 r5, r10
    mov64 r1, r8
    stxdw [r10-0x568], r4
    call deserialize_swap
    lddw r7, 0x400000000
    jeq r0, 0, lbb_6827
    mov64 r7, 1
    stxdw [r10-0x578], r8
    jne r8, 0, lbb_6733
    mov64 r7, 0
lbb_6733:
    mov64 r1, r10
    add64 r1, -1264
    mov64 r4, r10
    add64 r4, -1288
    ldxdw r8, [r10-0x548]
    mov64 r2, r8
    mov64 r3, r7
    call get_quote_and_liquidity
    ldxdw r1, [r6+0xf8]
    ldxdw r1, [r1+0x40]
    stxdw [r10-0x570], r1
    ldxdw r1, [r6+0x1f8]
    ldxb r2, [r10-0x4d1]
    stxdw [r10-0xfe0], r2
    mov64 r2, r10
    add64 r2, -1
    stxdw [r10-0xfd8], r2
    mov64 r2, r10
    add64 r2, -280
    stxdw [r10-0xfe8], r2
    mov64 r2, r10
    add64 r2, -1232
    stxdw [r10-0xff0], r2
    stxdw [r10-0xff8], r1
    ldxdw r1, [r10-0x550]
    stxdw [r10-0x1000], r1
    mov64 r5, r10
    mov64 r1, r7
    mov64 r2, r8
    mov64 r3, r8
    mov64 r4, r0
    call execute_swap
    mov64 r7, r0
    jne r7, 0, lbb_6827
    mov64 r7, 6003
    ldxdw r1, [r6+0xf8]
    ldxdw r1, [r1+0x40]
    ldxdw r2, [r10-0x570]
    jgt r2, r1, lbb_6827
    ldxdw r2, [r10-0x570]
    sub64 r1, r2
    ldxdw r3, [r10-0x558]
    ldxdw r2, [r3+0x60]
    add64 r1, r2
    stxdw [r3+0x60], r1
    mov64 r7, 0
    mov64 r8, 1
    ldxdw r1, [r10-0x578]
    jne r1, 0, lbb_6783
    mov64 r8, 0
lbb_6783:
    mov64 r2, r10
    add64 r2, -1288
    stxdw [r10-0x548], r2
    ldxdw r1, [r10-0x558]
    mov64 r3, r8
    call token_data_update_price
    ldxdw r1, [r6+0xc0]
    ldxdw r2, [r1+0x40]
    ldxdw r1, [r10-0x558]
    ldxdw r3, [r10-0x548]
    mov64 r4, r8
    call token_data_update_token_stats
    ldxdw r1, [r6+0x8]
    ldxdw r1, [r1+0x0]
    ldxdw r2, [r10-0x560]
    jge r1, r2, lbb_6827
    mov64 r2, 1802396002
    stxw [r10-0x50c], r2
    mov64 r2, r10
    add64 r2, -1
    stxdw [r10-0x520], r2
    mov64 r2, 4
    stxdw [r10-0x528], r2
    mov64 r2, r10
    add64 r2, -1292
    stxdw [r10-0x530], r2
    mov64 r2, 1
    stxdw [r10-0x518], r2
    mov64 r3, 2
    stxdw [r10-0x538], r3
    mov64 r3, r10
    add64 r3, -1328
    stxdw [r10-0x540], r3
    ldxdw r4, [r10-0x560]
    sub64 r4, r1
    mov64 r1, r10
    add64 r1, -1344
    stxdw [r10-0x1000], r1
    stxdw [r10-0xff8], r2
    mov64 r5, r10
    ldxdw r1, [r10-0x568]
    mov64 r2, r6
    mov64 r3, r9
    call transfer_
lbb_6827:
    mov64 r0, r7
    exit

exit_direct:
    ldxdw r2, [r1+0x0]
    ldxdw r1, [r1+0x10]
    call exit_
    exit

exit_periodic:
    mov64 r6, r1
    ldxdw r1, [r6+0x10]
    ldxb r2, [r1+0x12]
    stxdw [r10-0x60], r2
    ldxh r2, [r1+0x10]
    stxdw [r10-0x50], r2
    ldxdw r7, [r1+0x8]
    ldxdw r1, [r1+0x0]
    stxdw [r10-0x58], r1
    ldxdw r9, [r6+0x0]
    ldxdw r8, [r9+0x130]
    mov64 r1, r10
    add64 r1, -40
    syscall sol_get_clock_sysvar
    ldxdw r1, [r8+0x218]
    ldxdw r2, [r10-0x28]
    jgt r1, r2, lbb_6852
    ldxb r3, [r8+0x188]
    jeq r3, 0, lbb_6864
lbb_6852:
    lddw r1, 0x100018f6a
    mov64 r2, 18
    syscall sol_log_
    ldxdw r2, [r8+0x218]
    ldxdw r1, [r10-0x28]
lbb_6858:
    mov64 r3, 0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    mov64 r0, 6000
lbb_6863:
    exit
lbb_6864:
    mov64 r3, r2
    sub64 r3, r1
    jgt r2, r1, lbb_6869
    sub64 r1, r2
    mov64 r3, r1
lbb_6869:
    jge r3, r7, lbb_6878
    lddw r1, 0x100018eb9
    mov64 r2, 18
    mov64 r6, r3
    syscall sol_log_
    ldxdw r2, [r8+0x218]
    mov64 r1, r6
    ja lbb_6858
lbb_6878:
    ldxdw r1, [r9+0xc0]
    ldxdw r1, [r1+0x40]
    mov64 r2, 254
    stxb [r10-0x2f], r2
    ldxdw r2, [r10-0x60]
    stxb [r10-0x30], r2
    mov64 r2, 0
    stxdw [r10-0x38], r2
    ldxdw r2, [r10-0x58]
    stxdw [r10-0x48], r2
    ldxdw r2, [r10-0x50]
    mul64 r1, r2
    div64 r1, 10000
    stxdw [r10-0x40], r1
    ldxdw r2, [r6+0x0]
    mov64 r1, r10
    add64 r1, -72
    call exit_
    jne r0, 0, lbb_6863
    ldxdw r1, [r10-0x28]
    stxdw [r8+0x218], r1
    ja lbb_6863

exit_inactivity:
    mov64 r7, r1
    ldxdw r1, [r7+0x10]
    ldxb r2, [r1+0x21]
    stxdw [r10-0x50], r2
    ldxb r8, [r1+0x20]
    ldxdw r6, [r1+0x18]
    ldxdw r2, [r1+0x10]
    stxdw [r10-0x68], r2
    ldxdw r2, [r1+0x8]
    stxdw [r10-0x60], r2
    ldxdw r1, [r1+0x0]
    stxdw [r10-0x58], r1
    ldxdw r1, [r7+0x0]
    ldxdw r9, [r1+0x130]
    mov64 r1, r10
    add64 r1, -40
    syscall sol_get_clock_sysvar
    ldxdw r1, [r9+0x210]
    ldxdw r2, [r10-0x28]
    jge r2, r1, lbb_6921
    jne r1, -1, lbb_6923
lbb_6921:
    ldxb r3, [r9+0x188]
    jeq r3, 0, lbb_6935
lbb_6923:
    lddw r1, 0x100018f7d
    mov64 r2, 17
    syscall sol_log_
    ldxdw r2, [r9+0x210]
    ldxdw r1, [r10-0x28]
lbb_6929:
    mov64 r3, 0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    mov64 r0, 6000
lbb_6934:
    exit
lbb_6935:
    mov64 r3, r8
    mov64 r8, r2
    sub64 r8, r1
    jgt r2, r1, lbb_6941
    sub64 r1, r2
    mov64 r8, r1
lbb_6941:
    jge r8, r6, lbb_6949
    lddw r1, 0x100018e87
    mov64 r2, 18
    syscall sol_log_
    mov64 r1, r8
    mov64 r2, r6
    ja lbb_6929
lbb_6949:
    ldxdw r1, [r10-0x50]
    stxb [r10-0x2f], r1
    stxb [r10-0x30], r3
    ldxdw r1, [r10-0x68]
    stxdw [r10-0x38], r1
    ldxdw r1, [r10-0x60]
    stxdw [r10-0x40], r1
    ldxdw r1, [r10-0x58]
    stxdw [r10-0x48], r1
    ldxdw r2, [r7+0x0]
    mov64 r1, r10
    add64 r1, -72
    call exit_
    jne r0, 0, lbb_6934
    ldxdw r1, [r10-0x28]
    stxdw [r9+0x210], r1
    ja lbb_6934

exit_price:
    mov64 r0, 6000
    ldxdw r3, [r1+0x10]
    ldxb r4, [r3+0x18]
    ldxdw r2, [r1+0x0]
    ldxdw r6, [r2+0x130]
    ldxb r1, [r6+0x2b8]
    jge r1, r4, lbb_6994
    ldxb r1, [r6+0x188]
    jne r1, 0, lbb_6994
    ldxb r1, [r3+0x1a]
    ldxb r4, [r3+0x19]
    ldxdw r5, [r3+0x10]
    ldxdw r0, [r3+0x8]
    ldxdw r3, [r3+0x0]
    stxb [r10-0x7], r1
    stxb [r10-0x8], r4
    stxdw [r10-0x10], r5
    stxdw [r10-0x18], r0
    stxdw [r10-0x20], r3
    mov64 r1, r10
    add64 r1, -32
    call exit_
    jne r0, 0, lbb_6994
    ldxdw r1, [r6+0x2b0]
    stxdw [r6+0x2a8], r1
    ldxb r1, [r6+0x2b8]
    add64 r1, 1
    stxb [r6+0x2b8], r1
lbb_6994:
    exit

prepare_full:
    mov64 r7, r1
    mov64 r0, 0
    ldxdw r6, [r7+0x0]
    ldxdw r1, [r6+0xf0]
    jne r1, 0, lbb_7044
    ldxdw r1, [r7+0x10]
    mov64 r2, r6
    add64 r2, 392
    mov64 r8, r6
    add64 r8, 336
    mov64 r4, r6
    add64 r4, 280
    mov64 r9, r6
    add64 r9, 168
    mov64 r3, r6
    add64 r3, 112
    stxdw [r10-0x18], r1
    ldxb r1, [r1+0x0]
    stxdw [r10-0xff8], r8
    stxdw [r10-0xff0], r1
    stxdw [r10-0x10], r2
    stxdw [r10-0x1000], r2
    mov64 r5, r10
    mov64 r1, r6
    mov64 r2, r9
    stxdw [r10-0x8], r4
    call create_token_account_
    jne r0, 0, lbb_7044
    ldxdw r1, [r6+0xc0]
    ldxdw r1, [r1-0x8]
    stxdw [r6+0xb8], r1
    ldxdw r4, [r6+0x70]
    mov64 r1, r8
    mov64 r2, r9
    ldxdw r3, [r10-0x8]
    call token_initialize_account_3_
    jne r0, 0, lbb_7044
    mov64 r3, r6
    add64 r3, 224
    ldxdw r1, [r10-0x18]
    ldxb r1, [r1+0x1]
    ldxdw r2, [r7+0x20]
    stxdw [r10-0x1000], r2
    stxdw [r10-0xff8], r1
    mov64 r5, r10
    ldxdw r1, [r10-0x10]
    mov64 r2, r6
    ldxdw r4, [r10-0x8]
    call create_token_data_
lbb_7044:
    exit

topup_creditor:
    mov64 r0, 0
    ldxdw r6, [r1+0x0]
    ldxdw r1, [r1+0x10]
    ldxdw r9, [r1+0x0]
    ldxdw r1, [r6+0xb0]
    ldxdw r7, [r1+0x0]
    jge r7, r9, lbb_7111
    mov64 r4, r6
    add64 r4, 336
    mov64 r2, r6
    add64 r2, 224
    mov64 r3, r6
    add64 r3, 168
    mov64 r8, r6
    add64 r8, 56
    lddw r1, 0x10001a3e8
    stxdw [r10-0xfe8], r1
    mov64 r1, 1
    stxdw [r10-0xfe0], r1
    mov64 r1, r6
    add64 r1, 392
    stxdw [r10-0xff0], r1
    stxdw [r10-0x8], r4
    stxdw [r10-0xff8], r4
    mov64 r1, r6
    add64 r1, 448
    stxdw [r10-0x1000], r1
    mov64 r4, r6
    add64 r4, 280
    mov64 r5, r10
    mov64 r1, r8
    stxdw [r10-0x10], r2
    stxdw [r10-0x18], r3
    call associated_token_create_
    jne r0, 0, lbb_7111
    mov64 r2, r6
    add64 r2, 112
    ldxdw r1, [r6+0xf8]
    ldxdw r1, [r1-0x8]
    stxdw [r6+0xf0], r1
    lddw r1, 0x10001a3e8
    stxdw [r10-0xff8], r1
    mov64 r1, 1
    stxdw [r10-0xff0], r1
    ldxdw r1, [r10-0x8]
    stxdw [r10-0x1000], r1
    sub64 r9, r7
    mov64 r5, r10
    mov64 r1, r9
    ldxdw r3, [r10-0x10]
    mov64 r4, r8
    call token_transfer_
    jne r0, 0, lbb_7111
    lddw r1, 0x10001a3f8
    stxdw [r10-0x1000], r1
    mov64 r1, 1
    stxdw [r10-0xff8], r1
    mov64 r5, r10
    ldxdw r1, [r10-0x10]
    ldxdw r2, [r10-0x18]
    mov64 r3, r2
    ldxdw r4, [r10-0x8]
    call token_close_account_
lbb_7111:
    exit

create_creditor:
    ldxdw r6, [r1+0x0]
    ldxdw r1, [r1+0x20]
    lddw r2, 0x10001a448
    stxdw [r10-0xff8], r2
    mov64 r2, 1
    stxdw [r10-0xff0], r2
    stxdw [r10-0x1000], r1
    mov64 r2, r6
    add64 r2, 56
    mov64 r3, r6
    add64 r3, 112
    mov64 r5, r10
    mov64 r1, r6
    mov64 r4, 8
    call create_account_
    jne r0, 0, lbb_7133
    ldxdw r1, [r6+0x50]
    lddw r2, 0x99b5d4f3307208f7
    stxdw [r1+0x0], r2
lbb_7133:
    exit

tip_dynamic_:
    stxdw [r10-0x40], r3
    stxdw [r10-0x48], r2
    mov64 r7, r1
    ldxdw r1, [r4+0x18]
    stxdw [r10-0x30], r1
    mov64 r1, r10
    add64 r1, -40
    syscall sol_get_clock_sysvar
    ldxdw r8, [r7+0x0]
    lddw r1, 0x2f5cc235403ddc54
    xor64 r8, r1
    ldxdw r9, [r7+0x8]
    xor64 r9, r1
    ldxdw r6, [r7+0x10]
    xor64 r6, r1
    stxdw [r10-0x38], r7
    ldxdw r7, [r7+0x18]
    xor64 r7, r1
    mov64 r1, r8
    mov64 r2, r9
    mov64 r3, r6
    mov64 r4, r7
    mov64 r5, 0
    syscall sol_log_64_
    ldxdw r1, [r10-0x30]
    ldxdw r1, [r1+0x70]
    jne r9, 0, lbb_7165
    mov64 r9, r1
    mul64 r9, r8
    div64 r9, 10000
lbb_7165:
    mov64 r8, r1
    mul64 r8, r6
    div64 r8, 10000
    mov64 r2, r8
    mov64 r3, r9
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    ldxdw r1, [r10-0x28]
    jgt r7, r1, lbb_7176
    mov64 r9, r8
lbb_7176:
    ldxdw r1, [r10-0x48]
    ldxdw r1, [r1+0x8]
    ldxdw r2, [r1+0x0]
    sub64 r2, r9
    stxdw [r1+0x0], r2
    ldxdw r1, [r10-0x40]
    ldxdw r1, [r1+0x8]
    ldxdw r2, [r1+0x0]
    add64 r2, r9
    stxdw [r1+0x0], r2
    ldxdw r2, [r10-0x30]
    stxdw [r2+0x88], r9
    ldxdw r1, [r10-0x38]
    ldxb r1, [r1+0x20]
    stxb [r2+0x98], r1
    mov64 r0, 0
    exit

tip_static_:
    ldxdw r1, [r1+0x0]
    ldxdw r2, [r2+0x8]
    ldxdw r5, [r2+0x0]
    sub64 r5, r1
    stxdw [r2+0x0], r5
    ldxdw r2, [r3+0x8]
    ldxdw r3, [r2+0x0]
    add64 r3, r1
    stxdw [r2+0x0], r3
    ldxdw r2, [r4+0x18]
    stxdw [r2+0x90], r1
    mov64 r0, 0
    exit

topup_tipper_:
    mov64 r9, r5
    stxdw [r10-0x88], r3
    mov64 r3, r2
    stxdw [r10-0x90], r1
    mov64 r1, 254
    stxb [r10-0x1], r1
    mov64 r1, 1802396002
    stxw [r10-0x2c], r1
    mov64 r1, r10
    add64 r1, -1
    stxdw [r10-0x18], r1
    mov64 r1, 4
    stxdw [r10-0x20], r1
    mov64 r1, r10
    add64 r1, -44
    stxdw [r10-0x28], r1
    mov64 r5, 1
    stxdw [r10-0x10], r5
    mov64 r1, r10
    add64 r1, -40
    stxdw [r10-0x40], r1
    mov64 r1, 2
    stxdw [r10-0x38], r1
    mov64 r2, 253
    stxb [r10-0x41], r2
    mov64 r2, 29285
    stxh [r10-0x6a], r2
    mov64 r2, 1886415220
    stxw [r10-0x6e], r2
    mov64 r2, r10
    add64 r2, -65
    stxdw [r10-0x58], r2
    mov64 r2, 6
    stxdw [r10-0x60], r2
    mov64 r2, r10
    add64 r2, -110
    stxdw [r10-0x68], r2
    stxdw [r10-0x50], r5
    stxdw [r10-0x78], r1
    mov64 r1, r10
    add64 r1, -104
    stxdw [r10-0x80], r1
    mov64 r1, r10
    add64 r1, -64
    stxdw [r10-0xff8], r1
    stxdw [r10-0xff0], r5
    ldxdw r7, [r9-0xfe8]
    stxdw [r10-0x1000], r7
    ldxdw r2, [r9-0xff8]
    ldxdw r6, [r9-0x1000]
    mov64 r5, r10
    mov64 r1, r3
    mov64 r8, r4
    mov64 r3, r4
    mov64 r4, r6
    call token_transfer_
    jne r0, 0, lbb_7288
    ldxdw r9, [r9-0xfd8]
    mov64 r1, r10
    add64 r1, -128
    stxdw [r10-0x1000], r1
    mov64 r1, 1
    stxdw [r10-0xff8], r1
    mov64 r5, r10
    mov64 r1, r8
    ldxdw r2, [r10-0x88]
    mov64 r3, r2
    mov64 r4, r7
    call token_close_account_
    jne r0, 0, lbb_7288
    mov64 r1, r10
    add64 r1, -64
    stxdw [r10-0x1000], r1
    mov64 r1, 1
    stxdw [r10-0xff8], r1
    mov64 r5, r10
    mov64 r1, r6
    ldxdw r2, [r10-0x88]
    mov64 r3, r9
    ldxdw r4, [r10-0x90]
    call transfer_
    mov64 r0, 0
lbb_7288:
    exit

tip_dynamic:
    ldxdw r5, [r1+0x10]
    ldxdw r4, [r1+0x0]
    mov64 r2, r4
    add64 r2, 112
    mov64 r3, r4
    add64 r3, 168
    add64 r4, 224
    mov64 r1, r5
    call tip_dynamic_
    mov64 r0, 0
    exit

tip_static:
    ldxdw r2, [r1+0x10]
    ldxdw r2, [r2+0x0]
    ldxdw r1, [r1+0x0]
    ldxdw r3, [r1+0x78]
    ldxdw r4, [r3+0x0]
    sub64 r4, r2
    stxdw [r3+0x0], r4
    ldxdw r3, [r1+0x8]
    ldxdw r4, [r3+0x0]
    add64 r4, r2
    stxdw [r3+0x0], r4
    ldxdw r1, [r1+0xc0]
    stxdw [r1+0x90], r2
    mov64 r0, 0
    exit

topup_tipper:
    mov64 r3, r1
    ldxdw r1, [r3+0x10]
    ldxdw r2, [r1+0x8]
    ldxdw r1, [r1+0x0]
    ldxdw r4, [r3+0x0]
    mov64 r3, r4
    add64 r3, 336
    stxdw [r10-0xfe8], r3
    mov64 r3, r4
    add64 r3, 448
    stxdw [r10-0xfd8], r3
    mov64 r3, r4
    add64 r3, 224
    stxdw [r10-0xff8], r3
    mov64 r3, r4
    add64 r3, 168
    stxdw [r10-0x1000], r3
    mov64 r3, r4
    add64 r3, 56
    add64 r4, 112
    mov64 r5, r10
    call topup_tipper_
    exit

auto_swap_in_fast_path:
    mov64 r0, 6000
    lddw r1, 0x400002868
    ldxb r1, [r1+0x0]
    jne r1, 255, lbb_7616
    lddw r1, 0x400002869
    ldxb r1, [r1+0x0]
    jeq r1, 0, lbb_7616
    lddw r1, 0x4000028b8
    ldxdw r1, [r1+0x0]
    jne r1, 0, lbb_7616
    lddw r1, 0x4000050c8
    ldxb r1, [r1+0x0]
    jne r1, 255, lbb_7616
    lddw r1, 0x400005118
    ldxdw r1, [r1+0x0]
    jne r1, 40, lbb_7616
    lddw r1, 0x4000050f0
    ldxdw r1, [r1+0x0]
    lddw r2, 0x47872dc075ca93c2
    jne r1, r2, lbb_7616
    lddw r1, 0x4000050f8
    ldxdw r1, [r1+0x0]
    lddw r2, 0x2ec56c9e7020425
    jne r1, r2, lbb_7616
    lddw r1, 0x400005100
    ldxdw r1, [r1+0x0]
    lddw r2, 0x82930eec82511b93
    jne r1, r2, lbb_7616
    lddw r1, 0x400005108
    ldxdw r1, [r1+0x0]
    lddw r2, 0x9f5bb38b82546b1c
    jne r1, r2, lbb_7616
    lddw r1, 0x400005128
    ldxdw r1, [r1+0x0]
    lddw r2, 0x400002870
    ldxdw r2, [r2+0x0]
    jne r2, r1, lbb_7616
    lddw r1, 0x400005130
    ldxdw r1, [r1+0x0]
    lddw r2, 0x400002878
    ldxdw r2, [r2+0x0]
    jne r2, r1, lbb_7616
    lddw r1, 0x400005138
    ldxdw r1, [r1+0x0]
    lddw r2, 0x400002880
    ldxdw r2, [r2+0x0]
    jne r2, r1, lbb_7616
    lddw r1, 0x400005140
    ldxdw r1, [r1+0x0]
    lddw r2, 0x400002888
    ldxdw r2, [r2+0x0]
    jne r2, r1, lbb_7616
    lddw r1, 0x4000079a8
    lddw r2, 0x8f5c570f55dd7921
    stxdw [r1+0x0], r2
    lddw r1, 0x400047380
    ldxdw r1, [r1+0x0]
    lddw r2, 0x4000079e0
    stxdw [r2+0x0], r1
    mov64 r0, 6001
    lddw r1, 0x4000079a0
    ldxdw r1, [r1+0x0]
    jeq r1, 0, lbb_7616
    lddw r1, 0x400007a44
    ldxb r1, [r1+0x0]
    jne r1, 0, lbb_7616
    mov64 r1, 0
    stxw [r10-0x18], r1
    lddw r1, 0x400035430
    ldxdw r1, [r1+0x0]
    lddw r2, 0x40003a760
    ldxdw r2, [r2+0x0]
    sub64 r2, r1
    stxdw [r10-0x10], r2
    lddw r1, 0x400035438
    ldxdw r1, [r1+0x0]
    lddw r2, 0x40003d068
    ldxdw r2, [r2+0x0]
    sub64 r2, r1
    stxdw [r10-0x8], r2
    lddw r1, 0x400047398
    ldxdw r3, [r1+0x0]
    lddw r1, 0x400047390
    ldxdw r2, [r1+0x0]
    lddw r1, 0x400047388
    ldxdw r1, [r1+0x0]
    lddw r4, 0x4000473a0
    ldxh r4, [r4+0x0]
    lddw r5, 0x4000473a3
    ldxb r5, [r5+0x0]
    lddw r0, 0x4000473a6
    ldxb r0, [r0+0x0]
    stxdw [r10-0xff0], r0
    mov64 r0, r10
    add64 r0, -40
    stxdw [r10-0xfe8], r0
    stxdw [r10-0xff8], r5
    stxdw [r10-0x1000], r4
    mov64 r4, r10
    add64 r4, -24
    mov64 r5, r10
    call calculate_optimal_strategy_optimised
    mov64 r1, r0
    mov64 r0, 6002
    jeq r1, 0, lbb_7616
    lddw r1, 0x400042278
    ldxdw r1, [r1+0x0]
    stxdw [r10-0x30], r1
    lddw r1, 0x40003f970
    ldxdw r9, [r1+0x0]
    lddw r1, 0x4000473a5
    ldxb r1, [r1+0x0]
    stxdw [r10-0x38], r9
    jeq r1, 0, lbb_7494
    ldxdw r1, [r10-0x30]
    stxdw [r10-0x38], r1
lbb_7494:
    lddw r1, 0x400044b30
    ldxdw r8, [r1+0x0]
    lddw r1, 0x4000473a4
    ldxb r1, [r1+0x0]
    jeq r1, 0, lbb_7510
    mov64 r6, r8
    div64 r6, 100
    mul64 r6, 80
    lddw r1, 0xba43b7400
    jgt r1, r6, lbb_7513
    lddw r6, 0xba43b7400
    ja lbb_7513
lbb_7510:
    mov64 r6, r9
    div64 r6, 100
    mul64 r6, 95
lbb_7513:
    ldxdw r7, [r10-0x28]
    syscall sol_log_compute_units_
    mov64 r1, 0
    lddw r2, 0x400000060
lbb_7518:
    mov64 r3, r1
    add64 r3, r2
    lddw r4, 0x100019e20
    add64 r4, r1
    ldxb r4, [r4+0x0]
    stxb [r3+0x0], r4
    add64 r1, 1
    jne r1, 952, lbb_7518
    mov64 r1, 0
    lddw r2, 0x400000418
lbb_7530:
    mov64 r3, r1
    add64 r3, r2
    lddw r4, 0x10001a1d8
    add64 r4, r1
    ldxb r4, [r4+0x0]
    stxb [r3+0x0], r4
    add64 r1, 1
    jne r1, 272, lbb_7530
    syscall sol_log_compute_units_
    lddw r1, 0x4000473a3
    ldxb r1, [r1+0x0]
    mov64 r2, 17
    stxdw [r10-0xfe0], r2
    lddw r2, 0x4000473a2
    stxdw [r10-0xfd8], r2
    lddw r2, 0x400000418
    stxdw [r10-0xfe8], r2
    lddw r2, 0x400000060
    stxdw [r10-0xff0], r2
    lddw r2, 0x40000f678
    stxdw [r10-0xff8], r2
    mov64 r2, r10
    add64 r2, -24
    stxdw [r10-0x1000], r2
    jgt r6, r7, lbb_7562
    mov64 r7, r6
lbb_7562:
    mov64 r5, r10
    mov64 r2, r7
    mov64 r3, r6
    mov64 r4, 0
    call execute_swap_optimised
    jne r0, 0, lbb_7616
    lddw r1, 0x4000473a5
    ldxb r4, [r1+0x0]
    lddw r1, 0x4000473a4
    ldxb r3, [r1+0x0]
    lddw r1, 0x4000473a3
    ldxb r2, [r1+0x0]
    lddw r1, 0x400044b30
    ldxdw r1, [r1+0x0]
    lddw r5, 0x40003f970
    ldxdw r5, [r5+0x0]
    lddw r0, 0x400042278
    ldxdw r0, [r0+0x0]
    stxdw [r10-0xfd0], r0
    stxdw [r10-0xfd8], r5
    stxdw [r10-0xfe0], r1
    ldxdw r1, [r10-0x30]
    stxdw [r10-0xfe8], r1
    stxdw [r10-0xff0], r9
    ldxdw r1, [r10-0x38]
    stxdw [r10-0xff8], r1
    stxdw [r10-0x1000], r8
    lddw r1, 0x400047360
    stxdw [r10-0xfc8], r1
    lddw r1, 0x40000ce40
    stxdw [r10-0xfc0], r1
    mov64 r5, r10
    lddw r1, 0x4000079a8
    call sandwich_update_frontrun
    lddw r1, 0x40000a2a8
    lddw r2, 0x400047360
    lddw r3, 0x400035320
    lddw r4, 0x40000ce40
    call token_data_update_frontrun
    syscall sol_log_compute_units_
    mov64 r0, 0
lbb_7616:
    exit

auto_swap_in_optimised:
    mov64 r0, 0
    exit

create_token_data_:
    mov64 r6, r3
    mov64 r3, r1
    ldxdw r1, [r5-0xff8]
    stxb [r10-0x1], r1
    mov64 r1, 24948
    stxh [r10-0x8], r1
    lddw r1, 0x61645f6e656b6f74
    stxdw [r10-0x10], r1
    mov64 r1, 10
    stxdw [r10-0x38], r1
    mov64 r1, r10
    add64 r1, -16
    stxdw [r10-0x40], r1
    ldxdw r1, [r4+0x0]
    mov64 r4, r10
    add64 r4, -1
    stxdw [r10-0x20], r4
    mov64 r4, 32
    stxdw [r10-0x28], r4
    stxdw [r10-0x30], r1
    mov64 r1, 1
    stxdw [r10-0x18], r1
    mov64 r4, 3
    stxdw [r10-0x48], r4
    mov64 r4, r10
    add64 r4, -64
    stxdw [r10-0x50], r4
    mov64 r4, r10
    add64 r4, -80
    stxdw [r10-0xff8], r4
    stxdw [r10-0xff0], r1
    ldxdw r1, [r5-0x1000]
    stxdw [r10-0x1000], r1
    mov64 r5, r10
    mov64 r1, r2
    mov64 r2, r6
    mov64 r4, 824
    call create_account_
    jne r0, 0, lbb_7663
    ldxdw r1, [r6+0x18]
    lddw r2, 0x9850a6ebf333d68b
    stxdw [r1+0x0], r2
lbb_7663:
    exit

create_token_data:
    ldxdw r2, [r1+0x20]
    ldxdw r6, [r1+0x0]
    ldxdw r1, [r1+0x10]
    ldxb r1, [r1+0x1]
    stxb [r10-0x1], r1
    mov64 r1, 24948
    stxh [r10-0x8], r1
    lddw r1, 0x61645f6e656b6f74
    stxdw [r10-0x10], r1
    mov64 r1, 10
    stxdw [r10-0x38], r1
    mov64 r1, r10
    add64 r1, -16
    stxdw [r10-0x40], r1
    mov64 r1, r10
    add64 r1, -1
    ldxdw r3, [r6+0xa8]
    stxdw [r10-0x20], r1
    mov64 r1, 32
    stxdw [r10-0x28], r1
    stxdw [r10-0x30], r3
    mov64 r1, 1
    stxdw [r10-0x18], r1
    mov64 r3, 3
    stxdw [r10-0x48], r3
    mov64 r3, r10
    add64 r3, -64
    stxdw [r10-0x50], r3
    mov64 r3, r10
    add64 r3, -80
    stxdw [r10-0xff8], r3
    stxdw [r10-0xff0], r1
    stxdw [r10-0x1000], r2
    mov64 r2, r6
    add64 r2, 112
    mov64 r3, r6
    add64 r3, 224
    mov64 r5, r10
    mov64 r1, r6
    mov64 r4, 824
    call create_account_
    jne r0, 0, lbb_7711
    ldxdw r1, [r6+0x88]
    lddw r2, 0x9850a6ebf333d68b
    stxdw [r1+0x0], r2
lbb_7711:
    exit

create_sandwich_tracker:
    ldxdw r6, [r1+0x0]
    ldxdw r3, [r1+0x10]
    lddw r2, 0x72656b636172745f
    stxdw [r10-0x8], r2
    lddw r2, 0x68636977646e6173
    stxdw [r10-0x10], r2
    mov64 r2, r3
    add64 r2, 8
    stxdw [r10-0x20], r2
    mov64 r2, 8
    stxdw [r10-0x28], r2
    mov64 r2, 16
    stxdw [r10-0x38], r2
    mov64 r2, r10
    add64 r2, -16
    stxdw [r10-0x40], r2
    mov64 r2, 1
    stxdw [r10-0x18], r2
    stxdw [r10-0x68], r3
    stxdw [r10-0x30], r3
    mov64 r3, 3
    stxdw [r10-0x48], r3
    mov64 r3, r10
    add64 r3, -64
    stxdw [r10-0x50], r3
    ldxdw r1, [r1+0x20]
    mov64 r3, r10
    add64 r3, -80
    stxdw [r10-0xff8], r3
    stxdw [r10-0xff0], r2
    stxdw [r10-0x1000], r1
    mov64 r7, r6
    add64 r7, 56
    mov64 r8, r6
    add64 r8, 112
    mov64 r5, r10
    mov64 r1, r6
    mov64 r2, r7
    mov64 r3, r8
    mov64 r4, 24
    call create_account_
    mov64 r9, r0
    jne r9, 0, lbb_7784
    ldxdw r1, [r6+0x50]
    lddw r2, 0x8373e4e400d333af
    stxdw [r1+0x0], r2
    ldxdw r2, [r10-0x68]
    ldxdw r2, [r2+0x0]
    stxdw [r1+0x8], r2
    mul64 r2, 432000
    stxdw [r1+0x10], r2
    mov64 r1, 285656
    call calculate_rent
    ldxdw r1, [r6+0x40]
    ldxdw r1, [r1+0x0]
    mov64 r2, 0
    stxdw [r10-0x58], r2
    stxdw [r10-0x60], r2
    mov64 r3, r10
    add64 r3, -96
    stxdw [r10-0x1000], r3
    stxdw [r10-0xff8], r2
    sub64 r0, r1
    mov64 r5, r10
    mov64 r1, r6
    mov64 r2, r7
    mov64 r3, r8
    mov64 r4, r0
    call transfer_
lbb_7784:
    mov64 r0, r9
    exit

extend_sandwich_tracker:
    ldxdw r1, [r1+0x0]
    ldxdw r3, [r1+0x48]
    jsgt r3, 285655, lbb_7798
    mov64 r2, 285656
    sub64 r2, r3
    mov64 r3, 10240
    jsgt r3, r2, lbb_7794
    mov64 r2, 10240
lbb_7794:
    ldxdw r1, [r1+0x50]
    ldxdw r3, [r1-0x8]
    add64 r3, r2
    stxdw [r1-0x8], r3
lbb_7798:
    mov64 r0, 0
    exit

write_sandwich_tracker_identities:
    ldxdw r2, [r1+0x10]
    ldxdw r3, [r2+0x8]
    jeq r3, 0, lbb_7826
    ldxdw r1, [r1+0x0]
    ldxdw r1, [r1+0x50]
    mov64 r3, 0
    mov64 r4, r2
    add64 r4, 16
lbb_7808:
    ldxdw r5, [r2+0x0]
    mov64 r0, r3
    add64 r0, r5
    lsh64 r0, 5
    mov64 r5, r1
    add64 r5, r0
    ldxdw r0, [r4+0x0]
    ldxdw r6, [r4+0x8]
    ldxdw r7, [r4+0x10]
    ldxdw r8, [r4+0x18]
    stxdw [r5+0x30], r8
    stxdw [r5+0x28], r7
    stxdw [r5+0x20], r6
    stxdw [r5+0x18], r0
    add64 r4, 32
    add64 r3, 1
    ldxdw r5, [r2+0x8]
    jgt r5, r3, lbb_7808
lbb_7826:
    mov64 r0, 0
    exit

write_sandwich_tracker_leaders:
    ldxdw r2, [r1+0x10]
    ldxdw r3, [r2+0x8]
    jeq r3, 0, lbb_7845
    ldxdw r1, [r1+0x0]
    ldxdw r4, [r1+0x50]
    ldxdw r1, [r2+0x0]
    lsh64 r1, 1
    add64 r1, r4
    mov64 r4, 0
    add64 r2, 16
    add64 r1, 65560
lbb_7839:
    ldxh r5, [r2+0x0]
    stxh [r1+0x0], r5
    add64 r2, 2
    add64 r1, 2
    add64 r4, 1
    jgt r3, r4, lbb_7839
lbb_7845:
    mov64 r0, 0
    exit

token_initialize_account_3_:
    mov64 r7, r4
    mov64 r9, r3
    mov64 r8, r1
    ldxdw r1, [r2+0x0]
    mov64 r3, 1
    stxh [r10-0x28], r3
    stxdw [r10-0x30], r1
    ldxdw r1, [r9+0x0]
    stxdw [r10-0x20], r1
    mov64 r1, 0
    stxh [r10-0x18], r1
    mov64 r6, r10
    add64 r6, -160
    mov64 r1, r6
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -104
    mov64 r2, r9
    mov64 r3, 56
    call memcpy
    mov64 r1, 18
    stxb [r10-0xc1], r1
    ldxdw r1, [r7+0x0]
    stxdw [r10-0xc0], r1
    ldxdw r1, [r7+0x8]
    stxdw [r10-0xb8], r1
    ldxdw r1, [r7+0x10]
    stxdw [r10-0xb0], r1
    ldxdw r1, [r7+0x18]
    stxdw [r10-0xa8], r1
    ldxdw r1, [r8+0x0]
    mov64 r2, 33
    stxdw [r10-0xd0], r2
    mov64 r2, r10
    add64 r2, -193
    stxdw [r10-0xd8], r2
    mov64 r2, r10
    add64 r2, -48
    stxdw [r10-0xe8], r2
    stxdw [r10-0xf0], r1
    mov64 r1, 2
    stxdw [r10-0xe0], r1
    mov64 r1, 0
    stxdw [r10-0x8], r1
    stxdw [r10-0x10], r1
    mov64 r1, r10
    add64 r1, -240
    mov64 r4, r10
    add64 r4, -16
    mov64 r2, r6
    mov64 r3, 2
    mov64 r5, 0
    syscall sol_invoke_signed_c
    exit

token_initialize_immutable_owner_:
    mov64 r7, r1
    ldxdw r1, [r2+0x0]
    stxdw [r10-0x20], r1
    mov64 r8, 1
    stxh [r10-0x18], r8
    mov64 r6, r10
    add64 r6, -88
    mov64 r1, r6
    mov64 r3, 56
    call memcpy
    mov64 r1, 16
    stxb [r10-0x59], r1
    ldxdw r1, [r7+0x0]
    mov64 r2, r10
    add64 r2, -89
    stxdw [r10-0x70], r2
    mov64 r2, r10
    add64 r2, -32
    stxdw [r10-0x80], r2
    stxdw [r10-0x88], r1
    stxdw [r10-0x68], r8
    stxdw [r10-0x78], r8
    mov64 r1, 0
    stxdw [r10-0x8], r1
    stxdw [r10-0x10], r1
    mov64 r1, r10
    add64 r1, -136
    mov64 r4, r10
    add64 r4, -16
    mov64 r2, r6
    mov64 r3, 1
    mov64 r5, 0
    syscall sol_invoke_signed_c
    exit

token_close_account_:
    mov64 r6, r5
    stxdw [r10-0x110], r4
    mov64 r9, r3
    mov64 r7, r2
    mov64 r2, r1
    ldxdw r1, [r2+0x0]
    stxdw [r10-0x30], r1
    mov64 r3, 1
    stxh [r10-0x28], r3
    ldxdw r1, [r7+0x0]
    stxdw [r10-0x20], r1
    stxh [r10-0x18], r3
    ldxdw r1, [r9+0x0]
    mov64 r3, 256
    stxh [r10-0x8], r3
    stxdw [r10-0x10], r1
    mov64 r8, r10
    add64 r8, -216
    mov64 r1, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -160
    mov64 r2, r7
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -104
    mov64 r2, r9
    mov64 r3, 56
    call memcpy
    mov64 r1, 9
    stxb [r10-0xd9], r1
    ldxdw r1, [r10-0x110]
    ldxdw r1, [r1+0x0]
    mov64 r2, 1
    stxdw [r10-0xe8], r2
    mov64 r2, r10
    add64 r2, -217
    stxdw [r10-0xf0], r2
    mov64 r2, r10
    add64 r2, -48
    stxdw [r10-0x100], r2
    stxdw [r10-0x108], r1
    ldxdw r4, [r6-0x1000]
    ldxdw r5, [r6-0xff8]
    mov64 r1, 3
    stxdw [r10-0xf8], r1
    mov64 r1, r10
    add64 r1, -264
    mov64 r2, r8
    mov64 r3, 3
    syscall sol_invoke_signed_c
    exit

token_sync_native_:
    mov64 r7, r2
    mov64 r2, r1
    ldxdw r1, [r2+0x0]
    stxdw [r10-0x20], r1
    mov64 r8, 1
    stxh [r10-0x18], r8
    mov64 r6, r10
    add64 r6, -88
    mov64 r1, r6
    mov64 r3, 56
    call memcpy
    mov64 r1, 11
    stxb [r10-0x59], r1
    ldxdw r1, [r7+0x0]
    mov64 r2, r10
    add64 r2, -89
    stxdw [r10-0x70], r2
    mov64 r2, r10
    add64 r2, -32
    stxdw [r10-0x80], r2
    stxdw [r10-0x88], r1
    stxdw [r10-0x68], r8
    stxdw [r10-0x78], r8
    mov64 r1, 0
    stxdw [r10-0x8], r1
    stxdw [r10-0x10], r1
    mov64 r1, r10
    add64 r1, -136
    mov64 r4, r10
    add64 r4, -16
    mov64 r2, r6
    mov64 r3, 1
    mov64 r5, 0
    syscall sol_invoke_signed_c
    exit

token_transfer_:
    mov64 r6, r5
    mov64 r9, r4
    mov64 r8, r3
    stxdw [r10-0x118], r1
    ldxdw r1, [r2+0x0]
    stxdw [r10-0x30], r1
    mov64 r1, 1
    stxh [r10-0x28], r1
    ldxdw r3, [r8+0x0]
    stxh [r10-0x18], r1
    stxdw [r10-0x20], r3
    ldxdw r1, [r9+0x0]
    mov64 r3, 256
    stxh [r10-0x8], r3
    stxdw [r10-0x10], r1
    mov64 r7, r10
    add64 r7, -216
    mov64 r1, r7
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -160
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -104
    mov64 r2, r9
    mov64 r3, 56
    call memcpy
    ldxdw r1, [r10-0x118]
    stxdw [r10-0xe0], r1
    mov64 r1, 3
    stxb [r10-0xe1], r1
    ldxdw r2, [r6-0x1000]
    ldxdw r2, [r2+0x0]
    mov64 r3, 9
    stxdw [r10-0xf0], r3
    mov64 r3, r10
    add64 r3, -225
    stxdw [r10-0xf8], r3
    mov64 r3, r10
    add64 r3, -48
    stxdw [r10-0x108], r3
    stxdw [r10-0x110], r2
    ldxdw r4, [r6-0xff8]
    ldxdw r5, [r6-0xff0]
    stxdw [r10-0x100], r1
    mov64 r1, r10
    add64 r1, -272
    mov64 r2, r7
    mov64 r3, 3
    syscall sol_invoke_signed_c
    exit

fast_path_tip_static:
    lddw r1, 0x40005fe80
    ldxdw r1, [r1+0x0]
    lddw r2, 0x400058540
    ldxdw r3, [r2+0x0]
    sub64 r3, r1
    stxdw [r2+0x0], r3
    lddw r2, 0x4000031e8
    ldxdw r3, [r2+0x0]
    add64 r3, r1
    stxdw [r2+0x0], r3
    lddw r2, 0x400008370
    stxdw [r2+0x0], r1
    mov64 r0, 0
    exit

fast_path_create_tip_static:
    mov64 r7, r1
    ldxdw r1, [r7+0x10]
    ldxb r1, [r1+0x0]
    stxb [r10-0x1], r1
    ldxdw r6, [r7+0x0]
    mov64 r1, 104
    stxb [r10-0x8], r1
    lddw r1, 0x7461705f74736166
    stxdw [r10-0x10], r1
    mov64 r1, 256
    stxw [r10-0x14], r1
    mov64 r2, 0
    mov64 r3, 0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    mov64 r1, r10
    add64 r1, -1
    stxdw [r10-0x28], r1
    mov64 r1, 4
    stxdw [r10-0x30], r1
    mov64 r1, r10
    add64 r1, -20
    stxdw [r10-0x38], r1
    mov64 r1, 9
    stxdw [r10-0x40], r1
    mov64 r1, r10
    add64 r1, -16
    stxdw [r10-0x48], r1
    mov64 r1, 1
    stxdw [r10-0x20], r1
    mov64 r2, 3
    stxdw [r10-0x50], r2
    mov64 r2, r10
    add64 r2, -72
    stxdw [r10-0x58], r2
    ldxdw r2, [r7+0x20]
    mov64 r3, r10
    add64 r3, -88
    stxdw [r10-0xff8], r3
    stxdw [r10-0xff0], r1
    stxdw [r10-0x1000], r2
    mov64 r2, r6
    add64 r2, 56
    mov64 r3, r6
    add64 r3, 112
    mov64 r5, r10
    mov64 r1, r6
    mov64 r4, 2360
    call create_account_
    jne r0, 0, lbb_8155
    ldxdw r1, [r6+0x50]
    lddw r2, 0xc6440cf5fdda263f
    stxdw [r1+0x0], r2
    ldxw r2, [r10-0x14]
    stxw [r1+0x930], r2
lbb_8155:
    exit

prepare:
    mov64 r7, r1
    mov64 r0, 0
    ldxdw r6, [r7+0x0]
    ldxdw r1, [r6+0x128]
    jne r1, 0, lbb_8200
    ldxdw r1, [r7+0x10]
    mov64 r9, r6
    add64 r9, 448
    mov64 r8, r6
    add64 r8, 336
    stxdw [r10-0x8], r1
    ldxb r1, [r1+0x0]
    mov64 r2, r6
    add64 r2, 392
    stxdw [r10-0xff8], r2
    stxdw [r10-0xff0], r1
    stxdw [r10-0x1000], r9
    mov64 r2, r6
    add64 r2, 168
    mov64 r3, r6
    add64 r3, 112
    mov64 r5, r10
    mov64 r1, r6
    mov64 r4, r8
    call create_token_account_
    jne r0, 0, lbb_8200
    mov64 r3, r6
    add64 r3, 280
    ldxdw r1, [r10-0x8]
    ldxb r1, [r1+0x1]
    ldxdw r2, [r7+0x20]
    mov64 r7, r3
    stxdw [r10-0x1000], r2
    stxdw [r10-0xff8], r1
    mov64 r5, r10
    mov64 r1, r9
    mov64 r2, r6
    mov64 r4, r8
    call create_token_data_
    jne r0, 0, lbb_8200
    add64 r6, 224
    mov64 r1, r6
    mov64 r2, r7
    call migrate_token_data_
lbb_8200:
    exit

create_sandwich_:
    mov64 r6, r2
    ldxdw r2, [r5-0xff8]
    stxb [r10-0x1], r2
    lddw r2, 0x68636977646e6173
    stxdw [r10-0x10], r2
    mov64 r2, r10
    add64 r2, -1
    stxdw [r10-0x20], r2
    ldxdw r2, [r5-0x1000]
    stxdw [r10-0x30], r2
    mov64 r2, 8
    stxdw [r10-0x28], r2
    stxdw [r10-0x38], r2
    mov64 r2, r10
    add64 r2, -16
    stxdw [r10-0x40], r2
    mov64 r2, 1
    stxdw [r10-0x18], r2
    mov64 r5, 3
    stxdw [r10-0x48], r5
    mov64 r5, r10
    add64 r5, -64
    stxdw [r10-0x50], r5
    mov64 r5, r10
    add64 r5, -80
    stxdw [r10-0xff8], r5
    stxdw [r10-0xff0], r2
    stxdw [r10-0x1000], r4
    mov64 r5, r10
    mov64 r2, r6
    mov64 r4, 160
    call create_account_
    jne r0, 0, lbb_8239
    ldxdw r1, [r6+0x18]
    lddw r2, 0x8f5c570f55dd7921
    stxdw [r1+0x0], r2
lbb_8239:
    exit

create_auth_:
    mov64 r7, r3
    mov64 r6, r2
    ldxdw r2, [r5-0xff8]
    stxb [r10-0x1], r2
    mov64 r2, 1752462689
    stxw [r10-0x8], r2
    mov64 r2, 4
    stxdw [r10-0x30], r2
    mov64 r2, r10
    add64 r2, -8
    stxdw [r10-0x38], r2
    ldxdw r2, [r6+0x0]
    mov64 r3, r10
    add64 r3, -1
    stxdw [r10-0x18], r3
    mov64 r3, 32
    stxdw [r10-0x20], r3
    stxdw [r10-0x28], r2
    mov64 r2, 1
    stxdw [r10-0x10], r2
    mov64 r3, 3
    stxdw [r10-0x40], r3
    mov64 r3, r10
    add64 r3, -56
    stxdw [r10-0x48], r3
    mov64 r3, r10
    add64 r3, -72
    stxdw [r10-0xff8], r3
    stxdw [r10-0xff0], r2
    ldxdw r2, [r5-0x1000]
    stxdw [r10-0x1000], r2
    mov64 r5, r10
    mov64 r2, r7
    mov64 r3, r4
    mov64 r4, 40
    call create_account_
    jne r0, 0, lbb_8290
    ldxdw r1, [r7+0x18]
    lddw r2, 0xbdf49c3c3882102f
    stxdw [r1+0x0], r2
    ldxdw r2, [r6+0x0]
    ldxdw r3, [r2+0x18]
    stxdw [r1+0x20], r3
    ldxdw r3, [r2+0x10]
    stxdw [r1+0x18], r3
    ldxdw r3, [r2+0x8]
    stxdw [r1+0x10], r3
    ldxdw r2, [r2+0x0]
    stxdw [r1+0x8], r2
lbb_8290:
    exit

create_auth:
    ldxdw r2, [r1+0x20]
    ldxdw r6, [r1+0x0]
    ldxdw r1, [r1+0x10]
    ldxb r1, [r1+0x0]
    stxb [r10-0x1], r1
    mov64 r1, 1752462689
    stxw [r10-0x8], r1
    mov64 r1, 4
    stxdw [r10-0x30], r1
    mov64 r1, r10
    add64 r1, -8
    stxdw [r10-0x38], r1
    mov64 r1, r10
    add64 r1, -1
    ldxdw r3, [r6+0x38]
    stxdw [r10-0x18], r1
    mov64 r1, 32
    stxdw [r10-0x20], r1
    stxdw [r10-0x28], r3
    mov64 r1, 1
    stxdw [r10-0x10], r1
    mov64 r3, 3
    stxdw [r10-0x40], r3
    mov64 r3, r10
    add64 r3, -56
    stxdw [r10-0x48], r3
    mov64 r3, r10
    add64 r3, -72
    stxdw [r10-0xff8], r3
    stxdw [r10-0xff0], r1
    stxdw [r10-0x1000], r2
    mov64 r2, r6
    add64 r2, 112
    mov64 r3, r6
    add64 r3, 168
    mov64 r5, r10
    mov64 r1, r6
    mov64 r4, 40
    call create_account_
    jne r0, 0, lbb_8344
    ldxdw r1, [r6+0x88]
    lddw r2, 0xbdf49c3c3882102f
    stxdw [r1+0x0], r2
    ldxdw r2, [r6+0x38]
    ldxdw r3, [r2+0x18]
    stxdw [r1+0x20], r3
    ldxdw r3, [r2+0x10]
    stxdw [r1+0x18], r3
    ldxdw r3, [r2+0x8]
    stxdw [r1+0x10], r3
    ldxdw r2, [r2+0x0]
    stxdw [r1+0x8], r2
lbb_8344:
    exit

auto_swap_out:
    mov64 r7, r1
    syscall sol_log_compute_units_
    ldxdw r1, [r7+0x10]
    ldxh r8, [r1+0x10]
    ldxdw r6, [r1+0x8]
    ldxdw r2, [r1+0x0]
    stxdw [r10-0x6a0], r2
    ldxb r2, [r1+0x12]
    stxb [r10-0x1], r2
    ldxb r2, [r1+0x15]
    stxdw [r10-0x698], r2
    ldxb r1, [r1+0x13]
    stxdw [r10-0x648], r1
    stxdw [r10-0x658], r7
    ldxdw r7, [r7+0x0]
    ldxdw r1, [r7+0x210]
    stxdw [r10-0x650], r1
    ldxdw r1, [r7+0x1d8]
    stxdw [r10-0x690], r1
    ldxdw r1, [r7+0x168]
    stxdw [r10-0x660], r1
    ldxdw r1, [r7+0x1a0]
    stxdw [r10-0x668], r1
    mov64 r1, r10
    add64 r1, -48
    syscall sol_get_clock_sysvar
    jeq r8, 65535, lbb_8378
    ldxdw r2, [r10-0x30]
    ldxdw r1, [r10-0x650]
    mov64 r3, r8
    call sandwich_tracker_is_in_validator_id
    mov64 r9, 6000
    jeq r0, 0, lbb_8732
lbb_8378:
    stxdw [r10-0x6a8], r6
    mov64 r1, r7
    add64 r1, 728
    stxdw [r10-0x670], r1
    mov64 r8, r7
    add64 r8, 672
    mov64 r2, r7
    add64 r2, 616
    stxdw [r10-0x680], r2
    mov64 r6, r7
    add64 r6, 560
    mov64 r2, r7
    add64 r2, 224
    stxdw [r10-0x678], r2
    mov64 r9, r7
    add64 r9, 168
    stxdw [r10-0x688], r7
    add64 r7, 112
    ldxdw r2, [r10-0x30]
    ldxdw r1, [r10-0x650]
    call sandwich_tracker_register
    ldxdw r1, [r10-0x648]
    mov64 r3, r1
    mov64 r2, 0
    mov64 r1, 1
    jne r3, 0, lbb_8405
    mov64 r1, 0
lbb_8405:
    ldxdw r3, [r10-0x658]
    ldxdw r3, [r3+0x0]
    mov64 r4, r10
    add64 r4, -1273
    stxdw [r10-0xfc0], r4
    mov64 r4, r10
    add64 r4, -1304
    stxdw [r10-0xfb8], r4
    mov64 r4, r10
    add64 r4, -1272
    stxdw [r10-0xfc8], r4
    mov64 r4, r10
    add64 r4, -320
    stxdw [r10-0xfd0], r4
    stxdw [r10-0xfd8], r2
    add64 r3, 784
    stxdw [r10-0xfe0], r3
    ldxdw r2, [r10-0x670]
    stxdw [r10-0xfe8], r2
    stxdw [r10-0xff0], r6
    ldxdw r2, [r10-0x680]
    stxdw [r10-0xff8], r2
    stxdw [r10-0x1000], r8
    mov64 r5, r10
    mov64 r2, r9
    ldxdw r3, [r10-0x678]
    mov64 r4, r7
    call deserialize_swap
    lddw r9, 0x400000000
    jeq r0, 0, lbb_8732
    stxdw [r10-0x6b8], r7
    stxdw [r10-0x6b0], r8
    ldxdw r1, [r10-0x648]
    mov64 r8, 0
    mov64 r3, 1
    jne r1, 0, lbb_8443
    mov64 r3, 0
lbb_8443:
    ldxdw r7, [r10-0x660]
    ldxdw r9, [r7+0x58]
    mov64 r1, r10
    add64 r1, -1304
    mov64 r4, r10
    add64 r4, -1328
    mov64 r2, r9
    call get_quote_and_liquidity
    mov64 r6, r0
    ldxdw r5, [r7+0x58]
    ldxdw r4, [r7+0x50]
    mov64 r3, r6
    sub64 r3, r4
    stxdw [r10-0x650], r9
    mov64 r1, r9
    mov64 r2, r6
    stxdw [r10-0x658], r3
    syscall sol_log_64_
    ldxdw r7, [r10-0x688]
lbb_8462:
    lddw r1, 0x100018f8f
    add64 r1, r8
    add64 r8, 1
    ldxb r1, [r1+0x1]
    jne r1, 0, lbb_8462
    lddw r1, 0x100018f8f
    mov64 r2, r8
    syscall sol_log_
    mov64 r9, 6004
    mov64 r1, 1
    ldxdw r8, [r10-0x658]
    jsgt r1, r8, lbb_8732
    ldxdw r1, [r10-0x650]
    mov64 r2, r6
    mov64 r3, r8
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    lddw r1, 0x100018ecc
    mov64 r2, 42
    syscall sol_log_
    ldxdw r9, [r10-0x698]
    mov64 r1, r9
    jeq r1, 0, lbb_8496
    ldxdw r1, [r10-0x660]
    ldxdw r2, [r1+0x50]
    ldxdw r1, [r7+0xf8]
    mov64 r7, r2
    ldxdw r1, [r1+0x40]
    add64 r7, r1
    ja lbb_8501
lbb_8496:
    ldxdw r1, [r10-0x660]
    ldxdw r1, [r1+0x58]
    ldxdw r2, [r7+0xc0]
    ldxdw r7, [r2+0x40]
    sub64 r7, r1
lbb_8501:
    ldxdw r6, [r10-0x668]
    mov64 r0, 0
    jeq r7, 0, lbb_8517
    mov64 r2, r9
    mov64 r3, 1
    mov64 r1, 1
    jne r2, 0, lbb_8509
    mov64 r1, 0
lbb_8509:
    ldxdw r2, [r10-0x648]
    jne r2, 0, lbb_8512
    mov64 r3, 0
lbb_8512:
    xor64 r3, r1
    mov64 r1, r10
    add64 r1, -1328
    mov64 r2, r7
    call get_quote
lbb_8517:
    stxdw [r10-0x678], r0
    mov64 r1, r7
    mov64 r2, 0
    mov64 r3, 0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    lddw r1, 0x100018ef7
    mov64 r2, 15
    syscall sol_log_
    ldxdw r1, [r10-0x650]
    stxdw [r10-0x538], r1
    mov64 r1, r9
    mov64 r9, 1
    jne r1, 0, lbb_8534
    mov64 r9, 0
lbb_8534:
    stxdw [r10-0x670], r7
    add64 r6, 136
    mov64 r1, r6
    mov64 r2, r9
    call kpl_any_initialized
    jne r0, 0, lbb_8542
    ldxdw r6, [r10-0x690]
    add64 r6, 32
lbb_8542:
    mov64 r7, r10
    add64 r7, -1496
    mov64 r1, r7
    mov64 r2, r6
    mov64 r3, 160
    call memcpy
    ldxdw r1, [r10-0x648]
    mov64 r3, 1
    jne r1, 0, lbb_8552
    mov64 r3, 0
lbb_8552:
    ldxdw r2, [r10-0x668]
    ldxdw r1, [r2+0x68]
    stxdw [r10-0xff8], r1
    stxdw [r10-0xff0], r8
    mov64 r8, r10
    add64 r8, -1328
    stxdw [r10-0x1000], r8
    mov64 r6, r10
    add64 r6, -1336
    mov64 r5, r10
    mov64 r1, r7
    mov64 r7, r2
    mov64 r2, r6
    stxdw [r10-0x680], r3
    mov64 r4, r9
    call kpl_update_in_amount
    ldxdw r2, [r10-0x690]
    add64 r2, 8
    ldxdw r1, [r7+0x70]
    jeq r1, 0, lbb_8574
    mov64 r2, r7
    add64 r2, 112
lbb_8574:
    ldxdw r1, [r2+0x10]
    stxdw [r10-0x5e0], r1
    ldxdw r1, [r2+0x8]
    stxdw [r10-0x5e8], r1
    ldxdw r1, [r2+0x0]
    stxdw [r10-0x5f0], r1
    ldxdw r1, [r10-0x678]
    stxdw [r10-0xfe8], r1
    mov64 r1, r10
    add64 r1, -48
    stxdw [r10-0xfe0], r1
    ldxdw r1, [r10-0x670]
    stxdw [r10-0xff0], r1
    stxdw [r10-0xff8], r8
    stxdw [r10-0x1000], r9
    mov64 r1, r10
    add64 r1, -1520
    mov64 r5, r10
    mov64 r2, r6
    mov64 r3, r7
    ldxdw r9, [r10-0x680]
    mov64 r4, r9
    call periodic_sell_off_update_in_amount
    ldxdw r7, [r10-0x688]
    ldxdw r1, [r7+0x78]
    ldxdw r1, [r1+0x0]
    stxdw [r10-0x670], r1
    ldxdw r1, [r7+0xc0]
    ldxdw r8, [r1+0x40]
    ldxdw r1, [r7+0xf8]
    ldxdw r6, [r1+0x40]
    ldxdw r1, [r10-0x538]
    mov64 r2, 0
    mov64 r3, 0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    lddw r1, 0x100019030
    mov64 r2, 10
    syscall sol_log_
    ldxdw r1, [r7+0x2d8]
    ldxdw r2, [r10-0x538]
    ldxb r3, [r10-0x4f9]
    stxdw [r10-0xfe0], r3
    mov64 r3, r10
    add64 r3, -1
    stxdw [r10-0xfd8], r3
    mov64 r3, r10
    add64 r3, -320
    stxdw [r10-0xfe8], r3
    mov64 r3, r10
    add64 r3, -1272
    stxdw [r10-0xff0], r3
    stxdw [r10-0xff8], r1
    mov64 r1, 0
    stxdw [r10-0x1000], r1
    mov64 r5, r10
    mov64 r1, r9
    ldxdw r3, [r10-0x650]
    mov64 r4, 0
    call execute_swap
    mov64 r9, r0
    jne r9, 0, lbb_8732
    stxdw [r10-0x678], r6
    stxdw [r10-0x650], r8
    ldxdw r1, [r10-0x670]
    ldxdw r1, [r10-0x648]
    mov64 r3, 1
    jne r1, 0, lbb_8645
    mov64 r3, 0
lbb_8645:
    ldxdw r1, [r7+0xf8]
    ldxdw r9, [r1+0x40]
    ldxdw r1, [r7+0xc0]
    ldxdw r6, [r1+0x40]
    ldxdw r1, [r7+0x78]
    ldxdw r1, [r1+0x0]
    stxdw [r10-0x648], r1
    ldxdw r1, [r7+0x130]
    ldxdw r8, [r1+0x40]
    ldxdw r2, [r10-0x538]
    mov64 r1, r10
    add64 r1, -1304
    mov64 r7, r10
    add64 r7, -1544
    mov64 r4, r7
    call get_liquidity
    stxdw [r10-0xff0], r8
    ldxdw r1, [r10-0x648]
    stxdw [r10-0xff8], r1
    ldxdw r1, [r10-0x678]
    stxdw [r10-0x1000], r1
    stxdw [r10-0xfe8], r6
    stxdw [r10-0xfe0], r9
    mov64 r5, r10
    ldxdw r8, [r10-0x660]
    mov64 r1, r8
    ldxdw r2, [r10-0x658]
    ldxdw r3, [r10-0x670]
    ldxdw r4, [r10-0x650]
    call sandwich_update_backrun
    stxdw [r10-0x1000], r9
    mov64 r1, r10
    add64 r1, -48
    stxdw [r10-0xff8], r1
    mov64 r5, r10
    ldxdw r1, [r10-0x668]
    mov64 r2, r8
    mov64 r3, r7
    ldxdw r7, [r10-0x688]
    mov64 r4, r6
    call token_data_update_backrun
    ldxdw r1, [r8+0x70]
    ldxdw r6, [r10-0x6a0]
    mov64 r2, r6
    mov64 r3, 0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    lddw r1, 0x100018f07
    mov64 r2, 41
    syscall sol_log_
    mov64 r9, 6005
    ldxdw r1, [r8+0x70]
    ldxdw r4, [r10-0x6a8]
    jsgt r6, r1, lbb_8732
    mov64 r9, 0
    ldxdw r1, [r7+0x8]
    ldxdw r1, [r1+0x0]
    jge r1, r4, lbb_8732
    mov64 r2, 1802396002
    stxw [r10-0x60c], r2
    mov64 r2, r10
    add64 r2, -1
    stxdw [r10-0x620], r2
    mov64 r2, 4
    stxdw [r10-0x628], r2
    mov64 r2, r10
    add64 r2, -1548
    stxdw [r10-0x630], r2
    mov64 r2, 1
    stxdw [r10-0x618], r2
    mov64 r3, 2
    stxdw [r10-0x638], r3
    mov64 r3, r10
    add64 r3, -1584
    stxdw [r10-0x640], r3
    sub64 r4, r1
    mov64 r1, r10
    add64 r1, -1600
    stxdw [r10-0x1000], r1
    stxdw [r10-0xff8], r2
    mov64 r5, r10
    ldxdw r1, [r10-0x6b8]
    mov64 r2, r7
    ldxdw r3, [r10-0x6b0]
    call transfer_
lbb_8732:
    mov64 r0, r9
    exit

associated_token_create_:
    mov64 r6, r5
    stxdw [r10-0x1f0], r4
    stxdw [r10-0x1f8], r3
    mov64 r9, r2
    mov64 r2, r1
    ldxdw r1, [r2+0x0]
    mov64 r5, 257
    stxh [r10-0x58], r5
    stxdw [r10-0x60], r1
    ldxdw r1, [r9+0x0]
    stxdw [r10-0x50], r1
    mov64 r1, 1
    stxh [r10-0x48], r1
    ldxdw r1, [r3+0x0]
    stxdw [r10-0x40], r1
    mov64 r3, 0
    stxh [r10-0x38], r3
    ldxdw r1, [r4+0x0]
    stxdw [r10-0x30], r1
    stxh [r10-0x28], r3
    ldxdw r8, [r6-0x1000]
    ldxdw r1, [r8+0x0]
    stxdw [r10-0x20], r1
    stxh [r10-0x18], r3
    ldxdw r7, [r6-0xff8]
    ldxdw r1, [r7+0x0]
    stxdw [r10-0x10], r1
    stxh [r10-0x8], r3
    mov64 r1, r10
    add64 r1, -432
    stxdw [r10-0x1e8], r1
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -376
    mov64 r2, r9
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -320
    ldxdw r2, [r10-0x1f8]
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -264
    ldxdw r2, [r10-0x1f0]
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -208
    mov64 r2, r8
    mov64 r3, 56
    call memcpy
    mov64 r1, r10
    add64 r1, -152
    mov64 r2, r7
    mov64 r3, 56
    call memcpy
    mov64 r1, 0
    stxb [r10-0x1b1], r1
    ldxdw r1, [r6-0xff0]
    ldxdw r1, [r1+0x0]
    mov64 r2, 1
    stxdw [r10-0x1c0], r2
    mov64 r2, r10
    add64 r2, -433
    stxdw [r10-0x1c8], r2
    mov64 r2, r10
    add64 r2, -96
    stxdw [r10-0x1d8], r2
    stxdw [r10-0x1e0], r1
    ldxdw r4, [r6-0xfe8]
    ldxdw r5, [r6-0xfe0]
    mov64 r1, 6
    stxdw [r10-0x1d0], r1
    mov64 r1, r10
    add64 r1, -480
    ldxdw r2, [r10-0x1e8]
    mov64 r3, 6
    syscall sol_invoke_signed_c
    exit

close_account_:
    ldxdw r3, [r1+0x8]
    ldxdw r4, [r3+0x0]
    mov64 r5, 0
    stxdw [r3+0x0], r5
    ldxdw r2, [r2+0x8]
    ldxdw r3, [r2+0x0]
    add64 r3, r4
    stxdw [r2+0x0], r3
    ldxdw r1, [r1+0x18]
    stxdw [r1-0x8], r5
    mov64 r0, 0
    exit

close_account:
    ldxdw r1, [r1+0x0]
    ldxdw r2, [r1+0x40]
    ldxdw r3, [r2+0x0]
    mov64 r4, 0
    stxdw [r2+0x0], r4
    ldxdw r2, [r1+0x78]
    ldxdw r5, [r2+0x0]
    add64 r5, r3
    stxdw [r2+0x0], r5
    ldxdw r1, [r1+0x50]
    stxdw [r1-0x8], r4
    mov64 r0, 0
    exit

close_sandwich_:
    mov64 r6, r2
    mov64 r7, r1
    ldxdw r1, [r7+0x18]
    stxdw [r10-0x10], r1
    ldxdw r1, [r7+0x10]
    stxdw [r10-0x8], r1
    mov64 r1, r10
    add64 r1, -16
    mov64 r2, 1
    syscall sol_log_data
    mov64 r1, r7
    mov64 r2, r6
    call close_account_
    exit

close_sandwiches_and_topup_tipper:
    mov64 r8, 0
    ldxdw r7, [r1+0x0]
    mov64 r2, r7
    add64 r2, 448
    stxdw [r10-0x28], r2
    mov64 r2, r7
    add64 r2, 392
    stxdw [r10-0x40], r2
    mov64 r2, r7
    add64 r2, 336
    stxdw [r10-0x38], r2
    mov64 r2, r7
    add64 r2, 280
    stxdw [r10-0x48], r2
    mov64 r2, r7
    add64 r2, 224
    stxdw [r10-0x50], r2
    mov64 r2, r7
    add64 r2, 168
    stxdw [r10-0x20], r2
    mov64 r2, r7
    add64 r2, 112
    stxdw [r10-0x30], r2
    mov64 r2, r7
    add64 r2, 56
    stxdw [r10-0x58], r2
    stxdw [r10-0x18], r1
    ldxdw r1, [r1+0x8]
    mov64 r2, 10
    mov64 r9, 0
    jgt r2, r1, lbb_8911
    mov64 r2, 9
    mov64 r3, r7
    add64 r3, 528
    ja lbb_8897
lbb_8889:
    add64 r4, r9
    add64 r5, r8
    mov64 r8, r5
    mov64 r9, r4
lbb_8893:
    add64 r3, 56
    add64 r2, 1
    jgt r1, r2, lbb_8897
    ja lbb_8911
lbb_8897:
    ldxdw r4, [r3-0x8]
    jeq r4, 0, lbb_8893
    ldxdw r4, [r3+0x0]
    ldxdw r5, [r4+0x90]
    ldxdw r0, [r4+0x88]
    add64 r0, r5
    ldxb r6, [r4+0x9a]
    mov64 r4, 0
    mov64 r5, r0
    jeq r6, 0, lbb_8908
    mov64 r5, 0
lbb_8908:
    jeq r6, 0, lbb_8889
    mov64 r4, r0
    ja lbb_8889
lbb_8911:
    lddw r1, 0x100018f31
    mov64 r2, 29
    syscall sol_log_
    mov64 r1, r9
    mov64 r2, r8
    mov64 r3, 0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    ldxdw r1, [r10-0x40]
    stxdw [r10-0xfe0], r1
    ldxdw r1, [r10-0x28]
    stxdw [r10-0xfd8], r1
    ldxdw r1, [r10-0x38]
    stxdw [r10-0xfe8], r1
    ldxdw r1, [r10-0x48]
    stxdw [r10-0xff0], r1
    ldxdw r1, [r10-0x50]
    stxdw [r10-0xff8], r1
    ldxdw r1, [r10-0x20]
    stxdw [r10-0x1000], r1
    mov64 r5, r10
    mov64 r1, r9
    mov64 r2, r8
    ldxdw r3, [r10-0x58]
    ldxdw r4, [r10-0x30]
    call topup_tipper_
    jeq r0, 0, lbb_8941
    ja lbb_8981
lbb_8941:
    mov64 r1, 165
    call calculate_rent
    ldxdw r1, [r7+0x40]
    ldxdw r2, [r1+0x0]
    sub64 r2, r0
    stxdw [r1+0x0], r2
    ldxdw r1, [r7+0x8]
    ldxdw r2, [r1+0x0]
    add64 r2, r0
    stxdw [r1+0x0], r2
    ldxdw r1, [r10-0x18]
    ldxdw r1, [r1+0x8]
    mov64 r2, 10
    mov64 r0, 0
    jgt r2, r1, lbb_8981
    mov64 r8, 9
    mov64 r9, 0
lbb_8958:
    ldxdw r2, [r10-0x18]
    ldxdw r7, [r2+0x0]
    add64 r7, r9
    ldxdw r2, [r7+0x208]
    jeq r2, 0, lbb_8977
    ldxdw r1, [r7+0x210]
    stxdw [r10-0x8], r2
    stxdw [r10-0x10], r1
    mov64 r1, r10
    add64 r1, -16
    mov64 r2, 1
    syscall sol_log_data
    add64 r7, 504
    mov64 r1, r7
    ldxdw r2, [r10-0x20]
    call close_account_
    jne r0, 0, lbb_8981
    ldxdw r1, [r10-0x18]
    ldxdw r1, [r1+0x8]
lbb_8977:
    mov64 r0, 0
    add64 r9, 56
    add64 r8, 1
    jgt r1, r8, lbb_8958
lbb_8981:
    exit

close_sandwich:
    mov64 r0, 6000
    ldxdw r6, [r1+0x0]
    ldxdw r1, [r6+0x48]
    jeq r1, 0, lbb_8998
    ldxdw r2, [r6+0x50]
    stxdw [r10-0x8], r1
    stxdw [r10-0x10], r2
    mov64 r1, r10
    add64 r1, -16
    mov64 r2, 1
    syscall sol_log_data
    mov64 r1, r6
    add64 r1, 56
    add64 r6, 112
    mov64 r2, r6
    call close_account_
lbb_8998:
    exit

fast_path_tip_dynamic:
    lddw r1, 0x4000626f0
    ldxdw r1, [r1+0x0]
    lddw r3, 0x2f5cc235403ddc54
    xor64 r1, r3
    lddw r2, 0x400008350
    ldxdw r2, [r2+0x0]
    lddw r4, 0x400062700
    ldxdw r4, [r4+0x0]
    jne r1, 0, lbb_9018
    lddw r1, 0x4000626e8
    ldxdw r1, [r1+0x0]
    xor64 r1, r3
    mul64 r1, r2
    div64 r1, 10000
lbb_9018:
    xor64 r4, r3
    lddw r5, 0x40005adb8
    ldxdw r5, [r5+0x0]
    jgt r4, r5, lbb_9030
    lddw r1, 0x4000626f8
    ldxdw r1, [r1+0x0]
    xor64 r1, r3
    mul64 r2, r1
    div64 r2, 10000
    mov64 r1, r2
lbb_9030:
    lddw r2, 0x400058540
    ldxdw r3, [r2+0x0]
    sub64 r3, r1
    stxdw [r2+0x0], r3
    lddw r2, 0x40005feb8
    ldxdw r3, [r2+0x0]
    add64 r3, r1
    stxdw [r2+0x0], r3
    lddw r2, 0x400008368
    stxdw [r2+0x0], r1
    lddw r1, 0x400062708
    ldxb r1, [r1+0x0]
    lddw r2, 0x400008378
    stxb [r2+0x0], r1
    mov64 r0, 0
    exit

fast_path_create_tip_dynamic:
    mov64 r7, r1
    ldxdw r1, [r7+0x10]
    ldxb r1, [r1+0x0]
    stxb [r10-0x1], r1
    ldxdw r6, [r7+0x0]
    mov64 r1, 104
    stxb [r10-0x8], r1
    lddw r1, 0x7461705f74736166
    stxdw [r10-0x10], r1
    mov64 r1, 257
    stxw [r10-0x14], r1
    mov64 r2, 0
    mov64 r3, 0
    mov64 r4, 0
    mov64 r5, 0
    syscall sol_log_64_
    mov64 r1, r10
    add64 r1, -1
    stxdw [r10-0x28], r1
    mov64 r1, 4
    stxdw [r10-0x30], r1
    mov64 r1, r10
    add64 r1, -20
    stxdw [r10-0x38], r1
    mov64 r1, 9
    stxdw [r10-0x40], r1
    mov64 r1, r10
    add64 r1, -16
    stxdw [r10-0x48], r1
    mov64 r1, 1
    stxdw [r10-0x20], r1
    mov64 r2, 3
    stxdw [r10-0x50], r2
    mov64 r2, r10
    add64 r2, -72
    stxdw [r10-0x58], r2
    ldxdw r2, [r7+0x20]
    mov64 r3, r10
    add64 r3, -88
    stxdw [r10-0xff8], r3
    stxdw [r10-0xff0], r1
    stxdw [r10-0x1000], r2
    mov64 r2, r6
    add64 r2, 56
    mov64 r3, r6
    add64 r3, 112
    mov64 r5, r10
    mov64 r1, r6
    mov64 r4, 2360
    call create_account_
    jne r0, 0, lbb_9109
    ldxdw r1, [r6+0x50]
    lddw r2, 0xc6440cf5fdda263f
    stxdw [r1+0x0], r2
    ldxw r2, [r10-0x14]
    stxw [r1+0x930], r2
lbb_9109:
    exit

create_token_account_:
    ldxdw r3, [r5-0xff0]
    stxb [r10-0x1], r3
    lddw r3, 0x746e756f6363615f
    stxdw [r10-0xb], r3
    lddw r3, 0x63615f6e656b6f74
    stxdw [r10-0x10], r3
    mov64 r3, 13
    stxdw [r10-0x38], r3
    mov64 r3, r10
    add64 r3, -16
    stxdw [r10-0x40], r3
    ldxdw r3, [r4+0x0]
    mov64 r4, r10
    add64 r4, -1
    stxdw [r10-0x20], r4
    mov64 r4, 32
    stxdw [r10-0x28], r4
    stxdw [r10-0x30], r3
    mov64 r3, 1
    stxdw [r10-0x18], r3
    mov64 r4, 3
    stxdw [r10-0x48], r4
    mov64 r4, r10
    add64 r4, -64
    stxdw [r10-0x50], r4
    ldxdw r4, [r5-0xff8]
    ldxdw r4, [r4+0x0]
    mov64 r0, r10
    add64 r0, -80
    stxdw [r10-0xff8], r0
    stxdw [r10-0xff0], r3
    stxdw [r10-0x1000], r4
    ldxdw r3, [r5-0x1000]
    mov64 r5, r10
    mov64 r4, 165
    call create_account_
    exit

initialize_token_account_3:
    mov64 r0, 0
    ldxdw r6, [r1+0x0]
    ldxdw r1, [r6+0x118]
    ldxdw r1, [r1+0x0]
    ldxdw r2, [r6+0xf8]
    ldxdw r2, [r2+0x0]
    jeq r2, r1, lbb_9173
    mov64 r7, r6
    add64 r7, 280
    ldxdw r1, [r6+0x150]
    syscall sol_log_pubkey
    ldxdw r1, [r6+0xe0]
    syscall sol_log_pubkey
    ldxdw r1, [r6+0x118]
    syscall sol_log_pubkey
    ldxdw r1, [r6+0x138]
    syscall sol_log_pubkey
    mov64 r1, r6
    add64 r1, 336
    ldxdw r4, [r6+0x70]
    add64 r6, 224
    mov64 r2, r6
    mov64 r3, r7
    call token_initialize_account_3_
lbb_9173:
    exit

calculate_rent:
    mul64 r1, 3480
    add64 r1, 445440
    call function_12023
    mov64 r1, r0
    mov64 r2, r0
    call function_10889
    mov64 r1, r0
    call function_9815
    exit

token_data_update_frontrun:
    mov64 r6, r4
    mov64 r8, r3
    mov64 r9, r2
    mov64 r7, r1
    ldxb r1, [r7+0x8]
    jne r1, 0, lbb_9207
    mov64 r1, 1
    stxb [r7+0x8], r1
    mov64 r1, r7
    add64 r1, 296
    lddw r2, 0x10001a328
    mov64 r3, 96
    call memcpy
    ldxdw r1, [r9+0x0]
    stxdw [r7+0x10], r1
    ldxdw r1, [r9+0x8]
    stxdw [r7+0x18], r1
    ldxdw r1, [r9+0x10]
    stxdw [r7+0x20], r1
    ldxdw r1, [r9+0x18]
    stxdw [r7+0x28], r1
    ldxdw r1, [r6+0x0]
    stxdw [r7+0x208], r1
lbb_9207:
    jeq r8, 0, lbb_9216
    ldxdw r1, [r8+0x18]
    stxdw [r7+0x48], r1
    ldxdw r1, [r8+0x10]
    stxdw [r7+0x40], r1
    ldxdw r1, [r8+0x8]
    stxdw [r7+0x38], r1
    ldxdw r1, [r8+0x0]
    stxdw [r7+0x30], r1
lbb_9216:
    ldxdw r1, [r6+0x0]
    stxdw [r7+0x210], r1
    exit

token_data_update_backrun:
    mov64 r8, r3
    mov64 r7, r2
    mov64 r6, r1
    ldxb r2, [r7+0x99]
    mov64 r0, 0
    mov64 r9, 1
    mov64 r1, 1
    jne r2, 0, lbb_9228
    mov64 r1, 0
lbb_9228:
    jeq r2, 0, lbb_9230
    mov64 r9, 0
lbb_9230:
    ldxb r2, [r7+0x9b]
    jeq r2, 0, lbb_9233
    mov64 r9, r1
lbb_9233:
    jeq r2, 0, lbb_9235
    ldxdw r4, [r5-0x1000]
lbb_9235:
    stxdw [r6+0x50], r4
    ldxdw r1, [r5-0xff8]
    stxdw [r10-0x8], r1
    jeq r4, 0, lbb_9244
    mov64 r3, r9
    and64 r3, 1
    mov64 r1, r8
    mov64 r2, r4
    call get_quote
lbb_9244:
    stxdw [r6+0x58], r0
    xor64 r9, -1
    and64 r9, 1
    mov64 r1, r8
    mov64 r2, 1000000000
    mov64 r3, r9
    call get_quote
    stxdw [r6+0x2b0], r0
    ldxdw r1, [r6+0x2a8]
    add64 r1, -1
    jgt r0, r1, lbb_9256
    stxdw [r6+0x2a8], r0
lbb_9256:
    ldxdw r1, [r6+0x2a0]
    add64 r1, -1
    jgt r0, r1, lbb_9260
    stxdw [r6+0x2a0], r0
lbb_9260:
    ldxdw r1, [r7+0x78]
    ldxdw r2, [r6+0x60]
    add64 r2, r1
    ldxdw r1, [r10-0x8]
    ldxdw r8, [r1+0x0]
    ldxdw r1, [r7+0x70]
    stxdw [r6+0x60], r2
    ldxdw r2, [r6+0x68]
    add64 r2, r1
    stxdw [r6+0x68], r2
    mov64 r7, r6
    add64 r7, 296
    mov64 r1, r7
    call pg_get_next_goal
    jeq r0, 0, lbb_9282
lbb_9275:
    ldxdw r1, [r6+0x68]
    ldxdw r2, [r0+0x0]
    jgt r2, r1, lbb_9282
    stxdw [r0+0x8], r8
    mov64 r1, r7
    call pg_get_next_goal
    jne r0, 0, lbb_9275
lbb_9282:
    exit

token_data_update_token_stats:
    mov64 r6, r1
    stxdw [r6+0x50], r2
    mov64 r0, 0
    jeq r2, 0, lbb_9290
    mov64 r1, r3
    mov64 r3, r4
    call get_quote
lbb_9290:
    stxdw [r6+0x58], r0
    exit

token_data_update_price:
    mov64 r6, r1
    xor64 r3, 1
    mov64 r1, r2
    mov64 r2, 1000000000
    call get_quote
    stxdw [r6+0x2b0], r0
    ldxdw r1, [r6+0x2a8]
    add64 r1, -1
    jgt r0, r1, lbb_9302
    stxdw [r6+0x2a8], r0
lbb_9302:
    ldxdw r1, [r6+0x2a0]
    add64 r1, -1
    jgt r0, r1, lbb_9306
    stxdw [r6+0x2a0], r0
lbb_9306:
    exit

token_data_update_profits:
    mov64 r6, r4
    mov64 r7, r1
    ldxdw r1, [r7+0x60]
    add64 r1, r2
    stxdw [r7+0x60], r1
    ldxdw r1, [r7+0x68]
    add64 r1, r3
    stxdw [r7+0x68], r1
    mov64 r8, r7
    add64 r8, 296
    mov64 r1, r8
    call pg_get_next_goal
    jeq r0, 0, lbb_9327
lbb_9320:
    ldxdw r1, [r7+0x68]
    ldxdw r2, [r0+0x0]
    jgt r2, r1, lbb_9327
    stxdw [r0+0x8], r6
    mov64 r1, r8
    call pg_get_next_goal
    jne r0, 0, lbb_9320
lbb_9327:
    exit

get_should_do_periodic_sell:
    mov64 r0, 0
    ldxdw r6, [r1+0x8]
    jge r6, r5, lbb_9344
    mov64 r6, r2
    sub64 r6, r3
    jgt r2, r3, lbb_9336
    sub64 r3, r2
    mov64 r6, r3
lbb_9336:
    ldxdw r2, [r1+0x0]
    jge r2, r6, lbb_9344
    ldxh r1, [r1+0x12]
    div64 r5, 10000
    mul64 r5, r1
    mov64 r0, 1
    jgt r4, r5, lbb_9344
    mov64 r0, 0
lbb_9344:
    and64 r0, 1
    exit

periodic_sell_off_update_in_amount:
    mov64 r8, r4
    mov64 r6, r3
    mov64 r9, r1
    mov64 r0, 0
    ldxdw r1, [r6+0x68]
    ldxdw r3, [r9+0x8]
    jge r3, r1, lbb_9407
    ldxdw r3, [r5-0xfe0]
    ldxdw r7, [r3+0x0]
    ldxdw r3, [r6+0x218]
    mov64 r4, r7
    sub64 r4, r3
    jgt r7, r3, lbb_9361
    sub64 r3, r7
    mov64 r4, r3
lbb_9361:
    ldxdw r3, [r9+0x0]
    jge r3, r4, lbb_9407
    ldxdw r4, [r5-0xfe8]
    ldxh r3, [r9+0x12]
    div64 r1, 10000
    mul64 r1, r3
    jge r1, r4, lbb_9407
    ldxdw r3, [r5-0xff0]
    jeq r3, 0, lbb_9407
    ldxb r1, [r6+0x188]
    jne r1, 0, lbb_9407
    stxdw [r10-0x18], r2
    ldxdw r1, [r5-0xff8]
    stxdw [r10-0x20], r1
    ldxdw r1, [r5-0x1000]
    stxdw [r10-0x8], r1
    lddw r1, 0x100018e71
    mov64 r2, 21
    stxdw [r10-0x10], r3
    syscall sol_log_
    ldxh r2, [r9+0x10]
    ldxdw r1, [r10-0x10]
    mul64 r2, r1
    div64 r2, 10000
    ldxdw r1, [r10-0x8]
    jeq r1, 0, lbb_9399
    xor64 r8, 1
    ldxdw r1, [r10-0x20]
    mov64 r3, r8
    call get_quote
    ldxdw r3, [r10-0x18]
    ldxdw r1, [r3+0x0]
    jge r1, r0, lbb_9403
    mov64 r2, 10000
    jgt r2, r1, lbb_9404
    mov64 r1, 10000
    ja lbb_9404
lbb_9399:
    ldxdw r3, [r10-0x18]
    ldxdw r1, [r3+0x0]
    add64 r1, r2
    ja lbb_9404
lbb_9403:
    sub64 r1, r0
lbb_9404:
    stxdw [r3+0x0], r1
    stxdw [r6+0x218], r7
    mov64 r0, 1
lbb_9407:
    exit

sandwich_tracker_get_validator_id:
    mov64 r0, 0
    ldxdw r3, [r1+0x10]
    jgt r3, r2, lbb_9424
    mov64 r4, r3
    add64 r4, 432000
    jge r2, r4, lbb_9424
    sub64 r2, r3
    rsh64 r2, 1
    lddw r3, 0x7ffffffffffffffe
    and64 r2, r3
    add64 r1, r2
    add64 r1, 65560
    ldxh r2, [r1+0x0]
    jgt r2, 2047, lbb_9424
    mov64 r0, r1
lbb_9424:
    exit

sandwich_tracker_get_identity:
    mov64 r0, 0
    jgt r2, 2047, lbb_9431
    lsh64 r2, 5
    add64 r1, r2
    add64 r1, 24
    mov64 r0, r1
lbb_9431:
    exit

sandwich_tracker_is_in_validator_id:
    mov64 r0, 0
    ldxdw r4, [r1+0x10]
    mov64 r6, r4
    add64 r6, 432000
    mov64 r7, r2
    add64 r7, -4
    mov64 r5, 0
    jgt r4, r7, lbb_9453
    jge r7, r6, lbb_9453
    sub64 r7, r4
    rsh64 r7, 1
    lddw r5, 0x7ffffffffffffffe
    and64 r7, r5
    mov64 r8, r1
    add64 r8, r7
    add64 r8, 65560
    ldxh r7, [r8+0x0]
    mov64 r5, 0
    jgt r7, 2047, lbb_9453
    mov64 r5, r8
lbb_9453:
    jgt r4, r2, lbb_9471
    jge r2, r6, lbb_9471
    jeq r5, 0, lbb_9471
    sub64 r2, r4
    rsh64 r2, 1
    lddw r4, 0x7ffffffffffffffe
    and64 r2, r4
    add64 r1, r2
    add64 r1, 65560
    ldxh r1, [r1+0x0]
    mov64 r2, r1
    jgt r2, 2047, lbb_9471
    mov64 r0, 1
    ldxh r2, [r5+0x0]
    jeq r2, r3, lbb_9471
    jeq r1, r3, lbb_9471
    mov64 r0, 0
lbb_9471:
    and64 r0, 1
    exit

sandwich_tracker_register:
    mov64 r3, 0
    ldxdw r4, [r1+0x10]
    mov64 r5, r4
    add64 r5, 432000
    mov64 r0, r2
    add64 r0, -4
    jgt r4, r0, lbb_9493
    jge r0, r5, lbb_9493
    sub64 r0, r4
    rsh64 r0, 1
    lddw r3, 0x7ffffffffffffffe
    and64 r0, r3
    mov64 r6, r1
    add64 r6, r0
    add64 r6, 65560
    ldxh r0, [r6+0x0]
    mov64 r3, 0
    jgt r0, 2047, lbb_9493
    mov64 r3, r6
lbb_9493:
    jgt r4, r2, lbb_9520
    jge r2, r5, lbb_9520
    jeq r3, 0, lbb_9520
    sub64 r2, r4
    rsh64 r2, 1
    lddw r4, 0x7ffffffffffffffe
    and64 r2, r4
    mov64 r4, r1
    add64 r4, r2
    add64 r4, 65560
    ldxh r2, [r4+0x0]
    jgt r2, 2047, lbb_9520
    add64 r1, 281560
    ldxh r2, [r3+0x0]
    lsh64 r2, 1
    mov64 r3, r1
    add64 r3, r2
    ldxh r2, [r3+0x0]
    add64 r2, 1
    stxh [r3+0x0], r2
    ldxh r2, [r4+0x0]
    lsh64 r2, 1
    add64 r1, r2
    ldxh r2, [r1+0x0]
    add64 r2, 1
    stxh [r1+0x0], r2
lbb_9520:
    exit

kpl_any_initialized:
    mov64 r3, r1
    add64 r3, 80
    jne r2, 0, lbb_9525
    mov64 r3, r1
lbb_9525:
    mov64 r0, 1
    ldxb r1, [r3+0xa]
    jne r1, 0, lbb_9537
    ldxb r1, [r3+0x1a]
    jne r1, 0, lbb_9537
    ldxb r1, [r3+0x2a]
    jne r1, 0, lbb_9537
    ldxb r1, [r3+0x3a]
    jne r1, 0, lbb_9537
    ldxb r1, [r3+0x4a]
    jne r1, 0, lbb_9537
    mov64 r0, 0
lbb_9537:
    exit

kpl_get_keep_profit_bps:
    mov64 r4, r1
    add64 r4, 80
    jne r2, 0, lbb_9542
    mov64 r4, r1
lbb_9542:
    mov64 r0, 0
    mov64 r6, 80
    jne r2, 0, lbb_9546
    mov64 r6, 0
lbb_9546:
    ldxb r7, [r4+0xa]
    mov64 r5, 0
    jeq r7, 0, lbb_9554
    add64 r1, r6
    ldxdw r1, [r1+0x0]
    jgt r1, r3, lbb_9554
    ldxh r5, [r4+0x8]
    mov64 r0, r1
lbb_9554:
    ldxb r1, [r4+0x1a]
    jeq r1, 0, lbb_9561
    ldxdw r1, [r4+0x10]
    jgt r1, r3, lbb_9561
    jgt r0, r1, lbb_9561
    ldxh r5, [r4+0x18]
    mov64 r0, r1
lbb_9561:
    ldxb r1, [r4+0x2a]
    jeq r1, 0, lbb_9568
    ldxdw r1, [r4+0x20]
    jgt r1, r3, lbb_9568
    jgt r0, r1, lbb_9568
    ldxh r5, [r4+0x28]
    mov64 r0, r1
lbb_9568:
    ldxb r1, [r4+0x3a]
    jeq r1, 0, lbb_9575
    ldxdw r1, [r4+0x30]
    jgt r1, r3, lbb_9575
    jgt r0, r1, lbb_9575
    ldxh r5, [r4+0x38]
    mov64 r0, r1
lbb_9575:
    ldxb r1, [r4+0x4a]
    jeq r1, 0, lbb_9581
    ldxdw r1, [r4+0x40]
    jgt r1, r3, lbb_9581
    jgt r0, r1, lbb_9581
    ldxh r5, [r4+0x48]
lbb_9581:
    mov64 r0, 10000
    sub64 r0, r5
    jne r2, 0, lbb_9585
    mov64 r0, r5
lbb_9585:
    and64 r0, 65535
    exit

kpl_update_in_amount:
    stxdw [r10-0x8], r2
    mov64 r2, r1
    add64 r2, 80
    jne r4, 0, lbb_9592
    mov64 r2, r1
lbb_9592:
    mov64 r0, 0
    mov64 r9, 80
    jne r4, 0, lbb_9596
    mov64 r9, 0
lbb_9596:
    ldxdw r8, [r5-0xff8]
    ldxb r6, [r2+0xa]
    mov64 r7, 0
    jeq r6, 0, lbb_9605
    add64 r1, r9
    ldxdw r1, [r1+0x0]
    jgt r1, r8, lbb_9605
    ldxh r7, [r2+0x8]
    mov64 r0, r1
lbb_9605:
    ldxb r1, [r2+0x1a]
    jeq r1, 0, lbb_9612
    ldxdw r1, [r2+0x10]
    jgt r1, r8, lbb_9612
    jgt r0, r1, lbb_9612
    ldxh r7, [r2+0x18]
    mov64 r0, r1
lbb_9612:
    ldxb r1, [r2+0x2a]
    jeq r1, 0, lbb_9619
    ldxdw r1, [r2+0x20]
    jgt r1, r8, lbb_9619
    jgt r0, r1, lbb_9619
    ldxh r7, [r2+0x28]
    mov64 r0, r1
lbb_9619:
    ldxb r1, [r2+0x3a]
    jeq r1, 0, lbb_9626
    ldxdw r1, [r2+0x30]
    jgt r1, r8, lbb_9626
    jgt r0, r1, lbb_9626
    ldxh r7, [r2+0x38]
    mov64 r0, r1
lbb_9626:
    ldxb r1, [r2+0x4a]
    jeq r1, 0, lbb_9632
    ldxdw r1, [r2+0x40]
    jgt r1, r8, lbb_9632
    jgt r0, r1, lbb_9632
    ldxh r7, [r2+0x48]
lbb_9632:
    mov64 r2, 10000
    sub64 r2, r7
    jne r4, 0, lbb_9636
    mov64 r2, r7
lbb_9636:
    and64 r2, 65535
    jeq r2, 0, lbb_9648
    ldxdw r4, [r5-0xff0]
    ldxdw r1, [r5-0x1000]
    mul64 r2, r4
    div64 r2, 10000
    xor64 r3, 1
    call get_quote
    ldxdw r2, [r10-0x8]
    ldxdw r1, [r2+0x0]
    sub64 r1, r0
    stxdw [r2+0x0], r1
lbb_9648:
    exit

sandwich_update_frontrun:
    ldxdw r6, [r5-0x1000]
    mov64 r0, r6
    jne r3, 0, lbb_9653
    ldxdw r0, [r5-0xff8]
lbb_9653:
    stxdw [r1+0x40], r0
    ldxdw r0, [r5-0xfe8]
    ldxdw r7, [r5-0xff0]
    mov64 r8, r7
    jne r4, 0, lbb_9659
    mov64 r8, r0
lbb_9659:
    stxdw [r1+0x48], r8
    ldxdw r8, [r5-0xfe0]
    sub64 r6, r8
    jne r3, 0, lbb_9666
    ldxdw r6, [r5-0xfd8]
    sub64 r7, r6
    mov64 r6, r7
lbb_9666:
    stxdw [r1+0x50], r6
    ldxdw r6, [r5-0xfc8]
    ldxdw r7, [r6+0x0]
    stxdw [r1+0x8], r7
    ldxdw r7, [r6+0x8]
    stxdw [r1+0x10], r7
    ldxdw r7, [r6+0x10]
    stxdw [r1+0x18], r7
    ldxdw r6, [r6+0x18]
    stxdw [r1+0x20], r6
    ldxdw r6, [r5-0xfc0]
    ldxdw r7, [r6+0x0]
    stxdw [r1+0x28], r7
    ldxdw r5, [r5-0xfd0]
    sub64 r5, r0
    ldxdw r0, [r6+0x20]
    stxb [r1+0x9b], r4
    stxb [r1+0x9a], r3
    stxb [r1+0x99], r2
    stxdw [r1+0x58], r5
    stxdw [r1+0x30], r0
    exit

sandwich_update_backrun:
    stxdw [r10-0x8], r2
    ldxdw r7, [r5-0xff8]
    ldxdw r0, [r5-0xff0]
    ldxb r9, [r1+0x9a]
    jeq r9, 0, lbb_9694
    mov64 r0, r7
lbb_9694:
    ldxdw r6, [r5-0x1000]
    ldxdw r2, [r5-0xfe0]
    mov64 r8, r2
    sub64 r8, r6
    jeq r9, 0, lbb_9701
    sub64 r7, r3
    mov64 r8, r7
lbb_9701:
    mov64 r3, 1
    stxb [r1+0x9c], r3
    ldxdw r3, [r5-0xfe8]
    sub64 r4, r3
    stxdw [r1+0x60], r4
    stxdw [r1+0x68], r8
    ldxb r4, [r1+0x9b]
    jeq r4, 0, lbb_9710
    mov64 r3, r2
lbb_9710:
    ldxdw r2, [r1+0x40]
    sub64 r0, r2
    ldxdw r5, [r10-0x8]
    jeq r4, 0, lbb_9715
    mov64 r5, r0
lbb_9715:
    stxdw [r1+0x78], r0
    ldxdw r2, [r1+0x48]
    sub64 r3, r2
    stxdw [r1+0x80], r3
    stxdw [r1+0x70], r5
    exit

authenticate:
    mov64 r0, 0
    ldxdw r4, [r1+0x8]
    jeq r4, 0, lbb_9785
    ldxdw r3, [r1+0x0]
    ldxb r1, [r3+0x30]
    jeq r1, 0, lbb_9785
    ldxdw r1, [r3+0x0]
    ldxdw r2, [r1+0x0]
    lddw r5, 0xcae6fffe141ab90d
    jne r2, r5, lbb_9745
    ldxdw r5, [r1+0x8]
    lddw r0, 0x7dee05144cd9b763
    jne r5, r0, lbb_9745
    ldxdw r5, [r1+0x10]
    lddw r0, 0x1d84ef14c41ea393
    jne r5, r0, lbb_9745
    mov64 r0, 1
    ldxdw r5, [r1+0x18]
    lddw r6, 0x66014bc2470b2dd8
    jeq r5, r6, lbb_9785
lbb_9745:
    mov64 r5, 2
    mov64 r0, 0
    jgt r5, r4, lbb_9785
    ldxdw r4, [r3+0x48]
    jne r4, 40, lbb_9785
    ldxdw r4, [r3+0x58]
    ldxdw r5, [r4+0x0]
    lddw r6, 0x47872dc075ca93c2
    jne r5, r6, lbb_9785
    ldxdw r5, [r4+0x8]
    lddw r6, 0x2ec56c9e7020425
    jne r5, r6, lbb_9785
    ldxdw r5, [r4+0x10]
    lddw r6, 0x82930eec82511b93
    jne r5, r6, lbb_9785
    ldxdw r4, [r4+0x18]
    lddw r5, 0x9f5bb38b82546b1c
    jne r4, r5, lbb_9785
    ldxdw r3, [r3+0x50]
    ldxdw r4, [r3+0x0]
    lddw r5, 0xbdf49c3c3882102f
    jne r4, r5, lbb_9785
    ldxdw r4, [r3+0x8]
    jne r2, r4, lbb_9785
    ldxdw r2, [r3+0x10]
    ldxdw r4, [r1+0x8]
    jne r4, r2, lbb_9785
    ldxdw r2, [r3+0x18]
    ldxdw r4, [r1+0x10]
    jne r4, r2, lbb_9785
    ldxdw r2, [r3+0x20]
    ldxdw r1, [r1+0x18]
    mov64 r0, 1
    jeq r1, r2, lbb_9785
    mov64 r0, 0
lbb_9785:
    and64 r0, 1
    exit

pg_is_reached:
    ldxdw r1, [r1+0x8]
    mov64 r0, 1
    jne r1, 0, lbb_9791
    mov64 r0, 0
lbb_9791:
    exit

pg_get_next_goal:
    mov64 r2, 0
    ldxdw r3, [r1+0x8]
    jeq r3, 0, lbb_9811
    mov64 r2, 1
    ldxdw r3, [r1+0x18]
    jeq r3, 0, lbb_9811
    mov64 r2, 2
    ldxdw r3, [r1+0x28]
    jeq r3, 0, lbb_9811
    mov64 r2, 3
    ldxdw r3, [r1+0x38]
    jeq r3, 0, lbb_9811
    mov64 r2, 4
    ldxdw r3, [r1+0x48]
    jeq r3, 0, lbb_9811
    mov64 r0, 0
    mov64 r2, 5
    ldxdw r3, [r1+0x58]
    jne r3, 0, lbb_9814
lbb_9811:
    lsh64 r2, 4
    add64 r1, r2
    mov64 r0, r1
lbb_9814:
    exit

function_9815:
    mov64 r0, 0
    lddw r2, 0x3ff0000000000000
    jgt r2, r1, lbb_9838
    lddw r2, 0x43f0000000000000
    jgt r2, r1, lbb_9828
    mov64 r0, -1
    lddw r2, 0x7ff0000000000001
    jgt r2, r1, lbb_9838
    mov64 r0, 0
    ja lbb_9838
lbb_9828:
    mov64 r0, r1
    lsh64 r0, 11
    lddw r2, 0x8000000000000000
    or64 r0, r2
    rsh64 r1, 52
    mov64 r2, 62
    sub64 r2, r1
    and64 r2, 63
    rsh64 r0, r2
lbb_9838:
    exit

function_9839:
    stxdw [r10-0x10], r3
    stxdw [r10-0x8], r1
    mov64 r1, r2
    lsh64 r1, 32
    rsh64 r1, 32
    mov64 r7, r4
    lsh64 r7, 32
    rsh64 r7, 32
    mov64 r6, r2
    rsh64 r6, 32
    mov64 r3, r7
    mul64 r3, r1
    mul64 r7, r6
    mov64 r0, r4
    rsh64 r0, 32
    mov64 r9, r0
    mul64 r9, r1
    mov64 r1, r9
    add64 r1, r7
    mov64 r8, 1
    jgt r9, r1, lbb_9861
    mov64 r8, 0
lbb_9861:
    mov64 r9, r1
    lsh64 r9, 32
    mov64 r7, r3
    add64 r7, r9
    mov64 r9, 1
    jgt r3, r7, lbb_9868
    mov64 r9, 0
lbb_9868:
    ldxdw r3, [r10-0x8]
    stxdw [r3+0x0], r7
    rsh64 r1, 32
    lsh64 r8, 32
    or64 r8, r1
    ldxdw r1, [r10-0x10]
    mul64 r4, r1
    mul64 r5, r2
    mul64 r0, r6
    add64 r0, r8
    add64 r5, r4
    add64 r0, r9
    add64 r0, r5
    stxdw [r3+0x8], r0
    exit

function_9883:
    mov64 r6, r1
    mov64 r1, r10
    add64 r1, -32
    call function_9892
    ldxdw r1, [r10-0x20]
    ldxdw r2, [r10-0x18]
    stxdw [r6+0x8], r2
    stxdw [r6+0x0], r1
    exit

function_9892:
    stxdw [r10-0xc0], r4
    mov64 r7, r2
    mov64 r0, r1
    mov64 r1, r5
    rsh64 r1, 1
    stxdw [r10-0xb8], r5
    mov64 r6, r5
    mov64 r5, r3
    or64 r6, r1
    mov64 r1, r6
    rsh64 r1, 2
    or64 r6, r1
    mov64 r1, r6
    rsh64 r1, 4
    or64 r6, r1
    mov64 r1, r6
    rsh64 r1, 8
    or64 r6, r1
    mov64 r1, r6
    rsh64 r1, 16
    or64 r6, r1
    mov64 r1, r5
    rsh64 r1, 1
    mov64 r4, r5
    or64 r4, r1
    mov64 r1, r4
    rsh64 r1, 2
    or64 r4, r1
    mov64 r1, r6
    rsh64 r1, 32
    or64 r6, r1
    mov64 r1, r4
    rsh64 r1, 4
    or64 r4, r1
    mov64 r1, r4
    rsh64 r1, 8
    or64 r4, r1
    lddw r1, 0x5555555555555555
    xor64 r6, -1
    mov64 r2, r6
    rsh64 r2, 1
    and64 r2, r1
    sub64 r6, r2
    mov64 r2, r4
    rsh64 r2, 16
    or64 r4, r2
    mov64 r2, r4
    rsh64 r2, 32
    or64 r4, r2
    lddw r2, 0x3333333333333333
    mov64 r8, r6
    and64 r8, r2
    rsh64 r6, 2
    and64 r6, r2
    add64 r8, r6
    xor64 r4, -1
    mov64 r3, r4
    rsh64 r3, 1
    and64 r3, r1
    sub64 r4, r3
    mov64 r3, r8
    rsh64 r3, 4
    add64 r8, r3
    mov64 r9, r4
    and64 r9, r2
    rsh64 r4, 2
    and64 r4, r2
    add64 r9, r4
    mov64 r3, r9
    rsh64 r3, 4
    add64 r9, r3
    lddw r3, 0xf0f0f0f0f0f0f0f
    and64 r9, r3
    and64 r8, r3
    lddw r4, 0x101010101010101
    mul64 r8, r4
    mul64 r9, r4
    rsh64 r9, 56
    jne r5, 0, lbb_10017
    mov64 r6, r0
    mov64 r0, r7
    rsh64 r0, 1
    mov64 r1, r5
    mov64 r5, r7
    or64 r5, r0
    mov64 r0, r5
    rsh64 r0, 2
    or64 r5, r0
    mov64 r0, r5
    rsh64 r0, 4
    or64 r5, r0
    mov64 r0, r5
    rsh64 r0, 8
    or64 r5, r0
    mov64 r0, r5
    rsh64 r0, 16
    or64 r5, r0
    mov64 r0, r5
    rsh64 r0, 32
    or64 r5, r0
    xor64 r5, -1
    mov64 r0, r5
    rsh64 r0, 1
    lddw r9, 0x5555555555555555
    and64 r0, r9
    sub64 r5, r0
    mov64 r0, r6
    mov64 r9, r5
    and64 r9, r2
    rsh64 r5, 2
    and64 r5, r2
    add64 r9, r5
    mov64 r5, r9
    rsh64 r5, 4
    add64 r9, r5
    mov64 r5, r1
    and64 r9, r3
    mul64 r9, r4
    rsh64 r9, 56
    add64 r9, 64
lbb_10017:
    rsh64 r8, 56
    ldxdw r1, [r10-0xb8]
    jne r1, 0, lbb_10060
    mov64 r1, r5
    ldxdw r8, [r10-0xc0]
    mov64 r5, r8
    rsh64 r5, 1
    or64 r8, r5
    mov64 r5, r8
    rsh64 r5, 2
    or64 r8, r5
    mov64 r5, r8
    rsh64 r5, 4
    or64 r8, r5
    mov64 r5, r8
    rsh64 r5, 8
    or64 r8, r5
    mov64 r5, r8
    rsh64 r5, 16
    or64 r8, r5
    mov64 r5, r8
    rsh64 r5, 32
    or64 r8, r5
    xor64 r8, -1
    mov64 r5, r8
    rsh64 r5, 1
    lddw r6, 0x5555555555555555
    and64 r5, r6
    sub64 r8, r5
    mov64 r5, r1
    mov64 r1, r8
    rsh64 r1, 2
    and64 r8, r2
    and64 r1, r2
    add64 r8, r1
    mov64 r1, r8
    rsh64 r1, 4
    add64 r8, r1
    and64 r8, r3
    mul64 r8, r4
    rsh64 r8, 56
    add64 r8, 64
lbb_10060:
    jge r9, r8, lbb_10076
    mov64 r1, r9
    lsh64 r1, 32
    rsh64 r1, 32
    jgt r1, 63, lbb_10066
    ja lbb_10099
lbb_10066:
    mov64 r1, r7
    ldxdw r2, [r10-0xc0]
    div64 r1, r2
    jeq r2, 0, lbb_10071
    mov64 r6, r1
lbb_10071:
    mul64 r1, r2
    sub64 r7, r1
    mov64 r8, 0
    mov64 r1, 0
    ja lbb_10539
lbb_10076:
    mov64 r6, 0
    mov64 r1, 1
    mov64 r2, 1
    ldxdw r3, [r10-0xc0]
    jgt r3, r7, lbb_10082
    mov64 r2, 0
lbb_10082:
    ldxdw r4, [r10-0xb8]
    jgt r4, r5, lbb_10085
    mov64 r1, 0
lbb_10085:
    mov64 r8, r5
    jeq r5, r4, lbb_10088
    mov64 r2, r1
lbb_10088:
    and64 r2, 1
    mov64 r1, 0
    jne r2, 0, lbb_10539
    sub64 r8, r4
    mov64 r6, 1
    mov64 r2, 1
    jgt r3, r7, lbb_10096
    mov64 r2, 0
lbb_10096:
    sub64 r8, r2
    sub64 r7, r3
    ja lbb_10539
lbb_10099:
    mov64 r1, r8
    lsh64 r1, 32
    rsh64 r1, 32
    jgt r1, 95, lbb_10104
    ja lbb_10140
lbb_10104:
    ldxdw r4, [r10-0xc0]
    lsh64 r4, 32
    rsh64 r4, 32
    jeq r4, 0, lbb_10129
    mov64 r2, r5
    div64 r2, r4
    mov64 r1, r2
    mul64 r1, r4
    sub64 r5, r1
    lsh64 r5, 32
    mov64 r1, r7
    rsh64 r1, 32
    or64 r5, r1
    mov64 r6, r5
    div64 r6, r4
    mov64 r1, r6
    mul64 r1, r4
    sub64 r5, r1
    lsh64 r7, 32
    rsh64 r7, 32
    lsh64 r5, 32
    or64 r5, r7
    mov64 r3, r5
    div64 r3, r4
    mov64 r7, r5
lbb_10129:
    mov64 r1, r6
    rsh64 r1, 32
    or64 r1, r2
    lsh64 r6, 32
    or64 r6, r3
    mov64 r2, r7
    div64 r2, r4
    mul64 r2, r4
    sub64 r7, r2
    mov64 r8, 0
    ja lbb_10539
lbb_10140:
    stxdw [r10-0xc8], r5
    stxdw [r10-0x100], r0
    mov64 r1, r8
    sub64 r1, r9
    lsh64 r1, 32
    rsh64 r1, 32
    mov64 r2, 32
    jgt r2, r1, lbb_10149
    ja lbb_10217
lbb_10149:
    stxdw [r10-0xd0], r7
    mov64 r7, 64
    sub64 r7, r9
    and64 r7, 127
    mov64 r1, r10
    add64 r1, -128
    ldxdw r9, [r10-0xc0]
    mov64 r2, r9
    ldxdw r3, [r10-0xb8]
    mov64 r4, r7
    call function_12107
    ldxdw r8, [r10-0x80]
    jeq r8, 0, lbb_10170
    mov64 r1, r10
    add64 r1, -144
    ldxdw r2, [r10-0xd0]
    ldxdw r3, [r10-0xc8]
    mov64 r4, r7
    call function_12107
    ldxdw r6, [r10-0x90]
    div64 r6, r8
lbb_10170:
    mov64 r1, r10
    add64 r1, -160
    mov64 r2, r9
    mov64 r3, 0
    mov64 r4, r6
    mov64 r5, 0
    call function_9839
    mov64 r1, r10
    add64 r1, -176
    ldxdw r8, [r10-0xb8]
    mov64 r2, r8
    mov64 r3, 0
    mov64 r4, r6
    mov64 r5, 0
    call function_9839
    ldxdw r2, [r10-0x98]
    ldxdw r1, [r10-0xb0]
    mov64 r3, r2
    add64 r3, r1
    mov64 r1, 1
    jgt r2, r3, lbb_10192
    mov64 r1, 0
lbb_10192:
    ldxdw r4, [r10-0xa8]
    add64 r4, r1
    ldxdw r2, [r10-0xa0]
    ldxdw r0, [r10-0x100]
    ldxdw r7, [r10-0xd0]
    ldxdw r5, [r10-0xc8]
    jne r4, 0, lbb_10428
    mov64 r4, 1
    mov64 r1, 1
    jgt r2, r7, lbb_10203
    mov64 r1, 0
lbb_10203:
    jgt r3, r5, lbb_10205
    mov64 r4, 0
lbb_10205:
    jeq r5, r3, lbb_10207
    mov64 r1, r4
lbb_10207:
    and64 r1, 1
    jne r1, 0, lbb_10428
    sub64 r5, r3
    mov64 r1, 0
    mov64 r3, 1
    jgt r2, r7, lbb_10214
    mov64 r3, 0
lbb_10214:
    sub64 r5, r3
    sub64 r7, r2
    ja lbb_10538
lbb_10217:
    mov64 r1, 96
    sub64 r1, r8
    stxdw [r10-0xf0], r1
    mov64 r4, r1
    lsh64 r4, 32
    stxdw [r10-0xe8], r4
    arsh64 r4, 32
    mov64 r1, r10
    add64 r1, -16
    ldxdw r2, [r10-0xc0]
    ldxdw r3, [r10-0xb8]
    call function_12107
    mov64 r5, 0
    ldxdw r1, [r10-0x10]
    lsh64 r1, 32
    rsh64 r1, 32
    add64 r1, 1
    stxdw [r10-0xf8], r1
    mov64 r4, 0
    ldxdw r3, [r10-0xc8]
lbb_10237:
    stxdw [r10-0xe0], r5
    stxdw [r10-0xd8], r4
    mov64 r6, 64
    sub64 r6, r9
    mov64 r2, r7
    mov64 r7, r6
    and64 r7, 127
    mov64 r1, r10
    add64 r1, -32
    stxdw [r10-0xd0], r2
    stxdw [r10-0xc8], r3
    mov64 r4, r7
    call function_12107
    ldxdw r9, [r10-0x20]
    mov64 r1, r6
    lsh64 r1, 32
    rsh64 r1, 32
    ldxdw r2, [r10-0xe8]
    rsh64 r2, 32
    jge r1, r2, lbb_10268
    mov64 r1, r10
    add64 r1, -96
    ldxdw r2, [r10-0xc0]
    ldxdw r6, [r10-0xb8]
    mov64 r3, r6
    mov64 r4, r7
    call function_12107
    ldxdw r1, [r10-0x60]
    jeq r1, 0, lbb_10446
    div64 r9, r1
    ja lbb_10446
lbb_10268:
    ldxdw r1, [r10-0xf8]
    div64 r9, r1
    ldxdw r1, [r10-0xf0]
    sub64 r6, r1
    and64 r6, 127
    mov64 r1, r10
    add64 r1, -48
    mov64 r2, r9
    mov64 r3, 0
    mov64 r4, r6
    call function_12086
    mov64 r1, r10
    add64 r1, -64
    mov64 r2, r9
    mov64 r3, 0
    ldxdw r4, [r10-0xc0]
    ldxdw r5, [r10-0xb8]
    call function_9839
    mov64 r1, r10
    add64 r1, -80
    ldxdw r2, [r10-0x40]
    ldxdw r3, [r10-0x38]
    mov64 r4, r6
    call function_12086
    ldxdw r3, [r10-0x30]
    mov64 r6, r3
    ldxdw r1, [r10-0xe0]
    add64 r6, r1
    mov64 r1, 1
    mov64 r2, 1
    jgt r3, r6, lbb_10300
    mov64 r2, 0
lbb_10300:
    ldxdw r4, [r10-0x50]
    ldxdw r7, [r10-0xd0]
    ldxdw r3, [r10-0xc8]
    jgt r4, r7, lbb_10305
    mov64 r1, 0
lbb_10305:
    ldxdw r5, [r10-0x48]
    sub64 r3, r5
    sub64 r3, r1
    mov64 r1, r3
    rsh64 r1, 1
    mov64 r5, r3
    or64 r5, r1
    mov64 r1, r5
    rsh64 r1, 2
    or64 r5, r1
    ldxdw r1, [r10-0x28]
    sub64 r7, r4
    mov64 r4, r5
    rsh64 r4, 4
    or64 r5, r4
    mov64 r4, r5
    rsh64 r4, 8
    or64 r5, r4
    mov64 r4, r5
    rsh64 r4, 16
    or64 r5, r4
    mov64 r4, r5
    rsh64 r4, 32
    or64 r5, r4
    xor64 r5, -1
    mov64 r4, r5
    rsh64 r4, 1
    lddw r0, 0x5555555555555555
    and64 r4, r0
    sub64 r5, r4
    mov64 r9, r5
    lddw r4, 0x3333333333333333
    and64 r9, r4
    rsh64 r5, 2
    and64 r5, r4
    add64 r9, r5
    mov64 r4, r9
    rsh64 r4, 4
    add64 r9, r4
    lddw r4, 0xf0f0f0f0f0f0f0f
    and64 r9, r4
    lddw r4, 0x101010101010101
    mul64 r9, r4
    rsh64 r9, 56
    jne r3, 0, lbb_10396
    mov64 r4, r7
    rsh64 r4, 1
    mov64 r5, r7
    or64 r5, r4
    mov64 r4, r5
    rsh64 r4, 2
    or64 r5, r4
    mov64 r4, r5
    rsh64 r4, 4
    or64 r5, r4
    mov64 r4, r5
    rsh64 r4, 8
    or64 r5, r4
    mov64 r4, r5
    rsh64 r4, 16
    or64 r5, r4
    mov64 r4, r5
    rsh64 r4, 32
    or64 r5, r4
    xor64 r5, -1
    mov64 r4, r5
    rsh64 r4, 1
    and64 r4, r0
    sub64 r5, r4
    mov64 r9, r5
    lddw r4, 0x3333333333333333
    and64 r9, r4
    rsh64 r5, 2
    and64 r5, r4
    add64 r9, r5
    mov64 r4, r9
    rsh64 r4, 4
    add64 r9, r4
    lddw r4, 0xf0f0f0f0f0f0f0f
    and64 r9, r4
    lddw r4, 0x101010101010101
    mul64 r9, r4
    rsh64 r9, 56
    add64 r9, 64
lbb_10396:
    ldxdw r4, [r10-0xd8]
    add64 r1, r4
    add64 r1, r2
    mov64 r2, r8
    lsh64 r2, 32
    rsh64 r2, 32
    jgt r2, r9, lbb_10404
    ja lbb_10495
lbb_10404:
    mov64 r2, r9
    lsh64 r2, 32
    rsh64 r2, 32
    mov64 r5, r6
    mov64 r4, r1
    jgt r2, 63, lbb_10411
    ja lbb_10237
lbb_10411:
    mov64 r2, r7
    ldxdw r3, [r10-0xc0]
    div64 r2, r3
    jeq r3, 0, lbb_10416
    mov64 r4, r2
lbb_10416:
    mul64 r2, r3
    mov64 r3, r6
    add64 r3, r4
    mov64 r4, 1
    ldxdw r0, [r10-0x100]
    jgt r6, r3, lbb_10423
    mov64 r4, 0
lbb_10423:
    sub64 r7, r2
    add64 r1, r4
    mov64 r6, r3
    mov64 r8, 0
    ja lbb_10539
lbb_10428:
    add64 r8, r5
    mov64 r4, r9
    add64 r4, r7
    mov64 r1, 0
    mov64 r5, 1
    mov64 r0, 1
    jgt r9, r4, lbb_10436
    mov64 r0, 0
lbb_10436:
    add64 r8, r0
    sub64 r8, r3
    jgt r2, r4, lbb_10440
    mov64 r5, 0
lbb_10440:
    sub64 r8, r5
    sub64 r4, r2
    add64 r6, -1
    mov64 r7, r4
    ldxdw r0, [r10-0x100]
    ja lbb_10539
lbb_10446:
    mov64 r1, r10
    add64 r1, -112
    mov64 r2, r9
    mov64 r3, 0
    ldxdw r4, [r10-0xc0]
    mov64 r5, r6
    call function_9839
    mov64 r4, 1
    ldxdw r1, [r10-0x70]
    mov64 r3, 1
    ldxdw r7, [r10-0xd0]
    jgt r1, r7, lbb_10459
    mov64 r3, 0
lbb_10459:
    ldxdw r2, [r10-0x68]
    ldxdw r0, [r10-0x100]
    ldxdw r5, [r10-0xc8]
    jgt r2, r5, lbb_10464
    mov64 r4, 0
lbb_10464:
    jeq r5, r2, lbb_10466
    mov64 r3, r4
lbb_10466:
    and64 r3, 1
    jne r3, 0, lbb_10469
    ja lbb_10524
lbb_10469:
    add64 r5, r6
    mov64 r8, r5
    mov64 r3, r7
    ldxdw r4, [r10-0xc0]
    add64 r3, r4
    mov64 r4, 1
    mov64 r5, 1
    jgt r7, r3, lbb_10478
    mov64 r5, 0
lbb_10478:
    add64 r8, r5
    ldxdw r7, [r10-0xe0]
    add64 r9, r7
    add64 r9, -1
    mov64 r5, 1
    jgt r7, r9, lbb_10485
    mov64 r5, 0
lbb_10485:
    sub64 r8, r2
    jgt r1, r3, lbb_10488
    mov64 r4, 0
lbb_10488:
    sub64 r8, r4
    sub64 r3, r1
    ldxdw r1, [r10-0xd8]
    add64 r1, r5
    mov64 r7, r3
    mov64 r6, r9
    ja lbb_10539
lbb_10495:
    mov64 r9, r3
    mov64 r3, 1
    mov64 r2, 1
    ldxdw r0, [r10-0x100]
    ldxdw r4, [r10-0xc0]
    jgt r4, r7, lbb_10502
    mov64 r2, 0
lbb_10502:
    ldxdw r5, [r10-0xb8]
    mov64 r8, r9
    jgt r5, r8, lbb_10506
    mov64 r3, 0
lbb_10506:
    jeq r8, r5, lbb_10508
    mov64 r2, r3
lbb_10508:
    and64 r2, 1
    mov64 r8, r9
    jne r2, 0, lbb_10539
    mov64 r2, 1
    mov64 r3, 1
    jgt r4, r7, lbb_10515
    mov64 r3, 0
lbb_10515:
    mov64 r8, r9
    sub64 r8, r5
    add64 r6, 1
    jeq r6, 0, lbb_10520
    mov64 r2, 0
lbb_10520:
    sub64 r8, r3
    add64 r1, r2
    sub64 r7, r4
    ja lbb_10539
lbb_10524:
    ldxdw r8, [r10-0xe0]
    mov64 r6, r8
    add64 r6, r9
    mov64 r4, 1
    mov64 r3, 1
    jgt r8, r6, lbb_10531
    mov64 r3, 0
lbb_10531:
    jgt r1, r7, lbb_10533
    mov64 r4, 0
lbb_10533:
    sub64 r5, r2
    sub64 r5, r4
    sub64 r7, r1
    ldxdw r1, [r10-0xd8]
    add64 r1, r3
lbb_10538:
    mov64 r8, r5
lbb_10539:
    stxdw [r0+0x10], r7
    stxdw [r0+0x0], r6
    stxdw [r0+0x18], r8
    stxdw [r0+0x8], r1
    exit

compiler_builtins::int::specialized_div_rem::u64_div_rem::h09c70c91517def17:
    mov64 r8, 0
    jgt r3, r2, lbb_10686
    mov64 r4, r3
    rsh64 r4, 1
    mov64 r6, r3
    or64 r6, r4
    mov64 r4, r6
    rsh64 r4, 2
    or64 r6, r4
    mov64 r4, r6
    rsh64 r4, 4
    or64 r6, r4
    mov64 r4, r6
    rsh64 r4, 8
    or64 r6, r4
    mov64 r4, r6
    rsh64 r4, 16
    or64 r6, r4
    mov64 r4, r6
    rsh64 r4, 32
    or64 r6, r4
    xor64 r6, -1
    lddw r0, 0x5555555555555555
    mov64 r4, r6
    rsh64 r4, 1
    and64 r4, r0
    sub64 r6, r4
    lddw r5, 0x3333333333333333
    mov64 r4, r6
    and64 r4, r5
    rsh64 r6, 2
    and64 r6, r5
    add64 r4, r6
    mov64 r6, r4
    rsh64 r6, 4
    add64 r4, r6
    lddw r6, 0xf0f0f0f0f0f0f0f
    and64 r4, r6
    lddw r7, 0x101010101010101
    mul64 r4, r7
    mov64 r9, 64
    rsh64 r4, 56
    jeq r2, 0, lbb_10626
    mov64 r9, r2
    rsh64 r9, 1
    mov64 r8, r2
    or64 r8, r9
    mov64 r9, r8
    rsh64 r9, 2
    or64 r8, r9
    mov64 r9, r8
    rsh64 r9, 4
    or64 r8, r9
    mov64 r9, r8
    rsh64 r9, 8
    or64 r8, r9
    mov64 r9, r8
    rsh64 r9, 16
    or64 r8, r9
    mov64 r9, r8
    rsh64 r9, 32
    or64 r8, r9
    xor64 r8, -1
    mov64 r9, r8
    rsh64 r9, 1
    and64 r9, r0
    sub64 r8, r9
    mov64 r9, r8
    and64 r9, r5
    rsh64 r8, 2
    and64 r8, r5
    add64 r9, r8
    mov64 r5, r9
    rsh64 r5, 4
    add64 r9, r5
    and64 r9, r6
    mul64 r9, r7
    rsh64 r9, 56
lbb_10626:
    sub64 r4, r9
    mov64 r5, r4
    and64 r5, 63
    mov64 r0, r3
    lsh64 r0, r5
    mov64 r8, 1
    mov64 r5, 1
    jgt r0, r2, lbb_10635
    mov64 r5, 0
lbb_10635:
    lsh64 r4, 32
    rsh64 r4, 32
    sub64 r4, r5
    mov64 r0, r4
    and64 r0, 63
    lsh64 r8, r0
    mov64 r5, r3
    lsh64 r5, r0
    sub64 r2, r5
    jgt r3, r2, lbb_10686
    mov64 r0, r8
    mov64 r6, r8
    mov64 r7, r2
    jsgt r5, -1, lbb_10666
    add64 r4, -1
    mov64 r6, r4
    and64 r6, 63
    mov64 r0, 1
    lsh64 r0, r6
    rsh64 r5, 1
    mov64 r7, r2
    sub64 r7, r5
    mov64 r6, r0
    jsgt r7, -1, lbb_10660
    mov64 r6, 0
lbb_10660:
    jsgt r7, -1, lbb_10662
    mov64 r7, r2
lbb_10662:
    or64 r6, r8
    mov64 r2, r7
    mov64 r8, r6
    jgt r3, r7, lbb_10686
lbb_10666:
    add64 r0, -1
    jeq r4, 0, lbb_10680
    mov64 r2, 0
    mov64 r3, r4
    ja lbb_10673
lbb_10671:
    add64 r3, -1
    jeq r3, 0, lbb_10680
lbb_10673:
    lsh64 r7, 1
    mov64 r8, r7
    sub64 r8, r5
    add64 r8, 1
    jsgt r2, r8, lbb_10671
    mov64 r7, r8
    ja lbb_10671
lbb_10680:
    and64 r4, 63
    mov64 r2, r7
    rsh64 r2, r4
    and64 r7, r0
    or64 r7, r6
    mov64 r8, r7
lbb_10686:
    stxdw [r1+0x8], r2
    stxdw [r1+0x0], r8
    exit

compiler_builtins::int::specialized_div_rem::u32_div_rem::h57d597347f79cdee:
    mov64 r0, 0
    mov64 r5, r2
    lsh64 r5, 32
    rsh64 r5, 32
    mov64 r4, r3
    lsh64 r4, 32
    rsh64 r4, 32
    jgt r4, r5, lbb_10886
    lddw r0, 0xfffffffe
    mov64 r4, r3
    and64 r4, r0
    rsh64 r4, 1
    mov64 r0, r3
    or64 r0, r4
    lddw r6, 0xfffffffc
    mov64 r4, r0
    and64 r4, r6
    rsh64 r4, 2
    or64 r0, r4
    lddw r6, 0xfffffff0
    mov64 r4, r0
    and64 r4, r6
    rsh64 r4, 4
    or64 r0, r4
    lddw r6, 0xffffff00
    mov64 r4, r0
    and64 r4, r6
    rsh64 r4, 8
    or64 r0, r4
    lddw r6, 0xffff0000
    mov64 r4, r0
    and64 r4, r6
    rsh64 r4, 16
    or64 r0, r4
    xor64 r0, -1
    mov64 r4, r0
    rsh64 r4, 1
    and64 r4, 1431655765
    sub64 r0, r4
    mov64 r4, r0
    and64 r4, 858993459
    rsh64 r0, 2
    and64 r0, 858993459
    add64 r4, r0
    mov64 r0, r4
    rsh64 r0, 4
    add64 r4, r0
    and64 r4, 252645135
    mul64 r4, 16843009
    lddw r0, 0xff000000
    and64 r4, r0
    mov64 r6, 32
    rsh64 r4, 24
    jeq r5, 0, lbb_10797
    mov64 r6, r2
    lddw r5, 0xfffffffe
    and64 r6, r5
    rsh64 r6, 1
    mov64 r5, r2
    or64 r5, r6
    mov64 r6, r5
    lddw r8, 0xfffffff0
    lddw r9, 0xffffff00
    lddw r7, 0xfffffffc
    and64 r6, r7
    rsh64 r6, 2
    or64 r5, r6
    mov64 r6, r5
    and64 r6, r8
    rsh64 r6, 4
    or64 r5, r6
    mov64 r6, r5
    and64 r6, r9
    rsh64 r6, 8
    or64 r5, r6
    mov64 r6, r5
    lddw r7, 0xffff0000
    and64 r6, r7
    rsh64 r6, 16
    or64 r5, r6
    xor64 r5, -1
    mov64 r6, r5
    rsh64 r6, 1
    and64 r6, 1431655765
    sub64 r5, r6
    mov64 r6, r5
    and64 r6, 858993459
    rsh64 r5, 2
    and64 r5, 858993459
    add64 r6, r5
    mov64 r5, r6
    rsh64 r5, 4
    add64 r6, r5
    and64 r6, 252645135
    mul64 r6, 16843009
    and64 r6, r0
    rsh64 r6, 24
lbb_10797:
    sub64 r4, r6
    mov64 r5, r4
    and64 r5, 31
    mov64 r6, r3
    lsh64 r6, r5
    lsh64 r6, 32
    rsh64 r6, 32
    mov64 r7, r2
    lsh64 r7, 32
    rsh64 r7, 32
    mov64 r0, 1
    mov64 r5, 1
    jgt r6, r7, lbb_10811
    mov64 r5, 0
lbb_10811:
    lsh64 r4, 32
    rsh64 r4, 32
    sub64 r4, r5
    mov64 r6, r4
    and64 r6, 31
    lsh64 r0, r6
    mov64 r5, r3
    lsh64 r5, r6
    sub64 r2, r5
    mov64 r6, r3
    lsh64 r6, 32
    rsh64 r6, 32
    mov64 r7, r2
    lsh64 r7, 32
    rsh64 r7, 32
    jgt r6, r7, lbb_10886
    lsh64 r5, 32
    arsh64 r5, 32
    mov64 r7, r0
    mov64 r8, r0
    mov64 r6, r2
    jsgt r5, -1, lbb_10861
    lddw r6, 0xfffffffe
    and64 r5, r6
    add64 r4, -1
    mov64 r8, r4
    and64 r8, 31
    mov64 r7, 1
    rsh64 r5, 1
    mov64 r6, r2
    sub64 r6, r5
    mov64 r9, r6
    lsh64 r9, 32
    arsh64 r9, 32
    jsgt r9, -1, lbb_10848
    mov64 r6, r2
lbb_10848:
    lsh64 r7, r8
    mov64 r8, r7
    jsgt r9, -1, lbb_10852
    mov64 r8, 0
lbb_10852:
    or64 r8, r0
    mov64 r9, r6
    lsh64 r9, 32
    lsh64 r3, 32
    rsh64 r9, 32
    rsh64 r3, 32
    mov64 r2, r6
    mov64 r0, r8
    jgt r3, r9, lbb_10886
lbb_10861:
    add64 r7, -1
    jeq r4, 0, lbb_10878
    mov64 r2, 0
    mov64 r3, r4
    ja lbb_10868
lbb_10866:
    add64 r3, -1
    jeq r3, 0, lbb_10878
lbb_10868:
    lsh64 r6, 1
    mov64 r0, r6
    sub64 r0, r5
    add64 r0, 1
    mov64 r9, r0
    lsh64 r9, 32
    arsh64 r9, 32
    jsgt r2, r9, lbb_10866
    mov64 r6, r0
    ja lbb_10866
lbb_10878:
    mov64 r0, r6
    and64 r0, r7
    or64 r0, r8
    and64 r4, 31
    lsh64 r6, 32
    rsh64 r6, 32
    rsh64 r6, r4
    mov64 r2, r6
lbb_10886:
    stxw [r1+0x4], r2
    stxw [r1+0x0], r0
    exit

function_10889:
    call function_11202
    exit

compiler_builtins::float::add::__addsf3::hf1f9edd5721df459:
    mov64 r4, r2
    and64 r4, 2147483647
    mov64 r5, r1
    and64 r5, 2147483647
    mov64 r3, r5
    add64 r3, -2139095040
    lsh64 r3, 32
    rsh64 r3, 32
    lddw r0, 0x80800001
    jgt r0, r3, lbb_10909
    mov64 r3, r4
    add64 r3, -2139095040
    lsh64 r3, 32
    rsh64 r3, 32
    lddw r0, 0x80800000
    jgt r3, r0, lbb_10936
lbb_10909:
    jgt r5, 2139095040, lbb_10911
    ja lbb_10914
lbb_10911:
    or64 r5, 4194304
    mov64 r0, r5
    ja lbb_11195
lbb_10914:
    jgt r4, 2139095040, lbb_10916
    ja lbb_10919
lbb_10916:
    or64 r4, 4194304
    mov64 r0, r4
    ja lbb_11195
lbb_10919:
    jeq r5, 2139095040, lbb_10921
    ja lbb_10931
lbb_10921:
    xor64 r2, r1
    lsh64 r2, 32
    rsh64 r2, 32
    lddw r3, 0x80000000
    mov64 r0, r1
    jeq r2, r3, lbb_10929
    ja lbb_11195
lbb_10929:
    mov64 r0, 2143289344
    ja lbb_11195
lbb_10931:
    mov64 r0, r2
    jeq r4, 2139095040, lbb_11195
    jeq r5, 0, lbb_11196
    mov64 r0, r1
    jeq r4, 0, lbb_11195
lbb_10936:
    mov64 r3, r2
    jgt r4, r5, lbb_10939
    mov64 r3, r1
lbb_10939:
    jgt r4, r5, lbb_10941
    mov64 r1, r2
lbb_10941:
    mov64 r2, r3
    and64 r2, 8388607
    mov64 r0, r1
    rsh64 r0, 23
    and64 r0, 255
    mov64 r4, r3
    rsh64 r4, 23
    and64 r4, 255
    jeq r4, 0, lbb_10951
    ja lbb_10995
lbb_10951:
    mov64 r5, 32
    jeq r2, 0, lbb_10990
    mov64 r4, r2
    rsh64 r4, 1
    mov64 r5, r2
    or64 r5, r4
    mov64 r4, r5
    rsh64 r4, 2
    or64 r5, r4
    mov64 r4, r5
    rsh64 r4, 4
    or64 r5, r4
    mov64 r6, r5
    and64 r6, 8388352
    rsh64 r6, 8
    mov64 r4, r5
    or64 r4, r6
    and64 r5, 8323072
    rsh64 r5, 16
    or64 r4, r5
    xor64 r4, -1
    mov64 r5, r4
    rsh64 r5, 1
    and64 r5, 1431655765
    sub64 r4, r5
    mov64 r5, r4
    and64 r5, 858993459
    rsh64 r4, 2
    and64 r4, 858993459
    add64 r5, r4
    mov64 r4, r5
    rsh64 r4, 4
    add64 r5, r4
    and64 r5, 252645135
    mul64 r5, 16843009
    lddw r4, 0xff000000
    and64 r5, r4
    rsh64 r5, 24
lbb_10990:
    mov64 r4, 9
    sub64 r4, r5
    add64 r5, 24
    and64 r5, 31
    lsh64 r2, r5
lbb_10995:
    mov64 r5, r1
    and64 r5, 8388607
    jne r0, 0, lbb_11042
    mov64 r6, 32
    jeq r5, 0, lbb_11037
    mov64 r0, r5
    rsh64 r0, 1
    mov64 r6, r5
    or64 r6, r0
    mov64 r0, r6
    rsh64 r0, 2
    or64 r6, r0
    mov64 r0, r6
    rsh64 r0, 4
    or64 r6, r0
    mov64 r7, r6
    and64 r7, 8388352
    rsh64 r7, 8
    mov64 r0, r6
    or64 r0, r7
    and64 r6, 8323072
    rsh64 r6, 16
    or64 r0, r6
    xor64 r0, -1
    mov64 r6, r0
    rsh64 r6, 1
    and64 r6, 1431655765
    sub64 r0, r6
    mov64 r6, r0
    and64 r6, 858993459
    rsh64 r0, 2
    and64 r0, 858993459
    add64 r6, r0
    mov64 r0, r6
    rsh64 r0, 4
    add64 r6, r0
    and64 r6, 252645135
    mul64 r6, 16843009
    lddw r0, 0xff000000
    and64 r6, r0
    rsh64 r6, 24
lbb_11037:
    mov64 r0, 9
    sub64 r0, r6
    add64 r6, 24
    and64 r6, 31
    lsh64 r5, r6
lbb_11042:
    xor64 r1, r3
    lsh64 r5, 3
    or64 r5, 67108864
    lsh64 r2, 3
    lsh64 r1, 32
    arsh64 r1, 32
    mov64 r7, r5
    jeq r4, r0, lbb_11055
    mov64 r6, r4
    sub64 r6, r0
    mov64 r7, 1
    mov64 r0, 32
    jgt r0, r6, lbb_11176
lbb_11055:
    or64 r2, 67108864
    jsgt r1, -1, lbb_11058
    ja lbb_11071
lbb_11058:
    mov64 r1, r7
    add64 r1, r2
    mov64 r2, r1
    and64 r2, 134217728
    jeq r2, 0, lbb_11135
    lddw r2, 0xfffffffe
    and64 r1, r2
    rsh64 r1, 1
    and64 r7, 1
    or64 r1, r7
    add64 r4, 1
    ja lbb_11135
lbb_11071:
    sub64 r2, r7
    mov64 r0, 0
    mov64 r5, r2
    lsh64 r5, 32
    rsh64 r5, 32
    jeq r5, 0, lbb_11195
    mov64 r1, r2
    jgt r5, 67108863, lbb_11135
    lddw r1, 0xfffffffe
    mov64 r5, r2
    and64 r5, r1
    rsh64 r5, 1
    mov64 r1, r2
    or64 r1, r5
    lddw r5, 0xfffffffc
    mov64 r0, r1
    and64 r0, r5
    rsh64 r0, 2
    or64 r1, r0
    lddw r5, 0xfffffff0
    mov64 r0, r1
    and64 r0, r5
    rsh64 r0, 4
    or64 r1, r0
    lddw r5, 0xffffff00
    mov64 r0, r1
    and64 r0, r5
    rsh64 r0, 8
    or64 r1, r0
    lddw r5, 0xffff0000
    mov64 r0, r1
    and64 r0, r5
    rsh64 r0, 16
    or64 r1, r0
    xor64 r1, -1
    mov64 r5, r1
    rsh64 r5, 1
    and64 r5, 1431655765
    sub64 r1, r5
    mov64 r5, r1
    and64 r5, 858993459
    rsh64 r1, 2
    and64 r1, 858993459
    add64 r5, r1
    mov64 r1, r5
    rsh64 r1, 4
    add64 r5, r1
    and64 r5, 252645135
    mul64 r5, 16843009
    lddw r1, 0xff000000
    and64 r5, r1
    rsh64 r5, 24
    add64 r5, -5
    sub64 r4, r5
    lsh64 r5, 32
    rsh64 r5, 32
    lsh64 r2, r5
    mov64 r1, r2
lbb_11135:
    and64 r3, -2147483648
    jsgt r4, 254, lbb_11138
    ja lbb_11141
lbb_11138:
    or64 r3, 2139095040
    mov64 r0, r3
    ja lbb_11195
lbb_11141:
    mov64 r2, 1
    jsgt r2, r4, lbb_11144
    ja lbb_11161
lbb_11144:
    mov64 r5, r4
    add64 r5, -1
    and64 r5, 31
    mov64 r0, r1
    lsh64 r0, r5
    lsh64 r0, 32
    rsh64 r0, 32
    mov64 r5, 1
    jne r0, 0, lbb_11154
    mov64 r5, 0
lbb_11154:
    sub64 r2, r4
    and64 r2, 31
    lsh64 r1, 32
    rsh64 r1, 32
    rsh64 r1, r2
    or64 r1, r5
    mov64 r4, 0
lbb_11161:
    lsh64 r4, 23
    mov64 r2, r1
    rsh64 r2, 3
    mov64 r0, r2
    and64 r0, 8388607
    or64 r0, r4
    or64 r0, r3
    and64 r1, 7
    jgt r1, 4, lbb_11194
    jeq r1, 4, lbb_11172
    ja lbb_11195
lbb_11172:
    and64 r2, 536870911
    and64 r2, 1
    add64 r0, r2
    ja lbb_11195
lbb_11176:
    sub64 r0, r6
    lsh64 r0, 32
    rsh64 r0, 32
    mov64 r7, r5
    lsh64 r7, r0
    lsh64 r7, 32
    rsh64 r7, 32
    mov64 r0, 1
    jne r7, 0, lbb_11186
    mov64 r0, 0
lbb_11186:
    lsh64 r6, 32
    rsh64 r6, 32
    lsh64 r5, 32
    rsh64 r5, 32
    rsh64 r5, r6
    or64 r5, r0
    mov64 r7, r5
    ja lbb_11055
lbb_11194:
    add64 r0, 1
lbb_11195:
    exit
lbb_11196:
    mov64 r0, r2
    jeq r4, 0, lbb_11199
    ja lbb_11195
lbb_11199:
    and64 r2, r1
    mov64 r0, r2
    ja lbb_11195

function_11202:
    lddw r3, 0x7fffffffffffffff
    mov64 r4, r2
    and64 r4, r3
    mov64 r5, r1
    and64 r5, r3
    lddw r3, 0x8010000000000000
    mov64 r0, r5
    add64 r0, r3
    lddw r6, 0x8010000000000001
    jgt r6, r0, lbb_11218
    mov64 r0, r4
    add64 r0, r3
    jgt r0, r3, lbb_11250
lbb_11218:
    lddw r3, 0x7ff0000000000000
    jgt r5, r3, lbb_11222
    ja lbb_11227
lbb_11222:
    lddw r1, 0x8000000000000
    or64 r5, r1
    mov64 r0, r5
    ja lbb_11513
lbb_11227:
    jgt r4, r3, lbb_11229
    ja lbb_11234
lbb_11229:
    lddw r1, 0x8000000000000
    or64 r4, r1
    mov64 r0, r4
    ja lbb_11513
lbb_11234:
    jeq r5, r3, lbb_11236
    ja lbb_11245
lbb_11236:
    xor64 r2, r1
    lddw r3, 0x8000000000000000
    mov64 r0, r1
    jeq r2, r3, lbb_11242
    ja lbb_11513
lbb_11242:
    lddw r0, 0x7ff8000000000000
    ja lbb_11513
lbb_11245:
    mov64 r0, r2
    jeq r4, r3, lbb_11513
    jeq r5, 0, lbb_11514
    mov64 r0, r1
    jeq r4, 0, lbb_11513
lbb_11250:
    mov64 r3, r2
    jgt r4, r5, lbb_11253
    mov64 r3, r1
lbb_11253:
    jgt r4, r5, lbb_11255
    mov64 r1, r2
lbb_11255:
    lddw r6, 0xfffffffffffff
    mov64 r4, r3
    and64 r4, r6
    mov64 r0, r1
    rsh64 r0, 52
    and64 r0, 2047
    mov64 r2, r3
    rsh64 r2, 52
    and64 r2, 2047
    jeq r2, 0, lbb_11267
    ja lbb_11317
lbb_11267:
    mov64 r5, 64
    jeq r4, 0, lbb_11312
    mov64 r5, r4
    rsh64 r5, 1
    mov64 r2, r4
    or64 r2, r5
    mov64 r5, r2
    rsh64 r5, 2
    or64 r2, r5
    mov64 r5, r2
    rsh64 r5, 4
    or64 r2, r5
    mov64 r5, r2
    rsh64 r5, 8
    or64 r2, r5
    mov64 r5, r2
    rsh64 r5, 16
    or64 r2, r5
    mov64 r5, r2
    rsh64 r5, 32
    or64 r2, r5
    xor64 r2, -1
    lddw r5, 0x5555555555555555
    mov64 r7, r2
    rsh64 r7, 1
    and64 r7, r5
    sub64 r2, r7
    lddw r7, 0x3333333333333333
    mov64 r5, r2
    and64 r5, r7
    rsh64 r2, 2
    and64 r2, r7
    add64 r5, r2
    mov64 r2, r5
    rsh64 r2, 4
    add64 r5, r2
    lddw r2, 0xf0f0f0f0f0f0f0f
    and64 r5, r2
    lddw r2, 0x101010101010101
    mul64 r5, r2
    rsh64 r5, 56
lbb_11312:
    mov64 r2, 12
    sub64 r2, r5
    add64 r5, 53
    and64 r5, 63
    lsh64 r4, r5
lbb_11317:
    mov64 r5, r1
    and64 r5, r6
    jne r0, 0, lbb_11370
    mov64 r6, 64
    jeq r5, 0, lbb_11365
    mov64 r6, r5
    rsh64 r6, 1
    mov64 r0, r5
    or64 r0, r6
    mov64 r6, r0
    rsh64 r6, 2
    or64 r0, r6
    mov64 r6, r0
    rsh64 r6, 4
    or64 r0, r6
    mov64 r6, r0
    rsh64 r6, 8
    or64 r0, r6
    mov64 r6, r0
    rsh64 r6, 16
    or64 r0, r6
    mov64 r6, r0
    rsh64 r6, 32
    or64 r0, r6
    xor64 r0, -1
    lddw r6, 0x5555555555555555
    mov64 r7, r0
    rsh64 r7, 1
    and64 r7, r6
    sub64 r0, r7
    lddw r7, 0x3333333333333333
    mov64 r6, r0
    and64 r6, r7
    rsh64 r0, 2
    and64 r0, r7
    add64 r6, r0
    mov64 r0, r6
    rsh64 r0, 4
    add64 r6, r0
    lddw r0, 0xf0f0f0f0f0f0f0f
    and64 r6, r0
    lddw r0, 0x101010101010101
    mul64 r6, r0
    rsh64 r6, 56
lbb_11365:
    mov64 r0, 12
    sub64 r0, r6
    add64 r6, 53
    and64 r6, 63
    lsh64 r5, r6
lbb_11370:
    xor64 r1, r3
    lsh64 r4, 3
    lsh64 r5, 3
    lddw r6, 0x80000000000000
    or64 r5, r6
    mov64 r7, r5
    jeq r2, r0, lbb_11383
    mov64 r8, r2
    sub64 r8, r0
    mov64 r7, 1
    mov64 r0, 64
    jgt r0, r8, lbb_11497
lbb_11383:
    or64 r4, r6
    jsgt r1, -1, lbb_11386
    ja lbb_11398
lbb_11386:
    mov64 r1, r7
    add64 r1, r4
    lddw r4, 0x100000000000000
    mov64 r5, r1
    and64 r5, r4
    jeq r5, 0, lbb_11454
    and64 r7, 1
    rsh64 r1, 1
    or64 r1, r7
    add64 r2, 1
    ja lbb_11454
lbb_11398:
    sub64 r4, r7
    mov64 r0, 0
    jeq r4, 0, lbb_11513
    lddw r5, 0x7fffffffffffff
    mov64 r1, r4
    jgt r4, r5, lbb_11454
    mov64 r5, r4
    rsh64 r5, 1
    mov64 r1, r4
    or64 r1, r5
    mov64 r5, r1
    rsh64 r5, 2
    or64 r1, r5
    mov64 r5, r1
    rsh64 r5, 4
    or64 r1, r5
    mov64 r5, r1
    rsh64 r5, 8
    or64 r1, r5
    mov64 r5, r1
    rsh64 r5, 16
    or64 r1, r5
    mov64 r5, r1
    rsh64 r5, 32
    or64 r1, r5
    xor64 r1, -1
    lddw r5, 0x5555555555555555
    mov64 r0, r1
    rsh64 r0, 1
    and64 r0, r5
    sub64 r1, r0
    lddw r0, 0x3333333333333333
    mov64 r5, r1
    and64 r5, r0
    rsh64 r1, 2
    and64 r1, r0
    add64 r5, r1
    mov64 r1, r5
    rsh64 r1, 4
    add64 r5, r1
    lddw r1, 0xf0f0f0f0f0f0f0f
    and64 r5, r1
    lddw r1, 0x101010101010101
    mul64 r5, r1
    rsh64 r5, 56
    add64 r5, -8
    sub64 r2, r5
    lsh64 r5, 32
    rsh64 r5, 32
    lsh64 r4, r5
    mov64 r1, r4
lbb_11454:
    lddw r4, 0x8000000000000000
    and64 r3, r4
    jsgt r2, 2046, lbb_11459
    ja lbb_11464
lbb_11459:
    lddw r1, 0x7ff0000000000000
    or64 r3, r1
    mov64 r0, r3
    ja lbb_11513
lbb_11464:
    mov64 r4, 1
    jsgt r4, r2, lbb_11467
    ja lbb_11480
lbb_11467:
    mov64 r5, r2
    add64 r5, -1
    and64 r5, 63
    mov64 r0, r1
    lsh64 r0, r5
    mov64 r5, 1
    jne r0, 0, lbb_11475
    mov64 r5, 0
lbb_11475:
    sub64 r4, r2
    and64 r4, 63
    rsh64 r1, r4
    or64 r1, r5
    mov64 r2, 0
lbb_11480:
    mov64 r4, r1
    rsh64 r4, 3
    lddw r5, 0xfffffffffffff
    mov64 r0, r4
    and64 r0, r5
    lsh64 r2, 52
    or64 r2, r0
    or64 r2, r3
    and64 r1, 7
    jgt r1, 4, lbb_11511
    mov64 r0, r2
    jeq r1, 4, lbb_11494
    ja lbb_11513
lbb_11494:
    and64 r4, 1
    add64 r2, r4
    ja lbb_11512
lbb_11497:
    mov64 r0, r8
    neg64 r0
    and64 r0, 63
    mov64 r7, r5
    lsh64 r7, r0
    mov64 r0, 1
    jne r7, 0, lbb_11505
    mov64 r0, 0
lbb_11505:
    lsh64 r8, 32
    rsh64 r8, 32
    rsh64 r5, r8
    or64 r5, r0
    mov64 r7, r5
    ja lbb_11383
lbb_11511:
    add64 r2, 1
lbb_11512:
    mov64 r0, r2
lbb_11513:
    exit
lbb_11514:
    mov64 r0, r2
    jeq r4, 0, lbb_11517
    ja lbb_11513
lbb_11517:
    and64 r2, r1
    ja lbb_11512

function_11519:
    mov64 r0, 1
    lddw r5, 0x7fffffffffffffff
    mov64 r3, r1
    and64 r3, r5
    lddw r6, 0x7ff0000000000000
    jgt r3, r6, lbb_11549
    mov64 r4, r2
    and64 r4, r5
    jgt r4, r6, lbb_11549
    or64 r4, r3
    mov64 r0, 0
    jeq r4, 0, lbb_11549
    mov64 r3, r2
    and64 r3, r1
    jsgt r3, -1, lbb_11542
    lddw r0, 0xffffffff
    jsgt r1, r2, lbb_11549
    mov64 r0, 1
    jeq r1, r2, lbb_11548
    ja lbb_11549
lbb_11542:
    lddw r0, 0xffffffff
    jsgt r2, r1, lbb_11549
    mov64 r0, 1
    jeq r1, r2, lbb_11548
    ja lbb_11549
lbb_11548:
    mov64 r0, 0
lbb_11549:
    lsh64 r0, 32
    arsh64 r0, 32
    exit

function_11552:
    call function_11768
    exit

compiler_builtins::float::mul::__mulsf3::h196c5bccba4fb8c9:
    mov64 r3, r2
    and64 r3, 8388607
    mov64 r5, r1
    and64 r5, 8388607
    mov64 r0, r2
    xor64 r0, r1
    and64 r0, -2147483648
    mov64 r4, r2
    rsh64 r4, 23
    and64 r4, 255
    mov64 r6, r1
    rsh64 r6, 23
    and64 r6, 255
    mov64 r7, r6
    add64 r7, -255
    mov64 r8, -254
    jgt r8, r7, lbb_11575
    mov64 r7, 0
    mov64 r8, r4
    add64 r8, -255
    jgt r8, -255, lbb_11696
lbb_11575:
    mov64 r9, r1
    and64 r9, 2147483647
    jgt r9, 2139095040, lbb_11584
    mov64 r8, r2
    and64 r8, 2147483647
    jgt r8, 2139095040, lbb_11582
    ja lbb_11586
lbb_11582:
    or64 r2, 4194304
    ja lbb_11592
lbb_11584:
    or64 r1, 4194304
    ja lbb_11766
lbb_11586:
    jeq r9, 2139095040, lbb_11588
    ja lbb_11594
lbb_11588:
    mov64 r0, 2143289344
    jeq r8, 0, lbb_11767
    and64 r2, -2147483648
    xor64 r2, r1
lbb_11592:
    mov64 r0, r2
    ja lbb_11767
lbb_11594:
    jeq r8, 2139095040, lbb_11596
    ja lbb_11601
lbb_11596:
    mov64 r0, 2143289344
    jeq r9, 0, lbb_11767
    and64 r1, -2147483648
    xor64 r1, r2
    ja lbb_11766
lbb_11601:
    jeq r9, 0, lbb_11767
    jeq r8, 0, lbb_11767
    mov64 r7, 0
    mov64 r1, 8388608
    jgt r1, r9, lbb_11607
    ja lbb_11651
lbb_11607:
    mov64 r1, 32
    jeq r5, 0, lbb_11646
    mov64 r2, r5
    rsh64 r2, 1
    mov64 r1, r5
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 2
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 4
    or64 r1, r2
    mov64 r7, r1
    and64 r7, 8388352
    rsh64 r7, 8
    mov64 r2, r1
    or64 r2, r7
    and64 r1, 8323072
    rsh64 r1, 16
    or64 r2, r1
    xor64 r2, -1
    mov64 r1, r2
    rsh64 r1, 1
    and64 r1, 1431655765
    sub64 r2, r1
    mov64 r1, r2
    and64 r1, 858993459
    rsh64 r2, 2
    and64 r2, 858993459
    add64 r1, r2
    mov64 r2, r1
    rsh64 r2, 4
    add64 r1, r2
    and64 r1, 252645135
    mul64 r1, 16843009
    lddw r2, 0xff000000
    and64 r1, r2
    rsh64 r1, 24
lbb_11646:
    mov64 r7, 9
    sub64 r7, r1
    add64 r1, 24
    and64 r1, 31
    lsh64 r5, r1
lbb_11651:
    jgt r8, 8388607, lbb_11696
    mov64 r1, 32
    jeq r3, 0, lbb_11691
    mov64 r2, r3
    rsh64 r2, 1
    mov64 r1, r3
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 2
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 4
    or64 r1, r2
    mov64 r8, r1
    and64 r8, 8388352
    rsh64 r8, 8
    mov64 r2, r1
    or64 r2, r8
    and64 r1, 8323072
    rsh64 r1, 16
    or64 r2, r1
    xor64 r2, -1
    mov64 r1, r2
    rsh64 r1, 1
    and64 r1, 1431655765
    sub64 r2, r1
    mov64 r1, r2
    and64 r1, 858993459
    rsh64 r2, 2
    and64 r2, 858993459
    add64 r1, r2
    mov64 r2, r1
    rsh64 r2, 4
    add64 r1, r2
    and64 r1, 252645135
    mul64 r1, 16843009
    lddw r2, 0xff000000
    and64 r1, r2
    rsh64 r1, 24
lbb_11691:
    sub64 r7, r1
    add64 r1, 24
    and64 r1, 31
    lsh64 r3, r1
    add64 r7, 9
lbb_11696:
    lsh64 r3, 8
    lddw r1, 0x80000000
    or64 r3, r1
    add64 r4, r6
    add64 r4, r7
    lsh64 r3, 32
    rsh64 r3, 32
    or64 r5, 8388608
    lsh64 r5, 32
    rsh64 r5, 32
    mul64 r3, r5
    mov64 r1, r3
    rsh64 r1, 32
    mov64 r2, r1
    and64 r2, 8388608
    jeq r2, 0, lbb_11714
    ja lbb_11725
lbb_11714:
    lsh64 r1, 1
    mov64 r2, r3
    rsh64 r2, 31
    and64 r2, 1
    or64 r1, r2
    lsh64 r3, 1
    add64 r4, -127
    jsgt r4, 254, lbb_11723
    ja lbb_11727
lbb_11723:
    or64 r0, 2139095040
    ja lbb_11767
lbb_11725:
    add64 r4, -126
    jsgt r4, 254, lbb_11723
lbb_11727:
    mov64 r2, 1
    jsgt r2, r4, lbb_11733
    and64 r1, 8388607
    lsh64 r4, 23
    or64 r4, r1
    ja lbb_11752
lbb_11733:
    sub64 r2, r4
    jgt r2, 31, lbb_11767
    add64 r4, 31
    lsh64 r4, 32
    rsh64 r4, 32
    mov64 r5, r1
    or64 r5, r3
    lsh64 r5, r4
    lsh64 r2, 32
    rsh64 r2, 32
    lsh64 r3, 32
    rsh64 r3, 32
    rsh64 r3, r2
    or64 r5, r3
    lsh64 r1, 32
    rsh64 r1, 32
    rsh64 r1, r2
    mov64 r3, r5
    mov64 r4, r1
lbb_11752:
    mov64 r1, r4
    or64 r1, r0
    lsh64 r3, 32
    rsh64 r3, 32
    lddw r2, 0x80000000
    jgt r3, r2, lbb_11760
    ja lbb_11762
lbb_11760:
    add64 r1, 1
    ja lbb_11766
lbb_11762:
    mov64 r0, r1
    jne r3, r2, lbb_11767
    and64 r4, 1
    add64 r1, r4
lbb_11766:
    mov64 r0, r1
lbb_11767:
    exit

function_11768:
    mov64 r6, r2
    mov64 r3, r6
    xor64 r3, r1
    lddw r2, 0x8000000000000000
    and64 r3, r2
    stxdw [r10-0x18], r3
    lddw r5, 0xfffffffffffff
    mov64 r2, r6
    and64 r2, r5
    mov64 r4, r1
    and64 r4, r5
    mov64 r7, r6
    rsh64 r7, 52
    and64 r7, 2047
    mov64 r8, r1
    rsh64 r8, 52
    and64 r8, 2047
    mov64 r5, r8
    add64 r5, -2047
    mov64 r0, -2046
    jgt r0, r5, lbb_11795
    mov64 r9, 0
    mov64 r5, r7
    add64 r5, -2047
    jgt r5, -2047, lbb_11948
lbb_11795:
    lddw r9, 0x7fffffffffffffff
    mov64 r5, r1
    and64 r5, r9
    lddw r0, 0x7ff0000000000000
    jgt r5, r0, lbb_11810
    mov64 r3, r6
    and64 r3, r9
    jgt r3, r0, lbb_11806
    ja lbb_11815
lbb_11806:
    lddw r1, 0x8000000000000
    or64 r6, r1
    ja lbb_11824
lbb_11810:
    lddw r2, 0x8000000000000
    or64 r1, r2
    mov64 r0, r1
    ja lbb_12020
lbb_11815:
    jeq r5, r0, lbb_11817
    ja lbb_11826
lbb_11817:
    lddw r0, 0x7ff8000000000000
    jeq r3, 0, lbb_12020
    lddw r2, 0x8000000000000000
    and64 r6, r2
    xor64 r6, r1
lbb_11824:
    mov64 r0, r6
    ja lbb_12020
lbb_11826:
    jeq r3, r0, lbb_11828
    ja lbb_11837
lbb_11828:
    lddw r0, 0x7ff8000000000000
    jeq r5, 0, lbb_12020
    lddw r2, 0x8000000000000000
    and64 r1, r2
    xor64 r1, r6
    mov64 r0, r1
    ja lbb_12020
lbb_11837:
    jeq r5, 0, lbb_12021
    jeq r3, 0, lbb_12021
    mov64 r6, r3
    mov64 r9, 0
    lddw r1, 0x10000000000000
    jgt r1, r5, lbb_11845
    ja lbb_11895
lbb_11845:
    mov64 r3, 64
    jeq r4, 0, lbb_11890
    mov64 r3, r4
    rsh64 r3, 1
    mov64 r1, r4
    or64 r1, r3
    mov64 r3, r1
    rsh64 r3, 2
    or64 r1, r3
    mov64 r3, r1
    rsh64 r3, 4
    or64 r1, r3
    mov64 r3, r1
    rsh64 r3, 8
    or64 r1, r3
    mov64 r3, r1
    rsh64 r3, 16
    or64 r1, r3
    mov64 r3, r1
    rsh64 r3, 32
    or64 r1, r3
    xor64 r1, -1
    lddw r3, 0x5555555555555555
    mov64 r5, r1
    rsh64 r5, 1
    and64 r5, r3
    sub64 r1, r5
    lddw r5, 0x3333333333333333
    mov64 r3, r1
    and64 r3, r5
    rsh64 r1, 2
    and64 r1, r5
    add64 r3, r1
    mov64 r1, r3
    rsh64 r1, 4
    add64 r3, r1
    lddw r1, 0xf0f0f0f0f0f0f0f
    and64 r3, r1
    lddw r1, 0x101010101010101
    mul64 r3, r1
    rsh64 r3, 56
lbb_11890:
    mov64 r9, 12
    sub64 r9, r3
    add64 r3, 53
    and64 r3, 63
    lsh64 r4, r3
lbb_11895:
    lddw r1, 0xfffffffffffff
    jgt r6, r1, lbb_11948
    mov64 r3, 64
    jeq r2, 0, lbb_11943
    mov64 r3, r2
    rsh64 r3, 1
    mov64 r1, r2
    or64 r1, r3
    mov64 r3, r1
    rsh64 r3, 2
    or64 r1, r3
    mov64 r3, r1
    rsh64 r3, 4
    or64 r1, r3
    mov64 r3, r1
    rsh64 r3, 8
    or64 r1, r3
    mov64 r3, r1
    rsh64 r3, 16
    or64 r1, r3
    mov64 r3, r1
    rsh64 r3, 32
    or64 r1, r3
    xor64 r1, -1
    lddw r3, 0x5555555555555555
    mov64 r5, r1
    rsh64 r5, 1
    and64 r5, r3
    sub64 r1, r5
    lddw r5, 0x3333333333333333
    mov64 r3, r1
    and64 r3, r5
    rsh64 r1, 2
    and64 r1, r5
    add64 r3, r1
    mov64 r1, r3
    rsh64 r1, 4
    add64 r3, r1
    lddw r1, 0xf0f0f0f0f0f0f0f
    and64 r3, r1
    lddw r1, 0x101010101010101
    mul64 r3, r1
    rsh64 r3, 56
lbb_11943:
    sub64 r9, r3
    add64 r3, 53
    and64 r3, 63
    lsh64 r2, r3
    add64 r9, 12
lbb_11948:
    lsh64 r2, 11
    lddw r1, 0x8000000000000000
    or64 r2, r1
    lddw r6, 0x10000000000000
    or64 r4, r6
    mov64 r1, r10
    add64 r1, -16
    mov64 r3, 0
    mov64 r5, 0
    call function_9839
    add64 r7, r8
    add64 r7, r9
    ldxdw r2, [r10-0x8]
    mov64 r3, r2
    and64 r3, r6
    ldxdw r1, [r10-0x10]
    jeq r3, 0, lbb_11968
    ja lbb_11981
lbb_11968:
    lsh64 r2, 1
    mov64 r3, r1
    rsh64 r3, 63
    or64 r2, r3
    lsh64 r1, 1
    add64 r7, -1023
    ldxdw r0, [r10-0x18]
    jsgt r7, 2046, lbb_11977
    ja lbb_11984
lbb_11977:
    lddw r1, 0x7ff0000000000000
    or64 r0, r1
    ja lbb_12020
lbb_11981:
    add64 r7, -1022
    ldxdw r0, [r10-0x18]
    jsgt r7, 2046, lbb_11977
lbb_11984:
    mov64 r3, 1
    jsgt r3, r7, lbb_11992
    lddw r3, 0xfffffffffffff
    and64 r2, r3
    lsh64 r7, 52
    or64 r7, r2
    ja lbb_12007
lbb_11992:
    sub64 r3, r7
    jgt r3, 63, lbb_12020
    add64 r7, 63
    lsh64 r7, 32
    rsh64 r7, 32
    mov64 r4, r2
    or64 r4, r1
    lsh64 r4, r7
    lsh64 r3, 32
    rsh64 r3, 32
    rsh64 r1, r3
    or64 r4, r1
    rsh64 r2, r3
    mov64 r1, r4
    mov64 r7, r2
lbb_12007:
    mov64 r2, r7
    or64 r2, r0
    lddw r3, 0x8000000000000000
    jgt r1, r3, lbb_12013
    ja lbb_12015
lbb_12013:
    add64 r2, 1
    ja lbb_12019
lbb_12015:
    mov64 r0, r2
    jne r1, r3, lbb_12020
    and64 r7, 1
    add64 r2, r7
lbb_12019:
    mov64 r0, r2
lbb_12020:
    exit
lbb_12021:
    ldxdw r0, [r10-0x18]
    ja lbb_12020

function_12023:
    mov64 r0, 0
    jeq r1, 0, lbb_12085
    mov64 r3, r1
    rsh64 r3, 1
    mov64 r2, r1
    or64 r2, r3
    mov64 r3, r2
    rsh64 r3, 2
    or64 r2, r3
    mov64 r3, r2
    rsh64 r3, 4
    or64 r2, r3
    mov64 r3, r2
    rsh64 r3, 8
    or64 r2, r3
    mov64 r3, r2
    rsh64 r3, 16
    or64 r2, r3
    mov64 r3, r2
    rsh64 r3, 32
    or64 r2, r3
    xor64 r2, -1
    lddw r3, 0x5555555555555555
    mov64 r4, r2
    rsh64 r4, 1
    and64 r4, r3
    sub64 r2, r4
    lddw r4, 0x3333333333333333
    mov64 r3, r2
    and64 r3, r4
    rsh64 r2, 2
    and64 r2, r4
    add64 r3, r2
    mov64 r2, r3
    rsh64 r2, 4
    add64 r3, r2
    lddw r2, 0xf0f0f0f0f0f0f0f
    and64 r3, r2
    lddw r2, 0x101010101010101
    mul64 r3, r2
    rsh64 r3, 56
    lsh64 r1, r3
    lsh64 r3, 52
    mov64 r2, r1
    rsh64 r2, 11
    mov64 r0, r2
    sub64 r0, r3
    xor64 r2, -1
    lsh64 r1, 53
    mov64 r3, r1
    rsh64 r3, 63
    and64 r3, r2
    sub64 r1, r3
    rsh64 r1, 63
    add64 r0, r1
    lddw r1, 0x43d0000000000000
    add64 r0, r1
lbb_12085:
    exit

function_12086:
    mov64 r5, r4
    and64 r5, 64
    jne r5, 0, lbb_12100
    jeq r4, 0, lbb_12104
    mov64 r5, r4
    and64 r5, 63
    lsh64 r3, r5
    neg64 r4
    and64 r4, 63
    mov64 r0, r2
    rsh64 r0, r4
    or64 r3, r0
    lsh64 r2, r5
    ja lbb_12104
lbb_12100:
    and64 r4, 63
    mov64 r3, r2
    lsh64 r3, r4
    mov64 r2, 0
lbb_12104:
    stxdw [r1+0x0], r2
    stxdw [r1+0x8], r3
    exit

function_12107:
    mov64 r5, r4
    and64 r5, 64
    jne r5, 0, lbb_12122
    jeq r4, 0, lbb_12126
    mov64 r5, r4
    and64 r5, 63
    rsh64 r2, r5
    neg64 r4
    and64 r4, 63
    mov64 r0, r3
    lsh64 r0, r4
    or64 r0, r2
    rsh64 r3, r5
    mov64 r2, r0
    ja lbb_12126
lbb_12122:
    and64 r4, 63
    rsh64 r3, r4
    mov64 r2, r3
    mov64 r3, 0
lbb_12126:
    stxdw [r1+0x0], r2
    stxdw [r1+0x8], r3
    exit

function_12129:
    call function_12384
    exit

compiler_builtins::float::div::__divsf3::he4b30be0a454a8c2:
    mov64 r4, r2
    and64 r4, 8388607
    mov64 r5, r1
    and64 r5, 8388607
    mov64 r3, r2
    xor64 r3, r1
    and64 r3, -2147483648
    mov64 r6, r2
    rsh64 r6, 23
    and64 r6, 255
    mov64 r7, r1
    rsh64 r7, 23
    and64 r7, 255
    mov64 r0, r7
    add64 r0, -255
    mov64 r8, -254
    jgt r8, r0, lbb_12152
    mov64 r0, 0
    mov64 r8, r6
    add64 r8, -255
    jgt r8, -255, lbb_12270
lbb_12152:
    mov64 r9, r1
    and64 r9, 2147483647
    jgt r9, 2139095040, lbb_12162
    mov64 r8, r2
    and64 r8, 2147483647
    jgt r8, 2139095040, lbb_12159
    ja lbb_12165
lbb_12159:
    or64 r2, 4194304
lbb_12160:
    mov64 r0, r2
    ja lbb_12383
lbb_12162:
    or64 r1, 4194304
    mov64 r0, r1
    ja lbb_12383
lbb_12165:
    jeq r9, 2139095040, lbb_12167
    ja lbb_12172
lbb_12167:
    mov64 r0, 2143289344
    jeq r8, 2139095040, lbb_12383
    and64 r2, -2147483648
    xor64 r2, r1
    ja lbb_12160
lbb_12172:
    jeq r8, 2139095040, lbb_12382
    jeq r9, 0, lbb_12380
    jeq r8, 0, lbb_12333
    mov64 r0, 0
    mov64 r1, 8388608
    jgt r1, r9, lbb_12179
    ja lbb_12223
lbb_12179:
    mov64 r1, 32
    jeq r5, 0, lbb_12218
    mov64 r2, r5
    rsh64 r2, 1
    mov64 r1, r5
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 2
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 4
    or64 r1, r2
    mov64 r0, r1
    and64 r0, 8388352
    rsh64 r0, 8
    mov64 r2, r1
    or64 r2, r0
    and64 r1, 8323072
    rsh64 r1, 16
    or64 r2, r1
    xor64 r2, -1
    mov64 r1, r2
    rsh64 r1, 1
    and64 r1, 1431655765
    sub64 r2, r1
    mov64 r1, r2
    and64 r1, 858993459
    rsh64 r2, 2
    and64 r2, 858993459
    add64 r1, r2
    mov64 r2, r1
    rsh64 r2, 4
    add64 r1, r2
    and64 r1, 252645135
    mul64 r1, 16843009
    lddw r2, 0xff000000
    and64 r1, r2
    rsh64 r1, 24
lbb_12218:
    mov64 r0, 9
    sub64 r0, r1
    add64 r1, 24
    and64 r1, 31
    lsh64 r5, r1
lbb_12223:
    jgt r8, 8388607, lbb_12270
    mov64 r1, 32
    jeq r4, 0, lbb_12263
    mov64 r2, r4
    rsh64 r2, 1
    mov64 r1, r4
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 2
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 4
    or64 r1, r2
    mov64 r8, r1
    and64 r8, 8388352
    rsh64 r8, 8
    mov64 r2, r1
    or64 r2, r8
    and64 r1, 8323072
    rsh64 r1, 16
    or64 r2, r1
    xor64 r2, -1
    mov64 r1, r2
    rsh64 r1, 1
    and64 r1, 1431655765
    sub64 r2, r1
    mov64 r1, r2
    and64 r1, 858993459
    rsh64 r2, 2
    and64 r2, 858993459
    add64 r1, r2
    mov64 r2, r1
    rsh64 r2, 4
    add64 r1, r2
    and64 r1, 252645135
    mul64 r1, 16843009
    lddw r2, 0xff000000
    and64 r1, r2
    rsh64 r1, 24
lbb_12263:
    mov64 r2, r1
    add64 r2, 24
    and64 r2, 31
    lsh64 r4, r2
    add64 r1, r0
    add64 r1, -9
    mov64 r0, r1
lbb_12270:
    sub64 r7, r6
    add64 r0, r7
    or64 r4, 8388608
    mov64 r2, r4
    lsh64 r2, 8
    mov64 r6, 1963258675
    sub64 r6, r2
    lsh64 r6, 32
    rsh64 r6, 32
    lsh64 r2, 32
    rsh64 r2, 32
    mov64 r1, r6
    mul64 r1, r2
    rsh64 r1, 32
    neg64 r1
    lsh64 r1, 32
    rsh64 r1, 32
    mul64 r1, r6
    rsh64 r1, 31
    lsh64 r1, 32
    rsh64 r1, 32
    mov64 r6, r1
    mul64 r6, r2
    rsh64 r6, 32
    neg64 r6
    lsh64 r6, 32
    rsh64 r6, 32
    mul64 r6, r1
    rsh64 r6, 31
    lsh64 r6, 32
    rsh64 r6, 32
    mov64 r1, r6
    mul64 r1, r2
    rsh64 r1, 32
    neg64 r1
    lsh64 r1, 32
    rsh64 r1, 32
    mul64 r1, r6
    rsh64 r1, 31
    lddw r2, 0xfffffff4
    add64 r1, r2
    mov64 r2, r5
    or64 r2, 8388608
    mov64 r6, r2
    lsh64 r6, 1
    mov64 r7, r6
    lsh64 r7, 32
    rsh64 r7, 32
    lsh64 r1, 32
    rsh64 r1, 32
    mul64 r1, r7
    rsh64 r1, 32
    mov64 r7, 16777216
    jgt r7, r1, lbb_12336
    rsh64 r1, 1
    mov64 r6, r1
    mul64 r6, r4
    lsh64 r5, 23
    sub64 r5, r6
    add64 r0, 127
    jsgt r0, 254, lbb_12333
    ja lbb_12343
lbb_12333:
    or64 r3, 2139095040
    mov64 r0, r3
    ja lbb_12383
lbb_12336:
    mov64 r2, r4
    mul64 r2, r1
    lsh64 r5, 24
    sub64 r5, r2
    add64 r0, 126
    mov64 r2, r6
    jsgt r0, 254, lbb_12333
lbb_12343:
    jsgt r0, 0, lbb_12345
    ja lbb_12350
lbb_12345:
    and64 r1, 8388607
    lsh64 r0, 23
    or64 r0, r1
    lsh64 r5, 1
    ja lbb_12367
lbb_12350:
    mov64 r5, -23
    jsgt r5, r0, lbb_12382
    mov64 r5, 1
    sub64 r5, r0
    add64 r0, 23
    lsh64 r0, 32
    rsh64 r0, 32
    lsh64 r2, r0
    lsh64 r5, 32
    rsh64 r5, 32
    rsh64 r1, r5
    mov64 r5, r4
    mul64 r5, r1
    lsh64 r5, 1
    sub64 r2, r5
    mov64 r5, r2
    mov64 r0, r1
lbb_12367:
    mov64 r2, r0
    and64 r2, 1
    add64 r2, r5
    lsh64 r2, 32
    rsh64 r2, 32
    lsh64 r4, 32
    rsh64 r4, 32
    mov64 r1, 1
    jgt r2, r4, lbb_12377
    mov64 r1, 0
lbb_12377:
    add64 r0, r1
    or64 r0, r3
    ja lbb_12383
lbb_12380:
    mov64 r0, 2143289344
    jeq r8, 0, lbb_12383
lbb_12382:
    mov64 r0, r3
lbb_12383:
    exit

function_12384:
    mov64 r4, r2
    xor64 r4, r1
    lddw r3, 0x8000000000000000
    and64 r4, r3
    stxdw [r10-0x18], r4
    lddw r4, 0xfffffffffffff
    mov64 r3, r2
    and64 r3, r4
    mov64 r9, r1
    and64 r9, r4
    mov64 r8, r2
    rsh64 r8, 52
    and64 r8, 2047
    mov64 r4, r1
    rsh64 r4, 52
    and64 r4, 2047
    stxdw [r10-0x20], r4
    add64 r4, -2047
    mov64 r5, -2046
    jgt r5, r4, lbb_12410
    mov64 r6, 0
    mov64 r4, r8
    add64 r4, -2047
    jgt r4, -2047, lbb_12556
lbb_12410:
    lddw r0, 0x7fffffffffffffff
    mov64 r5, r1
    and64 r5, r0
    lddw r7, 0x7ff0000000000000
    jgt r5, r7, lbb_12426
    mov64 r4, r2
    and64 r4, r0
    jgt r4, r7, lbb_12421
    ja lbb_12431
lbb_12421:
    lddw r1, 0x8000000000000
    or64 r2, r1
lbb_12424:
    mov64 r0, r2
    ja lbb_12698
lbb_12426:
    lddw r2, 0x8000000000000
    or64 r1, r2
    mov64 r0, r1
    ja lbb_12698
lbb_12431:
    lddw r6, 0x7ff0000000000000
    jeq r5, r6, lbb_12435
    ja lbb_12443
lbb_12435:
    lddw r0, 0x7ff8000000000000
    jeq r4, r6, lbb_12698
    lddw r3, 0x8000000000000000
    and64 r2, r3
    xor64 r2, r1
    ja lbb_12424
lbb_12443:
    jeq r4, r6, lbb_12697
    jeq r5, 0, lbb_12694
    jeq r4, 0, lbb_12648
    mov64 r6, 0
    lddw r1, 0x10000000000000
    jgt r1, r5, lbb_12451
    ja lbb_12501
lbb_12451:
    mov64 r2, 64
    jeq r9, 0, lbb_12496
    mov64 r2, r9
    rsh64 r2, 1
    mov64 r1, r9
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 2
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 4
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 8
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 16
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 32
    or64 r1, r2
    xor64 r1, -1
    lddw r2, 0x5555555555555555
    mov64 r5, r1
    rsh64 r5, 1
    and64 r5, r2
    sub64 r1, r5
    lddw r5, 0x3333333333333333
    mov64 r2, r1
    and64 r2, r5
    rsh64 r1, 2
    and64 r1, r5
    add64 r2, r1
    mov64 r1, r2
    rsh64 r1, 4
    add64 r2, r1
    lddw r1, 0xf0f0f0f0f0f0f0f
    and64 r2, r1
    lddw r1, 0x101010101010101
    mul64 r2, r1
    rsh64 r2, 56
lbb_12496:
    mov64 r6, 12
    sub64 r6, r2
    add64 r2, 53
    and64 r2, 63
    lsh64 r9, r2
lbb_12501:
    lddw r1, 0xfffffffffffff
    jgt r4, r1, lbb_12556
    mov64 r2, 64
    jeq r3, 0, lbb_12549
    mov64 r2, r3
    rsh64 r2, 1
    mov64 r1, r3
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 2
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 4
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 8
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 16
    or64 r1, r2
    mov64 r2, r1
    rsh64 r2, 32
    or64 r1, r2
    xor64 r1, -1
    lddw r2, 0x5555555555555555
    mov64 r4, r1
    rsh64 r4, 1
    and64 r4, r2
    sub64 r1, r4
    lddw r4, 0x3333333333333333
    mov64 r2, r1
    and64 r2, r4
    rsh64 r1, 2
    and64 r1, r4
    add64 r2, r1
    mov64 r1, r2
    rsh64 r1, 4
    add64 r2, r1
    lddw r1, 0xf0f0f0f0f0f0f0f
    and64 r2, r1
    lddw r1, 0x101010101010101
    mul64 r2, r1
    rsh64 r2, 56
lbb_12549:
    mov64 r1, r2
    add64 r1, 53
    and64 r1, 63
    lsh64 r3, r1
    add64 r2, r6
    add64 r2, -12
    mov64 r6, r2
lbb_12556:
    lddw r1, 0x10000000000000
    mov64 r7, r9
    or64 r7, r1
    mov64 r2, r3
    or64 r2, r1
    stxdw [r10-0x28], r2
    mov64 r1, r2
    rsh64 r1, 21
    mov64 r4, 1963258675
    sub64 r4, r1
    lsh64 r1, 32
    rsh64 r1, 32
    lsh64 r4, 32
    rsh64 r4, 32
    mov64 r2, r4
    mul64 r2, r1
    rsh64 r2, 32
    neg64 r2
    lsh64 r2, 32
    rsh64 r2, 32
    mul64 r2, r4
    rsh64 r2, 31
    lsh64 r2, 32
    rsh64 r2, 32
    mov64 r4, r2
    mul64 r4, r1
    rsh64 r4, 32
    neg64 r4
    lsh64 r4, 32
    rsh64 r4, 32
    mul64 r4, r2
    rsh64 r4, 31
    lsh64 r4, 32
    rsh64 r4, 32
    mov64 r2, r4
    mul64 r2, r1
    rsh64 r2, 32
    neg64 r2
    lsh64 r2, 32
    rsh64 r2, 32
    mul64 r2, r4
    lsh64 r3, 11
    lsh64 r3, 32
    rsh64 r3, 32
    rsh64 r2, 31
    add64 r2, -1
    lsh64 r2, 32
    rsh64 r2, 32
    mov64 r4, r2
    mul64 r4, r1
    mov64 r1, r2
    mul64 r1, r3
    rsh64 r1, 32
    add64 r4, r1
    mov64 r1, 1
    sub64 r1, r4
    mov64 r3, r1
    lsh64 r3, 32
    rsh64 r3, 32
    mul64 r3, r2
    rsh64 r1, 32
    mul64 r2, r1
    lsh64 r2, 1
    rsh64 r3, 31
    add64 r2, r3
    add64 r2, -225
    stxdw [r10-0x30], r7
    lsh64 r7, 1
    mov64 r1, r10
    add64 r1, -16
    mov64 r3, 0
    mov64 r4, r7
    mov64 r5, 0
    call function_9839
    ldxdw r0, [r10-0x20]
    sub64 r0, r8
    add64 r0, r6
    ldxdw r1, [r10-0x8]
    lddw r2, 0x20000000000000
    jgt r2, r1, lbb_12654
    rsh64 r1, 1
    mov64 r2, r1
    ldxdw r3, [r10-0x28]
    mul64 r2, r3
    lsh64 r9, 52
    sub64 r9, r2
    add64 r0, 1023
    ldxdw r7, [r10-0x30]
    jsgt r0, 2046, lbb_12648
    ja lbb_12661
lbb_12648:
    lddw r1, 0x7ff0000000000000
    ldxdw r6, [r10-0x18]
    or64 r6, r1
    mov64 r0, r6
    ja lbb_12698
lbb_12654:
    ldxdw r3, [r10-0x28]
    mov64 r2, r3
    mul64 r2, r1
    lsh64 r9, 53
    sub64 r9, r2
    add64 r0, 1022
    jsgt r0, 2046, lbb_12648
lbb_12661:
    jsgt r0, 0, lbb_12663
    ja lbb_12670
lbb_12663:
    lddw r2, 0xfffffffffffff
    and64 r1, r2
    lsh64 r0, 52
    or64 r0, r1
    lsh64 r9, 1
    ja lbb_12684
lbb_12670:
    mov64 r2, -52
    jsgt r2, r0, lbb_12697
    mov64 r2, r0
    add64 r2, 52
    lsh64 r7, r2
    mov64 r2, 1
    sub64 r2, r0
    rsh64 r1, r2
    mov64 r2, r3
    mul64 r2, r1
    lsh64 r2, 1
    sub64 r7, r2
    mov64 r9, r7
    mov64 r0, r1
lbb_12684:
    mov64 r2, r0
    and64 r2, 1
    add64 r2, r9
    mov64 r1, 1
    jgt r2, r3, lbb_12690
    mov64 r1, 0
lbb_12690:
    add64 r0, r1
    ldxdw r1, [r10-0x18]
    or64 r0, r1
    ja lbb_12698
lbb_12694:
    lddw r0, 0x7ff8000000000000
    jeq r4, 0, lbb_12698
lbb_12697:
    ldxdw r0, [r10-0x18]
lbb_12698:
    exit