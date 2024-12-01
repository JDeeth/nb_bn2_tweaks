copy /b build.rs +,,
cargo build --release
@del compiled\nimbus-bn2-tweaks\win_x64\nimbus-bn2-tweaks.xpl
@copy target\release\nimbus_bn2_tweaks.dll compiled\nimbus-bn2-tweaks\win_x64\nimbus-bn2-tweaks.xpl
@del compiled\nimbus-bn2-tweaks\README.md
@copy README.md compiled\nimbus-bn2-tweaks\README.md
@rmdir /s /q compiled\nimbus-bn2-tweaks\img
@robocopy img compiled\nimbus-bn2-tweaks\img /E > nul
del "C:\X-Plane\12\Aircraft\Nimbus BN-2B Islander\plugins\nimbus-bn2-tweaks\win_x64\nimbus-bn2-tweaks.xpl"
copy target\release\nimbus_bn2_tweaks.dll "C:\X-Plane\12\Aircraft\Nimbus BN-2B Islander\plugins\nimbus-bn2-tweaks\win_x64\nimbus-bn2-tweaks.xpl"
