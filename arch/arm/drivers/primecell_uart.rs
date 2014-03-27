struct UARTRegisters {
    DR       : u32, // Data register, UARTDR on page 3-5
    RSR_ECR  : u32, // Receive status register/error clear register, UARTRSR/UARTECR on page 3-6
    reserved1: [u8, ..0x18-4-4],
    FR       : u32, // Flag register, UARTFR on page 3-8
    reserved2: u32,
    ILPR     : u32, // [0x20] IrDA low-power counter register, UARTILPR on page 3-9
    IBRD     : u32, // Integer baud rate register, UARTIBRD on page 3-10
    FBRD     : u32, // Fractional baud rate register, UARTFBRD on page 3-10
    LCR_H    : u32, // Line control register, UARTLCR_H on page 3-12
    CR       : u32, // Control register, UARTCR on page 3-15
    IFLS     : u32, // Interrupt FIFO level select register, UARTIFLS on page 3-17
    IMSC     : u32, // Interrupt mask set/clear register, UARTIMSC on page 3-17
    RIS      : u32, // Raw interrupt status register, UARTRIS on page 3-19
    MIS      : u32, // Masked interrupt status register, UARTMIS on page 3-20
    ICR      : u32, // Interrupt cler register, UARTICR on page 3-21
    DMACR    : u32, // DMA control register, UARTDMACR on page 3-22
    reserved3: [u8, ..0xFE0-0x048-4],
    PeriphID0: u32, // [0xFE0] UARTPeriphID0 register on page 3-23
    PeriphID1: u32, // UARTPeriphID1 register on page 3-24
    PeriphID2: u32, // UARTPeriphID2 register on page 3-24
    PeriphID3: u32, // UARTPeriphID3 register on page 3-24
    PCellID0 : u32, // UARTPCellID0 register on page 3-25
    PCellID1 : u32, // UARTPCellID1 register on page 3-26
    PCellID2 : u32, // UARTPCellID2 register on page 3-26
    PCellID3 : u32, // UARTPCellID3 register on page 3-26
}

struct UART {
    registers: *mut UARTRegisters,    
}

impl UART {
    
}

struct PICRegisters {
    irq_status    : IntSource, // IRQ status register
    fiq_status    : IntSource, // FIQ status register
    raw_intr      : IntSource, // Raw interrupt status register
    int_select    : IntSource, // Interrupt select register
    int_enable    : IntSource, // Interrupt enable register
    int_en_clear  : IntSource, // Interrupt enable clear register
    soft_int      : IntSource, // Software interrupt register
    soft_int_clear: IntSource, // Software interrupt clear register
    protection    : u32,       // Protection enable register
    // ...
}

define_flags!(IntSource: u32 {
    UART0 = 1 << 12,
    UART1 = 1 << 13,
    UART2 = 1 << 14,
})

static PIC: *mut PICRegisters = 0x10140000 as *mut PICRegisters;

// http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0181e/I1006637.html
