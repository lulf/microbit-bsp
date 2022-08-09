MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  MBR                               : ORIGIN = 0x00000000, LENGTH = 4K
  SOFTDEVICE                        : ORIGIN = 0x00001000, LENGTH = 114688
  FLASH                             : ORIGIN = 0x0001C000, LENGTH = 256K
  RAM                               : ORIGIN = 0x2000baa8, LENGTH = 83288
}
