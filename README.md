# BruteX — Log Scanner & Credential Checker

[![License: MPL-2.0](https://img.shields.io/badge/License-MPL2.0-8af7ff.svg)](https://opensource.org/license/mpl-2-0)
[![Visitors](https://visitor-badge.laobi.icu/badge?page_id=southxz.BruteX)](https://github.com/southxz/BruteX)

---
⚠️ **Legal Notice**  
Use this software **only** on systems where you have explicit permission. Unauthorized use may be illegal.

## Main Features

- High‑performance log file chunk reading  
- Automatic extraction of discovered credentials  
- Authentication testing for:  
  - FTP  
  - MySQL  
  - SSH  
  - WordPress  
  - cPanel / Webmail  
  - phpMyAdmin  
- Multi‑threading powered by **Rayon**  
- Automatic reporting saved to `.txt` files  

# Configuration (`config.json`)

```json
{
    "default_threads": 10,
    "openfile_chuks_mb": 10,
    "keys_search_logs": ["login", "user", "pass"],
    "default_login": true
}
```

### Field Description

- **`default_threads`** — Number of Rayon worker threads  
- **`openfile_chuks_mb`** — File read buffer size (in MB)  
- **`keys_search_logs`** — Keywords used to detect credentials in logs  
- **`default_login`** — If enabled, automatically tests default login combinations

## **1. Run the Program**
A window will appear asking you to select a `.txt` log file:

```
Select File Logs
```

## **2. The system will:**

- Read the log file in chunks  
- Automatically extract detected credentials  
- Test them in parallel against supported services  
- Save all successful logins into:

```
ftp_logade_sucess.txt
mysql_logade_sucess.txt
sshlogade.txt
wp_sucess.txt
cpanel_webmail.txt
phpmyadmin_sucess.txt
```

# Internal Logic

### **`openfile_return_buffer()`**
Reads the log file in chunks and stores unique credentials inside a `HashSet<Login>`.

### **Automatic Authentication Attempts**
Each credential is tested using parallel threads:

```
mysql_login_
ftp_login_
sshLogin
cp_panel_and_webmail
wp_login
phpmyadmin
```

### **Internal IP Filtering**
Lines containing internal/local IPs are ignored automatically:

- 192.168.x.x  
- 127.0.x.x  
- localhost  
- 10.x.x.x
