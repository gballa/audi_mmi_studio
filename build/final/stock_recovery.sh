#!/bin/sh
# stock_recovery.sh — Emergency Baseline Recovery Script for Audi MMI 3G+
echo "[1/4] Remounting /mnt/efs-system read-write..."
mount -uw /mnt/efs-system || exit 1
echo "[2/4] Restoring stock baseline..."
if [ -d /mnt/efs-system/backup/stock ]; then
    cp -rf /mnt/efs-system/backup/stock/* /mnt/efs-system/
fi
echo "[3/4] Synchronizing NAND flash blocks..."
sync
sync
echo "[4/4] Triggering reboot..."
sleep 1
shutdown -S
