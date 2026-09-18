"""
Pure Python 3.9 implementation of BLAKE3 cryptographic hash function.
Matches official BLAKE3 specification without external dependencies.
"""

import struct

IV = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
    0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
]

MSG_PERMUTATION = [
    2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8
]

CHUNK_START = 1 << 0
CHUNK_END = 1 << 1
PARENT = 1 << 2
ROOT = 1 << 3
KEYED_HASH = 1 << 4
DERIVE_KEY_CONTEXT = 1 << 5
DERIVE_KEY_MATERIAL = 1 << 6


def _rotr32(w: int, c: int) -> int:
    return ((w >> c) | (w << (32 - c))) & 0xFFFFFFFF


def _g(state: list, a: int, b: int, c: int, d: int, mx: int, my: int):
    state[a] = (state[a] + state[b] + mx) & 0xFFFFFFFF
    state[d] = _rotr32(state[d] ^ state[a], 16)
    state[c] = (state[c] + state[d]) & 0xFFFFFFFF
    state[b] = _rotr32(state[b] ^ state[c], 12)
    state[a] = (state[a] + state[b] + my) & 0xFFFFFFFF
    state[d] = _rotr32(state[d] ^ state[a], 8)
    state[c] = (state[c] + state[d]) & 0xFFFFFFFF
    state[b] = _rotr32(state[b] ^ state[c], 7)


def _round(state: list, msg: list):
    _g(state, 0, 4, 8, 12, msg[0], msg[1])
    _g(state, 1, 5, 9, 13, msg[2], msg[3])
    _g(state, 2, 6, 10, 14, msg[4], msg[5])
    _g(state, 3, 7, 11, 15, msg[6], msg[7])
    _g(state, 0, 5, 10, 15, msg[8], msg[9])
    _g(state, 1, 6, 11, 12, msg[10], msg[11])
    _g(state, 2, 7, 8, 13, msg[12], msg[13])
    _g(state, 3, 4, 9, 14, msg[14], msg[15])


def _compress(cv: list, block: bytes, block_len: int, counter: int, flags: int) -> list:
    state = [
        cv[0], cv[1], cv[2], cv[3],
        cv[4], cv[5], cv[6], cv[7],
        IV[0], IV[1], IV[2], IV[3],
        counter & 0xFFFFFFFF, (counter >> 32) & 0xFFFFFFFF,
        block_len, flags
    ]
    padded = block.ljust(64, b'\x00')
    msg = list(struct.unpack('<16I', padded[:64]))

    for _ in range(7):
        _round(state, msg)
        msg = [msg[MSG_PERMUTATION[i]] for i in range(16)]

    out = [0] * 16
    for i in range(8):
        out[i] = state[i] ^ state[i + 8]
        out[i + 8] = state[i + 8] ^ cv[i]
    return out


def _parent_cv(left_cv: bytes, right_cv: bytes, key: list, flags: int) -> bytes:
    block = left_cv + right_cv
    out16 = _compress(key, block, 64, 0, flags | PARENT)
    return struct.pack('<8I', *out16[:8])


class Blake3ChunkState:
    def __init__(self, key: list, chunk_counter: int, flags: int):
        self.cv = list(key)
        self.chunk_counter = chunk_counter
        self.buf = bytearray()
        self.blocks_compressed = 0
        self.flags = flags

    def len(self) -> int:
        return self.blocks_compressed * 64 + len(self.buf)

    def update(self, data: bytes):
        idx = 0
        total = len(data)
        while idx < total:
            if len(self.buf) == 64:
                flags = self.flags
                if self.blocks_compressed == 0:
                    flags |= CHUNK_START
                out = _compress(self.cv, bytes(self.buf), 64, self.chunk_counter, flags)
                self.cv = out[:8]
                self.blocks_compressed += 1
                self.buf.clear()
            take = min(64 - len(self.buf), total - idx)
            self.buf.extend(data[idx:idx + take])
            idx += take


class Blake3:
    def __init__(self, key: bytes = None):
        self.key = list(IV) if key is None else list(struct.unpack('<8I', key))
        self.flags = 0
        self.chunk = Blake3ChunkState(self.key, 0, self.flags)
        self.cv_stack = []

    def _push_cv(self, new_cv: bytes, total_chunks: int):
        while (total_chunks & 1) == 0:
            top = self.cv_stack.pop()
            new_cv = _parent_cv(top, new_cv, self.key, self.flags)
            total_chunks >>= 1
        self.cv_stack.append(new_cv)

    def update(self, data: bytes):
        if not data:
            return
        idx = 0
        total = len(data)
        while idx < total:
            if self.chunk.len() == 1024:
                flags = self.chunk.flags
                if self.chunk.blocks_compressed == 0:
                    flags |= CHUNK_START
                flags |= CHUNK_END
                out = _compress(self.chunk.cv, bytes(self.chunk.buf), len(self.chunk.buf), self.chunk.chunk_counter, flags)
                chunk_cv = struct.pack('<8I', *out[:8])
                total_chunks = self.chunk.chunk_counter + 1
                self._push_cv(chunk_cv, total_chunks)
                self.chunk = Blake3ChunkState(self.key, total_chunks, self.flags)

            want = 1024 - self.chunk.len()
            take = min(want, total - idx)
            self.chunk.update(data[idx:idx + take])
            idx += take

    def digest(self) -> bytes:
        chunk = self.chunk
        flags = chunk.flags
        if chunk.blocks_compressed == 0:
            flags |= CHUNK_START
        flags |= CHUNK_END

        if not self.cv_stack:
            flags |= ROOT
            out = _compress(chunk.cv, bytes(chunk.buf), len(chunk.buf), chunk.chunk_counter, flags)
            return struct.pack('<8I', *out[:8])

        out = _compress(chunk.cv, bytes(chunk.buf), len(chunk.buf), chunk.chunk_counter, flags)
        current_cv = struct.pack('<8I', *out[:8])

        stack = list(self.cv_stack)
        while stack:
            left = stack.pop()
            parent_flags = self.flags | PARENT
            if not stack:
                parent_flags |= ROOT
            out16 = _compress(self.key, left + current_cv, 64, 0, parent_flags)
            current_cv = struct.pack('<8I', *out16[:8])

        return current_cv

    def hexdigest(self) -> str:
        return self.digest().hex()


def blake3_hash(data: bytes) -> str:
    h = Blake3()
    h.update(data)
    return h.hexdigest()
