# Prompt for searching local host
Write-Host "Would you like to include the local host in the search (yes/no):"
$re_isIncludeHost = Read-Host
$re_includeHost = "localhost"
if ($re_isIncludeHost -eq "no") {$re_includeHost = ""} elseif ($re_isIncludeHost -eq "n") {$re_includeHost = ""}

# Prompt user to add networked machines
Get-PSSession
Write-Host "Enter desired hostnames seperated by a comma, such as 'srv1, srv2':"
$re_includedHosts = Read-Host
$re_finalHostList = $re_includeHost + ", " + $re_includedHosts

# get system info on remote and/or/excluding local computer
Get-WmiObject -ComputerName $re_finalHostList -Class win32_bios     # is win32_bios extendable?