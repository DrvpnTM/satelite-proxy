# DrVPN Desktop for Windows and Linux

نسخه دسکتاپ دکتر وی‌پی‌ان برای Windows و Linux از یک کدپایه ساخته می‌شود. پایه برنامه [Satelite](https://github.com/zn0wii/satelite-proxy) است و از هسته‌های sing-box، Xray و mihomo استفاده می‌کند.

## امکانات DrVPN

- اشتراک رایگان DrVPN به‌صورت پیش‌فرض اضافه و هر ۶ ساعت به‌روزرسانی می‌شود.
- لینک اشتراک خریداری‌شده از [drvpn.net](https://drvpn.net/) را می‌توانید از بخش افزودن کانفیگ وارد کنید.
- واردکردن کانفیگ و اشتراک‌های سازگار با هسته‌های sing-box، Xray و mihomo.
- اتصال از طریق System Proxy یا TUN و انتخاب سرور و هسته.

در نسخه اندروید DrVPN، اشتراک‌های پولی با لینک اشتراک وارد می‌شوند و API ورود یا حساب کاربری در سورس وجود ندارد؛ نسخه دسکتاپ نیز از همان روش لینک اشتراک استفاده می‌کند.

## ساخت نسخه Windows

نیازمندی‌ها: Windows 10/11، Rust با ابزار MSVC، Node.js، pnpm و PowerShell 7.

```powershell
pnpm install --frozen-lockfile
pwsh scripts/build-windows.ps1
```

اسکریپت به‌طور پیش‌فرض نصب‌کننده NSIS می‌سازد. برای نسخه قابل‌حمل:

```powershell
pwsh scripts/build-windows.ps1 -Bundle portable
```

## ساخت نسخه Linux

نسخه Linux از همین رابط و کد Rust استفاده می‌کند و به‌صورت AppImage بسته‌بندی می‌شود. در Ubuntu/Debian ابتدا پیش‌نیازهای Tauri را نصب کنید:

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf xdg-utils
```

سپس هسته‌ها و Rule Setهای موردنیاز را دریافت و AppImage را بسازید:

```bash
./scripts/fetch-bundled-core-linux-amd64.sh
./scripts/fetch-bundled-xray-linux-amd64.sh
./scripts/fetch-bundled-mihomo-linux-amd64.sh
./scripts/fetch-bundled-rule-sets.sh
pnpm install --frozen-lockfile
pnpm tauri build --target x86_64-unknown-linux-gnu --config src-tauri/tauri.linux.conf.json
```

فایل AppImage در `src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/appimage/` ساخته می‌شود. گردش‌کار انتشار GitHub نیز برای Windows x64 و Linux x64 از همین پروژه build جداگانه می‌گیرد.

## پایه و مجوز

این پروژه از [Satelite Proxy](https://github.com/zn0wii/satelite-proxy) مشتق شده است. مجوز و اعلان‌های کپی‌رایت بالادستی در فایل [LICENSE](./LICENSE) حفظ شده‌اند. نسخه اصلی README برای ارجاع فنی در [README-Satelite.md](./README-Satelite.md) نگه‌داری شده است.
