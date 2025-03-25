# vpn-orchestrator
```
vpn-orchestrator/
│── src/
│   ├── main.rs  (Ana giriş noktası)
│   ├── vpn/
│   │   ├── mod.rs  (Tüm VPN yönetimini yöneten modül)
│   │   ├── wireguard.rs  (WireGuard bağlantı yönetimi)
│   │   ├── openvpn.rs  (OpenVPN bağlantı yönetimi)
│   │   ├── shadowsocks.rs  (Shadowsocks bağlantı yönetimi)
│   │   ├── v2ray.rs  (V2Ray bağlantı yönetimi)
│   │   ├── l2tp.rs  (L2TP/IPSec bağlantı yönetimi)
│   ├── network/
│   │   ├── mod.rs  (Ağ işlemleri modülü)
│   │   ├── ping.rs  (Sunucuların ping sürelerini hesaplama)
│   │   ├── mac_spoof.rs  (MAC adresi değiştirme)
│   │   ├── dpi_bypass.rs  (DPI bypass teknikleri)
│   ├── config.rs  (Ayarları yöneten modül)
│── Cargo.toml  (Rust bağımlılıkları)
```