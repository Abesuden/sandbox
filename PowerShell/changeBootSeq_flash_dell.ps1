Import-Module DellBIOSProvider
cd DellSmbios:\BootSequence
dir .\BootSequence | Select-Object -ExpandProperty currentvalue
# change boot sequence, enter DeviceNumber(s) in the order you want them to boot
si .\BootSequence "2"