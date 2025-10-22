from pathlib import Path
import random

# --------------------------
# Constants
# --------------------------
OUTPUT_DIR = Path("random")
OUTPUT_DIR.mkdir(exist_ok=True)
TARGET_SIZE = 1 * 1024 * 1024  # 1 MB
BLOCK_SIZE = 16  # bytes per block


# --------------------------
# Helper functions
# --------------------------
def random_single_byte() -> bytes:
    """Return one random printable ASCII byte (1 byte)."""
    return bytes([random.randint(0x20, 0x7E)])  # avoid control chars


def random_double_byte() -> bytes:
    """Return one random valid GB2312 two-byte character (2 bytes).
    Keep trying until decode('gb2312') succeeds.
    """
    while True:
        high = random.randint(0xA1, 0xFE)
        low = random.randint(0xA1, 0xFE)
        b = bytes([high, low])
        try:
            b.decode("gb2312")
            return b
        except UnicodeDecodeError:
            # try again
            continue


def make_uniform_block(uniform_type: str) -> bytes:
    """Create a 16-byte block that is uniformly single or uniformly double.
    uniform_type: 'single' or 'double'
    """
    if uniform_type == "single":
        return b"".join(random_single_byte() for _ in range(BLOCK_SIZE))
    elif uniform_type == "double":
        data = bytearray()
        # fill with double-byte chars as much as possible
        while len(data) + 2 <= BLOCK_SIZE:
            data.extend(random_double_byte())
        # if odd leftover byte, pad with a single byte
        while len(data) < BLOCK_SIZE:
            data.extend(random_single_byte())
        return bytes(data)
    else:
        raise ValueError("uniform_type must be 'single' or 'double'")


def make_mixed_block() -> bytes:
    """Create a 16-byte mixed block.
    Behavior:
      - Randomly choose whether to start with a double or a single.
      - If starts with double: place one double-byte at the beginning, then fill the rest with single bytes.
      - If starts with single: place one single byte first, then attempt to place one double-byte (if space),
        then fill the remainder with single bytes.
    This guarantees both types exist in the block while keeping remainder single bytes.
    """
    data = bytearray()
    start_with_double = random.random() < 0.5

    if start_with_double:
        # place one double-byte at start
        db = random_double_byte()
        if len(data) + 2 <= BLOCK_SIZE:
            data.extend(db)
        # fill the rest with single bytes
        while len(data) < BLOCK_SIZE:
            data.extend(random_single_byte())
    else:
        # start with one single byte
        data.extend(random_single_byte())
        # try to place one double-byte next (if there are at least 2 bytes left)
        if len(data) + 2 <= BLOCK_SIZE:
            data.extend(random_double_byte())
        # fill remaining bytes with single bytes
        while len(data) < BLOCK_SIZE:
            data.extend(random_single_byte())

    # ensure exact BLOCK_SIZE
    return bytes(data[:BLOCK_SIZE])


def make_block(p_uniform: float) -> bytes:
    """Generate one 16-byte block according to p_uniform.
    p_uniform: probability in [0,1] that the block is uniform (SIMD-friendly).
    If uniform: choose uniformly between all-single and all-double.
    If not uniform: produce a mixed block.
    """
    if random.random() < p_uniform:
        # uniform block: choose single or double uniformly (50/50)
        if random.random() < 0.5:
            return make_uniform_block("single")
        else:
            return make_uniform_block("double")
    else:
        # mixed block
        return make_mixed_block()


# --------------------------
# File generation
# --------------------------
def generate_file(p: float, out_path: Path):
    """Generate a single file of size TARGET_SIZE with given p (fraction [0,1])."""
    print(f"Generating {out_path} with p = {p:.4f} (uniform block probability)...")
    written = 0
    progress_report_interval = 5 * 1024 * 1024  # report every ~5MB
    next_report = progress_report_interval

    with open(out_path, "wb") as f:
        while written < TARGET_SIZE:
            block = make_block(p)
            f.write(block)
            written += len(block)
            if written >= next_report:
                mb = written / (1024 * 1024)
                print(f"  written {mb:.2f} MB")
                next_report += progress_report_interval

    final_mb = written / (1024 * 1024)
    print(f"Done. Final size: {final_mb:.2f} MB")


def main():
    for p in [0, 10, 20, 50, 70, 100]:
        out_file = OUTPUT_DIR / f"gb2312_{p}.txt"
        generate_file(p / 100, out_file)
        print(f"Output written to: {out_file}")


if __name__ == "__main__":
    random.seed()
    main()
