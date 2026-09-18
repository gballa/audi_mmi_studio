#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

#define BLAKE3_OUT_LEN 32
#define BLAKE3_KEY_LEN 32
#define BLAKE3_BLOCK_LEN 64
#define BLAKE3_CHUNK_LEN 1024

enum blake3_flags {
    CHUNK_START         = 1 << 0,
    CHUNK_END           = 1 << 1,
    PARENT              = 1 << 2,
    ROOT                = 1 << 3,
    KEYED_HASH          = 1 << 4,
    DERIVE_KEY_CONTEXT  = 1 << 5,
    DERIVE_KEY_MATERIAL = 1 << 6,
};

static const uint32_t IV[8] = {
    0x6A09E667UL, 0xBB67AE85UL, 0x3C6EF372UL, 0xA54FF53AUL,
    0x510E527FUL, 0x9B05688CUL, 0x1F83D9ABUL, 0x5BE0CD19UL,
};

static const uint8_t MSG_PERMUTATION[16] = {
    2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8
};

static inline uint32_t rotr32(uint32_t w, uint32_t c) {
    return (w >> c) | (w << (32 - c));
}

static inline void g(uint32_t *state, size_t a, size_t b, size_t c, size_t d, uint32_t mx, uint32_t my) {
    state[a] = state[a] + state[b] + mx;
    state[d] = rotr32(state[d] ^ state[a], 16);
    state[c] = state[c] + state[d];
    state[b] = rotr32(state[b] ^ state[c], 12);
    state[a] = state[a] + state[b] + my;
    state[d] = rotr32(state[d] ^ state[a], 8);
    state[c] = state[c] + state[d];
    state[b] = rotr32(state[b] ^ state[c], 7);
}

static inline void round_fn(uint32_t state[16], const uint32_t msg[16]) {
    g(state, 0, 4, 8, 12, msg[0], msg[1]);
    g(state, 1, 5, 9, 13, msg[2], msg[3]);
    g(state, 2, 6, 10, 14, msg[4], msg[5]);
    g(state, 3, 7, 11, 15, msg[6], msg[7]);
    g(state, 0, 5, 10, 15, msg[8], msg[9]);
    g(state, 1, 6, 11, 12, msg[10], msg[11]);
    g(state, 2, 7, 8, 13, msg[12], msg[13]);
    g(state, 3, 4, 9, 14, msg[14], msg[15]);
}

static inline void compress(const uint32_t cv[8], const uint8_t block[64], uint32_t block_len, uint64_t counter, uint32_t flags, uint32_t out[16]) {
    uint32_t state[16] = {
        cv[0], cv[1], cv[2], cv[3],
        cv[4], cv[5], cv[6], cv[7],
        IV[0], IV[1], IV[2], IV[3],
        (uint32_t)counter, (uint32_t)(counter >> 32),
        block_len, flags
    };
    uint32_t msg[16];
    for (size_t i = 0; i < 16; i++) {
        msg[i] = (uint32_t)block[i * 4 + 0] |
                 ((uint32_t)block[i * 4 + 1] << 8) |
                 ((uint32_t)block[i * 4 + 2] << 16) |
                 ((uint32_t)block[i * 4 + 3] << 24);
    }
    for (size_t r = 0; r < 7; r++) {
        round_fn(state, msg);
        uint32_t perm[16];
        for (size_t i = 0; i < 16; i++) {
            perm[i] = msg[MSG_PERMUTATION[i]];
        }
        memcpy(msg, perm, sizeof(msg));
    }
    for (size_t i = 0; i < 8; i++) {
        out[i] = state[i] ^ state[i + 8];
        out[i + 8] = state[i + 8] ^ cv[i];
    }
}

typedef struct {
    uint32_t cv[8];
    uint64_t chunk_counter;
    uint8_t buf[64];
    uint8_t buf_len;
    uint8_t blocks_compressed;
    uint32_t flags;
} blake3_chunk_state;

static void chunk_state_init(blake3_chunk_state *self, const uint32_t key[8], uint64_t chunk_counter, uint32_t flags) {
    memcpy(self->cv, key, 32);
    self->chunk_counter = chunk_counter;
    memset(self->buf, 0, 64);
    self->buf_len = 0;
    self->blocks_compressed = 0;
    self->flags = flags;
}

static size_t chunk_state_len(const blake3_chunk_state *self) {
    return (size_t)self->blocks_compressed * BLAKE3_BLOCK_LEN + self->buf_len;
}

static uint32_t chunk_state_flags(const blake3_chunk_state *self) {
    uint32_t flags = self->flags;
    if (self->blocks_compressed == 0) {
        flags |= CHUNK_START;
    }
    if (self->blocks_compressed == 15 || self->buf_len == 0) {
        // Will be set when finalizing chunk
    }
    return flags;
}

static void chunk_state_update(blake3_chunk_state *self, const uint8_t *input, size_t input_len) {
    while (input_len > 0) {
        if (self->buf_len == 64) {
            uint32_t out[16];
            uint32_t flags = self->flags;
            if (self->blocks_compressed == 0) flags |= CHUNK_START;
            compress(self->cv, self->buf, 64, self->chunk_counter, flags, out);
            memcpy(self->cv, out, 32);
            self->blocks_compressed++;
            self->buf_len = 0;
            memset(self->buf, 0, 64);
        }
        size_t take = 64 - self->buf_len;
        if (take > input_len) take = input_len;
        memcpy(self->buf + self->buf_len, input, take);
        self->buf_len += (uint8_t)take;
        input += take;
        input_len -= take;
    }
}

typedef struct {
    uint32_t key[8];
    blake3_chunk_state chunk;
    uint8_t cv_stack[54 * 32];
    uint8_t cv_stack_len;
    uint32_t flags;
} blake3_hasher;

void blake3_hasher_init(blake3_hasher *self) {
    memcpy(self->key, IV, 32);
    self->flags = 0;
    chunk_state_init(&self->chunk, self->key, 0, self->flags);
    self->cv_stack_len = 0;
}

static void parent_cv(const uint8_t left_child_cv[32], const uint8_t right_child_cv[32], const uint32_t key[8], uint32_t flags, uint8_t out[32]) {
    uint8_t block[64];
    memcpy(block, left_child_cv, 32);
    memcpy(block + 32, right_child_cv, 32);
    uint32_t out16[16];
    compress(key, block, 64, 0, flags | PARENT, out16);
    for (size_t i = 0; i < 8; i++) {
        out[i * 4 + 0] = (uint8_t)(out16[i]);
        out[i * 4 + 1] = (uint8_t)(out16[i] >> 8);
        out[i * 4 + 2] = (uint8_t)(out16[i] >> 16);
        out[i * 4 + 3] = (uint8_t)(out16[i] >> 24);
    }
}

static void hasher_push_cv(blake3_hasher *self, uint8_t new_cv[32], uint64_t total_chunks) {
    while ((total_chunks & 1) == 0) {
        self->cv_stack_len--;
        uint8_t *top = &self->cv_stack[self->cv_stack_len * 32];
        uint8_t parent[32];
        parent_cv(top, new_cv, self->key, self->flags, parent);
        memcpy(new_cv, parent, 32);
        total_chunks >>= 1;
    }
    memcpy(&self->cv_stack[self->cv_stack_len * 32], new_cv, 32);
    self->cv_stack_len++;
}

void blake3_hasher_update(blake3_hasher *self, const uint8_t *input, size_t input_len) {
    while (input_len > 0) {
        if (chunk_state_len(&self->chunk) == BLAKE3_CHUNK_LEN) {
            uint32_t out[16];
            uint32_t flags = self->chunk.flags;
            if (self->chunk.blocks_compressed == 0) flags |= CHUNK_START;
            flags |= CHUNK_END;
            compress(self->chunk.cv, self->chunk.buf, self->chunk.buf_len, self->chunk.chunk_counter, flags, out);
            uint8_t chunk_cv[32];
            for (size_t i = 0; i < 8; i++) {
                chunk_cv[i * 4 + 0] = (uint8_t)(out[i]);
                chunk_cv[i * 4 + 1] = (uint8_t)(out[i] >> 8);
                chunk_cv[i * 4 + 2] = (uint8_t)(out[i] >> 16);
                chunk_cv[i * 4 + 3] = (uint8_t)(out[i] >> 24);
            }
            uint64_t total_chunks = self->chunk.chunk_counter + 1;
            hasher_push_cv(self, chunk_cv, total_chunks);
            chunk_state_init(&self->chunk, self->key, total_chunks, self->flags);
        }
        size_t want = BLAKE3_CHUNK_LEN - chunk_state_len(&self->chunk);
        size_t take = want < input_len ? want : input_len;
        chunk_state_update(&self->chunk, input, take);
        input += take;
        input_len -= take;
    }
}

void blake3_hasher_finalize(const blake3_hasher *self, uint8_t out[32]) {
    blake3_chunk_state chunk = self->chunk;
    uint32_t flags = chunk.flags;
    if (chunk.blocks_compressed == 0) flags |= CHUNK_START;
    flags |= CHUNK_END;

    if (self->cv_stack_len == 0) {
        // Root chunk
        flags |= ROOT;
        uint32_t out16[16];
        compress(chunk.cv, chunk.buf, chunk.buf_len, chunk.chunk_counter, flags, out16);
        for (size_t i = 0; i < 8; i++) {
            out[i * 4 + 0] = (uint8_t)(out16[i]);
            out[i * 4 + 1] = (uint8_t)(out16[i] >> 8);
            out[i * 4 + 2] = (uint8_t)(out16[i] >> 16);
            out[i * 4 + 3] = (uint8_t)(out16[i] >> 24);
        }
        return;
    }

    uint32_t out16[16];
    compress(chunk.cv, chunk.buf, chunk.buf_len, chunk.chunk_counter, flags, out16);
    uint8_t current_cv[32];
    for (size_t i = 0; i < 8; i++) {
        current_cv[i * 4 + 0] = (uint8_t)(out16[i]);
        current_cv[i * 4 + 1] = (uint8_t)(out16[i] >> 8);
        current_cv[i * 4 + 2] = (uint8_t)(out16[i] >> 16);
        current_cv[i * 4 + 3] = (uint8_t)(out16[i] >> 24);
    }

    uint8_t num_cvs = self->cv_stack_len;
    while (num_cvs > 0) {
        num_cvs--;
        const uint8_t *left = &self->cv_stack[num_cvs * 32];
        uint32_t parent_flags = self->flags | PARENT;
        if (num_cvs == 0) {
            parent_flags |= ROOT;
        }
        uint8_t block[64];
        memcpy(block, left, 32);
        memcpy(block + 32, current_cv, 32);
        compress(self->key, block, 64, 0, parent_flags, out16);
        for (size_t i = 0; i < 8; i++) {
            current_cv[i * 4 + 0] = (uint8_t)(out16[i]);
            current_cv[i * 4 + 1] = (uint8_t)(out16[i] >> 8);
            current_cv[i * 4 + 2] = (uint8_t)(out16[i] >> 16);
            current_cv[i * 4 + 3] = (uint8_t)(out16[i] >> 24);
        }
    }
    memcpy(out, current_cv, 32);
}

// C-accessible API for ctypes
void blake3_hash_buffer(const uint8_t *input, size_t len, uint8_t out[32]) {
    blake3_hasher h;
    blake3_hasher_init(&h);
    blake3_hasher_update(&h, input, len);
    blake3_hasher_finalize(&h, out);
}

blake3_hasher* blake3_new(void) {
    blake3_hasher *h = (blake3_hasher*)malloc(sizeof(blake3_hasher));
    if (h) {
        blake3_hasher_init(h);
    }
    return h;
}

void blake3_update(blake3_hasher *h, const uint8_t *data, size_t len) {
    if (h && data && len > 0) {
        blake3_hasher_update(h, data, len);
    }
}

void blake3_final(const blake3_hasher *h, uint8_t out[32]) {
    if (h && out) {
        blake3_hasher_finalize(h, out);
    }
}

void blake3_free(blake3_hasher *h) {
    if (h) {
        free(h);
    }
}

