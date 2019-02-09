pub const FORK: usize = 1;
pub const EXIT: usize = 2;
pub const WAIT: usize = 3;
pub const PIPE: usize = 4;
pub const READ: usize = 5;
pub const KILL: usize = 6;
pub const EXEC: usize = 7;
pub const FSTAT: usize = 8;
pub const CHDIR: usize = 9;
pub const DUP: usize = 10;
pub const GETPID: usize = 11;
pub const SBRK: usize = 12;
pub const SLEEP: usize = 13;
pub const UPTIME: usize = 14;
pub const OPEN: usize = 15;
pub const WRITE: usize = 16;
pub const MKNOD: usize = 17;
pub const UNLINK: usize = 18;
pub const LINK: usize = 19;
pub const MKDIR: usize = 20;
pub const CLOSE: usize = 21;

pub const O_READ: usize = 0x0000;
pub const O_WRITE: usize = 0x0001;
pub const O_RDWR: usize = 0x0002;
pub const O_CREATE: usize = 0x0200;
