---
title: How to make DLink DWA-140 B2 perfectly work on Linux
author: Gaetan
layout: post
permalink: /2011/01/how-to-make-dlink-dwa-140-perfectly-work-on-linux/
tags:
  - linux
---

**[EDIT] Note: the DWA-140 B2 is now supported by recent Linux version.**

I’m using ArchLinux and I finally make my DWA-140 B2 Wifi USB adaptor work !

If you have the same problem, you can read this article to fix it !

<!--more-->

**My conf**:

* Linux Kernel: *2.6.36*
* Architecture: *x64*
* Device: *ID 07d1:3c0a D-Link System DWA-140 RangeBooster N Adapter(rev.B2) [Ralink RT2870]*


## What to use ?

So the only way I found to make it work nicely is to use **ndiswrapper**, a Windows XP driver wrapper, and **wpa_supplicant**, a WLAN tool.

First, I’ve tried some RaLink drivers but this was not really great, I’ve succeed to make `ra2870` work only one time but there was some lags each 20 seconds (like 1000ms ping frequently).

Moreover, I recommend not using NetworkManager with this method, it seems ndiswrapper and networkmanager produced awful results for this bundle (like waiting 30s for wifi to connect (or not connecting!), or freezing my Linux, …). Use wpa_supplicant utils like netcfg instead of ! 

## Procedure

Alternatives rejected, here is the solution step by step :

### Pre-conditions

With a wired connection, install these packages (with your package manager) : **ndiswrapper**, **wpa_supplicant** and (optional) **netcfg**.

Unfortunately, I had some freeze issues with the current ndiswrapper repository version so I checkout ndiswrapper SVN source code on  and recompile it. **Maybe you have to do the same.**

1.  Don’t plug your bundle yet. Insert the CD of the product. It contains the driver we need for ndiswrapper. You need to find the `/Drivers/WinXPx64/` (it may depend on your arch) **.inf** file. You can also download the last driver on the [D-Link website][2].
2.  Go on commandline in root mode and type:

```bash
ndiswrapper -i {path of the .inf file}
```
    
Example:
    
```bash
ndiswrapper -i /media/cdrom0/Drivers/WinXPx64/Drt2870.inf
```
   
* Plug your bundle and check if the command `ndiswrapper -l` say something like: 

```
drt2870 : driver installed  
device (07D1:3C0A) present
```   

If not you maybe need to `ndiswrapper -r drt2870` to remove the driver and return to the second step trying another `/Drivers/*/*.inf` file. 

*   Next, you need to configure your `wpa_supplicant.conf` configuration file for  your wifi access point. Refer to the [documentation][3].
*   Try to run `wpa_supplicant` : 

```bash
wpa_supplicant -i wlan0 -c /etc/wpa_supplicant.conf
```

Once this working, you maybe need to run dhcpd :
      
```bash
dhcpd wlan0
```
      
*   Now you can try your internet connection, by pinging google.com or try to browse the web.
*   If it works, you can save your configuration, refer to the [documentation][3]. 
        
**For any problem, feel free to comment this article.**

 [2]: http://www.dlink.com/products/?tab=3&pid=DWA-140&rev=DWA-140_revB
 [3]: https://wiki.archlinux.org/index.php/WPA_supplicant
