pub const WANNACRY_EXTENSIONS: &[&str] = &[
    // Certificates & keys
    ".der", ".pfx", ".key", ".crt", ".csr", ".p12", ".pem",
    // OpenDocument / LibreOffice
    ".odt", ".ott", ".sxw", ".stw", ".uot",
    ".ods", ".ots", ".sxc", ".stc", ".dif", ".slk", ".wb2",
    ".odp", ".otp", ".sxd", ".std", ".uop",
    ".odg", ".otg", ".sxm", ".mml",
    // 3D
    ".3ds", ".max", ".3dm",
    // Misc docs
    ".lay", ".lay6", ".asc",
    // Databases
    ".sqlite3", ".sqlitedb", ".sql", ".accdb", ".mdb", ".db",
    ".dbf", ".odb", ".frm", ".myd", ".myi", ".ibd", ".mdf", ".ldf",
    // Dev / source code
    ".sln", ".suo", ".cs", ".c", ".cpp", ".pas", ".h", ".asm",
    ".js", ".cmd", ".bat", ".ps1", ".vbs", ".vb", ".pl",
    ".dip", ".dch", ".sch", ".brd", ".jsp", ".php", ".asp",
    ".rb", ".java", ".jar", ".class", ".sh",
    // Audio / Video
    ".mp3", ".wav", ".swf", ".fla", ".wmv", ".mpg", ".vob",
    ".mpeg", ".asf", ".avi", ".mov", ".mp4", ".3gp", ".mkv",
    ".3g2", ".flv", ".wma", ".mid", ".m3u", ".m4u",
    // Images / Graphics
    ".djvu", ".svg", ".ai", ".psd", ".nef", ".tiff", ".tif",
    ".cgm", ".raw", ".gif", ".png", ".bmp", ".jpg", ".jpeg",
    // Archives / backups
    ".vcd", ".iso", ".backup", ".zip", ".rar", ".7z", ".gz",
    ".tgz", ".tar", ".bak", ".tbk", ".bz2", ".PAQ", ".ARC",
    // Crypto
    ".aes", ".gpg",
    // Virtual machines
    ".vmx", ".vmdk", ".vdi",
    // Office (MS)
    ".sldm", ".sldx", ".sti", ".sxi", ".602", ".hwp", ".snt",
    ".onetoc2", ".dwg", ".pdf", ".wk1", ".wks", ".123", ".rtf",
    ".csv", ".txt", ".vsdx", ".vsd",
    // Email
    ".edb", ".eml", ".msg", ".ost", ".pst",
    // PowerPoint
    ".potm", ".potx", ".ppam", ".ppsx", ".ppsm", ".pps",
    ".pot", ".pptm", ".pptx", ".ppt",
    // Excel
    ".xltm", ".xltx", ".xlc", ".xlm", ".xlt", ".xlw",
    ".xlsb", ".xlsm", ".xlsx", ".xls",
    // Word
    ".dotx", ".dotm", ".dot", ".docm", ".docb", ".docx", ".doc",
];

pub fn is_wannacry_target(extension: &str) -> bool {
    let ext_lower = extension.to_lowercase();
    WANNACRY_EXTENSIONS
        .iter()
        .any(|e| e.to_lowercase() == ext_lower)
}