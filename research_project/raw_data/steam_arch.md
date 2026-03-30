Title: Steam/Troubleshooting - ArchWiki

URL Source: https://wiki.archlinux.org/title/Steam/Troubleshooting

Published Time: Mon, 30 Mar 2026 00:46:20 GMT

Markdown Content:
# Steam/Troubleshooting - ArchWiki

[](https://archlinux.org/)

*   [Home](https://archlinux.org/)
*   [Packages](https://archlinux.org/packages/)
*   [Forums](https://bbs.archlinux.org/)
*   [Wiki](https://wiki.archlinux.org/)
*   [GitLab](https://gitlab.archlinux.org/archlinux)
*   [Security](https://security.archlinux.org/)
*   [AUR](https://aur.archlinux.org/)
*   [Download](https://archlinux.org/download/)

[Jump to content](https://wiki.archlinux.org/title/Steam/Troubleshooting#bodyContent)

- [x] Main menu 

Main menu

move to sidebar hide

 Navigation 

*   [Main page](https://wiki.archlinux.org/title/Main_page "Visit the main page [z]")
*   [Table of contents](https://wiki.archlinux.org/title/Table_of_contents)
*   [Getting involved](https://wiki.archlinux.org/title/Getting_involved "Various ways Archers can contribute to the community")
*   [Wiki news](https://wiki.archlinux.org/title/ArchWiki:News "The latest lowdown on the wiki")
*   [Random page](https://wiki.archlinux.org/title/Special:Random "Load a random page [x]")

 Interaction 

*   [Help](https://wiki.archlinux.org/title/Category:Help "Wiki navigation, reading, and editing help")
*   [Contributing](https://wiki.archlinux.org/title/ArchWiki:Contributing)
*   [Recent changes](https://wiki.archlinux.org/title/Special:RecentChanges "A list of recent changes in the wiki [r]")
*   [Recent talks](https://wiki.archlinux.org/index.php?title=Special:RecentChanges&namespace=all-discussions)
*   [New pages](https://wiki.archlinux.org/title/Special:NewPages)
*   [Statistics](https://wiki.archlinux.org/title/ArchWiki:Statistics)
*   [Requests](https://wiki.archlinux.org/title/ArchWiki_talk:Requests)

[**ArchWiki**](https://wiki.archlinux.org/title/Main_page)

[Search](https://wiki.archlinux.org/title/Special:Search "Search ArchWiki [f]")

Search

- [x] Appearance 

*   [Create account](https://wiki.archlinux.org/index.php?title=Special:CreateAccount&returnto=Steam%2FTroubleshooting "You are encouraged to create an account and log in; however, it is not mandatory")
*   [Log in](https://wiki.archlinux.org/index.php?title=Special:UserLogin&returnto=Steam%2FTroubleshooting "You are encouraged to log in; however, it is not mandatory [o]")

- [x] Personal tools 

*   [Create account](https://wiki.archlinux.org/index.php?title=Special:CreateAccount&returnto=Steam%2FTroubleshooting "You are encouraged to create an account and log in; however, it is not mandatory")
*   [Log in](https://wiki.archlinux.org/index.php?title=Special:UserLogin&returnto=Steam%2FTroubleshooting "You are encouraged to log in; however, it is not mandatory [o]")

## Contents

move to sidebar hide

*   [Beginning](https://wiki.archlinux.org/title/Steam/Troubleshooting#)
*   [1 Steam runtime](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_runtime)Toggle Steam runtime subsection
    *   [1.1 Steam native runtime](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_native_runtime)

*   [2 Debugging shared libraries](https://wiki.archlinux.org/title/Steam/Troubleshooting#Debugging_shared_libraries)Toggle Debugging shared libraries subsection
    *   [2.1 Finding missing game libraries](https://wiki.archlinux.org/title/Steam/Troubleshooting#Finding_missing_game_libraries)

    *   [2.2 Finding missing runtime libraries](https://wiki.archlinux.org/title/Steam/Troubleshooting#Finding_missing_runtime_libraries)

*   [3 Debugging Steam](https://wiki.archlinux.org/title/Steam/Troubleshooting#Debugging_Steam)

*   [4 Runtime issues](https://wiki.archlinux.org/title/Steam/Troubleshooting#Runtime_issues)Toggle Runtime issues subsection
    *   [4.1'GLBCXX_3.X.XX' not found when using Bumblebee](https://wiki.archlinux.org/title/Steam/Troubleshooting#'GLBCXX_3.X.XX'_not_found_when_using_Bumblebee)

    *   [4.2 Steam>Warning: failed to init SDL thread priority manager: SDL not found](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam%3EWarning:_failed_to_init_SDL_thread_priority_manager:_SDL_not_found)

    *   [4.3 Game crashes immediately](https://wiki.archlinux.org/title/Steam/Troubleshooting#Game_crashes_immediately)

    *   [4.4 Game and Steam crashes after game start](https://wiki.archlinux.org/title/Steam/Troubleshooting#Game_and_Steam_crashes_after_game_start)

    *   [4.5 Some games freeze at start when in focus](https://wiki.archlinux.org/title/Steam/Troubleshooting#Some_games_freeze_at_start_when_in_focus)

    *   [4.6 Version `CURL_OPENSSL_3` not found](https://wiki.archlinux.org/title/Steam/Troubleshooting#Version_%60CURL_OPENSSL_3%60_not_found)

    *   [4.7 Steam webview/game browser not working in native runtime (Black screen)](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_webview/game_browser_not_working_in_native_runtime_(Black_screen))

    *   [4.8 Steam: An X Error occurred](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam:_An_X_Error_occurred)

    *   [4.9 Steam: Compatibility tool configuration failed](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam:_Compatibility_tool_configuration_failed)

    *   [4.10 Game starts but closes immediately with custom kernel](https://wiki.archlinux.org/title/Steam/Troubleshooting#Game_starts_but_closes_immediately_with_custom_kernel)

    *   [4.11 Steam Library won't start](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_Library_won't_start)

*   [5 Graphical issues](https://wiki.archlinux.org/title/Steam/Troubleshooting#Graphical_issues)Toggle Graphical issues subsection
    *   [5.1 Black main screen on Intel iGPUs](https://wiki.archlinux.org/title/Steam/Troubleshooting#Black_main_screen_on_Intel_iGPUs)

    *   [5.2 Blurry text and graphics with Xwayland and HiDPI](https://wiki.archlinux.org/title/Steam/Troubleshooting#Blurry_text_and_graphics_with_Xwayland_and_HiDPI)

    *   [5.3 Steam flicker/blink with black screen not loading Store/Library or other pages](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_flicker/blink_with_black_screen_not_loading_Store/Library_or_other_pages)
        *   [5.3.1 Fix by editing desktop entry](https://wiki.archlinux.org/title/Steam/Troubleshooting#Fix_by_editing_desktop_entry)

        *   [5.3.2 Bypass desktop entry](https://wiki.archlinux.org/title/Steam/Troubleshooting#Bypass_desktop_entry)

*   [6 Audio issues](https://wiki.archlinux.org/title/Steam/Troubleshooting#Audio_issues)Toggle Audio issues subsection
    *   [6.1 Configure PulseAudio](https://wiki.archlinux.org/title/Steam/Troubleshooting#Configure_PulseAudio)

    *   [6.2 No audio or 756 Segmentation fault](https://wiki.archlinux.org/title/Steam/Troubleshooting#No_audio_or_756_Segmentation_fault)

    *   [6.3 FMOD sound engine](https://wiki.archlinux.org/title/Steam/Troubleshooting#FMOD_sound_engine)

    *   [6.4 PulseAudio & OpenAL: Audio streams cannot be moved between devices](https://wiki.archlinux.org/title/Steam/Troubleshooting#PulseAudio_&_OpenAL:_Audio_streams_cannot_be_moved_between_devices)

    *   [6.5 Cracking Microphone in Steam Voice and Games](https://wiki.archlinux.org/title/Steam/Troubleshooting#Cracking_Microphone_in_Steam_Voice_and_Games)

*   [7 Steam client issues](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_client_issues)Toggle Steam client issues subsection
    *   [7.1 Cannot browse filesystem to add a library folder or library folder appears as empty](https://wiki.archlinux.org/title/Steam/Troubleshooting#Cannot_browse_filesystem_to_add_a_library_folder_or_library_folder_appears_as_empty)

    *   [7.2 Cannot add library folder because of missing execute permissions](https://wiki.archlinux.org/title/Steam/Troubleshooting#Cannot_add_library_folder_because_of_missing_execute_permissions)

    *   [7.3 Unusually slow download speed](https://wiki.archlinux.org/title/Steam/Troubleshooting#Unusually_slow_download_speed)

    *   [7.4"Needs to be online" error](https://wiki.archlinux.org/title/Steam/Troubleshooting#%22Needs_to_be_online%22_error)

    *   [7.5 Steam forgets password](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_forgets_password)

    *   [7.6 Preventing crash memory dumps](https://wiki.archlinux.org/title/Steam/Troubleshooting#Preventing_crash_memory_dumps)

    *   [7.7 Steam license problem with playing videos](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_license_problem_with_playing_videos)

    *   [7.8 No context menu for joining/inviting friends](https://wiki.archlinux.org/title/Steam/Troubleshooting#No_context_menu_for_joining/inviting_friends)

    *   [7.9 Slow and unresponsive user interface](https://wiki.archlinux.org/title/Steam/Troubleshooting#Slow_and_unresponsive_user_interface)

    *   [7.10 Steam fails to start correctly](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_fails_to_start_correctly)

    *   [7.11 Missing taskbar menu](https://wiki.archlinux.org/title/Steam/Troubleshooting#Missing_taskbar_menu)

    *   [7.12"Your browser does not support the minimum set of features required to watch this broadcast" error](https://wiki.archlinux.org/title/Steam/Troubleshooting#%22Your_browser_does_not_support_the_minimum_set_of_features_required_to_watch_this_broadcast%22_error)

    *   [7.13 Using system titlebar and frame](https://wiki.archlinux.org/title/Steam/Troubleshooting#Using_system_titlebar_and_frame)

    *   [7.14 More selective DPMS inhibition](https://wiki.archlinux.org/title/Steam/Troubleshooting#More_selective_DPMS_inhibition)

    *   [7.15 Enabling fractional scaling](https://wiki.archlinux.org/title/Steam/Troubleshooting#Enabling_fractional_scaling)

    *   [7.16 Steam Beta crashes](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_Beta_crashes)

    *   [7.17 Cannot access store page (client displays error -105 or -102)](https://wiki.archlinux.org/title/Steam/Troubleshooting#Cannot_access_store_page_(client_displays_error_-105_or_-102))

    *   [7.18 Steam client restarts while a game is running](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_client_restarts_while_a_game_is_running)

*   [8 Steam Remote Play issues](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_Remote_Play_issues)Toggle Steam Remote Play issues subsection
    *   [8.1 Remote Play does not work from Arch Linux host to Arch Linux guest](https://wiki.archlinux.org/title/Steam/Troubleshooting#Remote_Play_does_not_work_from_Arch_Linux_host_to_Arch_Linux_guest)

    *   [8.2 Hardware decoding not available](https://wiki.archlinux.org/title/Steam/Troubleshooting#Hardware_decoding_not_available)

    *   [8.3 Big Picture Mode minimizes itself after losing focus](https://wiki.archlinux.org/title/Steam/Troubleshooting#Big_Picture_Mode_minimizes_itself_after_losing_focus)

*   [9 Other issues](https://wiki.archlinux.org/title/Steam/Troubleshooting#Other_issues)Toggle Other issues subsection
    *   [9.1 Steam Library in NTFS partition](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_Library_in_NTFS_partition)

    *   [9.2 Wrong ELF class](https://wiki.archlinux.org/title/Steam/Troubleshooting#Wrong_ELF_class)

    *   [9.3 Multiple monitors setup](https://wiki.archlinux.org/title/Steam/Troubleshooting#Multiple_monitors_setup)

    *   [9.4 Text is corrupt or missing](https://wiki.archlinux.org/title/Steam/Troubleshooting#Text_is_corrupt_or_missing)

    *   [9.5 SetLocale('en_US.UTF-8') fails at game startup or typing non-ASCII characters does not work in the Steam client](https://wiki.archlinux.org/title/Steam/Troubleshooting#SetLocale('en_US.UTF-8')_fails_at_game_startup_or_typing_non-ASCII_characters_does_not_work_in_the_Steam_client)

    *   [9.6 Missing libc](https://wiki.archlinux.org/title/Steam/Troubleshooting#Missing_libc)

    *   [9.7 Games do not launch on older Intel hardware](https://wiki.archlinux.org/title/Steam/Troubleshooting#Games_do_not_launch_on_older_Intel_hardware)

    *   [9.8 Mesa: Game does not launch, complaining about OpenGL version supported by the card](https://wiki.archlinux.org/title/Steam/Troubleshooting#Mesa:_Game_does_not_launch,_complaining_about_OpenGL_version_supported_by_the_card)

    *   [9.9 2K games do not run on XFS partitions](https://wiki.archlinux.org/title/Steam/Troubleshooting#2K_games_do_not_run_on_XFS_partitions)

    *   [9.10 Steam controller not being detected correctly](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_controller_not_being_detected_correctly)

    *   [9.11 Steam controller makes a game crash](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_controller_makes_a_game_crash)

    *   [9.12 Steam hangs on "Installing breakpad exception handler..."](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_hangs_on_%22Installing_breakpad_exception_handler...%22)

    *   [9.13 Killing standalone compositors when launching games](https://wiki.archlinux.org/title/Steam/Troubleshooting#Killing_standalone_compositors_when_launching_games)

    *   [9.14 Symbol lookup error using DRI3](https://wiki.archlinux.org/title/Steam/Troubleshooting#Symbol_lookup_error_using_DRI3)

    *   [9.15 Launching games on NVIDIA Optimus laptops](https://wiki.archlinux.org/title/Steam/Troubleshooting#Launching_games_on_NVIDIA_Optimus_laptops)

    *   [9.16 HiDPI](https://wiki.archlinux.org/title/Steam/Troubleshooting#HiDPI)

    *   [9.17 Protocol support under KDE Plasma](https://wiki.archlinux.org/title/Steam/Troubleshooting#Protocol_support_under_KDE_Plasma)

    *   [9.18 The game crashes when using Steam Linux Runtime - Soldier](https://wiki.archlinux.org/title/Steam/Troubleshooting#The_game_crashes_when_using_Steam_Linux_Runtime_-_Soldier)

    *   [9.19 Games running with Proton 5.13+ have no Internet connectivity](https://wiki.archlinux.org/title/Steam/Troubleshooting#Games_running_with_Proton_5.13+_have_no_Internet_connectivity)

    *   [9.20"could not determine 32/64 bit of java"](https://wiki.archlinux.org/title/Steam/Troubleshooting#%22could_not_determine_32/64_bit_of_java%22)

    *   [9.21 Stuttering with Vulkan](https://wiki.archlinux.org/title/Steam/Troubleshooting#Stuttering_with_Vulkan)

    *   [9.22 Force OpenGL emulation](https://wiki.archlinux.org/title/Steam/Troubleshooting#Force_OpenGL_emulation)

    *   [9.23 File picker does not see anything but Steam library](https://wiki.archlinux.org/title/Steam/Troubleshooting#File_picker_does_not_see_anything_but_Steam_library)

    *   [9.24 DirectX errors on hybrid graphics](https://wiki.archlinux.org/title/Steam/Troubleshooting#DirectX_errors_on_hybrid_graphics)

    *   [9.25 No Internet Connection when downloading](https://wiki.archlinux.org/title/Steam/Troubleshooting#No_Internet_Connection_when_downloading)

    *   [9.26 Poor performance or stuttering after launching Steam](https://wiki.archlinux.org/title/Steam/Troubleshooting#Poor_performance_or_stuttering_after_launching_Steam)

    *   [9.27 Very long startup and slow user interface response](https://wiki.archlinux.org/title/Steam/Troubleshooting#Very_long_startup_and_slow_user_interface_response)

*   [10 See also](https://wiki.archlinux.org/title/Steam/Troubleshooting#See_also)

- [x] Toggle the table of contents 

# Steam/Troubleshooting

- [x] 4 languages 

*   [Magyar](https://wiki.archlinux.org/title/Steam_(Magyar)/Troubleshooting_(Magyar) "Steam (Magyar)/Troubleshooting – magyar")
*   [日本語](https://wiki.archlinux.jp/index.php/Steam/%E3%83%88%E3%83%A9%E3%83%96%E3%83%AB%E3%82%B7%E3%83%A5%E3%83%BC%E3%83%86%E3%82%A3%E3%83%B3%E3%82%B0 "Steam/トラブルシューティング – 日本語")
*   [Русский](https://wiki.archlinux.org/title/Steam_(%D0%A0%D1%83%D1%81%D1%81%D0%BA%D0%B8%D0%B9)/Troubleshooting_(%D0%A0%D1%83%D1%81%D1%81%D0%BA%D0%B8%D0%B9) "Steam (Русский)/Troubleshooting – русский")
*   [中文（简体）](https://wiki.archlinuxcn.org/wiki/Steam/%E7%96%91%E9%9A%BE%E8%A7%A3%E7%AD%94 "Steam/疑难解答 – 中文（简体）")

*   [Page](https://wiki.archlinux.org/title/Steam/Troubleshooting "View the content page [c]")
*   [Discussion](https://wiki.archlinux.org/title/Talk:Steam/Troubleshooting "Discussion about the content page [t]")

- [x] English 

*   [Read](https://wiki.archlinux.org/title/Steam/Troubleshooting)
*   [View source](https://wiki.archlinux.org/index.php?title=Steam/Troubleshooting&action=edit "This page is protected.
You can view its source [e]")
*   [View history](https://wiki.archlinux.org/index.php?title=Steam/Troubleshooting&action=history "Past revisions of this page [h]")

- [x] Tools 

Tools

move to sidebar hide

 Actions 

*   [Read](https://wiki.archlinux.org/title/Steam/Troubleshooting)
*   [View source](https://wiki.archlinux.org/index.php?title=Steam/Troubleshooting&action=edit)
*   [View history](https://wiki.archlinux.org/index.php?title=Steam/Troubleshooting&action=history)

 General 

*   [What links here](https://wiki.archlinux.org/title/Special:WhatLinksHere/Steam/Troubleshooting "A list of all wiki pages that link here [j]")
*   [Related changes](https://wiki.archlinux.org/title/Special:RecentChangesLinked/Steam/Troubleshooting "Recent changes in pages linked from this page [k]")
*   [Printable version](javascript:print(); "Printable version of this page [p]")
*   [Permanent link](https://wiki.archlinux.org/index.php?title=Steam/Troubleshooting&oldid=868814 "Permanent link to this revision of this page")
*   [Page information](https://wiki.archlinux.org/index.php?title=Steam/Troubleshooting&action=info "More information about this page")

Appearance

move to sidebar hide

From ArchWiki

<[Steam](https://wiki.archlinux.org/title/Steam "Steam")

1.   Make sure that you have followed [Steam#Installation](https://wiki.archlinux.org/title/Steam#Installation "Steam").
2.   If the Steam client / a game is not starting and/or you have error message about a library, read [#Steam runtime](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_runtime) and see [#Debugging shared libraries](https://wiki.archlinux.org/title/Steam/Troubleshooting#Debugging_shared_libraries).
3.   If the issue is related to networking, make sure that you have forwarded the [required ports for Steam](https://help.steampowered.com/en/faqs/view/2EA8-4D75-DA21-31EB).
4.   If the issue is about a game, consult [Steam/Game-specific troubleshooting](https://wiki.archlinux.org/title/Steam/Game-specific_troubleshooting "Steam/Game-specific troubleshooting").

## Steam runtime

Steam for Linux ships with its own set of libraries called the [Steam runtime](https://github.com/ValveSoftware/steam-runtime). By default Steam launches all Steam Applications within the runtime environment. The Steam runtime is located at `~/.steam/root/ubuntu12_32/steam-runtime/`.

If you mix the Steam runtime libraries with system libraries you will run into binary incompatibility issues, see [steam-for-linux issue #4768](https://github.com/ValveSoftware/steam-for-linux/issues/4768). Binary incompatibility can lead to the Steam client and games not starting (manifesting as a crash, as hanging or silently returning), audio issues and various other problems.

The [steam](https://archlinux.org/packages/?name=steam) package offers two ways to launch Steam:

*   `/usr/bin/steam` (alias `steam`), which overrides runtime libraries known to cause problems via the `LD_PRELOAD`[environment variable](https://wiki.archlinux.org/title/Environment_variable "Environment variable") (see [ld.so(8)](https://man.archlinux.org/man/ld.so.8)).
*   `/usr/lib/steam/steam`, the default Steam launch script

As the Steam runtime libraries are older they can lack newer features, e.g. the OpenAL version of the Steam runtime lacks [HRTF](https://wiki.archlinux.org/title/Gaming#Binaural_audio_with_OpenAL "Gaming") and surround71 support.

### Steam native runtime

**Warning** Using the Steam native runtime is not recommended as it might break some games due to binary incompatibility and it might miss some libraries present in the Steam runtime.

The [steam-native-runtime](https://aur.archlinux.org/packages/steam-native-runtime/)AUR package depends on over 130 packages to pose a native replacement of the Steam runtime, some games may however still require additional packages.

This package provides the `steam-native` script, which launches Steam with the `STEAM_RUNTIME=0` environment variable and `-compat-force-slr off` flag, making it ignore its runtime and only use system libraries.

**Note** This will only apply to games that use the Steam Linux Runtime 1 and Steam itself, if developers choose to use Steam Linux Runtime 3 or newer, take for example Barony, the only way to escape that will be to either launch the game not from Steam or modify the launch arguments to something like `./<game_executable>; exit; %command%`, which launches the game in system environment since runtimes are only applied for `%command%`.

You can also use the Steam native runtime without [steam-native-runtime](https://aur.archlinux.org/packages/steam-native-runtime/)AUR by manually installing just the packages you need. See [#Finding missing runtime libraries](https://wiki.archlinux.org/title/Steam/Troubleshooting#Finding_missing_runtime_libraries).

## Debugging shared libraries

To see the shared libraries required by a program or a shared library run the `ldd` command on it, see [ldd(1)](https://man.archlinux.org/man/ldd.1). The `LD_LIBRARY_PATH` and `LD_PRELOAD`[environment variables](https://wiki.archlinux.org/title/Environment_variables "Environment variables") can alter which shared libraries are loaded, see [ld.so(8)](https://man.archlinux.org/man/ld.so.8). To correctly debug a program or shared library it is therefore important that these environment variables in your debug environment match the environment you wish to debug.

If you figure out a missing library you can use [pacman](https://wiki.archlinux.org/title/Pacman "Pacman") or [pkgfile](https://wiki.archlinux.org/title/Pkgfile "Pkgfile") to search for packages that contain the missing library.

### Finding missing game libraries

If a game fails to start, a possible reason is that it is missing required libraries. You can find out what libraries it requests by running `ldd game_executable`. `game_executable` is likely located somewhere in `~/.steam/root/steamapps/common/`. Please note that most of these "missing" libraries are actually already included with Steam, and do not need to be installed globally.

### Finding missing runtime libraries

If individual games or Steam itself is failing to launch when using `steam-native` you are probably missing libraries. To find the required libraries run:

$ cd ~/.steam/root/ubuntu12_32
$ file * | grep ELF | cut -d: -f1 | LD_LIBRARY_PATH=. xargs ldd | grep 'not found' | sort | uniq

Alternatively, run Steam with `steam` and use the following command to see which non-system libraries Steam is using (not all of these are part of the Steam runtime):

$ for i in $(pgrep steam); do sed '/\.local/!d;s/.*  //g' /proc/$i/maps; done | sort | uniq

## Debugging Steam

![Image 1](https://wiki.archlinux.org/images/4/4e/View-refresh-red.svg)**This article or section is out of date.**

**Reason:** Steam no longer redirects stdout and stderr to `/tmp/dumps/USER_stdout.txt` by default. See: [steam-for-linux issue 7114](https://github.com/ValveSoftware/steam-for-linux/issues/7114) A similar effect can be achieved by starting steam with `steam 2>&1` (Discuss in [Talk:Steam/Troubleshooting](https://wiki.archlinux.org/title/Talk:Steam/Troubleshooting))

The Steam launcher redirects its stdout and stderr to `/tmp/dumps/USER_stdout.txt`. This means you do not have to run Steam from the command-line to see that output.

It is possible to debug Steam to gain more information which could be useful to find out why something does not work.

You can set `DEBUGGER` environment variable with one of `gdb`, `cgdb`, `valgrind`, `callgrind`, `strace` and then start `steam`.

For example with [gdb](https://archlinux.org/packages/?name=gdb)

$ DEBUGGER=gdb steam

`gdb` will open, then type `run` which will start `steam` and once crash happens you can type `backtrace` to see call stack.

## Runtime issues

### 'GLBCXX_3.X.XX' not found when using Bumblebee

This error is likely caused because Steam packages its own out of date `libstdc++.so.6`. See [#Finding missing runtime libraries](https://wiki.archlinux.org/title/Steam/Troubleshooting#Finding_missing_runtime_libraries) about working around the bad library. See also [steam-for-linux issue 3773](https://github.com/ValveSoftware/steam-for-linux/issues/3773).

### Steam>Warning: failed to init SDL thread priority manager: SDL not found

Solution: [install](https://wiki.archlinux.org/title/Install "Install") the [lib32-sdl2](https://aur.archlinux.org/packages/lib32-sdl2/)AUR package.

### Game crashes immediately

This is likely due to [#Steam runtime](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_runtime) issues, see [#Debugging shared libraries](https://wiki.archlinux.org/title/Steam/Troubleshooting#Debugging_shared_libraries).

Disabling the in-game Steam Overlay in the game properties might help.

And finally, if those do not work, you should check Steam's output for any error from the game. You may encounter the following:

*   `munmap_chunk(): invalid pointer`
*   `free(): invalid pointer`

In these cases, try replacing the `libsteam_api.so` file from the problematic game with one of a game that works. This error usually happens for games that were not updated recently when Steam runtime is disabled. This error has been encountered with AYIM, Bastion and Monaco.

If the game crashes with

`terminate called after throwing an instance of 'dxvk::DxvkError'`

it's likely that conflicting versions of Vulkan are [installed](https://www.reddit.com/r/archlinux/comments/s1vjjg/comment/hsb10ac/?context=3). [lib32-vulkan-intel](https://archlinux.org/packages/?name=lib32-vulkan-intel) and NVIDIA Vulkan drivers are mutually exclusive. This is solved by uninstalling the unneeded driver. To obtain information about the chipset vendor one can run:

# lshw -C display | grep vendor

To get a list of installed packages

# pacman -Qs vulkan

### Game and Steam crashes after game start

If the following error is output:

failed to dlopen engine.so error=/home/_GAMEPATH_/bin/libgcc_s.so.1: version `GCC_7.0.0' not found (required by /usr/lib32/libopenal.so.1)

moving the incompatible lib can be a workaround.

mv .local/share/Steam/steamapps/common/_GAME_/bin/libgcc_s.so.1 .local/share/Steam/steamapps/common/_GAME_/bin/libgcc_s.so.1.b

### Some games freeze at start when in focus

A combination of using `ForceFullCompositionPipeline`, specific Proton versions and NVIDIA driver version 535 is known [to freeze some games](https://github.com/ValveSoftware/Proton/issues/6869)[using DXVK/Vulkan](https://github.com/doitsujin/dxvk/issues/3670) at launch under Xorg. Using Alt+Tab allows bringing Steam in focus, and the game seems to run properly in the background. Solution: disable `ForceFullCompositionPipeline` or downgrade NVIDIA drivers.

### Version `CURL_OPENSSL_3` not found

This is because [curl](https://archlinux.org/packages/?name=curl) alone is not compatible with previous versions. You need to install the compatibility libraries:

One of the following messages may show up:

# Nuclear Throne
./nuclearthrone: /usr/lib32/libcurl.so.4: version `CURL_OPENSSL_3' not found (required by ./nuclearthrone)

# Devil Daggers
./devildaggers: /usr/lib/libcurl.so.4: version `CURL_OPENSSL_3' not found (required by ./devildaggers)

You need to install either [libcurl-compat](https://archlinux.org/packages/?name=libcurl-compat) or [lib32-libcurl-compat](https://archlinux.org/packages/?name=lib32-libcurl-compat) and link the compatibility library manually:

# Nuclear Throne
$ ln -s /usr/lib32/libcurl-compat.so.4.4.0 "_LIBRARY_/steamapps/common/Nuclear Throne/lib/libcurl.so.4"

# Devil Daggers
$ ln -s /usr/lib/libcurl-compat.so.4.4.0 _LIBRARY_/steamapps/common/devildaggers/lib64/libcurl.so.4

### Steam webview/game browser not working in native runtime (Black screen)

Since the new Steam Friends UI update, the client webview is not working correctly with the native-runtime.

./steamwebhelper: error while loading shared libraries: libpcre.so.3: cannot open shared object file: No such file or directory

It can be solved preloading glib libraries; Those do not require libpcre and selinux to work.

$ LD_PRELOAD="/usr/lib/libgio-2.0.so.0 /usr/lib/libglib-2.0.so.0" steam-native

Alternatively, you may create a symbolic link to the native Arch libpcre library.

# ln -s /usr/lib/libpcre.so /usr/lib64/libpcre.so.3

Since update from around 3/3/2022, there are some reports of black screen still persisting after applying above workaround.

The workaround for now is to run Steam with the `-no-cef-sandbox` option. More information can be found in Github Steam-For-Linux repository Issue [#8451](https://github.com/ValveSoftware/steam-for-linux/issues/8451) and [#8420](https://github.com/ValveSoftware/steam-for-linux/issues/8420).

### Steam: An X Error occurred

When using an NVIDIA GPU and proprietary drivers, Steam may fail to start and (if run from the terminal) produce errors of the form:

Steam: An X Error occurred
X Error of failed request:  GLXBadContext
Major opcode of failed request:  151
Serial number of failed request:  51
xerror_handler: X failed, continuing

Ensure the _lib32-_[NVIDIA](https://wiki.archlinux.org/title/NVIDIA "NVIDIA") driver for your card is installed, and matches the main package version with:

# pacman -Qs nvidia

You may need to change which [mirrors](https://wiki.archlinux.org/title/Mirrors "Mirrors") you are using to install the drivers if they do not match.

If you are using AMD, have enabled 10-bit color depth, and are having this problem. You will likely need to disable 10-bit color depth.

Another issue that causes this error message can be [solved by removing the config.vdf file](https://github.com/ValveSoftware/steam-for-linux/issues/4340#issuecomment-258593713):

$ rm ~/.local/share/Steam/config/config.vdf

### Steam: Compatibility tool configuration failed

If you are trying to run a native game using Proton but get a Steam compatibility tool error immediately after starting the game, you might have to reinstall the runtime.

1.   Navigate to your Steam library.
2.   In the dropdown above your game list check the _Tools_ option to make them visible.
3.   Search for _Proton_, right click on each installed tool, visit _Properties_, open the _Local files_ tab and click _Verify integrity of tool files_ for each entry.
4.   Search for _Steam Linux Runtime_ and repeat the same procedure. If none are available, install the latest _Steam Linux Runtime - Soldier_.

### Game starts but closes immediately with custom kernel

Make sure that you have enabled _User namespace_ in _General setup -> Namespaces support_.

### Steam Library won't start

Opening the Steam library either displays nothing, or a brief splash, but no window appears. Running `/usr/bin/steam` in a terminal window gives this error:

Assertion 'device' failed at src/libsystemd/sd-device/device-private.c:103, function device_get_tags_generation(). Aborting.

Bugs reports are filed: [#79006](https://bugs.archlinux.org/task/79006)

See also discussion at: [Steam failing to launch since systemd 253.5-2 update](https://bbs.archlinux.org/viewtopic.php?id=287033)

A workaround is to install [lib32-libnm](https://archlinux.org/packages/?name=lib32-libnm).

## Graphical issues

### Black main screen on Intel iGPUs

On some systems that use Intel integrated graphics running Steam on [Wayland](https://wiki.archlinux.org/title/Wayland "Wayland") will make only the webviews not render, with functional dropdowns and other menus. Going to _Steam -> Settings -> Interface_, then disabling _Enable GPU accelerated rendering in web views_ might fix the problem.

### Blurry text and graphics with Xwayland and HiDPI

When Steam runs as an [Xwayland](https://wiki.archlinux.org/title/Xwayland "Xwayland") client under a compositor that uses [HiDPI](https://wiki.archlinux.org/title/HiDPI "HiDPI") scaling, you may find that Steam and games are rendered at half resolution and then upscaled to fit the HiDPI screen. This results in blurry graphics.

One option is to run Steam under a nested [gamescope](https://wiki.archlinux.org/title/Gamescope "Gamescope") compositor. Install the [gamescope](https://archlinux.org/packages/?name=gamescope) package:

$ gamescope -f -m 1 -e -- steam -gamepadui

This runs Steam in "big picture" mode (actually Steam Deck mode), in fullscreen, without scaling (i.e. at full resolution). The same settings should also propagate to games run under Steam.

Another option is to configure your compositor to prevent Xwayland from scaling applications entirely. For example, [Hyprland](https://wiki.archlinux.org/title/Hyprland "Hyprland") users can add

xwayland {
  force_zero_scaling = true
}

to the hyprland.conf file to prevent Xwayland from scaling any applications. Note that **all** applications that use Xwayland will stop scaling, and so on HiDPI displays, text and other elements in those applications may become too small to be comfortably viewed.

### Steam flicker/blink with black screen not loading Store/Library or other pages

When Steam is started on Wayland (not confirmed X11) with dual graphics in some cases Steam client is unstable display black screen and flicker/blink. This is due to the option `PrefersNonDefaultGPU` being enabled in the [desktop entry](https://wiki.archlinux.org/title/Desktop_entry "Desktop entry").

#### Fix by editing desktop entry

First, [make a user copy of the desktop entry](https://wiki.archlinux.org/title/Desktop_entries#Modify_desktop_files "Desktop entries") for Steam (from `/usr/share/applications/steam.desktop`). Then, change the option:

~/.local/share/applications/steam.desktop...
**PrefersNonDefaultGPU=false**
...
If opened, close Steam and relaunch.

**Tip** Some desktop environments provide a GUI for editing application options. For KDE Plasma: Right click on .desktop file > Edit application... > select tab "Application" > Advanced Options > Uncheck option "Run using dedicated graphics card". 

#### Bypass desktop entry

The desktop entry options do not take effect if you start Steam from the terminal, bypassing the issue.

$ steam &

Ampersand (&) at the end is to run Steam in background, terminal can be closed after Steam starts.

## Audio issues

If the sections below do not address the issue, using the [#Steam native runtime](https://wiki.archlinux.org/title/Steam/Troubleshooting#Steam_native_runtime) might help.

### Configure PulseAudio

Games that explicitly depend on ALSA can break PulseAudio. Follow the directions for [PulseAudio#ALSA](https://wiki.archlinux.org/title/PulseAudio#ALSA "PulseAudio") to make these games use PulseAudio instead.

If you are using [PipeWire](https://wiki.archlinux.org/title/PipeWire "PipeWire"), then instead install [lib32-pipewire](https://archlinux.org/packages/?name=lib32-pipewire) and set up [PipeWire#PulseAudio clients](https://wiki.archlinux.org/title/PipeWire#PulseAudio_clients "PipeWire").

### No audio or 756 Segmentation fault

First [#Configure PulseAudio](https://wiki.archlinux.org/title/Steam/Troubleshooting#Configure_PulseAudio) and see if that resolves the issue. If you do not have audio in the videos which play within the Steam client, it is possible that the ALSA libraries packaged with Steam are not working.

Attempting to playback a video within the Steam client results in an error similar to:

ALSA lib pcm_dmix.c:1018:(snd_pcm_dmix_open) unable to open slave

A workaround is to rename or delete the `alsa-lib` folder and the `libasound.so.*` files. They can be found at:

~/.steam/steam/ubuntu12_32/steam-runtime/i386/usr/lib/i386-linux-gnu/

An alternative workaround is to add the `libasound.so.*` library to the `LD_PRELOAD` environment variable:

$ LD_PRELOAD='/usr/$LIB/libasound.so.2 '${LD_PRELOAD} steam

If audio still will not work, adding the PulseAudio libraries to the `LD_PRELOAD` variable may help:

$ LD_PRELOAD='/usr/$LIB/libpulse.so.0 /usr/$LIB/libpulse-simple.so.0 '${LD_PRELOAD} steam

Be advised that their names may change over time. If so, it is necessary to take a look in

~/.steam/ubuntu12_32/steam-runtime/i386/usr/lib/i386-linux-gnu

and find the new libraries and their versions.

Bugs reports have been filed: [#3376](https://github.com/ValveSoftware/steam-for-linux/issues/3376) and [#3504](https://github.com/ValveSoftware/steam-for-linux/issues/3504)

### FMOD sound engine

![Image 2](https://wiki.archlinux.org/images/0/0b/Inaccurate.svg)**The factual accuracy of this article or section is disputed.**

**Reason:** No source / bug report. (Discuss in [Talk:Steam/Troubleshooting](https://wiki.archlinux.org/title/Talk:Steam/Troubleshooting))

The [FMOD](https://www.fmod.com/) audio middleware package is a bit buggy, and as a result games using it may have sound problems.

It usually occurs when an unused sound device is used as default for ALSA. See [Advanced Linux Sound Architecture#Set the default sound card](https://wiki.archlinux.org/title/Advanced_Linux_Sound_Architecture#Set_the_default_sound_card "Advanced Linux Sound Architecture").

Affected games: Hotline Miami, Hotline Miami 2, Transistor

### PulseAudio & OpenAL: Audio streams cannot be moved between devices

If you use [PulseAudio](https://wiki.archlinux.org/title/PulseAudio "PulseAudio") and cannot move an audio stream between sinks, it might be because recent OpenAL versions default to disallow audio streams from being moved. Try to add the following to your `~/.alsoftrc`:

[pulse]
allow-moves=true

### Cracking Microphone in Steam Voice and Games

If you experience cracking with your audio input while using Steam Voice or in games, you can try to launch Steam with the environmental variable `PULSE_LATENCY_MSEC=30`

## Steam client issues

### Cannot browse filesystem to add a library folder or library folder appears as empty

If the file chooser is empty when trying add a library folder, or if a previously set up folder now appears with 0 games installed, this can be the result of an incorrect timestamp on the root directory or in the library folder. Timestamps can be checked with _stat_:

$ stat _path_

If the timestamp is in the future, run

$ touch _path_

to reinitialize it to the current date, then re-run Steam.

### Cannot add library folder because of missing execute permissions

If you add another Steam library folder on another drive, you might get the error message:

New Steam library folder must be on a filesystem mounted with execute permissions

Make sure you are mounting the filesystem with the correct flags in your `/etc/fstab`, usually by adding `exec` to the list of mount parameter. The parameter must occur after any `user` or `users` parameter since these can imply `noexec`.

This error might also occur if your library folder does not contain a `steamapps` directory. Previous versions used `SteamApps` instead, so ensure the name is fully lowercase.

This error can also occur because of Steam runtime issues and may be fixed following the [#Finding missing runtime libraries](https://wiki.archlinux.org/title/Steam/Troubleshooting#Finding_missing_runtime_libraries) section or due to no space being left on the device. For debugging purposes it might be useful to run Steam from the console and observe the log.

### Unusually slow download speed

If your Steam (games, software…) download speed through the client is unusually slow, but browsing the Steam store and streaming videos is unaffected, installing a DNS cache program, such as [dnsmasq](https://wiki.archlinux.org/title/Dnsmasq "Dnsmasq") can help [[1]](https://steamcommunity.com/app/221410/discussions/2/616189106498372437/).

Something else that might help would be disabling [IPv6](https://wiki.archlinux.org/title/IPv6 "IPv6"). See [[2]](https://github.com/ValveSoftware/steam-for-linux/issues/6126) for more information.

Another potential fix is to disable HTTP2 [[3]](https://github.com/ValveSoftware/steam-for-linux/issues/10248) by creating the file:

~/.steam/steam/steam_dev.cfg@nClientDownloadEnableHTTP2PlatformLinux 0

To increase the server connections at the potential cost of negatively affecting speeds, add:

~/.steam/steam/steam_dev.cfg...
@fDownloadRateImprovementToAddAnotherConnection 1.0

### "Needs to be online" error

![Image 3](https://wiki.archlinux.org/images/4/4e/View-refresh-red.svg)**This article or section is out of date.**

**Reason:**_nscd_ was removed in [glibc 2.38-4](https://gitlab.archlinux.org/archlinux/packaging/packages/glibc/-/commit/25032a8abb2760257c3dceb78e649a0a2c4e3ab2). (Discuss in [Talk:Steam/Troubleshooting](https://wiki.archlinux.org/title/Talk:Steam/Troubleshooting))

![Image 4](https://wiki.archlinux.org/images/1/19/Tango-view-fullscreen.svg)**This article or section needs expansion.**

**Reason:** Unclear why enabling nscd can help (Discuss in [Talk:Steam/Troubleshooting#Needs to be online error: Enabling nscd.service](https://wiki.archlinux.org/title/Talk:Steam/Troubleshooting#Needs_to_be_online_error:_Enabling_nscd.service "Talk:Steam/Troubleshooting"))

If the Steam launcher refuses to start and you get an error saying: "_Fatal Error: Steam needs to be online to update_" while you are online, then there might be issues with name resolving.

Try installing [lib32-systemd](https://archlinux.org/packages/?name=lib32-systemd), [lib32-libcurl-compat](https://archlinux.org/packages/?name=lib32-libcurl-compat), [nss-mdns](https://archlinux.org/packages/?name=nss-mdns), [lib32-nss](https://archlinux.org/packages/?name=lib32-nss), [lib32-glu](https://archlinux.org/packages/?name=lib32-glu) or [lib32-dbus](https://archlinux.org/packages/?name=lib32-dbus).

This may also be as simple as DNS resolution not correctly working and is not always obvious since modern browsers will use their own DNS servers. Follow [Domain name resolution](https://wiki.archlinux.org/title/Domain_name_resolution "Domain name resolution").

Steam may have issues if _systemd-resolved_ is providing DNS resolution. Make sure [lib32-systemd](https://archlinux.org/packages/?name=lib32-systemd) is present to resolve this.

If DNS resolution works but the Steam launcher still shows the same error message, [enabling](https://wiki.archlinux.org/title/Enabling "Enabling") DNS caching e.g. via the "Name Service Caching Daemon", `nscd.service`, has shown to work around this issue.

It is unclear what exactly running `nscd` does to make it work again though. Please check the [talk page](https://wiki.archlinux.org/title/Talk:Steam/Troubleshooting#Needs_to_be_online_error:_Enabling_nscd.service "Talk:Steam/Troubleshooting") for more info.

### Steam forgets password

Related: [steam-for-linux#5030](https://github.com/ValveSoftware/steam-for-linux/issues/5030)
Steam for Linux has a bug which causes it to forget the password of some users.

As a workaround, after logging in to Steam, run

# chattr +i ~/.steam/registry.vdf

This will set the file's immutable bit so Steam cannot modify, delete, or rename it and thus not log you out.

### Preventing crash memory dumps

Every time Steam crashes, it writes a memory dump to `/tmp/dumps/`. If Steam falls into a crash loop, the dump files can become quite large. When `/tmp` is mounted as [tmpfs](https://wiki.archlinux.org/title/Tmpfs "Tmpfs"), memory and swap file can be consumed needlessly.

To prevent this, link `/tmp/dumps/` to `/dev/null`:

# ln -s /dev/null /tmp/dumps

Or alternatively, create and modify permissions on `/tmp/dumps`. Then Steam will be unable to write dump files to the directory.

# mkdir /tmp/dumps
# chmod 600 /tmp/dumps

This also has the added benefit of Steam not uploading these dumps to Valve's servers.

### Steam license problem with playing videos

Steam uses [Google's Widevine DRM](https://en.wikipedia.org/wiki/Widevine "w:Widevine") for some videos. If it is not installed you will get the following error:

This video requires a license to play which cannot be retrieved. This may be a temporary network condition. Please restart the video to try again.

To solve this issue follow the [_Streaming Videos on Steam_ support page](https://help.steampowered.com/en/faqs/view/2FC3-54BF-465F-E556#15).

### No context menu for joining/inviting friends

Since the new Steam Friends UI update, it may be the case that in the right-click menu the entries for "Join Game", "Invite to Game" and "View Game Info" are missing.

In order to fix this, it maybe be necessary to install [lsof](https://archlinux.org/packages/?name=lsof).

### Slow and unresponsive user interface

If you experience extremely slow and sluggish performance when using the Steam client it might help to disable the _Enable GPU accelerated rendering in web views_ option under the _Interface_ tab in the Steam client settings.

The friends list can also cause this problem. Two workarounds are mentioned in [https://github.com/ValveSoftware/steam-for-linux/issues/7245](https://github.com/ValveSoftware/steam-for-linux/issues/7245):

*   Moving the friends list to another monitor [[4]](https://github.com/ValveSoftware/steam-for-linux/issues/7245#issuecomment-663629964).
*   Disabling animated avatars. Open Settings and navigate to Friends & Chat. Set _Enable Animated Avatars & Animated Avatar Frames in your Friends List and Chat > OFF_[[5]](https://github.com/ValveSoftware/steam-for-linux/issues/7245#issuecomment-813443906).

### Steam fails to start correctly

One troubleshooting step is to run

$ steam --reset

This can fix various issues that come with a broken install.

### Missing taskbar menu

If clicking your Steam taskbar icon does not give you a menu, it may be necessary to install the [libappindicator-gtk2](https://aur.archlinux.org/packages/libappindicator-gtk2/)AUR and [lib32-libappindicator-gtk2](https://aur.archlinux.org/packages/lib32-libappindicator-gtk2/)AUR packages and restart Steam.

### "Your browser does not support the minimum set of features required to watch this broadcast" error

See [steam-for-linux issue 6780](https://github.com/ValveSoftware/steam-for-linux/issues/6780)

If you get an error stating "_Your browser does not support the minimum set of features required to watch this broadcast_" when attempting to watch a stream/broadcast try the following troubleshooting steps:

1.   Navigate to _Community > Broadcasts_. If the page displays "_Updating Steam_" wait a few minutes to see if the process completes and cancel it after a while in case it does not. Now test if you are able to watch broadcasts, e.g. by clicking on one of the ones display under _Community > Broadcasts_.
2.   Start a broadcast while in Big Picture mode (_View > Big Picture Mode_). If a broadcast starts fine while in Big Picture mode check if it still works after switching back to the main interface.
3.   Trigger the Steam client to directly unlock H.264 decoding using the following command: `steam steam://unlockh264/`. The Steam client should start headless and run the unlock command. Wait a minute to be sure the process completes, then close and relaunch the Steam client.

### Using system titlebar and frame

Currently Steam client tries to manage its windows itself, but it does it improperly, see [steam-for-linux#1040](https://github.com/ValveSoftware/steam-for-linux/issues/1040). As a workaround you can use [steamwm](https://github.com/dscharrer/steamwm) project. Run Steam like this: `./steamwm.cpp steam`. Also the project provides a skin that removes unnative control buttons and frame, but leaves default skin decorations.

### More selective DPMS inhibition

By default, the Steam client totally disables screensaving when it is running, whether a game is launched or not.

A workaround to this issue is provided by [steam-screensaver-fix](https://aur.archlinux.org/packages/steam-screensaver-fix/)AUR: run `steam-screensaver-fix-native` or `steam-screensaver-fix-runtime`.

This will allow your screen to turn off if Steam is running, but will keep inhibiting the screensaver if a game is launched.

See [Issue 5607](https://github.com/ValveSoftware/steam-for-linux/issues/5607) at Valve's GitHub for the details.

### Enabling fractional scaling

If the text and icons in the Steam client window are too small to read on your display, it may be beneficial to enable fractional scaling. The Steam client has a settings option to enable it, at _Settings > Interface > Scale text and icons to match monitor settings_. Enabling this should tell the client to use the system's fractional scaling settings.

However, if this does not automatically work, there is a command line parameter to force fractional scaling. Running Steam with the `-forcedesktopscaling 1.5` flag will scale the client to 1.5x size. This value can be changed to the correct scaling factor for your monitor. If you wish to make this change permanent, you can edit the `Exec` field in the `steam.desktop` file.

### Steam Beta crashes

If you are using Steam Beta (which can be confirmed with the presence of `You are in the 'publicbeta' client beta` in the logs) and encounter breaking bugs, manually switch back to non-Beta:

$ rm -f ~/.local/share/Steam/package/beta

Report the issue after looking for duplicates at [https://github.com/ValveSoftware/steam-for-linux](https://github.com/ValveSoftware/steam-for-linux).

### Cannot access store page (client displays error -105 or -102)

If the store page is inaccessible but other networking features (such as game downloads) are working, it may be a DNS resolution failure. A possible solution is to ensure [systemd-resolved](https://wiki.archlinux.org/title/Systemd-resolved "Systemd-resolved") is enabled and started, then create the `/etc/resolv.conf` symlink as explained in [systemd-resolved#DNS](https://wiki.archlinux.org/title/Systemd-resolved#DNS "Systemd-resolved").

Other solution would be to flush DNS as explained here [[6]](https://askubuntu.com/a/929478):

Run `resolvectl flush-caches` or `systemd-resolve --flush-caches` as root.

### Steam client restarts while a game is running

A work around is to disable the _Enable GPU accelerated rendering in web views_ option under the _Interface_ tab in the Steam client settings.

## Steam Remote Play issues

See [Steam#Steam Remote Play](https://wiki.archlinux.org/title/Steam#Steam_Remote_Play "Steam").

### Remote Play does not work from Arch Linux host to Arch Linux guest

Chances are you are missing [lib32-libcanberra](https://archlinux.org/packages/?name=lib32-libcanberra). Once you [install](https://wiki.archlinux.org/title/Install "Install") that, it should work as expected.

With that, Steam should no longer crash when trying to launch a game through Remote Play.

### Hardware decoding not available

Remote Play hardware decoding uses `vaapi`, but Steam requires `libva2_32bit`, where as Arch defaults to 64bit.

As a basic set, this is [libva](https://archlinux.org/packages/?name=libva) and [lib32-libva](https://archlinux.org/packages/?name=lib32-libva). Intel graphics users will also require both [libva-intel-driver](https://archlinux.org/packages/?name=libva-intel-driver) and [lib32-libva-intel-driver](https://archlinux.org/packages/?name=lib32-libva-intel-driver).

For more information about vaapi see [hardware video acceleration](https://wiki.archlinux.org/title/Hardware_video_acceleration "Hardware video acceleration").

It may also be necessary to remove the Steam runtime version of libva, in order to force it to use system libraries. The current library in use can be found by using:

$ pgrep steam | xargs -I {} cat /proc/{}/maps | grep libva

If this shows locations in `~/.local/Share/steam` Steam is still using its packaged version of libva. This can be rectified by deleting the libva library files at `~/.local/share/Steam/ubuntu12_32/steam-runtime/i386/usr/lib/i386-linux-gnu/libva*`, so that Steam falls back to the system libraries.

### Big Picture Mode minimizes itself after losing focus

This can occur when you play a game via Remote Play or if you have a multi-monitor setup and move the mouse outside of BPM's window. To prevent this, set the `SDL_VIDEO_MINIMIZE_ON_FOCUS_LOSS=0` environment variable and restart Steam.

See also the [steam-for-linux issue 4769](https://github.com/ValveSoftware/steam-for-linux/issues/4769).

## Other issues

### Steam Library in NTFS partition

**Note** The kernel [NTFS](https://wiki.archlinux.org/title/NTFS "NTFS") driver is vastly more performant than the [NTFS-3G](https://wiki.archlinux.org/title/NTFS-3G "NTFS-3G")[FUSE](https://wiki.archlinux.org/title/FUSE "FUSE") driver [[7]](https://openbenchmarking.org/result/2009092-NE-NTFSCOMPA56). So, for gaming scenario, it is better to uninstall [ntfs-3g](https://archlinux.org/packages/?name=ntfs-3g) package as GUI file managers driven by [udisks](https://wiki.archlinux.org/title/Udisks "Udisks") prefer it when present.

If your Steam library resides in [NTFS](https://wiki.archlinux.org/title/NTFS "NTFS") partition it is probable that games residing there could not start.

The trouble is that Wine uses a colon in its `$WINEPREFIX/dosdevices` directory, and when mounted with the `windows_names` option, is instructed to not create such colon names which can confuse Windows. Not adding it is not that unsafe: Windows will act fine besides being unable to open the symlink (which it will not need to do anyways); `chkdsk` may delete the link, but it is easily recreated.

Better workaround: mount without `windows_names`. This option is often added by GUI file explorers via [udisks](https://wiki.archlinux.org/title/Udisks "Udisks") for caution, but adding a real [fstab](https://wiki.archlinux.org/title/Fstab "Fstab") line will give it a proper way to do so.

1.   Run `genfstab -U /` and extract the line containing the ntfs partition, e.g. UUID=12345678ABCDEF00 /run/media/user/Gamez ntfs3 rw,uid=1000,gid=1000,windows_names 0 0
2.   Write the line into `/etc/fstab`, editing it to use the proper options without `windows_names`. With the earlier example, we would write UUID=12345678ABCDEF00 /run/media/user/Gamez ntfs3 rw,uid=1000,gid=1000 0 0
3.   Unmount the partition, then remount.

Alternatively, disable udisks use of `windows_names` globally following instructions in [udisks#NTFS file creation failing (filename-dependent)](https://wiki.archlinux.org/title/Udisks#NTFS_file_creation_failing_(filename-dependent) "Udisks").

Other workaround: move the `steamapps/common/Proton x.y` and `steamapps/compatdata` to a non-NTFS drive, then create symbolic link in their original locations. You may be wasting some space on your otherwise important Linux drive, however.

$ mv _SteamLibrary_/steamapps/common/Proton\ _x_._y_ /home/_user_/dir/
$ mv _SteamLibrary_/steamapps/compatdata /home/_user_/dir/
$ ln -s /home/_user_/dir/Proton\ _x_._y_/ _SteamLibrary_/steamapps/common/Proton\ _x_._y_
$ ln -s /home/_user_/dir/compatdata/ _SteamLibrary_/steamapps/compatdata

### Wrong ELF class

If you see this message in Steam's console output

ERROR: ld.so: object '~/.local/share/Steam/ubuntu12_32/gameoverlayrenderer.so' from LD_PRELOAD cannot be preloaded (wrong ELF class: ELFCLASS32): ignored.

you can safely ignore it. It is not really any error: Steam includes both 64- and 32-bit versions of some libraries and only one version will load successfully. This "error" is displayed even when Steam (and the in-game overlay) is working perfectly.

### Multiple monitors setup

![Image 5](https://wiki.archlinux.org/images/1/19/Tango-view-fullscreen.svg)**This article or section needs expansion.**

**Reason:** Is this Nvidia-only? Can this be reproduced by anyone? Is there an upstream report? (Discuss in [Talk:Steam/Troubleshooting](https://wiki.archlinux.org/title/Talk:Steam/Troubleshooting))

A setup with multiple monitors may prevent games from starting. Try to disable all additional displays, and then run a game. You can enable them after the game successfully started.

Also you can try running Steam with this environment variable set:

$ export LD_LIBRARY_PATH=/usr/lib32/nvidia:/usr/lib/nvidia:$LD_LIBRARY_PATH

### Text is corrupt or missing

Try installing [lib32-fontconfig](https://archlinux.org/packages/?name=lib32-fontconfig), [ttf-liberation](https://archlinux.org/packages/?name=ttf-liberation), [xorg-fonts-misc](https://archlinux.org/packages/?name=xorg-fonts-misc) (Steam updater window shows void squares instead of all non-Latin characters if this package not installed) and [wqy-zenhei](https://archlinux.org/packages/?name=wqy-zenhei) (for Asian characters), then restart Steam to see whether the problem is solved.

**Note**
*   Steam for Linux does not follow system-level font configurations.[[8]](https://github.com/ValveSoftware/steam-for-linux/issues/10422#issuecomment-1944396010) Thus, modify user-level configuration if you want change fontconfig for Steam.
*   When Steam cannot find the Arial font, font-config likes to fall back onto the Helvetica bitmap font. Steam does not render this and possibly other bitmap fonts correctly, so either removing problematic fonts or [disabling bitmap fonts](https://wiki.archlinux.org/title/Font_configuration#Disable_bitmap_fonts "Font configuration") will most likely fix the issue without installing the Arial or ArialBold fonts. The font being used in place of Arial can be found with the command: $ fc-match -v Arial

### SetLocale('en_US.UTF-8') fails at game startup or typing non-ASCII characters does not work in the Steam client

You need to generate the `en_US.UTF-8 UTF-8` locale. See [Locale#Generating locales](https://wiki.archlinux.org/title/Locale#Generating_locales "Locale").

### Missing libc

![Image 6](https://wiki.archlinux.org/images/0/0b/Inaccurate.svg)**The factual accuracy of this article or section is disputed.**

**Reason:** Issue #3730 is closed. Is `$HOME` ending in a slash still relevant? (Discuss in [Talk:Steam/Troubleshooting](https://wiki.archlinux.org/title/Talk:Steam/Troubleshooting))

This could be due to a corrupt Steam executable. Check the output of:

$ ldd ~/.local/share/Steam/ubuntu12_32/steam

Should `ldd` claim that it is not a dynamic executable, then Steam likely corrupted the binary during an update. The following should fix the issue:

$ cd ~/.local/share/Steam/
$ ./steam.sh --reset

If it does not, try to delete the `~/.local/share/Steam/` directory and launch Steam again, telling it to reinstall itself.

This error message can also occur due to a bug in Steam which occurs when your `$HOME` directory ends in a slash (Valve GitHub [issue 3730](https://github.com/ValveSoftware/steam-for-linux/issues/3730)). This can be fixed by editing `/etc/passwd` and changing `/home/username/` to `/home/username`, then logging out and in again. Afterwards, Steam should repair itself automatically.

### Games do not launch on older Intel hardware

[source](https://steamcommunity.com/app/8930/discussions/1/540744299927655197/)
On older Intel hardware which does not support OpenGL 3, such as Intel GMA chips or Westmere CPUs, games may immediately crash when run. It appears as a `gameoverlayrenderer.so` error in `/tmp/dumps/mobile_stdout.txt`, but looking in `/tmp/gameoverlayrenderer.log` it shows a GLXBadFBConfig error.

This can be fixed, by forcing the game to use a later version of OpenGL than it wants. Add `MESA_GL_VERSION_OVERRIDE=3.1 MESA_GLSL_VERSION_OVERRIDE=140` to your [launch options](https://wiki.archlinux.org/title/Launch_option "Launch option").

### Mesa: Game does not launch, complaining about OpenGL version supported by the card

Some games are badly programmed, to use any OpenGL version above 3.0. With Mesa, an application has to request a specific core profile. If it does not make such a request, only OpenGL 3.0 and lower are available.

This can be fixed, by forcing the game to use a version of OpenGL it actually needs. Add `MESA_GL_VERSION_OVERRIDE=4.1 MESA_GLSL_VERSION_OVERRIDE=410` to your [launch options](https://wiki.archlinux.org/title/Launch_option "Launch option").

### 2K games do not run on XFS partitions

![Image 7](https://wiki.archlinux.org/images/1/19/Tango-view-fullscreen.svg)**This article or section needs expansion.**

**Reason:** Seems to be a general issue, e.g. [[9]](https://github.com/ValveSoftware/Source-1-Games/issues/1685) (Discuss in [Talk:Steam/Troubleshooting](https://wiki.archlinux.org/title/Talk:Steam/Troubleshooting))

If you are running 2K games such as Civilization 5 on [XFS](https://wiki.archlinux.org/title/XFS "XFS") partitions, then the game may not start or run properly due to how the game loads files as it starts. [[10]](https://bbs.archlinux.org/viewtopic.php?id=185222)

### Steam controller not being detected correctly

See [Gamepad#Steam Controller](https://wiki.archlinux.org/title/Gamepad#Steam_Controller "Gamepad").

### Steam controller makes a game crash

See [Gamepad#Steam Controller makes a game crash or not recognized](https://wiki.archlinux.org/title/Gamepad#Steam_Controller_makes_a_game_crash_or_not_recognized "Gamepad").

### Steam hangs on "Installing breakpad exception handler..."

[BBS#177245](https://bbs.archlinux.org/viewtopic.php?id=177245)

You have an NVIDIA GPU and Steam has the following output:

Running Steam on arch rolling 64-bit
STEAM_RUNTIME is enabled automatically
Installing breakpad exception handler for appid(steam)/version(0_client)

Then nothing else happens. Ensure you have the correct drivers installed as well as their 32-bit versions (the 64-bit and 32-bit variants have to have the same versions): see [NVIDIA#Installation](https://wiki.archlinux.org/title/NVIDIA#Installation "NVIDIA").

### Killing standalone compositors when launching games

Utilizing the `%command%` switch, you can kill standalone compositors (such as [Xcompmgr](https://wiki.archlinux.org/title/Xcompmgr "Xcompmgr") or [picom](https://wiki.archlinux.org/title/Picom "Picom")) - which can cause lag and tearing in some games on some systems - and relaunch them after the game ends by adding the following to your game's launch options.

killall _compositor_ && %command%; nohup _compositor_ &

You can also add -options to `%command%` or `compositor`, of course.

Steam will latch on to any processes launched after `%command%` and your Steam status will show as in game. So in this example, we run the compositor through `nohup` so it is not attached to Steam (it will keep running if you close Steam) and follow it with an ampersand so that the line of commands ends, clearing your Steam status.

If your compositor supports running in daemon mode, you can use it instead. For example, [picom(1)](https://man.archlinux.org/man/picom.1) has the `--daemon` / `-b` option to daemonize its process:

killall picom && %command%; picom -b

### Symbol lookup error using DRI3

Steam outputs this error and exits.

symbol lookup error: /usr/lib/libxcb-dri3.so.0: undefined symbol: xcb_send_request_with_fds

To work around this, run Steam with `LIBGL_DRI3_DISABLE=1`, disabling DRI3 for Steam.

### Launching games on NVIDIA Optimus laptops

![Image 8](https://wiki.archlinux.org/images/4/4e/View-refresh-red.svg)**This article or section is out of date.**

**Reason:** Was for using bumblebee: what is the equivalent for recommended setup now, using prime-run instead does not work (Discuss in [Talk:Steam/Troubleshooting](https://wiki.archlinux.org/title/Talk:Steam/Troubleshooting))

To be able to play games which require using NVIDIA GPU (for example, Hitman 2016) on Optimus enabled laptop, you should start game with _primusrun_ prefix in launch options. Otherwise, game will not work.

Right click the game in your Steam library and select _Properties > GENERAL > LAUNCH OPTIONS_. Change options to

primusrun %command%

Running Steam with _primusrun_ used to work. While Steam has changed some behavior that now running Steam with _primusrun_ would not have effect on launching games. As a result, you need to set launch options for each game (and you do NOT have to run Steam with _primusrun_).

For _primusrun_, VSYNC is enabled by default it could result in a mouse input delay lag, slightly decrease performance and in-game FPS might be locked to a refresh rate of a monitor/display. In order to disable VSYNC for primusrun default value of option `vblank_mode` needs to be overridden by environment variable.

vblank_mode=0 primusrun %command%

Same with optirun that uses primus as a bridge.

vblank_mode=0 optirun -b primus %command%

If that did not work try:

LD_PRELOAD="libpthread.so.0 libGL.so.1" __GL_THREADED_OPTIMIZATIONS=1 optirun %command%

For more details see [Bumblebee#Primusrun mouse delay (disable VSYNC)](https://wiki.archlinux.org/title/Bumblebee#Primusrun_mouse_delay_(disable_VSYNC) "Bumblebee").

### HiDPI

[HiDPI](https://wiki.archlinux.org/title/HiDPI "HiDPI") support should work out of the box, although on some systems it is necessary to [force it](https://wiki.archlinux.org/title/HiDPI#Steam "HiDPI") setting the `-forcedesktopscaling factor` cli option or the `STEAM_FORCE_DESKTOPUI_SCALING` environment variable to set the desired scale factor.

### Protocol support under KDE Plasma

If you are getting an error after running a game through web browser _(or executing the link through xdg-open)_

Error — KIOExec
File not found: steam://run/440

Go to _System Settings -> Applications -> File Associations_, add new, select `inode` group and name it `vnd.kde.service.steam`, then under _Application Preference Order_ you have to add Steam. Apply changes, It should be working now.

### The game crashes when using Steam Linux Runtime - Soldier

![Image 9](https://wiki.archlinux.org/images/4/4e/View-refresh-red.svg)**This article or section is out of date.**

**Reason:** Was a fix 2021-03: is this still relevant today? (Discuss in [Talk:Steam/Troubleshooting](https://wiki.archlinux.org/title/Talk:Steam/Troubleshooting))

Since Proton 5.13 Steam uses the Steam Linux Runtime - Soldier by default. Some games crash when using it.

To bypass it, you can:

*   Manually [build](https://github.com/ValveSoftware/Proton#alternative-building-without-the-steam-runtime) a proton without the Steam Runtime
*   Replace the Soldier entry point script:

~/.steam/steam/steamapps/common/SteamLinuxRuntime_soldier/_v2-entry-point#!/bin/bash

shift 2
exec "${@}"

### Games running with Proton 5.13+ have no Internet connectivity

If you are using [systemd-resolved](https://wiki.archlinux.org/title/Systemd-resolved "Systemd-resolved") as your DNS resolver, ensure you have created the `resolv.conf` symlink as described in [systemd-resolved#DNS](https://wiki.archlinux.org/title/Systemd-resolved#DNS "Systemd-resolved").

The file should contain something similar to:

/etc/resolv.conf# This is /run/systemd/resolve/stub-resolv.conf managed by man:systemd-resolved(8).
# Do not edit.

### "could not determine 32/64 bit of java"

A forgotten install of the [linux-steam-integration](https://aur.archlinux.org/packages/linux-steam-integration/)AUR package caused this with at least one game. Early on there were conflicts between the system and the Steam runtime versions of some libraries, and that package helped resolve some of them. It is unclear whether it is still helpful, but uninstalling it resolved the above error message for Project Zomboid. The solution was discovered by noticing that running the `projectzomboid.sh` command from the command line worked, but switching the launch options to `sh -xc 'echo %command%; declare -p'` showed Steam was trying to run the exact same command, but there were a lot of `lsi-`-prefixed libraries inserted in the preload and path.

### Stuttering with Vulkan

If you notice a constant intense stutter every 1-2 seconds, there may be conflicts in your vsync settings. Manually configuring vsync in the parameters will possibly fix it.

Go to the game properties and configure it in Launch Options:

DXVK_FRAME_RATE=60 %command%

### Force OpenGL emulation

Some, especially older games might not work with the default Vulkan ([DXVK](https://wiki.archlinux.org/title/DXVK "DXVK")) wrapper Proton uses. Try running the application with WineD3D OpenGL wrapper instead:

PROTON_USE_WINED3D=1 %command%

### File picker does not see anything but Steam library

See [FS#78625](https://bugs.archlinux.org/task/78625). You need to install [xdg-desktop-portal](https://archlinux.org/packages/?name=xdg-desktop-portal).

### DirectX errors on hybrid graphics

For laptop with Intel/NVIDIA [Hybrid graphics](https://wiki.archlinux.org/title/Hybrid_graphics "Hybrid graphics") encountering the following error:

A d3d11-compatible gpu (feature level 11.0, shader model 5.0) is required to run the engine.

It's probably because your game is running on the iGPU instead of the dedicated GPU and you need to configure [PRIME](https://wiki.archlinux.org/title/PRIME "PRIME"). If it's still not doing it, try using [Direct3D instead of DXVK](https://wiki.archlinux.org/title/Steam/Troubleshooting#Force_OpenGL_emulation).

### No Internet Connection when downloading

If you see _No Internet Connection_ while downloading games, a possible solution is clearing the download cache (_Steam > Settings > Downloads > Clear Download Cache_).

### Poor performance or stuttering after launching Steam

If you experience reduced performance or stuttering, lasting anywhere from a few seconds to a couple of minutes after launching Steam, it may be cased by bugged or outdated Proton installations.

Remove bugged Proton installed under app ID 0: `~/.steam/root/steamapps/compatdata/0`. You may also need to remove outdated and problematic Proton versions, including custom ones like GE-Proton, especially `5.21-GE-1`.

For more details, see [steam-for-linux#8114](https://github.com/ValveSoftware/steam-for-linux/issues/8114).

### Very long startup and slow user interface response

Steam's use of `steamloopback.host` in its Chromium backend to refer to itself. Due to the way _systemd-resolvd_ attempts to resolve this host (via `mdns` by default for some users) this issue can hang the interface. This causes very long startups (if it ever starts) and a slow-responding (or not at all) user interface. This issue can temporary be addressed by editing `/etc/nsswitch.conf` to change `mdns` to `mdns_minimal` and restarting systemd-resolvd. For more details, see [[11]](https://github.com/ValveSoftware/steam-for-linux/issues/10879).

## See also

*   [Multimedia and Games / Arch Linux Forums](https://bbs.archlinux.org/viewforum.php?id=32)
*   [ValveSoftware/steam-for-linux](https://github.com/ValveSoftware/steam-for-linux) – Issue tracking for the Steam for Linux client
*   [Steam Community discussions of the game](https://steamcommunity.com/)
*   [Steam Support FAQ](https://help.steampowered.com/en/)

Retrieved from "[https://wiki.archlinux.org/index.php?title=Steam/Troubleshooting&oldid=868814](https://wiki.archlinux.org/index.php?title=Steam/Troubleshooting&oldid=868814)"

[Category](https://wiki.archlinux.org/title/Special:Categories "Special:Categories"): 
*   [Gaming](https://wiki.archlinux.org/title/Category:Gaming "Category:Gaming")

Hidden categories: 
*   [Pages or sections flagged with Template:Out of date](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Out_of_date "Category:Pages or sections flagged with Template:Out of date")
*   [Pages or sections flagged with Template:Accuracy](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Accuracy "Category:Pages or sections flagged with Template:Accuracy")
*   [Pages or sections flagged with Template:Expansion](https://wiki.archlinux.org/title/Category:Pages_or_sections_flagged_with_Template:Expansion "Category:Pages or sections flagged with Template:Expansion")

*    This page was last edited on 12 March 2026, at 02:33.
*   Content is available under [GNU Free Documentation License 1.3 or later](https://www.gnu.org/copyleft/fdl.html) unless otherwise noted.

*   [Privacy policy](https://terms.archlinux.org/docs/privacy-policy/)
*   [About ArchWiki](https://wiki.archlinux.org/title/ArchWiki:About)
*   [Disclaimers](https://wiki.archlinux.org/title/ArchWiki:General_disclaimer)
*   [Code of conduct](https://terms.archlinux.org/docs/code-of-conduct/ "archlinux-service-agreements:code-of-conduct")
*   [Terms of service](https://terms.archlinux.org/docs/terms-of-service/ "archlinux-service-agreements:terms-of-service")

*   [![Image 10: GNU Free Documentation License 1.3 or later](https://wiki.archlinux.org/resources/assets/licenses/gnu-fdl.png)](https://www.gnu.org/copyleft/fdl.html)
*   ![Image 11](https://wiki.archlinux.org/resources/assets/mediawiki_compact.svg)

Search

Search

- [x] Toggle the table of contents 

Steam/Troubleshooting

[](https://wiki.archlinux.org/title/Steam/Troubleshooting#)[](https://wiki.archlinux.org/title/Steam/Troubleshooting#)[](https://wiki.archlinux.org/title/Steam/Troubleshooting#)[](https://wiki.archlinux.org/title/Steam/Troubleshooting#)[](https://wiki.archlinux.org/title/Steam/Troubleshooting#)[](https://wiki.archlinux.org/title/Steam/Troubleshooting#)[](https://wiki.archlinux.org/title/Steam/Troubleshooting#)[](https://wiki.archlinux.org/title/Steam/Troubleshooting#)

[Add topic](https://wiki.archlinux.org/title/Steam/Troubleshooting#)
