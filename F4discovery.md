# LEDs

- North: Orange =>  PD13
- East:  Red    =>  PD14
- South: Blue   =>  PD15
- East:  Green  =>  PD12

# レジスタベースアドレス

- RCC:      `0x4002_3800`
- GPIOA:    `0x4002_0000`
- GPIOB:    `0x4002_0400`
- GPIOC:    `0x4002_0800`
- GPIOD:    `0x4002_0C00`

## GPIOレジスタ

- MODER:    オフセット `0x00`
    - `00`: 入力（リセット値）
    - `01`: 汎用出力モード
    - `10`: オルタネートモード
    - `11`: アナログモード
- OTYPER:   オフセット `0x04`
    - OT: `bit 15-0`:
        - `1`: 出力プッシュプル（リセット値）
        - `0`: 出力オープンドレイン
- PURDR:    オフセット `0x0C`
    - `00`: なし
    - `01`: プルアップ
    - `10`: プルダウン
    - `11`: 予約済み
- IDR:      オフセット `0x10`
    - IDR: `bit 15-0`:
- ODR:      オフセット `0x14`
    - ODR: `bit 15-0`:
- BSRR:     オフセット `0x18`
    - BR: `bit 31-16`:`1`でリセット
    - BS: `bit 15-0`: `1`でセット