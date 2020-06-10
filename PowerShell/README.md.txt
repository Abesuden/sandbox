## Power Commands
  - Get-Command -noun nameHere
  - Sort-Object
  - Get-Help
  - Format-List
  - Select-Object

## generateDirIntegrity.ps1

This tool is used to create the original directory integrity hash value

```
Get-FileHash * | Out-File ../dirIntegrity.log
```

## remoteSysInfo.ps1

This tool is used to look up all desired networked computers' system information. This tool will also work for just the localhost if desired.

```
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
```

## HyperV_createVM.ps1

This tool is used to spin up VM's with Microsofts Hyper-V virtualization plateform on Windows servers or on Windows 10 Pro/Enterprise editions.

```
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
```

Cmdlet for starting a VM

```
Start-VM -Name vmName
```
> taken from [here](https://youtu.be/iopzjWsLZM4) with gui (here)[https://youtu.be/_liDPu7zz3I] and documentation [here](https://docs.microsoft.com/en-us/virtualization/hyper-v-on-windows/about/)

## activeApp.ps1

```
$y = Get-Process notepad
$y.ID | Sort-Object -Property StartTime -Descending | Out-File activeApp.txt
```

## Services.ink

```
services.msc
```

## showPATH.ps1

```
ls Env:\Path
echo .
ls Env:\PATHEXT
```

## flipMouseWheel_regEdit.ps1

```
cd HKLM:
cd .\SYSTEM\CurrentControlSet\Enum\HID\
# cd into mouse hardware ID
# create or flip bit for "flipFlopWheel" from 0 to 1
```

## changeBootSeq_flash_dell.ps1

```
Import-Module DellBIOSProvider
cd DellSmbios:\BootSequence
dir .\BootSequence | Select-Object -ExpandProperty currentvalue
# change boot sequence, enter DeviceNumber(s) in the order you want them to boot
si .\BootSequence "2"
```

## changeBootSequence_Dell.ps1

```
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
```

--------------                                                       --------------------
            -----------------------------------------------------------
--------------                                                       --------------------

# PowerShell Commands

These are some PowerShell commands that I have learned:
> Note: a list of cmdlet verbs can be found (here)[https://docs.microsoft.com/en-us/powershell/scripting/developer/cmdlet/approved-verbs-for-windows-powershell-commands?view=powershell-7] and the open source github for PowerShell can be found [here](https://github.com/powershell/powershell)


<details>
<summary>>> Useful Tips and Knowledge<<</summary>

---

## Autocomplete Command / Cycle Through cmdlets

Pressing the keybord {TAB} key during the middle of writing the cmdlet allows for autocompletion of the cmdlet. This autocompletion guesses what you may be trying to type and it may be wronge. However, by pressing {TAB} key again, ConselHost will start to cycle through relevent cmdlets. By pressing and holding {SHIFT} key and then pressing the {TAB} key, the ConselHost will start to cycle backwords. This also works if you type the `-` character and start tab to cycle through parameters.

```
{TAB}
```
> autocomplete / cycle cmdlets

```
{SHIFT} + {TAB}
```
> cycle cmdlets backwards

## F7 Key

The F7 key is useful to show a graphical dipiction of you command history

## cmdlet Naming Structure

There is a naming structure associated with each command and it is as follows:

```
Verb-Noun
```

Because of this, you can filter cmdlets, like searches, by using the `-Noun` or `-Verb`  parameters. Below is an example of a use case:

```
Get-Command -Noun history
```

## Figuring Out Commands

A useful way of getting to know commands in PowerShell is to find useful commands. Using the below commands are very helpful for doing this:

```
Get-Command -Noun History
```

```
Get-Command -Verb Get
```

## Get Help

Similar to the `commandName --help` in the Linux terminal and the `commandName /?` in cmd, `Get-Help cmdletName` gets more information on a specific cmdlet in PowerShell. Part of the displayed help is a section called SYNTAX and it will give the [[-parameterName] <datatype>] to help with using the cmdlet.
> keep in mind you will have to update the `Get-Help` cmdlet
> also,  `help` is not the alias of `Get-Help`, rather it is a function and acts like `more` in the Linux terminal. It will break the info into pages.

Some more useful tips following the parameter for a cmdlet are:

```
Get-Help histoy -online
```
> this will open the most updated help guide from Microsoft.com and will open in your default browser.


```
Get-Help histoy -Full
```
> this will display all help information on the cmdlet, just like `man commandName` in the Linux terminal.


```
Get-Help histoy -ShowWindow
```
> this will open the full help page in a seperate PowerShell help window.

```
Get-Help Get-EventLog -Example
```
> displays examples of how to use the `Get-EventLog` cmdlet

```
Get-Help about_*powershell* -ShowWindow
```
> returns all powershell related about files. About files are very useful for gaining an even deeper understanding then the `-full` parameter. This will also open in a new window but it is not required.


## A Quick Tell If A cmdlet Requires Parameters

If there is only one open square bracket before the parameter, then it is required. If there are two open brackets befor the parameter, then it is not required.

```
Get-EventLog [-ComputerName <string[]>]
             ^--- only one
```
> required

```
Get-ChildItem [[-Filter] <string>]
              ^^--- two
```
> NOT required

## Syntax Parameter Sets

Parameter sets are different ways to use the cmdlet. Notice below that there are two `Get-EventLog` entries. It is as though there are two different paths you can follow that are both called the same name. They lead to different places but start at the same spot.

For this example, the first `Get-EventLog` is used to get specific logs on the local machine or on networked machines. The second `Get-EventLog` is used to get all logs on the local machine.

```
Syntax
    Get-EventLog [-LogName] <string> [[-InstanceId] <long[]>] [-ComputerName <string[]>] [-Newest <int>] [-After <datetime>] [-Before <datetime>] [-UserName <string[]>] [-Index <int[]>] [-EntryType {Error | Information | FailureAudit | SuccessAudit | Warning}] [-Source <string[]>] [-Message <string>] [-AsBaseObject]  [<CommonParameters>]

    Get-EventLog [-ComputerName <string[]>] [-List] [-AsString]  [<CommonParameters>]
```

## Switch Parameter

A switch parameter is a parameter that takes no input, for instance notice the `-list` parameter in the below parameter set:

```
Get-EventLog [-ComputerName <string[]>] [-List] [-AsString] [<CommonParameters>]
```
> if you add `-list` then it is on, else if you don't add `-list` then it is off. Always add a switch parameter at the end of the command.


## Parameter Information (PowerShell v5)

```
Parameter
    -List <SwitchParameter>

        Required?                    false
        Position?                    Named
        Accept pipeline input?       false
        Parameter set name           List
        Aliases                      None
        Dynamic?                     false
```

Above, you can see a parameter from the help menu. You can also see different aspects listed out below the parameter, such as if it is required in the cmdlet or not. Let's break down how each line works:

- Required?
    - true = must include this parameter in the cmdlet
    - false = do not need to include this parameter in the cmdlet
- Position?
    - Named = have to type the name in order for the parameter to be useable. This is normally true for parameters like switch parameters (ie. this example)
    - <int> = a number (ie. 4) that represents where in the command the parameter can be placed (ie. commandName 1 2 3 4 5 ...) without typing the flag (ie. commandName -flag).
        - With position, `Get-EventLog -LogName system` can be shortened to `Get-EventLog system` becuase the position for the -LogName parameter is 1.
- Default value
    - 
- Accept pipeline input?
    - Defines whether it takes input from a pipe (ie. `|`)
    - true = allows piping input
    - false = does not allow piping input
- Accept wildcard characters?
    - true = allows the use of the `*` character
    - false = cannot use the `*` character. So you have to specify what you are looking for.
- Parameter set name
    - Name of the parameter
- Aliases
    - This will list all known aliases
- Dynamic?
    - Either a true or false value

## Positional Parameters

You can utilize the positional parameters as stated above. For example, `Get-EventLog -LogName system` can be shortened to `Get-EventLog system` becuase the position for the -LogName parameter is 1. However, keep in mind that if you try to export a script and use it on another machine, there is a small chance the positional parameter could be different. A good way to avoid that pit fall is with named parameters, `Get-EventLog -LogName system`. The beauty of named parameters is that you can also override positional constrants. So, if you wanted to use a positional parameter with a position of 2 before a positional parameter with a position of 1, then you can.

## PowerShell Provider

A provider is a .NET program that allows you to work with data stores in Windows PowerShell as if they were mounted disk drives. This includes such things as:

- Active Directory
- Remote Computers
- ect.

Use this command to get more info:

```
help about_Providers -ShowWindow
```

## PowerShell Objects

Everything in PowerShell is a data object. This means that using the pipe "`|`" symbol can transfer an object into another cmdlet.

## Trouble Shoot Pipeline Commands

When you run into a problem, remove whatever is at the end of the last pipe and replace with `gm`. For example:

```
Get-Service | Select-Object -Property status,name -ExpandProperty | Group-Object -Property status | gm
```

## Creating Good Pipeline Command Structures

You want to do the sorting before the formatting because formatting mutates the PS object. Think of formatting as the end of the command or second to last when using `Out-` cmdlets. Use the following command to understand more:

```
help About_Pipelines
```

## .Net Programming Inside PowerShell with Add-Type

PowerShell is tied with the Microsoft .Net framework and can take in C# and compile the code using the cmdlet ([Add-Type])(https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/add-type?view=powershell-6). Type this command for more information

```
Get-Help Add-Type -ShowWindow
```

## Multi-Line Commands In PowerShell's Shell

In order to have multiple lines for a given command in the shell environment, not the IDE, you will need to use the tick character `'`.

## WSMan Provider

Vendor neutral web intrernet intranet freindly protocols called Web Services for Management and is not owned by Microsoft. The WSMan protocols listen on one or to two ports (tcp) in the 5900 range depending if it is HTTP (5985) or HTTPS (5986) which makes it firewall freindly. Allows for PowerShell remoting to control one to many (contected in parallel) or one to one senerios. 

## WinRM

Windows Remote Management is the Mircosoft proprietary system service that runs int the background on the domain computers that support the WSMan protocols. WSMan is the protocol and WinRM is the "engine." 

## PSSession

Whenever you are working in PowerShell and remoting to other machines, you should be in administrator mode.

## Enable Remoting for PowerShell

You will need to use the `Enable-PSRemoting` cmdlets. Keep in mind, enabeling remoting will make firewall exceptions for specific ports (5985, 5986) to allow for WSMan to work. By default, the cmdlet will do the following:
 
  1. Starting or restarting the WinRM service
  2. Setting the WinRM service startup type to Automatic
  3. Creating a listener to accept requests on any IP address
  4. Enabling Windows Firewall inbound rule exceptions for WS-Management traffic (for http only)
> notice that by default, only http is set up and not https. Https requires more configuration like setting up digital certificates.

## Windows WMI Cmdlet

PowerShell allos you to conncet, through wmi, to an interface into Windows system configuration. An example cmdlet is:

```
Get-WmiObject
```

## Parentheses

Parentheses tell PowerShell to do the commands inside first, similar to order of opperations in math. For example:

```
4 + (6 - 2)
```
> 6 - 2 is evaluated first

```
Get-Command -Name (Get-Content -Path C:\commands.txt)
```
> `Get-Content -Path C:\commands.txt` is run first and then `Get-Command`

## PowerShell Modules

You can make a PowerShell module by saving with the `.psm` extension. The module file will need to be placed in the modules folder and inside a new folder with the same name as the module name that you just created. That will allow for autoloading the module.

## Condense PowerShell Commands

Here is a list of a few ways to make one liners or just shorten the PowerShell command:

  - Use the ';' character to split the statement
  - Shorten a parameter, only to the point where it can be mistaken for no other parameter
    - `Get-Commmand -CommandType cmdlet` --> `Get-Command -Co cmdlet` --> `gcm -co cmdlet`
  - Use aliases that are standard across machines

## File Types of PowerShell

These are different file types you will see in PowerShell:

  - .ps1
    - a PowerShell script
  - .dll
    - a PowerShell v1 way of making modules called SnapIns
  - .psm
    - a PowerShell module file
  - .psd1
    - 
  - .psm1
    - 
  - .Format.ps1xml
    - defines the way the properties will be displayed, whether it is a table, list, ect.

## Escape Character

Normally, in programming the excape character is a back-slash but in PowerShell, it is the backtick (aka. the grave accent) '`' which is found under the Esc key.

## Regular Expressions

PowerShell does support Regular Expressions, but there is no documentation here yet for that.

## Change Default Application

By default, PowerShell opens a `.ps1` script in notepad when a user double clicks. However, this can be changed by right clicking and selecting properties, then change the "Open with:" to the powershell.exe typically located: `C:\Windows\SysWOW64\WindowsPowerShell\v1.0\powershell.exe`.

## OneGet Package Manager

Just like in Linux with its `apt-get`, Microsoft has implimented `OneGet` to download and install applications in PowerShell v5. This works for not just PowerShell applications but also other software as well.

---

</details>


```
Get-Help
```
> get help with a cmdlet

```
Update-Help
```
> update the `Get-Help` command

<details>
<summary>>> for air gapped computers <<</summary>

---

Use `Update-Help` first before saving to ensure the most updated files.

```
Save-Help
```
> save the current `Get-Help` files to a removable media to update computers off a network

---

</details>

```
gcm -verb Ge*
```
> prints all commands that start with Ge [Get-Command]

```
New-Item newFile.txt
```
> this is `touch` in Unix

```
explorer .
```
> open current directory in file explorer

```
$PSVersionTable
```
> displays info about the PowerShell version

```
Get-Host |  Select-Object-Object Version
```
> another way to display PowerShell`s version

```
powershell -version 4
```
> opens a PowerShell in version specified (4 in this case)

```
New-Alias -Name "python3" -Value py
```
> creates a new alias for the current session

<details>
<summary>>> make lasting aliases <<</summary>

---

In order to make an alias that is persistant, then you will need to add it to the PS profile. This profile has to already have been created. To get to it, you can us the following command:

```
notepade $PROFILE
```

The profile is the text file that holds the commands that get executed every time a PS session is started. Thus, the alias is getting set at the start of every session. Add the command below to the bottom of the profile text file and save:

```
New-Alias -Name "aliasName" -Value commandName
```

---

</details>

```
cd ~
```
> brings back to the home directory

```
alias |  Select-Object-String echo
```
> similar to `grep` on Unix

```
Get-Alias echo
```
> return what `echo` is aliased for

```
Write-Host "Hello World!"
```
> similar to `echo`

```
Get-Content fileName.txt
```
> use like `cat fileName.txt` in the Linux terminal, the shortcut is `type`

```
$host
```
> it is a variable in PS and shows the PS engine (ie. ConHost or another tool)

```
Get-Alias | Sort-Object
```
> sorts the list piped to it

```
Get-History
```
> displays command history

```
Clear-History
```
> use this cmdlet to clear command history or just open a new PS session

```
Get-Eventlog system
```
> Allows you to get logs from Windows like in event viewer. This one specifically shows the system logs.

```
Get-Eventlog -list
```
> displays all logs on the computer that can be looked at

```
Invoke-History
```
> runs commands from the session history

```
$PSVersionTable
```
> displays the version of PS

```
$PSVersionTable.PSVersion
```
> displays the host variable and the PSVersion field

```
AddCommand
```
> used to add commands to the pipeline

```
AddParameter
```
> Assists in adding parameters to the chosen command

```
AddStatement
```
> used to add an additional statement to the end of the pipeline

```
Get-Alias -Name dir
```
> get the alias by name (ie. `dir`)

```
Get-Alias -Definition Get-ChildItem
```
> returns all the aliases for `Get-ChildItem`

```
Get-Verb | Sort-Object -Property verb
```
> `Sort-Object` is used by PS to order objects (ie. lists, tables, ect.) and in this case the sort is by the property verb which is a table header. Also note, `Sort-Object` sorts in ascending order by default.

```
Get-Verb | Measure-Object
```
> by default, displays things like count, sum, average, ect.

```
 Get-PSProvider
```
> lists the PS Providers

```
Import-Module ActiveDirectory
```
> imports the active directory provider? But also shifts into that provider (look at the current drive to the left of the cursor)

```
Import-Module sqlps
```
> imports the SQL provider? But also shifts into that provider (look at the current drive to the left of the cursor)

```
cd SQLSERVER:
```
> shift provider, can work with `FileSystem` and so forth

```
Resolve-Path ~
```
> resolve home path of provider

```
Get-PSDrive
```
> gives another view for providers

```
Get-Command -noun item
```
> items are useful for PS drives

```
Get-Command -noun ItemProperty
```
> ItemProperty is used for regedit.exe registry keys values

```
ItemProperty -Path Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Run
```
> Useful for seeing what starts up automagically anything above the PSPath entry (good for malware detection). 

```
Remove-ItemProperty .\keyName -whatif
```
> removes the registry entry and the `-whatif` was taked on just for testing if the command would work

```
ls Env:\Path
```
> shows the current windows PATH variable

```
ls Env:\PATHEXT
```
> shows the types of executables for the current user

```
$varName = Get-ChildItem function:help
```
> this is how to make a variable in PS

```
$varName.Definition
```
> gives the underlying code of a Function (bottom of page)

```
Get-Service
```
> background processes that allow your computer to do what it does

```
Get-Service | Format-List
```
> the default output in PS is a table format, but that is limited to display 5 columns. So, use the `Format-List` to display in list format.

```
Get-Service | Format-List *
```
> shows everything so you can see a broader view

```
Get-Process | Get-Member
```
> see what it is and what it does. Notice the TypeName's and the Class name (after the last '.'). Each item listed below is an active instance of that object.

```
Get-Command -noun *object
```
> see all useful "object" commands

```
Get-Service | Select-Object -Property status,name
```
> select properties (column names) we want to see

```
Get-Service | Select-Object -Property status,name | Group-Object -Property status
```
> see the group column, this is a hash table. Do the next command to expand the table

```
Get-Service | Select-Object -Property status,name -ExpandProperty Name | Group-Object -Property status | Format-List
```
> results in expanded PS hash table, and you can keep going

```
Get-Process | Out-File process.txt
```
> outputs to a file

```
Get-Process | Out-Printer
```
> send file to print on machines default printer

```
Get-Process | Out-Printer -Name \\server\printqueue
```
> send file to print on network printer

```
Get-Process | Out-Printer -Name "Microsoft Print to PDF"
```
> make the file a .pdf

```
Get-Process | Out-GridView
```
> pop up window for a grid view

```
Show-Command Get-EventLog
```
> shows each cmdlet parameter set

```
Get-Content -Path .\fileName.txt | Stop-Service
```
> pulls in the content as an object and then pipes into the `Stop-Service` cmdlet with parameter binding

```
Import-CSV .\data.csv
```
> import a .csv file as a PS object (gm shows a PSCustomObjet)

```
Get-Module
```
> shows the loaded modules

```
Get-Module -ListAvailable
```
> shows all that can be used

```
Get-Command -module applocker
```
> 1st time, loads in the available module, 2nd time can use to see available cmdlets and/or functions

```
Remove-Module applocker
```
> remove a loaded module

```
Get-Help * | Where {$_.synopsis-match "path"} | ft name.synopsis -auto
```
> returns anything in help that has the name path

```
10 -gt 20
```
> greater than comparison operator, returns true/false

```
10 -ge 20
```
> greater than or equal to comparison operator, returns true/false

```
10 -lt 20
```
> less than comparison operator, returns true/false

```
10 -le 20
```
> less than or equal to comparison operator, returns true/false

```
15 -eq 10 + 5
```
> equals comparison operator, returns true/false

```
15 -ne 10 + 5
```
> NOT equals comparison operator, returns true/false

```
"hello" -eq "hello"
```
> equals comparison operator with strings, returns true/false

```
Get-Help about_*
```
> look for expressions and comparison operators because there are more then in this documentation.

```
Where-Object
```
> selects object with certain property objects and used to check IDs and Dates

```
Get-Process | Where-Object {$_.priorityclass - eq "normal"}
```

```
Get-Process | Where-Object PriorityClass -eq "normal"
```
> this is a script blocks with all processes running as "normal"

```
$num = 10
if ($num -eq 10) {Write-Host "It is 10"}
```
> If statement in PS

```
$num = 10
if ($num -eq 10) {Write-Host "It is 10"} else {Write-Host "It is not 10"}
```
> If Else statement in PS

```
$num = 11
if ($num -eq 10) {Write-Host "It is 10"} elseif ($num -eq 11) {Write-Host "It is 11"} else {Write-Host "It is not 10"}
```
> If ElseIf Else statement in PS

```
switch((Get-Date).dayofweek) {'         # start of switch
"saturday" {Write-Host "Sports Day"}'   # condition one
"sunday" {Write-Host "Day Off"}'        # condition two (no fall through)
default {Write-Host "Work Day"}'        # default case if no other cases are meet
}

```
> switch statements in PS, note adding the tick tells the PS shell environment (not IDE) that it is a multi-line command.

```
Enable-PSRemoting
```
> allow for remote PS sessions over the internet/intranet on the machine you run the cmdlet (WSMan)

<details>
<summary>>> If Error <<</summary>

`Enable-PSRemoting` will not work if the network was set up as a public network because it causes the Windows firewall to be more restrictive. So you can bypass the public location issue by doing:

```
Enable-PSRemoting -SkipNetworkProfileCheck -Force
```
> by adding on the `-Force` switch, it allows to skip all of the confirmation prompts

</details>

```
hostname
```
> displays current host machine

```
ping hostnameOfMachine
```
> ping the machine you would want to connct with (WSMan)

```
Enter-PSSession -ComputerName hostnameOfMachine
```
> similar to `ssh` on a Linux/Unix box. However, only one session can connect at a time (WSMan)

```
Exit-PSSession
```
> exits the remote connection (WSMan)

```
Get-Process | Out-File \\hostmachineOfLocalMachine\c$\fileLog-process.txt
```
> should save the file in the C: drive for the machine you are using to remotely access the currently logged in remote session (WSMan)

```
$hostnameVar = New-Session -ComputerName hostnameOfMachine
$hostnameVar
```
> create a persistant session and print to see Id, Name, ComputerName, state, ConfiguationName and Availability (WSMan)

```
Get-PSSession
```
> shows all current persistant PS sessions (WSMan)

```
Disconnect-PSSession -Id 2
```
> Do not kill session but just disconnected, thus things like variables, jobs, ect. are still in memory (WSMan)

```
Enter-PSSession -Id 2
```
> Enters, does not start new, session and pick up where you left off before, even on a different machine (WSMan)

```
Get-PSSession | Remove-PSSession
```
> kill all sessions in the queue (WSMan)

```
Invoke-Command -ComputerName srv1, srv2 -ScriptBlock {Get-EventLog Security -Newest 10}
```
> one to many connection and give cmdlets (WSMan)

```
Invoke-Command -ComputerName srv1, srv2 -FilePath "C:\admin\script.ps1"
```
> one to many connection and feed in script file (WSMan)

```
Invoke-Command -ComputerName (Get-Content servers.txt) -ScriptBlock {Get-Service}
```
> one to many connection via feeding in a .csv into the computer name (WSMan)

```
Get-Command -ParameterName computerName
```
> show all cmdlets with the parameter of `computerName`

```
Get-WmiObject -ComputerName localhost, mem1 -Class win32_bios
```
> can get system information about a remote machine, you can do local machines just take out the mem1 (WSMan)

```
Invoke-Command -ComputerName (Get-Content servers.txt) -ScriptBlock {Get-EventLog -LogName security -Newest 5}
```
> notice that the column "PSComputerName" is always returned back so that you can tell which computer the information came from. This is with all scripts in the `Invoke-Command` cmdlet (WSMan)

```
Get-Execution Policy
```
> get the execution policy for running scripts

```
Set-ExecutionPolicy -ExecutionPolicy Unrestricted -Scope Process
```
> changes the execution policy of running scripts to unrestricted for the current process

```
Invoke-Command -ComputerName localhost,srv1 -ScriptBlock {Get-ExecutionPolicy}
```
> used to figure out exectution policy on remote machines to make sure you can run scripts (WSMan)

```
cd WSMan:
cd localhost\Client
ls .\TrustedHosts
Set-Item .\TrustedHosts -Value *
```
> notice the value of trusted hosts, it is then set to star (trusts all machines)

```
Get-Module -ListAvailable | Import-Module ; gcm -co cmdlet | measure
#                                          `--> splits the command, a statement separator
```
> counts how many modules are available

```
Get-Module -ListAvailable | Select-Object -Property name
```
> give short names of all available modules

```
Get-Command -verb export | Select-Object -Property name | Export-Csv -Path availModules.csv
```
> exports results as a .csv file

```
Get-Command -verb export | Select-Object -Property name | ConvertTo-Xml
```
> makes normal xml but does not produce a file because it just shifts the data to xml type

```
Get-Command -verb export | Select-Object -Property name | ConvertTo-Xml | Out-File availModules.xml
```
> adding the outfile allows for the data to be written to a file

```
Get-Command -verb export | Select-Object -Property name | Export-Clixml -Path availModules.xml
```
> exports results as a .xml file

```
Get-Command -verb export | Select-Object -Property name | ConvertTo-Html | Out-File availModules.html
```
> converts data type to html and then writes it to a file

```
Import-Clixml -Path availModules.xml
```
> As long as you exported using `Export-Clixml`, then you can import the .xml file into any PS as objects. Notice though, if `Import-Clixml -Path availModules.xml | Get-Member` that the `TypeName` starts with `Deserialized` which means the object has been dismantled.

```
dir Env:\PSModulePath | Format-List
```
> `Value:` shows the module "PATHs," if you will, and if you put a module in one of those directories, then you can autoload them.

```
$PROFILE
```
> check to see if and where a profile PS script is

```
New-Item -Type file -Path $PROFILE -Force
```
> only use to creates a PS profile script if one does not exist

```
notepad $PROFILE
```
> open the PS profile script file. Add anything you want to run every time PS starts a session

```
cls
Write-Host ""
$StartDate=(GET-DATE)
$EndDate=[datetime]"05/10/2020 07:30"
$intDays = (NEW-TIMESPAN -Start $StartDate -End $EndDate).Days
$divDays = $intDays/7-1
$intWeekDays = (($intDays%7*5)+[math]::floor($divDays))
Write-Host "                 ____________________________________                                     "
Write-Host "                / Welcome Alex! It's now day $intWeekDays-    \                           "
Write-Host "                \ of your internship at Florida Blue /                                    "
Write-Host "                 ------------------------------------                                     "
Write-Host "                   \                                                                      "
Write-Host "                    \                                                                     "
Write-Host "                     \                             .::!!!!!!!:.                           "
Write-Host "                    .!!!!!:.                     .:!!!!!!!!!!!!                           "
Write-Host "                    ----!!!!!!.              .:!!!!!!XUWW`$`$`$`$`$`$`$`$`$P              "
Write-Host "                        :`$`$NWX!!:         .<!!!!UW`$`$`$`$' '`$`$`$`$`$`$`$`$*          "
Write-Host "                        `$`$`$`$`$wwWX!:     :!!UW`$`$`$`$`$`$`$`$`$   4`$`$`$`$`$*       "
Write-Host "                        `$`$`$`$`$  `$`$`$UX      `$`$`$`$`$`$`$`$`$`$`$`$   d`$`$R`"     "
Write-Host "                        ^`$`$`$B  `$`$`$`$|      '*`$`$`$`$`$`$`$`$`$`$`$o+#`"            "
Write-Host "                          `"*`$bd`$`$`$`$'          `"`"`"`"`"`"`"                        "
Write-Host "                               `"`"`"`"                                                   "
Write-Host ""
Write-Host ""
```
> profile art

```
function funcName {
    Set-Location C:\
    Clear-Host
    Write-Host "Hi there $env:UserName!"
}

funcName()
```
> creates a function and then runs every time you log into PS (if you put into your $PROFILE)

```
Function functionName ([string] $url, [int] $delayTime = 500) {
    Write-Host "The url is: $url and the delay time is $delayTime ms"
}
```
> These are parameters in a function

```
dir variable:
```
> PSDrive that displays known variables

```
Get-Service | Select-Object -Property Status, Name | Where-Object { $_.Status -eq "Stopped" }
```
> This is row filtering with the `Where-Object` cmdlet. `Where-Object`'s shortcut is '?' and `PSItem`'s shortcut is `_.`

```
Get-Service | Select-Object -Property Status, Name | Where-Object`
{ $_.Status -eq "Stopped" }
```
> use the backtick character '`' to add a line break but a better way is the next command

```
Get-Service | Select-Object -Property Status, Name |
Where-Object { $_.Status -eq "Stopped" } |
Start-Service -whatif
```
> break a line on a pipe

```
 { $_.Name -like "w*" }
```
> fuzzy matches, similar to SQL

```
.\fileName.ps1
```
> loads the function into memory but does not run the functions. You should now be able to run the functions.

```
# Write-Host "Hello World!"
```
> this is a single line comment

```
Get-Package -Name notepadplusplus
```
> Find if a program is installed on the machine. Package Manager in PowerShell v5

```
Find-Package -Name notepadplusplus
```
> Finds program on chocolatey. Package Manager in PowerShell v5

```
Install-Package -Name notepadplusplus -Force
```
> Install. Package Manager in PowerShell v5

```
Get-FileHash fileName.txt
```
> this will give the hash value of the file

## Further Learning

### Windows PowerShell Script Repositories

 * CodePlex.com
    * Microsoft's open source repository
 * Poshcode.org
 * powershell.com
    * A Microsoft PowerShell Script Library

### Windows PowerShell Community Sites

 * powershell.org
    * I recommend reading [this book](https://powershell.org/2012/08/ebook-secrets-of-powershell-remoting/)
 * https://social.technet.microsoft.com/wiki/contents/articles/7383.powershell-communities.aspx
    * TechNet PowerShell Communities
 * https://stackoverflow.com/questions/tagged/powershell
    * StackOverFlow PowerShell Topics
 * devblogs.microsoft.com/scripting/
    * Microsoft Scripting Blog

### Windows PowerShell Books*
    * Windows PowerShell in Action
    * Learn Windows PowerShelll 3 in a Month of Lunches
    * Windows PowerShell in Depth
    * Windows PowerShell First Steps
    * Windows PowerShell Step-By-Step

    \* Be on the lookout for new editions