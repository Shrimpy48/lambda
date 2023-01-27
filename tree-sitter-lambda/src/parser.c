#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 25
#define LARGE_STATE_COUNT 8
#define SYMBOL_COUNT 21
#define ALIAS_COUNT 0
#define TOKEN_COUNT 13
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 4
#define MAX_ALIAS_SEQUENCE_LENGTH 4
#define PRODUCTION_ID_COUNT 3

enum {
  anon_sym_LPAREN = 1,
  anon_sym_RPAREN = 2,
  sym_identifier = 3,
  anon_sym_BSLASH = 4,
  anon_sym_ = 5,
  anon_sym_DOT = 6,
  anon_sym_SLASH_STAR = 7,
  aux_sym_block_comment_token1 = 8,
  anon_sym_STAR_SLASH = 9,
  anon_sym_SLASH_SLASH = 10,
  aux_sym_line_comment_token1 = 11,
  aux_sym_line_comment_token2 = 12,
  sym_source = 13,
  sym__term = 14,
  sym_application = 15,
  sym_abstraction = 16,
  sym_block_comment = 17,
  sym_line_comment = 18,
  aux_sym_block_comment_repeat1 = 19,
  aux_sym_line_comment_repeat1 = 20,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [sym_identifier] = "identifier",
  [anon_sym_BSLASH] = "\\",
  [anon_sym_] = "λ",
  [anon_sym_DOT] = ".",
  [anon_sym_SLASH_STAR] = "/*",
  [aux_sym_block_comment_token1] = "block_comment_token1",
  [anon_sym_STAR_SLASH] = "*/",
  [anon_sym_SLASH_SLASH] = "//",
  [aux_sym_line_comment_token1] = "line_comment_token1",
  [aux_sym_line_comment_token2] = "line_comment_token2",
  [sym_source] = "source",
  [sym__term] = "_term",
  [sym_application] = "application",
  [sym_abstraction] = "abstraction",
  [sym_block_comment] = "block_comment",
  [sym_line_comment] = "line_comment",
  [aux_sym_block_comment_repeat1] = "block_comment_repeat1",
  [aux_sym_line_comment_repeat1] = "line_comment_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_identifier] = sym_identifier,
  [anon_sym_BSLASH] = anon_sym_BSLASH,
  [anon_sym_] = anon_sym_,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_SLASH_STAR] = anon_sym_SLASH_STAR,
  [aux_sym_block_comment_token1] = aux_sym_block_comment_token1,
  [anon_sym_STAR_SLASH] = anon_sym_STAR_SLASH,
  [anon_sym_SLASH_SLASH] = anon_sym_SLASH_SLASH,
  [aux_sym_line_comment_token1] = aux_sym_line_comment_token1,
  [aux_sym_line_comment_token2] = aux_sym_line_comment_token2,
  [sym_source] = sym_source,
  [sym__term] = sym__term,
  [sym_application] = sym_application,
  [sym_abstraction] = sym_abstraction,
  [sym_block_comment] = sym_block_comment,
  [sym_line_comment] = sym_line_comment,
  [aux_sym_block_comment_repeat1] = aux_sym_block_comment_repeat1,
  [aux_sym_line_comment_repeat1] = aux_sym_line_comment_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_BSLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_STAR] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_block_comment_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_STAR_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_SLASH] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_line_comment_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_line_comment_token2] = {
    .visible = false,
    .named = false,
  },
  [sym_source] = {
    .visible = true,
    .named = true,
  },
  [sym__term] = {
    .visible = false,
    .named = true,
  },
  [sym_application] = {
    .visible = true,
    .named = true,
  },
  [sym_abstraction] = {
    .visible = true,
    .named = true,
  },
  [sym_block_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_line_comment] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_block_comment_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_line_comment_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_body = 1,
  field_bound = 2,
  field_lhs = 3,
  field_rhs = 4,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_body] = "body",
  [field_bound] = "bound",
  [field_lhs] = "lhs",
  [field_rhs] = "rhs",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_lhs, 0},
    {field_rhs, 1},
  [2] =
    {field_body, 3},
    {field_bound, 1},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(7);
      if (lookahead == '(') ADVANCE(8);
      if (lookahead == ')') ADVANCE(9);
      if (lookahead == '*') ADVANCE(6);
      if (lookahead == '.') ADVANCE(13);
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == '\\') ADVANCE(11);
      if (lookahead == 955) ADVANCE(12);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(10);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(23);
      if (lookahead == '\r') ADVANCE(2);
      if (lookahead == '/') ADVANCE(22);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(21);
      if (lookahead != 0) ADVANCE(21);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(23);
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(5)
      END_STATE();
    case 3:
      if (lookahead == '\n') SKIP(3)
      if (lookahead == '*') ADVANCE(18);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(17);
      if (lookahead != 0) ADVANCE(15);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(14);
      if (lookahead == '/') ADVANCE(20);
      END_STATE();
    case 5:
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(5)
      END_STATE();
    case 6:
      if (lookahead == '/') ADVANCE(19);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(10);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_BSLASH);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_SLASH_STAR);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      if (lookahead == '*') ADVANCE(14);
      if (lookahead == '/') ADVANCE(20);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      if (lookahead == '*') ADVANCE(18);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(17);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(15);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(aux_sym_block_comment_token1);
      if (lookahead == '/') ADVANCE(19);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_STAR_SLASH);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(aux_sym_line_comment_token1);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(aux_sym_line_comment_token1);
      if (lookahead == '*') ADVANCE(14);
      if (lookahead == '/') ADVANCE(20);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(aux_sym_line_comment_token2);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 1},
  [11] = {.lex_state = 3},
  [12] = {.lex_state = 1},
  [13] = {.lex_state = 3},
  [14] = {.lex_state = 1},
  [15] = {.lex_state = 3},
  [16] = {.lex_state = 3},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {(TSStateId)(-1)},
  [22] = {(TSStateId)(-1)},
  [23] = {(TSStateId)(-1)},
  [24] = {(TSStateId)(-1)},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [sym_block_comment] = STATE(0),
    [sym_line_comment] = STATE(0),
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_BSLASH] = ACTIONS(1),
    [anon_sym_] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_STAR_SLASH] = ACTIONS(1),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [1] = {
    [sym_source] = STATE(20),
    [sym__term] = STATE(4),
    [sym_application] = STATE(8),
    [sym_abstraction] = STATE(8),
    [sym_block_comment] = STATE(1),
    [sym_line_comment] = STATE(1),
    [anon_sym_LPAREN] = ACTIONS(7),
    [sym_identifier] = ACTIONS(9),
    [anon_sym_BSLASH] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [2] = {
    [sym__term] = STATE(3),
    [sym_application] = STATE(8),
    [sym_abstraction] = STATE(8),
    [sym_block_comment] = STATE(2),
    [sym_line_comment] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(13),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(13),
    [sym_identifier] = ACTIONS(9),
    [anon_sym_BSLASH] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [3] = {
    [sym__term] = STATE(3),
    [sym_application] = STATE(8),
    [sym_abstraction] = STATE(8),
    [sym_block_comment] = STATE(3),
    [sym_line_comment] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(15),
    [anon_sym_LPAREN] = ACTIONS(15),
    [anon_sym_RPAREN] = ACTIONS(15),
    [sym_identifier] = ACTIONS(15),
    [anon_sym_BSLASH] = ACTIONS(15),
    [anon_sym_] = ACTIONS(15),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [4] = {
    [sym__term] = STATE(3),
    [sym_application] = STATE(8),
    [sym_abstraction] = STATE(8),
    [sym_block_comment] = STATE(4),
    [sym_line_comment] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(17),
    [anon_sym_LPAREN] = ACTIONS(7),
    [sym_identifier] = ACTIONS(9),
    [anon_sym_BSLASH] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [5] = {
    [sym__term] = STATE(3),
    [sym_application] = STATE(8),
    [sym_abstraction] = STATE(8),
    [sym_block_comment] = STATE(5),
    [sym_line_comment] = STATE(5),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(19),
    [sym_identifier] = ACTIONS(9),
    [anon_sym_BSLASH] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [6] = {
    [sym__term] = STATE(5),
    [sym_application] = STATE(8),
    [sym_abstraction] = STATE(8),
    [sym_block_comment] = STATE(6),
    [sym_line_comment] = STATE(6),
    [anon_sym_LPAREN] = ACTIONS(7),
    [sym_identifier] = ACTIONS(9),
    [anon_sym_BSLASH] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
  [7] = {
    [sym__term] = STATE(2),
    [sym_application] = STATE(8),
    [sym_abstraction] = STATE(8),
    [sym_block_comment] = STATE(7),
    [sym_line_comment] = STATE(7),
    [anon_sym_LPAREN] = ACTIONS(7),
    [sym_identifier] = ACTIONS(9),
    [anon_sym_BSLASH] = ACTIONS(11),
    [anon_sym_] = ACTIONS(11),
    [anon_sym_SLASH_STAR] = ACTIONS(3),
    [anon_sym_SLASH_SLASH] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    STATE(8), 2,
      sym_block_comment,
      sym_line_comment,
    ACTIONS(21), 6,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_identifier,
      anon_sym_BSLASH,
      anon_sym_,
  [19] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    STATE(9), 2,
      sym_block_comment,
      sym_line_comment,
    ACTIONS(23), 6,
      ts_builtin_sym_end,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_identifier,
      anon_sym_BSLASH,
      anon_sym_,
  [38] = 5,
    ACTIONS(25), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(27), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(29), 1,
      aux_sym_line_comment_token1,
    ACTIONS(32), 1,
      aux_sym_line_comment_token2,
    STATE(10), 3,
      sym_block_comment,
      sym_line_comment,
      aux_sym_line_comment_repeat1,
  [56] = 6,
    ACTIONS(25), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(27), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(34), 1,
      aux_sym_block_comment_token1,
    ACTIONS(36), 1,
      anon_sym_STAR_SLASH,
    STATE(13), 1,
      aux_sym_block_comment_repeat1,
    STATE(11), 2,
      sym_block_comment,
      sym_line_comment,
  [76] = 6,
    ACTIONS(25), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(27), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(38), 1,
      aux_sym_line_comment_token1,
    ACTIONS(40), 1,
      aux_sym_line_comment_token2,
    STATE(14), 1,
      aux_sym_line_comment_repeat1,
    STATE(12), 2,
      sym_block_comment,
      sym_line_comment,
  [96] = 6,
    ACTIONS(25), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(27), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(34), 1,
      aux_sym_block_comment_token1,
    ACTIONS(42), 1,
      anon_sym_STAR_SLASH,
    STATE(15), 1,
      aux_sym_block_comment_repeat1,
    STATE(13), 2,
      sym_block_comment,
      sym_line_comment,
  [116] = 6,
    ACTIONS(25), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(27), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(38), 1,
      aux_sym_line_comment_token1,
    ACTIONS(44), 1,
      aux_sym_line_comment_token2,
    STATE(10), 1,
      aux_sym_line_comment_repeat1,
    STATE(14), 2,
      sym_block_comment,
      sym_line_comment,
  [136] = 5,
    ACTIONS(25), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(27), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(46), 1,
      aux_sym_block_comment_token1,
    ACTIONS(49), 1,
      anon_sym_STAR_SLASH,
    STATE(15), 3,
      sym_block_comment,
      sym_line_comment,
      aux_sym_block_comment_repeat1,
  [154] = 4,
    ACTIONS(25), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(27), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(51), 2,
      aux_sym_block_comment_token1,
      anon_sym_STAR_SLASH,
    STATE(16), 2,
      sym_block_comment,
      sym_line_comment,
  [169] = 5,
    ACTIONS(25), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(27), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(53), 1,
      aux_sym_line_comment_token1,
    ACTIONS(55), 1,
      aux_sym_line_comment_token2,
    STATE(17), 2,
      sym_block_comment,
      sym_line_comment,
  [186] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(57), 1,
      sym_identifier,
    STATE(18), 2,
      sym_block_comment,
      sym_line_comment,
  [200] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(59), 1,
      anon_sym_DOT,
    STATE(19), 2,
      sym_block_comment,
      sym_line_comment,
  [214] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_STAR,
    ACTIONS(5), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(61), 1,
      ts_builtin_sym_end,
    STATE(20), 2,
      sym_block_comment,
      sym_line_comment,
  [228] = 1,
    ACTIONS(63), 1,
      ts_builtin_sym_end,
  [232] = 1,
    ACTIONS(65), 1,
      ts_builtin_sym_end,
  [236] = 1,
    ACTIONS(67), 1,
      ts_builtin_sym_end,
  [240] = 1,
    ACTIONS(69), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(8)] = 0,
  [SMALL_STATE(9)] = 19,
  [SMALL_STATE(10)] = 38,
  [SMALL_STATE(11)] = 56,
  [SMALL_STATE(12)] = 76,
  [SMALL_STATE(13)] = 96,
  [SMALL_STATE(14)] = 116,
  [SMALL_STATE(15)] = 136,
  [SMALL_STATE(16)] = 154,
  [SMALL_STATE(17)] = 169,
  [SMALL_STATE(18)] = 186,
  [SMALL_STATE(19)] = 200,
  [SMALL_STATE(20)] = 214,
  [SMALL_STATE(21)] = 228,
  [SMALL_STATE(22)] = 232,
  [SMALL_STATE(23)] = 236,
  [SMALL_STATE(24)] = 240,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_abstraction, 4, .production_id = 2),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_application, 2, .production_id = 1),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 1),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__term, 1),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__term, 3),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [29] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_line_comment_repeat1, 2), SHIFT_REPEAT(17),
  [32] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_line_comment_repeat1, 2),
  [34] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [36] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [38] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [40] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [42] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [46] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_comment_repeat1, 2), SHIFT_REPEAT(16),
  [49] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_comment_repeat1, 2),
  [51] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_comment_repeat1, 1),
  [53] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_line_comment_repeat1, 1),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_line_comment_repeat1, 1),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [61] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line_comment, 3),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block_comment, 3),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line_comment, 2),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block_comment, 2),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_lambda(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
