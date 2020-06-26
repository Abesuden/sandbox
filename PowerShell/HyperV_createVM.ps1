# Prompt user for PSSession
Write-Host "Enter Hyper-V session:"
$re_pssession = Read-Host

# Connect to PSSession
Enter-PSSession $re_pssession

# Prompt user for VM details
Write-Host "Enter VM name:"
$re_serverName = Read-Host
Write-Host "Is this a 32 or 64 bit machine:"
$re_sysInfo_bits = Read-Host
if ($re_sysInfo_bits -eq 64) {$re_generation = 2} else {$re_generation = 1}
Write-Host "Enter ram size in GBs:"
$re_gigabs = Read-Host
$re_ramSize = "" + $re_gigabs + "gb"
$virtualHardDiskpath = "" + $re_serverName + ".vhdx"
Write-Host "Enter storage size in GBs:"
$re_gbytes = Read-Host
$re_memSize = $re_gbytes * 1000000000     # mutliplies to find the input parameter memory size

# Spin up new VM
New-VM -name $re_serverName -generation $re_generation -MemoryStartupBytes $re_ramSize -newvhdpath $virtualHardDiskpath -NewVHDSizeBytes $re_memSize

# Add the VM .iso
Add-VMDvdDrive -VMName $re_serverName

# Get info from the added VM
$VM_infoObject = Get-VMDvdDrive -VMName $re_serverName
$conNum = $VM_infoObject.ControllerNumber[0]
$conLoc = $VM_infoObject.ControllerLocation[0]

# Mount the .iso
Write-Host "Enter the .iso file path to mount to:"
$re_isoFilePath = Read-Host
Set-VMDvdDrive -Path $re_isoFilePath -ControllerNumber 0 -ControllerLocation 1 -VMName $re_serverName

# Start up the VM
Start-VM -Name $re_serverName