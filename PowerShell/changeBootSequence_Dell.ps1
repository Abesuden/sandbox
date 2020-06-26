# import the Dell provider
Import-Module DellBIOSProvider

# change into the Dell provider
cd DellSmbios:\BootSequence

# list the boot sequence and its order
dir .\BootSequence | Select-Object -ExpandProperty currentvalue

# change boot sequence, enter DeviceNumber(s) in the order you want them to boot
si .\BootSequence "2,0,4,3"
#                 '--.----'
#                     `-----> any enlisted DeviceNumber(s) will be automatically concatenated to the end and effectively moved down the boot sequence.

# verify the boot sequence order
dir .\BootSequence | Select-Object -ExpandProperty currentvalue