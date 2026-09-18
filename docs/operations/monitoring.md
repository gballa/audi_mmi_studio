# Operations — Monitoring & Diagnostics

This document outlines methods for observing update execution in real time and capturing low-level diagnostic logs via hardware UART serial consoles.

---

## 1. Real-Time On-Screen Telemetry

During update execution, the QNX installer displays module-by-module status indicators:
- `[-]`: Module not selected or checksum skipped.
- `[>]`: Module currently flashing.
- `[OK]`: Module successfully written and verified against CRC32 hash.
- `[ERR]`: Checksum failure or write error. Flashing halts immediately.

---

## 2. Low-Level UART Serial Monitoring

For bench testing and deep diagnostic observation, the Audi MMI 3G main unit exposes a 3.3V TTL UART interface via the Quadlock connector.

### Hardware Connection Parameters
- **Baud Rate**: `115200`
- **Data Bits**: `8`
- **Parity**: `None`
- **Stop Bits**: `1`
- **Flow Control**: `None`

### Pinout (Harman/Becker MU9411 Quadlock Blue Connector)
- **Pin 5**: `UART_TX` (Connect to Serial Adapter RX)
- **Pin 7**: `UART_RX` (Connect to Serial Adapter TX)
- **Pin 11**: `GND` (Connect to Serial Adapter Ground)

### Observing QNX Boot Logs
Connect using `picocom`, `minicom`, or `screen`:
```bash
picocom -b 115200 /dev/tty.usbserial-XXXXX
```
The console prints kernel boot messages, driver loading sequences, and detailed error codes during flash update execution.
