from os import open, close, read, write, fstat,  \
                            O_RDONLY, O_WRONLY, O_CREAT, O_EXCL
from sys import stdin, stdout, stderr
from stat import S_IRUSR, S_IWUSR
from ctypes import CDLL

@ifndef BUFFER_LENGTH
@define BUFFER_LENGTH       1024
@endif

@define EXIT_FAILURE        1

@define COPY_LAST_MOD       1
@define COPY_USER_ID        2
@define COPY_GROUP_ID       4
@define COPY_PERMISSIONS    8

libc = CDLL(None)

def copy_file(src: str, dst: str, options=0) -> None:
    src_fd = open(src, O_RDONLY)
    dst_fd = open(dst, O_WRONLY | O_CREAT | O_EXCL)

    if src_fd < 0:
        sys.stderr.write("Error occured reading source file\n")
        exit(EXIT_FAILURE)
    elif dst_fd < 0:
        sys.stderr.write("Error occured opening destination file\n")
        exit(EXIT_FAILURE)

    read_bytes = bytearray(BUFFER_LENGTH)

    while read_bytes:
        read_bytes = read(src_fd, BUFFER_LENGTH)
        write(dst_fd, read_bytes)

    if (options & COPY_LAST_MOD):
        last_mod = fstat(src_fd).st_mtime
        last_acc = fstat(src_fd).st_atime

    close(src_fd)
    close(dst_fd)

